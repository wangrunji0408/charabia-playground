# Charabia Tokenizer Playground

A web-based playground for [meilisearch/charabia](https://github.com/meilisearch/charabia), a multilingual tokenizer written in Rust. Type any text and see tokenization results in real time, with tokens color-coded by kind.

## Features

- Real-time tokenization as you type
- Color-coded tokens: **Word**, **StopWord**, **HardSeparator**, **SoftSeparator**, **Unknown**
- Hover over any token to see details (normalized lemma, script, language, position)
- Supported languages: Chinese (jieba segmentation), Hebrew, Thai, Greek, Khmer, Vietnamese, Swedish, Turkish, German, and all Latin-script languages

> Japanese and Korean are excluded because their segmenter (lindera) depends on `zstd-sys`, a C library that cannot compile to WebAssembly.

## How It Works

Charabia is compiled to WebAssembly via `wasm-pack`. A patched fork of [jieba-rs](https://github.com/messense/jieba-rs) is included under `deps/jieba-rs/` to replace `include-flate` (which depends on zstd) with plain `include_str!`, making Chinese segmentation WASM-compatible.

## Build Locally

Prerequisites: [Rust](https://rustup.rs/) and [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)

```sh
bash build.sh
python3 -m http.server -d web 8080
# Open http://localhost:8080
```

## Deploy to GitHub Pages

Push to `main` and GitHub Actions will build and deploy automatically. Make sure to enable **GitHub Actions** as the source in your repo's **Settings > Pages**.

## License

MIT
