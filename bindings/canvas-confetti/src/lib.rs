//! [canvas-confetti](https://github.com/catdad/canvas-confetti) 1.9, for
//! rust-js: confetti on a page, from Rust.
//!
//! ```rust,ignore
//! use canvas_confetti::{confetti, Options, Origin};
//!
//! confetti(Options {
//!     particle_count: Some(100),
//!     spread: Some(70.0),
//!     origin: Some(Origin { y: Some(0.6), ..Default::default() }),
//!     ..Default::default()
//! });
//! ```
//!
//! Each option is `None` unless it's given, which canvas-confetti takes as
//! its default.

#![allow(unused_variables)]

use core::marker::PhantomData;

use js::{JsObject, Promise};
use webapi::HtmlCanvasElement;

/// Fire confetti on a canvas of its own, over the page, as `confetti(options)`.
/// It resolves when the confetti is all gone.
#[cfg_attr(rust_js, rust_js::link_name = "canvas-confetti#default")]
pub fn confetti(options: Options) -> Promise<()> {
    unreachable!()
}

/// Stop all the confetti, and clear it from the page.
#[cfg_attr(rust_js, rust_js::link_name = "canvas-confetti#default.reset")]
pub fn reset() {
    unreachable!()
}

/// A confetti cannon for a `<canvas>` of the page's own, as `confetti.create`.
#[cfg_attr(rust_js, rust_js::link_name = "canvas-confetti#default.create")]
pub fn create(canvas: &HtmlCanvasElement, options: GlobalOptions) -> Cannon {
    unreachable!()
}

/// A shape from an SVG path's data, as `confetti.shapeFromPath`.
#[cfg_attr(rust_js, rust_js::link_name = "canvas-confetti#default.shapeFromPath")]
pub fn shape_from_path(path: &str) -> &'static Shape {
    unreachable!()
}

/// A shape from text, an emoji say, as `confetti.shapeFromText`.
#[cfg_attr(rust_js, rust_js::link_name = "canvas-confetti#default.shapeFromText")]
pub fn shape_from_text(text: &str) -> &'static Shape {
    unreachable!()
}

/// One of the shapes canvas-confetti draws itself.
#[cfg_attr(rust_js, rust_js::link_name = "this")]
pub fn shape(this: ShapeName) -> &'static Shape {
    unreachable!()
}

/// A confetti shape: one of canvas-confetti's own, [`shape`], or one made
/// by [`shape_from_path`] or [`shape_from_text`].
pub struct Shape(PhantomData<JsObject>);

/// The shapes canvas-confetti draws itself.
pub enum ShapeName {
    #[cfg_attr(rust_js, rust_js::name = "square")]
    Square,
    #[cfg_attr(rust_js, rust_js::name = "circle")]
    Circle,
    #[cfg_attr(rust_js, rust_js::name = "star")]
    Star,
}

/// What [`create`] gives: confetti, on its canvas.
#[derive(Clone, Copy)]
pub struct Cannon(PhantomData<JsObject>);

impl Cannon {
    /// Fire confetti on the cannon's canvas, as [`confetti`] does on its own.
    #[cfg_attr(rust_js, rust_js::link_name = "this()")]
    pub fn fire(&self, options: Options) -> Promise<()> {
        unreachable!()
    }

    /// Stop the cannon's confetti, and clear its canvas.
    #[cfg_attr(rust_js, rust_js::link_name = "reset")]
    pub fn reset(&self) {
        unreachable!()
    }
}

/// How confetti is fired: each `None` is canvas-confetti's default.
#[derive(Default)]
pub struct Options {
    /// How many confetti: 50.
    #[cfg_attr(rust_js, rust_js::name = "particleCount")]
    pub particle_count: Option<u32>,
    /// The direction, in degrees, 90 straight up: 90.
    pub angle: Option<f64>,
    /// How far from the direction they spread, in degrees: 45.
    pub spread: Option<f64>,
    /// How fast they start, in pixels: 45.
    #[cfg_attr(rust_js, rust_js::name = "startVelocity")]
    pub start_velocity: Option<f64>,
    /// How fast they slow down, from 0 to 1: 0.9.
    pub decay: Option<f64>,
    /// How fast they fall: 1, 0.5 half as fast.
    pub gravity: Option<f64>,
    /// How far they drift sideways, left below 0: 0.
    pub drift: Option<f64>,
    /// Without tilting and wobbling: false.
    pub flat: Option<bool>,
    /// How many frames they last: 200.
    pub ticks: Option<u32>,
    /// Where they start, from 0 to 1 across the page and down it: the middle.
    pub origin: Option<Origin>,
    /// Their colors, as hex strings, `"#bb0000"`.
    pub colors: Option<Vec<String>>,
    /// Their shapes: squares and circles.
    pub shapes: Option<Vec<&'static Shape>>,
    /// How big they are: 1.
    pub scalar: Option<f64>,
    /// The canvas's z-index: 100.
    #[cfg_attr(rust_js, rust_js::name = "zIndex")]
    pub z_index: Option<i32>,
    /// None at all for a user who asks for less motion: false.
    #[cfg_attr(rust_js, rust_js::name = "disableForReducedMotion")]
    pub disable_for_reduced_motion: Option<bool>,
}

/// Where confetti starts: `x` across the page, `y` down it, each from 0 to 1.
#[derive(Default)]
pub struct Origin {
    pub x: Option<f64>,
    pub y: Option<f64>,
}

/// How a [`create`]d cannon fires, for each time it does.
#[derive(Default)]
pub struct GlobalOptions {
    /// The canvas sized to the page as it's resized: false.
    pub resize: Option<bool>,
    /// Drawn in a web worker, off the page's thread, where it can be: false.
    #[cfg_attr(rust_js, rust_js::name = "useWorker")]
    pub use_worker: Option<bool>,
    /// None at all for a user who asks for less motion: false.
    #[cfg_attr(rust_js, rust_js::name = "disableForReducedMotion")]
    pub disable_for_reduced_motion: Option<bool>,
}
