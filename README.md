# solitaire

[![Build Status](https://travis-ci.org/solitaire/solitaire.svg?branch=master)](https://travis-ci.org/solitaire/solitaire)
[![Crate](https://meritbadge.herokuapp.com/solitaire)](https://crates.io/crates/solitaire)
[![Docs](https://docs.rs/solitaire/badge.svg)](https://docs.rs/solitaire)
[![Dependencies](https://deps.rs/repo/github/solitaire/solitaire/status.svg)](https://deps.rs/repo/github/solitaire/solitaire)

solitaire is a work in progress...

Running as a client application or shared server, `solitaire` has a focus on performance, correctness, and developer comfort.

See [installing.md](doc/installing.md) for installation guidance. After installing, run `solitaire -h` to get started.

See [scripts.md](doc/scripts.md) for available tools for building, running, and packaging the app.

## Crates

`solitaire` splits its code into several library crates:

- `solitaire-assets`: Contains embedded static files intended to be served from the web application
- `solitaire-client`: Run in the client's browser as a WebAssembly package, includes templates
- `solitaire-controllers`: Contains actix-web HTTP controllers, usually calling methods from `solitaire-service`
- `solitaire-core`: Contains definitions that are shared between server and client
- `solitaire-service`: Contains the primary logic for the application. It receives RequestMessages and emits ResponseMessages
- `solitaire-templates`: Contains Maud templates used by the server to render responses
- `solitaire`: Stored in the root of the project, this is the app's main library and binary

## Config

The project currently exclusively uses the filesystem for saved data, no database is involved.

### Directories

By default, solitaire stores config files in your system's user configuration directory.

- macOS: ~/Library/Application Support/solitaire
- Linux: ~/.config/solitaire
- Windows: %APPDATA%/solitaire/solitaire

### Files

`profile/*`: User profile information
