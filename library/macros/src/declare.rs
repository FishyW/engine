// source code for the declare macro

/*
Usage
declare!(<priv> <path>, {
    <rest>
})

if <priv> is not present, then <pub> is present
if <priv> is present, then <pub> is not present


Auto generated code sample
// pub here is optional
<pub> mod <folder> {
    // no pub here? users won't be able to write actions::action::test;
   mod <file1>;
   // pub is always here, since the person who declares the module
   // should always be able to access its contents
   pub use <file1>::*;
   
    <rest>
}
*/


use std::{env, io, path::PathBuf};

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{
    braced, parse::{Parse, ParseStream}, spanned::Spanned, Ident, LitStr, Token
};

// anyhow makes writing errors easier
use anyhow::{Context, Result};
use thiserror::Error;

use crate::rules::{ident, parse, try_ident, unwrap};

#[derive(Debug, Error)]
enum ModuleError {
    #[error("invalid module name `{}` \n\
    make sure your file/folder's name can be converted into a Rust module", .0)]
    NameError(String),

    #[error("error when reading path `{}`", .0)]
    PathError(String)
}

struct DeclareArgs {
    // we don't store as a boolean, so we can get the span of the token
    vis: Option<Token![priv]>,
    // LitStr, string literal token
    path: LitStr,
    rest: Option<TokenStream2>
}

// seems like on error, input.parse() does not advance to the next token 
// which is expected
impl Parse for DeclareArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let vis = input.parse::<Token![priv]>().ok();
        let path = input.parse()?;
        let content;
        
        // parse the rest of the contents inside of a block into a token stream
        let rest = if let Some(_) = input.parse::<Token![,]>().ok()  {
            braced!(content in input);
            let content = content.parse::<TokenStream2>()?;
            Some(content)
            
        } else {
            None
        };


        Ok(DeclareArgs {
            // goes to the next token
            vis,
            path,
            rest
        })
    }
}

// gets a vector of all the filenames inside of the directory
fn read_directory(dir_path: PathBuf) -> Result<Vec<PathBuf>> {
    let entries = dir_path.read_dir()
        .context("Error when reading directory. Does the directory exist?")?;

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


// reads a directory and returns all Rust modules
// this function extracts the module name from file names in a folder
fn extract_modules_ident(dir_path: &str) -> Result<Vec<Ident>> {
    let project_path = env::var("CARGO_MANIFEST_DIR")?;
    let path = PathBuf::from(project_path).join(dir_path);
    let filenames = read_directory(path)?;


    // filter_map filters the element when it is None
    // the first map checks if the extension is correct
    // the second map converts the pathbuf to a string
    let filenames = filenames.into_iter()
        .filter_map(|file| {
            let ext = file.extension()?;
            if ext == "rs" { Some(file) } else { None }
        })
        .filter_map(|file| 
            Some(file.file_stem()?
            .to_str()?
            .to_string()))
        .filter(|file| file != "mod")
        .map(|file| try_ident!(&file)
            .or(Err(ModuleError::NameError(file.clone()))))
        .collect::<Result<Vec<_>, ModuleError>>()?;

            
    Ok(filenames)
}

// no need to set the span since caller sets the span
// this function extracts the module name from a folder name
fn extract_module_ident(dir_path: &str) -> Result<Ident> {
    let folder_path = PathBuf::from(dir_path);
    
    let module_name = folder_path.file_name()
        .ok_or(ModuleError::PathError(dir_path.to_string()))?
        .to_str()
        .ok_or(ModuleError::PathError(dir_path.to_string()))?;

    Ok(try_ident!(module_name)
        .or(Err(ModuleError::NameError(module_name.to_string())))?)
        

}

// main parse function for the declare macro
pub fn parse_declare(args: TokenStream2) -> TokenStream2 {
    let DeclareArgs { vis, path, rest } = parse!(args as DeclareArgs);

    // note that ident!("") -> is not allowed since an empty string isn't an identifier
    // an empty string is represented by the None type
    let pub_key = match vis {
        Some(_) => None,
        None => Some(ident!("pub", vis.span())),
    };


    // extract the folder name
    let module = unwrap!(extract_module_ident(&path.value()), |e| {
        return syn::Error::new(path.span(), e.to_string())
            .into_compile_error();
    });


    // compiler error in case if there are any IO errors
    let modules = unwrap!(extract_modules_ident(&path.value()), |e| {
        return syn::Error::new(path.span(), e.to_string())
            .into_compile_error();
    });


    quote!(
            #pub_key mod #module {
                use engine_lib::macros::declare;
            #(
                // no pub here? users won't be able to write actions::action::test;
               mod #modules;
               // pub is always here, since the person who declares the module
               // should always be able to access its contents
               pub use #modules::*;
               // optionally allows users to put in some data in here
               // useful for writing further module declarations inside of an already declared module
            )*
                #rest
            }
        
    )
}
