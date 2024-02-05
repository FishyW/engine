use proc_macro::TokenStream;



#[proc_macro_attribute]
pub fn r#mod(attr: TokenStream, input: TokenStream) -> TokenStream {
    input
}