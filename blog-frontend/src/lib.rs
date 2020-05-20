#![recursion_limit = "256"]

#[macro_use]
extern crate yew_router;

mod app;
mod navbar;
mod api;
mod errors;
mod route;

use app::Model;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[wasm_bindgen(start)]
pub fn run_app() {
    //wasm_logger::init(wasm_logger::Config::default());
    App::<Model>::new().mount_to_body();
}
