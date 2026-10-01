# rust-js bindings

Bindings to JavaScript libraries for [rust-js](https://github.com/rust-js-lang/rust-js),
which compiles Rust to readable JavaScript. Each binding is a Rust crate that
declares a library's API, in an npm package, `@rust-js-bindings/<library>`,
installed beside the library it binds.

| Binding | Library | npm |
|---|---|---|
| [canvas-confetti](bindings/canvas-confetti) | [canvas-confetti](https://github.com/catdad/canvas-confetti) 1.9 | `@rust-js-bindings/canvas-confetti` |

## Using a binding

Install the library and its binding with the same major and minor version:

```bash
bun add canvas-confetti@~1.9 @rust-js-bindings/canvas-confetti@~1.9
```

Then name the binding's crate in the app's `Cargo.toml`, by version:

```toml
[dependencies]
canvas_confetti = { package = "rust-js-bindings-canvas-confetti", version = "~1.9" }
```

`rust-js-patch`, from `@rust-js/build`, tells Cargo where each installed
binding is, in the app's `.cargo/config.toml`. Run it as the app's
`postinstall`. The rust-js Vite plugin runs it too, as it starts.

## How a binding is versioned

A binding's version is its library's major and minor, plus a patch of its
own: `@rust-js-bindings/canvas-confetti@1.9.x` is for canvas-confetti 1.9. A
fix to a binding is its next patch. What it shares with the app, the library,
`@rust-js/builtins` and `@rust-js/webapi`, are peer dependencies, so the app
has one copy of each.

## Layout

```
bindings/<library>/   the binding: package.json, Cargo.toml, README.md, src/lib.rs
examples/<library>/   an app that uses it, built and run by the tests
test/                 the tests: each example, built by Vite, run in a browser
scripts/              install-examples.ts, which installs each example before the tests
```

## Contributing

Each binding is for a library someone uses. A binding is a claim about another
project's API, and a wrong one compiles, then fails in the browser, so each
binding has an example, which the tests run against the real library. See
[AGENTS.md](AGENTS.md) for how a binding is written.

```bash
bun install
bun run test
```
