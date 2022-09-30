# Client Changelog

All notable changes to this project will be documented in this file.

The format is based on [CHANGELOG.md](http://changelog.md/)
and this project adheres to [Semantic Versioning](http://semver.org/).

<!-- 
TEMPLATE

## [major.minor.patch] - yyyy-mm-dd

A message that notes the main changes in the update.

### Added

### Changed

### Deprecated

### Fixed

### Removed

### Security

_______________________________________________________________________________
 
 -->

<!--
EXAMPLE

## [0.2.0] - 2021-06-02

Lorem Ipsum dolor sit amet.

### Added

- Cat pictures hidden in the library
- Added beeswax to the gears

### Changed

- Updated localisation files

-->


_______________________________________________________________________________

## [0.2.0] - 2022-09-30

Core structure created. Used a custom window frame from egui GitHub repo 
examples. Configured fonts and text styles. Added two large components,
side pane and chatroom. Side pane has all chatrooms listed and functionality
for searching from the list and selecting the chatroom currently in use. 
Chatroom has a scroll area for messages and a message input field. 

### Added

- Window frame
- Font and text style configuration
- Side pane component
    - All chatrooms in a list
    - Search from chatrooms
    - Select the chatroom currently in use
- Chatroom component
    - Scroll area to display messages
    - Input field to write messages

_______________________________________________________________________________

## [0.1.1] - 2022-09-14

Added GitHub Actions scripts for building cross-platform
executables, running unit and integration tests, linting,
and scheduled security audits. Marks pre-releases automatically.

The changelog is not copied to the release, that needs to be done manually.

### Added

- CI/CD Cross-platform builder
- CI/CD Unit test runner
- CI/CD Linter
- CI/CD Security audit

### Changed

- Updated localisation files

_______________________________________________________________________________

## [0.1.0] - 2022-09-08

This is the initial version of the project.

### Added

- The base project

<!-- markdownlint-configure-file {
    "MD022": false,
    "MD024": false,
    "MD030": false,
    "MD032": false
} -->
<!--
    MD022: Blanks around headings
    MD024: No duplicate headings
    MD030: Spaces after list markers
    MD032: Blanks around lists
-->
