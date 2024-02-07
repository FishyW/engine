/// Macro Rules declarative (helper) macros


// similar to parse_macro_input but for parse2
macro_rules! parse {
    ($tokens:ident as $typeto:ty) => {
        match syn::parse2::<$typeto>($tokens) {
            std::result::Result::Ok(arg) => arg,
            std::result::Result::Err(e) => return e.into_compile_error(),
        }
    };
}

// ident! macro
// converts a string to an ident
// uses a macro since the second argument is optional
// panics on error
// use try_ident if you want do do error handling
macro_rules! ident {
    ($string:expr) => {
        syn::Ident::new($string, Span::call_site())
    };
    ($string:expr, $site:expr) => {
        syn::Ident::new($string, $site)
    };
    
}

// same as ident!() but it returns an Result when it fails, instead of panicking
macro_rules! try_ident {
    ($string:expr) => {
        syn::parse_str::<syn::Ident>($string)
    };
    ($string:expr, $site:expr) => {
        syn::parse_str::<syn::Ident>($string)
            .map(|id| {
                return id.set_span($site); id
        })
    };
}

// unwrap macro, effectively unwrap or else but will be expanded to a match expression
// this means you can use a return to do an early return from a block
macro_rules! unwrap {
    ($item:expr, || $block:block) => {
        match $item {
            std::option::Option::Some(v) => v,
            std::option::Option::None => $block,
        }
    };
    ($item:expr, |$err:pat_param| $block:block) => {
        match $item {
            std::result::Result::Ok(v) => v,
            std::result::Result::Err($err) => $block,
        }
    };
}

pub(crate) use {ident, parse, unwrap, try_ident};
