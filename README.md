![logo](resources/logo-128x128.png)

# Tipiak

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-stable-orange.svg)](https://www.rust-lang.org)
[![Web-Framework](https://img.shields.io/badge/Web_Framework-Dioxus-purple)](https://dioxuslabs.com/)

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
    - [Search Engine Configuration](#search-engine-configuration)
    - [Client Configuration](#client-configuration)
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

* **Web framework:** [Dioxus](https://dioxuslabs.com/)
* **Database:** SQLite using FTS5 for fuzzy search.

## Installation

1. Download the appropriate `tipiak-cli` executable for your server from [the latest release](https://github.com/zool-rig/tipiak/releases/latest). Then copy it there.

2. Run the crawler on a storage with `tipiak-cli crawl --path <YOUR PATH>`

>[!NOTE]
>You can set the `TIPIAK_SE_DB_OVERRIDE_PATH` environment variable to override where the database file will be created, by default it's in the storage directory`

3. If you want the crawler to be triggered automatically when a file is created in your storage, you can start a shell and run the `tipiak-cli watch` command.

    Or setup a systemctl service (assuming your server runs on linux) :

    a. Create a `tipiak-watch.service` file in `/etc/systemd/system`
    
    b. Paste and edit the following content in the file :
    
    ```ini
    [Unit]
    Description=Watch a directory and run the Tipiak search engine crawler
    After=network.target

    [Service]
    Type=simple
    User=<YOUR USER>
    WorkingDirectory=<YOUR STORAGE DIRECTORY>
    <!-- Environment="TIPIAK_SE_CONFIG_PATH=<PATH TO YOUR CONFIG>" -->
    <!-- OR -->
    <!-- Environment="TIPIAK_SE_DB_OVERRIDE_PATH=<PATH TO YOUR OVERRIDE>" -->
    ExecStart=<ABS PATH OF YOUR BIN>/tipiak-cli watch -p <YOUR STORAGE DIRECTORY>

    Restart=always
    RestartSec=5

    [Install]
    WantedBy=multi-user.target
    ```

    c. Run the following commands : 

    ```bash
    sudo systemctl daemon-reload
    sudo systemctl start tipiak-watch.service
    sudo systemctl status tipiak-watch.service
    ```

    The watcher should be running now, try adding some files in your storage and then rerun `sudo systemctl status tipiak-watch.service` to see changed logs

4. [Install Docker](https://docs.docker.com/get-started/) if you don't have it already on your server.

5. Copy the `docker-compose.yml` file from this repository somewhere in your server.

6. `cd` where you copied the file then run : 

    ```bash
    docker compose pull
    docker compose up -d
    ```

    Your done ! Tipiak should be served on your local network !

### Search Engine Configuration

You can set the `TIPIAK_SE_DB_OVERRIDE_PATH` environment variable to specify the directory where the database file is created.

Or

Create a `tipiak_se.toml` file in your working directory, or at a location referenced by the `TIPIAK_SE_CONFIG_PATH` environment variable.

Contents:

| Key | Type | Description | Default |
| - | - | - | - |
| file_types | `HashMap<String, Vec<String>>` | Map of file extensions grouped by category/type | [Default config here](/docs/default_file_types.md) |
| db_override_path | `Option<String>` | Path where the SQLite database file should be saved. By default, it is saved at the root of the storage directory | None |

### Client Configuration

You can set the `TIPIAK_APP_STORAGE_DIR` environment variable to specify where is the storage directory for an instance of the app.

Or

Create a `tipiak_app.toml` file in your working directory, or at a location referenced by the `TIPIAK_APP_CONFIG_PATH` environment variable.

Contents:

| Key | Type | Description | Default |
| - | - | - | - |
| storage_dir | `String` | Path to the storage directory to index and search. This lets you run multiple Tipiak instances against different storage roots | Required |

## Project state

* The project already supports a wide range of file types, and more can be added over time.

* The web client's UI/UX can still be improved.

* For now the docker image is only for an armv7 32bit architecture (the raspberry pi that I use).

* The configuration system should be smoother, by default the database file is created inside the storage for this reason the watcher is triggered every time a something is inserted into the database. The database override path configuration makes it possible to bypass this performance issue but it's not very clean.

## Contributing

Contributions are welcome!

To set up your local development environment, follow these steps:

1. Clone this repository
2. Install the [Dioxus CLI](https://dioxuslabs.com/learn/0.7/getting_started/)
3. Follow [Search Engine Configuration](#search-engine-configuration) and [Client Configuration](#client-configuration) to set up your local storage directory.
4. Run the crawler with the CLI:

    ```bash
    cargo run --bin tipiak-cli -- crawl --path <YOUR_STORAGE_PATH>
    ```
5. Run the client:

    ```bash
    dx serve --package tipiak-app
    ```


## License

See [LICENSE](LICENSE).