# `koreader-highlight-to-markdown`

quick 'n dirty script to convert [koreader](https://koreader.rocks/) highlights to a markdown file.

to compile/install:

```
$ cargo build --release
$ cp target/release/koreader-highlight-to-markdown ~/.local/bin/ # adjust to taste
```

to use:

```
$ koreader-highlight-to-markdown /path/to/book.sdr/metadata.epub.lua > /path/to/output.md
```

this is super janky and i've only really tested it on my exact setup. should be easy to edit, though. good luck!
