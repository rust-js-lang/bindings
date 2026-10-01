# Writing a binding

A binding declares a JavaScript library's API in Rust, for rust-js. It is
written by hand, by a person or a coding agent. Nothing generates it.

## Before writing

- **Bind a library someone uses.** A PR is for its author's own use. One a
  coding agent wrote says so, and none is one of a batch for libraries nobody
  asked for.
- **Read the library, not only its types**: its `.d.ts` or `@types/*`, its
  documentation, and its source. A `.d.ts` leaves out what a binding needs:
  which options are really optional, and what can be `null`. The source says.

## The crate

- **Declarations only.** Every item is a `#[cfg_attr(rust_js, rust_js::link_name = "...")]`
  function or method whose body is `unreachable!()`, a type, or a struct of
  options. A function with a real body has no JS when an app installs the
  binding, and no build script or procedural macro is admitted.
- **The library's exports**: `"<module>#<export>"`, with the default export
  as `default`: `"canvas-confetti#default"`. A property path follows it:
  `"canvas-confetti#default.reset"`.
- **A JS object** is an opaque struct: `pub struct Cannon(PhantomData<JsObject>);`.
  A method on it takes `&self`. `"this()"` calls the object itself, when it's
  a function.
- **An options object** is a `#[derive(Default)]` struct of `Option` fields.
  `None` is `undefined`, which a library reads as "not given". A field whose
  JS name isn't its Rust name has `#[cfg_attr(rust_js, rust_js::name = "particleCount")]`.
- **A string union**, `"square" | "circle"`, is an enum, each variant with its
  `rust_js::name`.
- **A promise** is `js::Promise<T>`, and `Promise<()>` for one of nothing.
- **The web platform's types** are `webapi`'s. If one is missing, add it to
  rust-js's `webapi` rather than declare it here, so every binding shares it.

## The package

- `package.json`: `name` is `@rust-js-bindings/<library>`, `version` is the
  library's major and minor and a patch of the binding's own, and
  `"rust-js": { "crate": "rust-js-bindings-<library>" }` names the crate. The
  library, `@rust-js/builtins` and `@rust-js/webapi` are `peerDependencies`.
  `owners` are the people who review its changes.
- `Cargo.toml`: the crate, `rust-js-bindings-<library>`, at the same version.
  It names the crates it uses by version only, never by path.

## The example and its test

Each binding has an example in `examples/<library>/`: an app that installs it
as a user would, built by Vite in Cargo's way. Its test in `test/` runs it in
a browser, against the real library. Each fix adds to the example what the
fix is about, so it stays fixed.

Run the tests before a PR:

```bash
bun install
bun run test
```
