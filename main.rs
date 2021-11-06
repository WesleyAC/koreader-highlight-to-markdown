struct Bookmark {
    datetime: chrono::NaiveDateTime,
    chapter: String,
    notes: String,
}

fn main() {
    let args: Vec<String> = std::env::args().into_iter().collect();
    let contents = std::fs::read_to_string(args[1].clone()).unwrap();

    let lua = mlua::Lua::new();
    let result: mlua::Table = lua.load(&contents).eval().unwrap();

    let stats: mlua::Table = result.get("stats").unwrap();
    let title: String = stats.get("title").unwrap();
    let authors: String = stats.get("authors").unwrap();

    let mut bookmarks = vec![];

    let lua_bookmarks: mlua::Table = result.get("bookmarks").unwrap();
    for (_, bookmark) in lua_bookmarks
        .pairs::<mlua::Integer, mlua::Table>()
        .map(Result::unwrap)
    {
        if bookmark.get("highlighted").unwrap() {
            let datetime: mlua::String = bookmark.get("datetime").unwrap();
            let chapter: mlua::String = bookmark.get("chapter").unwrap();
            let notes: mlua::String = bookmark.get("notes").unwrap();

            bookmarks.push(Bookmark {
                datetime: chrono::NaiveDateTime::parse_from_str(
                    datetime.to_str().unwrap(),
                    "%Y-%m-%d %H:%M:%S",
                )
                .unwrap(),
                chapter: chapter.to_str().unwrap().to_string(),
                notes: notes.to_str().unwrap().to_string(),
            });
        }
    }

    bookmarks.sort_by(|a, b| a.datetime.partial_cmp(&b.datetime).unwrap());

    println!("# {} ({}) - highlights\n", title, authors);
    for bookmark in bookmarks {
        println!(
            "## {} @ {}\n\n> {}\n\n",
            bookmark.chapter, bookmark.datetime, bookmark.notes
        );
    }
}
