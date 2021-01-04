use std::fs;
use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let path = env::current_dir().unwrap();
    println!("Building blog-frontend...");
    fs::remove_dir(path.join("pkg"));
    Command::new("wasm-pack")
        .args(&["build", "--target"])
        .arg("web")
        .current_dir("./blog-frontend")
        .output()
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
