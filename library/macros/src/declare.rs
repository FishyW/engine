// source code for the declare macro

use std::{  env, io, path::{Path, PathBuf}};

use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use syn::{
    parse::{Parse, ParseStream}, spanned::Spanned, token::Mod, LitStr, Token
};

use thiserror::Error;
use crate::rules::{ident, next, parse, unwrap};

struct DeclareArgs {
    // we don't store as a boolean, so we can get the span of the token
    vis: Option<Token![priv]>,
    // LitStr, string literal token
    path: LitStr,
}

impl Parse for DeclareArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(DeclareArgs {
            vis: next!(input, Token![priv]),
            path: input.parse()?,
        })
    }
}

// gets a vector of all the filenames inside of the directory
fn read_directory(dir_path: PathBuf) -> Result<Vec<PathBuf>, io::Error> {
    let entries = dir_path.read_dir()?;

    // error propagation unfortunately is best done using a loop
    // cant use iterators
    let mut filenames = vec![];
    for entry in entries {
        let entry = entry?;
        let file_type = entry.file_type()?;
        if !file_type.is_file() {
            continue;
        }

        filenames.push(entry.file_name().into());
    }
        
    Ok(filenames)
}


// for now I'm using an Error helper macro,
// eventually #[from] io::Error should be properly implemented
#[derive(Debug, Error)]
enum ModuleError {
    #[error("File contains an invalid module name. Rename your file to something else.")]
    NameError,
    #[error(transparent)]
    IOError(#[from] io::Error),
    #[error(transparent)]
    VarError(#[from] env::VarError)
}

// reads a directory and returns all Rust modules
fn find_rs_modules(dir_path: &str) -> Result<Vec<String>, ModuleError> {
    let project_path = env::var("CARGO_MANIFEST_DIR")?;
    let path = PathBuf::from(project_path).join(dir_path);
    let filenames = read_directory(path)?;

    // filter_map filters the element when it is None
    // the first map checks if the extension is correct
    // the second map converts the pathbuf to a string
    let filenames: Vec<_> = filenames.into_iter()
        .filter_map(|file| {
            let ext = file.extension()?;
            if ext == "rs" { Some(file) } else { None }
        })
        .filter_map(|file| 
            Some(file.file_stem()?
            .to_str()?
            .to_string()))
        .filter(|file| file != "mod")
        .collect();

    Ok(filenames)
}

fn extract_dir_name(dir_path: &str) -> Result<String, ModuleError> {
    let folder_path = PathBuf::from(dir_path);
    
    let folder_name = folder_path.file_name()
        .ok_or(ModuleError::NameError)?;

    Ok(folder_name.to_str()
        .ok_or(ModuleError::NameError)?
        .into())

}

pub fn parse_declare(args: TokenStream2) -> TokenStream2 {
    let DeclareArgs { vis, path } = parse!(args as DeclareArgs);

    // note that ident!("") -> is not allowed since an empty string isn't an identifier
    // an empty string is represented by the None type
    let pub_key = match vis {
        Some(_) => None,
        None => Some(ident!("pub", vis.span())),
    };

    // extract the folder name
    let module = unwrap!(extract_dir_name(&path.value()), |e| {
        return syn::Error::new(path.span(), e.to_string())
            .into_compile_error();
    });

    let module = ident!(&module, path.span());

    // compiler error in case if there are any IO errors
    let modules = unwrap!(find_rs_modules(&path.value()), |e| {
        return syn::Error::new(path.span(), e.to_string())
            .into_compile_error();
    });

    // convert string paths vector to an ident vector
    let modules = modules.into_iter()
        .map(|module| ident!(&module, path.span()))
        .collect::<Vec<_>>();
    
    quote!(
        
            #pub_key mod #module {
            #(
                // no pub here? users won't be able to write actions::action::test;
               mod #modules;
               // pub is always here, since the person who declares the module
               // should always be able to access its contents
               pub use #modules::*;
            )*
            }
        
    ).into()
}
