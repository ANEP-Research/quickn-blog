use std::fs;
use std::process::Command;

fn main() {
    println!("Building blog-frontend...");
    Command::new("wasm-pack")
        .args(&["build", "--target"])
        .arg("web")
        .current_dir("./blog-frontend")
        .status()
        .unwrap();
    Command::new("rollup")
        .args(&["./build.js", "--format"])
        .arg("iife")
        .args(&["--file"])
        .arg("./pkg/bundle.js")
        .current_dir("./blog-frontend")
        .status()
        .unwrap();
    fs::copy("./blog-frontend/pkg/bundle.js", "./static/js/bundle.js")
        .expect("Failed to copy bundle.js");
    fs::copy(
        "./blog-frontend/pkg/blog_frontend_bg.wasm",
        "./static/wasm/blog_frontend_bg.wasm",
    )
    .expect("Failed to copy blog_frontend_bg.wasm");
}
