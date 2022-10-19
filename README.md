# AsciiFire

[![Github][github-badge]][github-url]
[![Crates.io][crates-badge]][crates-url]
[![MIT licensed][mit-badge]][mit-url]

[crates-badge]: https://img.shields.io/badge/crates.io-v0.1.0-blue
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[github-badge]: https://img.shields.io/badge/github-aaronknudtson/asciifire-brightgreen
[github-url]: https://github.com/aaronknudtson/asciifire
[crates-url]: https://crates.io/crates/asciifire
[mit-url]: https://en.wikipedia.org/wiki/MIT_License

A Library and CLI for converting images into ASCII characters of varying "density".

## Install
Make sure you have cargo installed. [Help can be found here.](https://doc.rust-lang.org/cargo/getting-started/installation.html)

Once cargo is installed, open a terminal and run
```rust
cargo install asciifire
```

This will install a binary and allow you to use asciifire in your preferred terminal.

## Examples

Output to console at appropriate terminal size:
```bash
$ asciifire Savior.jpeg
```
![Ascii Savior Big](https://github.com/aaronknudtson/asciifire/raw/fe8547d9f4e7d46e6aef40b2b2ea22a92cde40ec/images/SaviorBig.png)

Output result to file:
```bash
$ asciifire Savior.jpeg -o Savior.txt
```

Resize height and maintain aspect ratio:
```bash
$ asciifire Savior.jpeg --height 50
```

![Ascii Savior 50](https://github.com/aaronknudtson/asciifire/raw/fe8547d9f4e7d46e6aef40b2b2ea22a92cde40ec/images/Savior50.png)
