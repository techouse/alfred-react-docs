# React Docs Workflow for Alfred

![GitHub release](https://img.shields.io/github/release/techouse/alfred-react-docs.svg)
![GitHub All Releases](https://img.shields.io/github/downloads/techouse/alfred-react-docs/total.svg)
![GitHub](https://img.shields.io/github/license/techouse/alfred-react-docs.svg)


Search the [React documentation](https://reactjs.org/docs/getting-started.html) using [Alfred](https://www.alfredapp.com/).

![demo](demo.gif)

## Installation

1. [Download the latest version](https://github.com/techouse/alfred-react-docs/releases/latest)
2. Install the workflow by double-clicking the `.alfredworkflow` file
3. You can add the workflow to a category, then click "Import" to finish importing. You'll now see the workflow listed in the left sidebar of your Workflows preferences pane.

## Usage

Just type `rjs` followed by your search query.

```
rjs state
```

Either press `⌘Y` to Quick Look the result, or press `<enter>` to open it in your web browser.

### Note

The lightning fast search is powered by [Algolia](https://www.algolia.com) using the _same_ index as the official [React](https://react.dev/learn) website.

## Development

The workflow is implemented in Rust and requires Rust 1.88 or newer. Copy `.env.example` to `.env` and fill in the three Algolia search values, then run a local query with:

```sh
cargo run -- -q "state hooks"
```

Run the complete local check suite with `make ci`. To build the release directory or create an installable workflow for the current architecture, install `cargo-about` with `cargo install cargo-about --locked --features cli`, then run `make build-release` or `make package`. GitHub releases contain one universal binary supporting Apple Silicon from macOS 11 and Intel from macOS 10.15. Release builds embed the Algolia values from the environment or `.env`; runtime environment values continue to take precedence for local overrides. The `.env` file is never copied into a package.
