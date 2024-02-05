/*

NOTE: I DIDNT WRITE THIS CODE, THIS CODE IS FROM AUTOMOD, I JUST ADAPTED IT TO FIT MY PURPOSE
- THIS CRATE NEEDS REWRITING

proc_macro source code + little bit of modification (rewrite this later)
syn also has .to_compile_error -> can be used to return errors (for production ready code)
i also changed a bunch of "?" into unwrap()
*/



use quote::quote;
use proc_macro2::{Ident, Span, TokenStream as TokenStream2};

use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use syn::parse::{Parse, ParseStream};
use syn::{ LitStr, Visibility};

pub struct Arg {
    pub vis: Visibility,
    pub path: LitStr,
}

impl Parse for Arg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Arg {
            vis: input.parse()?,
            path: input.parse()?,
        })
    }
}



pub fn mod_item(name: String) -> TokenStream2 {
    let mut module_name = name.replace('-', "_");
    if module_name.starts_with(|ch: char| ch.is_ascii_digit()) {
        module_name.insert(0, '_');
    }

    let path = Option::into_iter(if name == module_name {
        None
    } else {
        Some(format!("{}.rs", name))
    });

    let ident = Ident::new(&module_name, Span::call_site());

    quote! {
        #(#[path = #path])*
        pub mod #ident; 
        pub use #ident::*;
    }
}

pub fn source_file_names<P: AsRef<Path>>(dir: P) -> Vec<String> {
    let mut names = Vec::new();
    let mut failures = Vec::new();

    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        if !entry.file_type().unwrap().is_file() {
            continue;
        }

        let file_name = entry.file_name();
        if file_name == "mod.rs" || file_name == "lib.rs" || file_name == "main.rs" {
            continue;
        }

        let path = Path::new(&file_name);
        if path.extension() == Some(OsStr::new("rs")) {
            match file_name.into_string() {
                Ok(mut utf8) => {
                    utf8.truncate(utf8.len() - ".rs".len());
                    names.push(utf8);
                }
                Err(non_utf8) => {
                    failures.push(non_utf8);
                }
            }
        }
    }

    failures.sort();
    if let Some(failure) = failures.into_iter().next() {
        panic!("UTF8 Error");
    }

    if names.is_empty() {
        panic!("Empty");
    }

    names.sort();
    names
}
