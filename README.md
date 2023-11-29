# chess

This repo includes:

* A simple chess engine written in Rust (in `core/`)
* A command-line interface (in `cli/`)
* A web UI using Svelte/TypeScript (in `web/`)
  * The web UI is hosted at https://https://chess-rust.pages.dev/
  * It uses a wasm build of the chess engine (in `wasm/`)
* A lichess bot (in `lichess/`)
  * The bot is hosted at https://lichess.org/@/rust-bot and is usually online
  * Feel free to open an issue if it's not working!
  * It accepts rated and unrated challenges from players and other bots

## Build instructions

### CLI

```sh
cd cli
cargo run
```

### Web UI

```sh
make web
cd web
npm run preview
```
