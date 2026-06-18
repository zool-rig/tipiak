![logo](resources/logo-128x128.png)

# Tipiak

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-stable-orange.svg)](https://www.rust-lang.org)
[![Rust](https://img.shields.io/badge/Framework-Dioxus-purple)](https://dioxuslabs.com/)

Tipiak is a self-hosted search engine for local files.

It is designed to run on your home server and provide easy access to the files stored there.

![demo-gif](/resources/demo.gif)

## Table of Contents

- [Features](#features)
    - [Crawler](#crawler)
    - [CLI](#cli)
    - [Web App](#web-app)
- [Stack](#stack)
- [Installation](#installation)
    - [Configuration](#configuration)
- [Project state](#project-state)
- [Contributing](#contributing)
- [License](#license)

## Features

### Crawler

Tipiak's search engine includes a crawler that tokenizes files.

This crawler supports multiple file types and metadata formats:

| Tokenizer | Description | Supported extensions |
| - | - | - |
| `FilePathTokenizer` | Extracts tokens from the file path and file name | `*` |
| `MarkdownTitleTokenizer` | Extracts tokens from Markdown headings found in text files | `.txt`,  `.md` |
| `ParagraphTokenizer` | Extracts tokens from the first 10 words of a text file | `.txt`,  `.md` |
| `ExifTokenizer` | Extracts tokens from EXIF metadata fields such as description, artist, and user comments | `.tiff`, `.jpeg`, `.jpg`, `.heif`, `.png`, `.webp` |
| `XmpTokenizer` | Extracts tokens from XMP metadata fields such as headline, title, description, subject, and creator | `.tiff`, `.jpeg`, `.jpg`, `.heif`, `.png`, `.webp`, `.mp4`, `.mov` |
| `IptcTokenizer` | Extracts tokens from IPTC metadata fields such as headline, keywords, caption | `.jpeg`, `.jpg` |
| `Id3Tokenizer` | Extracts tokens from ID3 metadata fields such as artist, title, album, genre | `.mp3` |
| `ZimTokenizer` | Extracts tokens from ZIM file metadata | `.zim` |

### CLI

Tipiak comes with a CLI to run the crawler on your file system.

It also has a `watch` command that uses [notify](https://docs.rs/notify/latest/notify/) to trigger the crawler whenever a new file is created.

See `tipiak-cli --help` for more details.

### Web App

Finally, the web client lets you search for and download your files from any machine on your network.

## Stack

* **Web framework :** [Dioxus](https://dioxuslabs.com/)
* **Database :** SQLite using fts5

## Installation

🚧 TODO 🚧

### Configuration

## Project state

## Contributing

## License

See [LICENSE](LICENSE).