extern crate actix_files;
extern crate actix_rt;
extern crate actix_web;
extern crate serde;
extern crate serde_derive;
extern crate toml;

#[macro_use]
extern crate lazy_static;

mod api;
mod config;

use actix_web::{web, get, App, HttpResponse, HttpServer};
use std::sync::Arc;

use config::Config;

const CONFIG_PATH: &'static str = "./Blog.toml";
const STATIC_PATH: &'static str = "./static";
const TEMPLATE: &'static str = include_str!("../templates/template.html");

lazy_static! {
    pub static ref CONFIG: Arc<Config> = Arc::new(Config::open(CONFIG_PATH).unwrap());
}

fn render(title: String, wasm_js: String) -> String {
    TEMPLATE
        .replace("{title}", title.as_str())
        .replace("{wasm_js}", wasm_js.as_str())
}

const INDEX_WASM_JS: &'static str = "bundle.js";

async fn index() -> HttpResponse {
    let config = CONFIG.clone();
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(render(
            config.general().blog_name(),
            INDEX_WASM_JS.to_string(),
        ))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let config = CONFIG.clone();
    HttpServer::new(|| {
        App::new()
            .service(api::info)
            .service(actix_files::Files::new("/static", STATIC_PATH))
            .default_service(web::get().to(index))
    })
    .bind(&format!(
        "{}:{}",
        config.general().address(),
        config.general().port()
    ))?
    .run()
    .await
}
