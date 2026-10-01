//! canvas-confetti, from Rust: a button that fires confetti, and says so
//! when it's all gone.

use canvas_confetti::{ShapeName, confetti, shape, Options, Origin};
use webapi::{document, element, event_target, node};

pub fn start() {
    let app = document::get_element_by_id(document, "app").expect("the page has an #app");
    let button = document::create_element(document, "button");
    node::set_text_content(button, "Celebrate");
    let status = document::create_element(document, "p");
    element::append(app, button);
    element::append(app, status);
    event_target::add_event_listener(
        button,
        "click",
        Box::new(move |_| {
            node::set_text_content(status, "firing");
            let fired = confetti(Options {
                particle_count: Some(150),
                spread: Some(70.0),
                origin: Some(Origin { y: Some(0.6), ..Default::default() }),
                shapes: Some(vec![shape(ShapeName::Star), shape(ShapeName::Circle)]),
                ..Default::default()
            });
            js::spawn(Box::new(async move {
                fired.await;
                node::set_text_content(status, "done");
            }));
        }),
    );
}
