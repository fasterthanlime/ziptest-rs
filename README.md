# ziptest-rs

Try reading a zip with HTML5 Filesystem APIs & Rust + Wasm

## Rust part

crate in root of project directory, install wasm-pack then run
`wasm-pack build`.

## Frontend part

webpack project in `web/`, run `npm i` then `npx wp` (from `web/`, uses webpack-nano)
frontend, builds into `dist/`.

## Serve

I use `http-server` right now (the node.js package), serves `web/dist`

