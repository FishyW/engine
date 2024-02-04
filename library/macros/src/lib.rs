
mod automod;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::parse_macro_input;
use std::{env, path::PathBuf};


use automod::*;

#[proc_macro]
pub fn automod(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Arg);
    let rel_path = input.path.value();

    let dir = match env::var_os("CARGO_MANIFEST_DIR") {
        Some(manifest_dir) => PathBuf::from(manifest_dir).join(rel_path),
        None => PathBuf::from(rel_path),
    };

    let expanded = source_file_names(dir)
        .into_iter()
        .map(|name| mod_item(name))
        .collect::<TokenStream2>();

    TokenStream::from(expanded)
}

#[proc_macro]
pub fn generate_project_imports(_input: TokenStream) -> TokenStream {
    quote!(
        // auto mods all files in the action directory
        // for example if action contains action.rs, action2.rs -> will generate
        // pub mod action; pub mod action2;
        mod actions {
            use engine_lib::macros::automod;
            // expands to pub use action::*; pub use action2::*;
            automod!("src/actions");
        }

        mod components {
            use engine_lib::macros::automod;
            automod!("src/components");
        }

        mod events {
            use engine_lib::macros::automod;
            automod!("src/events");
        }

        mod objects {
            use engine_lib::macros::automod;
            automod!("src/objects");
        }

        mod managers {
            use engine_lib::macros::automod;
            automod!("src/managers");
        }


        mod engine {
            pub mod prelude;
            mod scene {
                use engine_lib::macros::automod;
                automod!("src/engine/scene");
            }
        }
    ).into()
}