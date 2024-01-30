// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::de::IntoDeserializer;
use tauri::{http::{header::{CONTENT_LENGTH, CONTENT_TYPE}, Request, Response, ResponseBuilder}, AppHandle, Wry};
use url::Url;
use std::{fs::{self, File}, io::Read};

#[tauri::command]
fn greet(name: &str) -> String {
  format!("Hello {}!", name)
}


fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![greet])
    .register_uri_scheme_protocol("test",  |_app, req| {

    // decode the URL
      let buf = fs::read(req.uri()
        .strip_prefix("test:/")
        .unwrap())
        .unwrap();      

        println!("{}", String::from_utf8(buf.clone()).unwrap());

      tauri::http::ResponseBuilder::new()
        .header("Access-Control-Allow-Origin", "*")
        .mimetype("text/javascript")
        .status(200)
        .body(buf)
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
