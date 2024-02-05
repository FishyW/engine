use proc_macro::TokenStream;
use quote::quote;



// declare macro
// allows you to specify a certain folder and expose all of its files as a module
// use it like declare!("path/to/folder")
// "path/to/folder" is the path from the root directory of the project
#[proc_macro]
pub fn declare(input: TokenStream) -> TokenStream {
    let input = proc_macro2::TokenStream::from(input);


    let output = quote!(
        #input
    );

    proc_macro::TokenStream::from(output)
}