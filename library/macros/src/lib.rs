use proc_macro::TokenStream;

mod declare;

mod rules;


/*
Auto generated code sample
// pub here is optional
<pub> mod actions {
    // no pub here? users won't be able to write actions::action::test;
   mod action;
   // pub is always here, since the person who declares the module
   // should always be able to access its contents
   pub use action::*;
   fn test() {
    Test;
   }
}
*/

// declare macro
// allows you to specify a certain folder and expose all of its files as a module
// use it like declare!("path/to/folder")
// "path/to/folder" is the path from the root directory of the project
// also accepts an optional visibility (priv) modifier keyword
#[proc_macro]
pub fn declare(input: TokenStream) -> TokenStream {
    let input = proc_macro2::TokenStream::from(input);
    declare::parse_declare(input).into()
}
