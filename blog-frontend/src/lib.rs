#![recursion_limit = "10000000"]

#[macro_use]
extern crate log;
#[macro_use]
extern crate yew_router;
extern crate mailchecker;
extern crate ybc;
extern crate yewtil;
extern crate web_sys;

mod services;
mod api;
mod app;
mod constants;
mod errors;
mod footer;
mod navbar;
mod pages;
mod route;

use app::Model;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[wasm_bindgen(start)]
pub fn run_app() {
    wasm_logger::init(wasm_logger::Config::default());
    App::<Model>::new().mount_to_body();
}
