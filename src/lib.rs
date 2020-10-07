mod cleanup;
mod types;
mod views;
use crate::{types::view::View, views::container::Container};
use moonlight_macros::moonlight_main;

#[moonlight_main]
fn run() -> Box<dyn View> {
    Box::new(Container {
        name: "hello".to_string(),
        tag: 0,
        children: vec![Box::new(Container {
            name: "there".to_string(),
            tag: 1,
            children: vec![],
        })],
    })
}
