extern crate core;

use crate::components::app;

mod components;

fn main() {
    yew::start_app::<app::App>();
}