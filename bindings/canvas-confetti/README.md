# @rust-js-bindings/canvas-confetti

[canvas-confetti](https://github.com/catdad/canvas-confetti) 1.9, for
[rust-js](https://github.com/rust-js-lang/rust-js): confetti on a page, from Rust.

```bash
bun add canvas-confetti@~1.9 @rust-js-bindings/canvas-confetti@~1.9
```

```toml
# Cargo.toml
[dependencies]
canvas_confetti = { package = "rust-js-bindings-canvas-confetti", version = "~1.9" }
```

```rust
use canvas_confetti::{confetti, Options, Origin};

confetti(Options {
    particle_count: Some(100),
    spread: Some(70.0),
    origin: Some(Origin { y: Some(0.6), ..Default::default() }),
    ..Default::default()
});
```

Each option is `None` unless it's given, which canvas-confetti takes as its
default. `confetti` returns a `js::Promise<()>`, which resolves when the
confetti is all gone.

| canvas-confetti | Rust |
|---|---|
| `confetti(options)` | `confetti(Options { .. })` |
| `confetti.reset()` | `reset()` |
| `confetti.create(canvas, options)` | `create(canvas, GlobalOptions { .. })`, a `Cannon` |
| `myConfetti(options)`, `myConfetti.reset()` | `cannon.fire(Options { .. })`, `cannon.reset()` |
| `confetti.shapeFromPath(path)`, `confetti.shapeFromText(text)` | `shape_from_path(path)`, `shape_from_text(text)` |
| `"square"`, `"circle"`, `"star"` | `shape(ShapeName::Square)`, … |

Its version is canvas-confetti's major and minor, and a patch of its own:
1.9.x is for canvas-confetti 1.9.
