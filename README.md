# chess

This repo includes:

* A simple chess engine written in Rust (in `core/`)
* A command-line interface (in `cli/`)
* A web UI using Svelte/TypeScript (in `web/`)
  * The web UI is hosted at https://https://chess-rust.pages.dev/
  * It uses a wasm build of the chess engine (in `wasm/`)

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
