use proc_macro::TokenStream;

mod declare;
mod rules;


/// Set a path from the project directory to a certain directory
/// and expose all of its files as a module.
/// Use it like `declare!("path/to/folder")`,
/// where `"path/to/folder"`` is the path from the root directory of the project.
/// Also accepts an optional visibility (priv) modifier keyword.
/// This makes the module private (though this is rarely used).
/// To declare a submodule, see the example below
/// # Example
/// ```no_run
/// declare!(priv "/src/path/to/module", {
///     declare!("/src/path/to/submodule");
/// });
/// ```
/// Note, due to how modules work, you can only write declare macros inside of `lib.rs`.
#[proc_macro]
pub fn declare(input: TokenStream) -> TokenStream {
    let input = proc_macro2::TokenStream::from(input);
    declare::parse_declare(input).into()
}

/// Annotates that the struct is of a certain asset type.
/// An asset macro can take the following forms:
/// - asset(object)
/// - asset(event)
/// - asset(action)
/// - asset(component)
/// - asset(manager)
/// Internally this macro expands to `#[asset_object]`.
/// This allows users to create their own asset macros while using the same API
#[proc_macro_attribute]
pub fn asset(attr: TokenStream, input: TokenStream) -> TokenStream {
    let mut attr = syn::parse_macro_input!(attr as syn::Ident);
    attr.set_span(proc_macro2::Span::call_site());
    let input = proc_macro2::TokenStream::from(input);
    let attr = quote::format_ident!("asset_{}", attr);
    
    quote::quote!(
        #[#attr]
        #input
    ).into()
}

/// Includes a component
/// only ever put this below an asset(object) macro
/// this macro is used to include a list of components 
/// ```
/// use macros::include;
/// 
/// #[include(MyComponent)] 
/// struct Hello {}
/// ```
/// For instance, the code above tells the compiler to include MyComponent to Hello.
/// This will make MyComponent avaiable as an atrribute to all Hello instances 
/// as the property `my_component`. 
#[proc_macro_attribute]
pub fn include(attr: TokenStream, input: TokenStream) -> TokenStream {
    TokenStream::new()
}

#[proc_macro_attribute]
pub fn asset_object(attr: TokenStream, input: TokenStream) -> TokenStream {
    TokenStream::new()
}



