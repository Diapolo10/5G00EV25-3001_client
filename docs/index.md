# EguiValet Client - Documentation - Index <!-- omit in toc -->

This document contains the documentation for the EguiValet Client project.

## Table of Contents <!-- omit in toc -->

- [1. Introduction](#1-introduction)
- [2. General information](#2-general-information)
  - [2.1. Design](#21-design)
  - [2.2. Project requirements and Scrum](#22-project-requirements-and-scrum)
- [3. Installation](#3-installation)
- [4. Development](#4-development)
  - [4.1. Project file structure](#41-project-file-structure)
    - [4.1.1. Root directory](#411-root-directory)
    - [4.1.2. Docs directory](#412-docs-directory)
    - [4.1.3. Source directory](#413-source-directory)
    - [4.1.4. Target directory](#414-target-directory)
  - [4.2. Cargo docs](#42-cargo-docs)
- [5. Usage](#5-usage)

## 1. Introduction

This is the client half of the EguiValet chat application project, 
which features chatrooms where users can send and receive messages.

This document will outline the requirements for the project and 
instruct you on the development and how to use the app.

## 2. General information

The client is written in Rust and using the Egui framework, but it's still super bare-boned
and missing important functionality. We chose Rust because it's the most liked programming language
and we wanted to learn more about it. Egui was chosen simply because it seemed like it was easy to use,
even though it's still in development.

The [Git repository](https://github.com/Diapolo10/5G00EV25-3001_server) for the server half of the project.

### 2.1 Design

The original rough design for the UI can be found [here](https://www.figma.com/file/1pDmm10dDVeWY5wwZdLglZ/ChatAppDraft).

### 2.2 Project requirements and Scrum

The [GitHub project page](https://github.com/users/Diapolo10/projects/1) shows all the issues and sprints during the course. 
Most of the issues in the project page are described well, have linked pull requests and can be viewed as functional requirements.

## 3. Installation

> **Note**
> At the moment there is no up to date executable file, so you'll have to install from source

To install the client application, it is recommended to download the latest
packaged executable file from the project's
[GitHub releases section](https://github.com/Diapolo10/5G00EV25-3001_client/releases) as that simplifies the installation
process.

1. Download the latest executable file
2. Run the executable to launch the server

If one doesn't exist or it's outdated, you'll likely need to install from source.

1. Download the [project's Git repository](https://github.com/Diapolo10/5G00EV25-3001_client)

```shell
    # This will work on all platforms if you don't want to use the GUI
    cd ~
    curl -LO https://github.com/Diapolo10/5G00EV25-3001_client/archive/refs/heads/main.zip -o main.zip
```
2. Unpack the downloaded ZIP-file to a convenient location
3. For instructions on how to run the application go to [section 5](#5-usage)

## 4. Development

This part of the documentation will instruct you on how the project is
structured, and how to continue developing it.

### 4.1 Project file structure

This part of the documentation will attempt to provide an overview of the project's file structure.

```text
📂root
 ┣ 📂.github
 ┣ 📂docs
 ┣ 📦src
 ┣ 📂target
 ┣ 📜.gitattributes
 ┣ 📜.gitignore
 ┣ 📜Cargo.lock
 ┣ 📜Cargo.toml
 ┣ 📜CHANGELOG.md
 ┣ 📜CODE_OF_CONDUCT.md
 ┣ 📜CONTRIBUTING.md
 ┣ 📜LICENSE
 ┣ 📜README.md
```

### 4.1.1 Root directory

The repository houses several metadata files which are mostly either
information about the project itself, configuration files, or utility
files.

- `.github`
    This directory contains workflows and issue templates for the project.

- `.gitattributes`
    Tells Git how certain kinds of files should be treated when checking for
    differences, for instance.

- `.gitignore`
    Contains files and filename patterns that should be excluded from the Git
    repository.

- `Cargo.lock`
    Contains exact information about your dependencies.
    It is maintained by Cargo and should not be manually edited.

- `Cargo.toml`
    This is the *manifest* file for Rust's package manager, cargo.
    This file contains metadata such as name, version, and dependencies for packages,
    which are call "crates" in Rust.

- `CHANGELOG.md`
    This file documents all meaningful changes made to the project. It is
    updated by hand, so sometimes changes may slip by, but it nevertheless
    offers a goood overview regarding what changes have been made over time.

- `CODE_OF_CONDUCT.md`
    This file describes how people working on the project should
    conduct themselves. Or, in other words, it's a guide for decent behaviour.
    Commonly used in open-source projects.

- `CONTRIBUTING.md`
    Contains a rough overview on how to contribute to the project.

- `LICENSE`
    Contains the licensing information for the project, in this case a copy of
    the MIT license.

- `README.md`
    Contains a brief overview of the project, with links to the documentation,
    the GitHub project, and other general information.

### 4.1.2 Docs directory

The `docs` directory contains the documentation, which you are reading right now.

### 4.1.3 Source directory

The src directory contains the project's source code. The source code is well commented
and rust documentation is also in use so this document won't go into too much detail.

```text
📦src
 ┣ 📂components
 ┃ ┣ 📜chatroom.rs
 ┃ ┣ 📜loginpage.rs
 ┃ ┣ 📜mod.rs
 ┃ ┣ 📜side_pane.rs
 ┃ ┗ 📜window_frame.rs
 ┣ 📂fonts
 ┃ ┗ 📜Raleway-Regular.ttf
 ┣ 📂structs
 ┃ ┣ 📜app.rs
 ┃ ┣ 📜http_client.rs
 ┃ ┣ 📜message.rs
 ┃ ┣ 📜mod.rs
 ┃ ┣ 📜rooms.rs
 ┃ ┗ 📜user.rs
 ┣ 📂tests
 ┃ ┗ 📜integration_tests.rs
 ┣ 📜app.rs
 ┣ 📜lib.rs
 ┣ 📜main.rs

```

- `components`
    This directory contains all the components that are rendered on screen at different times.

    - `chatroom.rs` contains the chatroom where all the messages are shown. It takes up a majority of the screen.
    - `loginpage.rs` contains the login page that is shown to an unauthorized user. 
    It takes the entire screen and will disappear once user is logged in.
    - `mod.rs` used to export all content from the module, all other files are basically submodules.
    - `side_pane.rs` contains the user profile and all the available chatrooms. Takes up the left side of the screen.
    - `window_frame.rs` contains the window frame used instead of default OS window decorations. 

- `fonts`
    This directory contains alternative fonts to be used instead of the default one.

    - `Raleway-Regular.ttf` contains the raleway font.

- `structs`
    This directory contains all the structs used to handle data.

    - `app.rs` is the main struct of which the app is built around. It saves all data used on the client.
    - `http_client.rs` contains an instance of reqwest client and stores base url.
    - `message.rs` contains 3 structs (Message, ResMessage and Messages) used for receiving and posting messages
    as well as storing them in a vector in an instance of Messages.
    - `mod.rs` used to export all content from the module, all other files are basically submodules.
    - `room.rs` contains 2 structs (Room and Rooms). Used for receiving and posting chatrooms
    as well as storing them in a vector in an instance of Rooms.
    - `user.rs` used for creating and logging users in as well as storing non-sensitive information.

- `tests`
    This directory should contain tests but that turned out to be hard so time wasn't wasted here.

    - `integration_tests.rs` contains template for integration tests

- `app.rs`
    This file is where the app trait is implemented to the ChatApp struct

- `lib.rs`
    This file defines that this package contains a library crate. Contains exports of the files in the library.

- `main.rs`
    This file defines that this package contains a binary crate. Contains an entry point for the program.

### 4.1.4 Target directory

Cargo stores the output of a build into this directory. The dev and test profiles are stored in the debug directory,
and the release and bench profiles are stored in the release directory. Some commands like (`cargo doc`)
place their output in dedicated directories in the top level of the target directory.

- `debug`
    Contains output for the dev and test profiles.

- `doc`
    Contains rustdoc documentation.

### 4.2 Cargo docs

Rust generates documentation for the local package and all dependencies if wanted.
The output is placed in target/doc in rustdoc's usual format. The documentation can be opened in browser with command

```console
cargo doc --open --no-deps
```

I recommend running the command with the --no-deps option as long as you don't need documentation for the dependencies.

## 5. Usage

> **Note**
> You'll need the server half of the project running locally as well since it's not running on a server.

To run the client you'll need Rust and Cargo. If you are on Linux or macOS run command

```sh
curl https://sh.rustup.rs -sSf | sh
```

and if on Windows you need to go to the link below and download and run the rustup-init.exe file

```text
https://doc.rust-lang.org/cargo/getting-started/installation.html
```

After installation all you need to do is run (`cargo run`) from the root folder.
Dependencies will also be installed when running the command.
