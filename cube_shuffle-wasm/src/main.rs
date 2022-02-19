extern crate core;

use std::panic;

use crate::components::app;

mod components;

fn main() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    yew::start_app::<app::App>();
}
