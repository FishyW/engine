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

#[proc_macro_attribute]
pub fn asset_object(attr: TokenStream, input: TokenStream) -> TokenStream {
    TokenStream::new()
}


#[proc_macro_attribute]
pub fn asset_component(attr: TokenStream, input: TokenStream) -> TokenStream {
    TokenStream::new()
}

#[proc_macro_attribute]
pub fn asset_manager(attr: TokenStream, input: TokenStream) -> TokenStream {
    TokenStream::new()
}

#[proc_macro_attribute]
pub fn asset_action(attr: TokenStream, input: TokenStream) -> TokenStream {
    TokenStream::new()
}

#[proc_macro_attribute]
pub fn asset_event(attr: TokenStream, input: TokenStream) -> TokenStream {
    TokenStream::new()
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
pub fn receiver(attr: TokenStream, input: TokenStream) -> TokenStream {
    TokenStream::new()
}

#[proc_macro_attribute]
pub fn interceptor(attr: TokenStream, input: TokenStream) -> TokenStream {
    TokenStream::new()
}


// fn uppercase_ident(ident: &Ident) -> Ident {
//     ident!(&ident.to_string().to_uppercase())
// }

// fields -> (attribute name, attribute type, is component?)
// fn generate_object_derive(name: Ident, fields: Vec<(Ident, Type, bool)>) -> TokenStream {
//     let type_id = format_ident!("__{}_TYPE_ID", uppercase_ident(&name));
//     let field_names = fields.iter().map(|field| &field.0).collect::<Vec<_>>();
    
    
//     // let transform = Transform::default()
//     let default_initializations = fields.iter().map(|field| {
//         let name = &field.0;
//         let field_type = &field.1;
//         quote::quote!(
//             let mut #name = #field_type::default();
//         )
//     }).collect::<Vec<_>>();

//     // let transform = Transform::new()
//     let component_initializations = fields.iter().filter_map(|field| {
//         if field.2 {
//             return None;
//         }

//         let name = &field.0;
//         let field_type = &field.1;
//         Some(quote::quote!(
//             let mut #name = #field_type::new(object_ref);
//         ))
//     }).collect::<Vec<_>>();
    
//     quote::quote!(
//         impl Default for #name {
//             fn default() -> Self {
//                 let metadata = InstanceMetadata::default();
        
//                 #(
//                     #default_initializations
//                 )*
                    
//                 let object = #name {
//                     metadata,
//                     #(
//                         #field_names
//                     ),*
//                 };

//                 let object_ref = Self::register(object);
                


//                 object
//             }
//         } 
//     ).into()
// }

// fn parse_object_derive(input: DeriveInput) -> TokenStream {
//     let DeriveInput{
//         ident: name,
//         data: Data::Struct(DataStruct {
//             fields, ..
//         }), 
//         ..} = input else {
//             return 
//             syn::Error::new(Span::call_site(), "Failed to parse struct!")
//             .to_compile_error().into()
//         };

//         let fields = fields.into_iter().filter_map(|field| {
//             let field_name = field.ident.expect("Invalid field!");
//             if field_name == "metadata" {
//                 return None;
//             }

//             let attr = field.attrs.into_iter()
//                 .find(|attr| {
//                     let Some(ident) = attr.meta.path().get_ident() else {
//                         return false;
//                     };
//                     ident == "component"
//                 });
//             // if component attribute exists
//             if let Some(_) = attr {
//                 return Some((field_name, field.ty, true))
//             }
            
//             Some((field_name,  field.ty, false))
//         }).collect::<Vec<_>>();

//     generate_object_derive(name, fields)
// }

// Derive Default but specifically for objects.
// This is needed since the object needs to fill in its components' component metadata,
// the component metadata includes the parent's type id and id.
// #[proc_macro_derive(DefaultObject, attributes(component))]
// pub fn default_object_derive(input: TokenStream) -> TokenStream {
//     let input = parse_macro_input!(input as DeriveInput);
    
//    parse_object_derive(input)
// }