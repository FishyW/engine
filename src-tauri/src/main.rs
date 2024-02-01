// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{http::{status::StatusCode, Request, Response}, AppHandle};
use std::{error::Error, fs};

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

    // percent decode the string since convertFileSrc() -> percent encodes the path
    let url = match urlencoding::decode(req.uri()) {
        Ok(buf) => buf,
        Err(e) => {
          return http_error(&e.to_string())
        }
    }.to_string();

    // gets the path from -> http://localhost/<path>
    // can't use url package since ../../ isn't a valid url
    let path = url.split("/")
      .skip(3).collect::<Vec<&str>>().join("/");

    // reads the file and stores it into a binary vector
    let buf = match fs::read(&path) {
      Ok(buf) => buf,
      Err(_) => {
        return http_error(&path);
      }
    };


    // first line transforms ../../foo/bar.js -> bar.js
    // second line transforms bar.js -> js
    let extension = path.split('/').next_back()
      .unwrap_or("text/plain")
      .split('.').next_back()
      .unwrap_or("text/plain");

    // gets the mime type, if the mime type isn't properly set for wasm.
    // browser gives a warning
    let mime_type = match extension {
      "js" => "text/javascript",
      "wasm" => "application/wasm",
      _ => "text/plain"
    };

    // Cache-Control -> prevent import cache
    // Access-Control-Allow-Origin -> prevent CORS error
    tauri::http::ResponseBuilder::new()
      .header("Access-Control-Allow-Origin", "*")
      .header("Cache-Control", "no-cache")
      .mimetype(mime_type)
      .status(200)
      .body(buf.into())
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![greet])
    .register_uri_scheme_protocol(PROTOCOL,  fetch_uri_handler)
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
