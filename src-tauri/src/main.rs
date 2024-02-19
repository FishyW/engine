// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{error::Error, ffi::OsStr, fs, path::Path};
use tauri::{
    http::{status::StatusCode, Request, Response, Uri},
    AppHandle,
};

// type alias
type Result<T> = core::result::Result<T, Box<dyn Error>>;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello {}!", name)
}

const PROTOCOL: &str = "fetch";

fn http_error(e: &str) -> Result<Response> {
    println!("{}", e);
    tauri::http::ResponseBuilder::new()
        .header("Access-Control-Allow-Origin", "*")
        .status(StatusCode::BAD_REQUEST)
        .mimetype("text/plain")
        .body(e.into())
}

fn fetch_uri_handler(_app: &AppHandle, req: &Request) -> Result<Response> {
    let uri = req.uri().parse::<Uri>().unwrap();

    // percent decode the string since convertFileSrc() -> percent encodes the path
    // uri.path()[1..] -> removes the leading "/"
    let path = match urlencoding::decode(&uri.path()[1..]) {
        Ok(buf) => buf,
        Err(e) => return http_error(&e.to_string()),
    }
    .to_string();


    let path = Path::new(&path);

    // reads the file and stores it into a binary vector
    let buf = match fs::read(&path) {
        Ok(buf) => buf,
        Err(_) => {
            return http_error(&format!("Invalid Path: {:?}", &path));
        }
    };

    // first line transforms ../../foo/bar.js -> bar.js
    // second line transforms bar.js -> js
    let extension = path.extension()
        .unwrap_or(OsStr::new(""))
        .to_str()
        .unwrap_or("");

    // gets the mime type, if the mime type isn't properly set for wasm.
    // browser gives a warning
    let mime_type = match extension {
        "js" => "text/javascript",
        "wasm" => "application/wasm",
        _ => "text/plain",
    };

    // Cache-Control -> prevent import cache
    // Access-Control-Allow-Origin -> prevent CORS error
    tauri::http::ResponseBuilder::new()
        .header("Access-Control-Allow-Origin", "*")
        .header("Cache-Control", "no-cache")
        .header("Content-Length", buf.len())
        .mimetype(mime_type)
        .status(200)
        .body(buf.into())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .register_uri_scheme_protocol(PROTOCOL, fetch_uri_handler)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
