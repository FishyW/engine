/// Macro Rules declarative (helper) macros

// this needs to be implemented as a macro
// since input.peek(T: Token) doesn't work for some reason
// next helper macro, goes to the next token and returns Some(token)
// if the next token is the specified token
// otherwise returns None and it doesn't go to the next token
macro_rules! next {
    // $buf is a metavariable called buf, which is of type ident: identifier
    // $token is a metavariable called token, which is of type tt -> token tree
    // token tree is the most generic
    // $(A)+ -> matches A one or more times
    ($input:ident, $($token:tt)+) => {
        if $input.peek($($token)+) {
            // specify the entire path to avoid naming collisions
            // unwrap is fine since we know the token exists
            std::option::Option::Some($input.parse::<$($token)+>().unwrap())
        } else { None }
    }
}

// similar to parse_macro_input but for parse2
macro_rules! parse {
    ($tokens:ident as $typeto:tt) => {
        match syn::parse2::<$typeto>($tokens) {
            std::result::Result::Ok(arg) => arg,
            std::result::Result::Err(e) => return e.into_compile_error(),
        }
    };
}

// ident! macro
// converts a string to an ident
// uses a macro since the second argument is optional
macro_rules! ident {
    ($string:expr) => {
        syn::Ident::new($string, Span::call_site())
    };
    ($string:expr, $site:expr) => {
        syn::Ident::new($string, $site)
    };
}

// unwrap macro, effectively unwrap or else but will be expanded to a match expression
// this means you can use a return to do an early return from a block
macro_rules! unwrap {
    ($item:expr, || $block:block) => {
        match $item {
            Some(v) => v,
            None => $block,
        }
    };
    ($item:expr, |$err:ident| $block:block) => {
        match $item {
            Ok(v) => v,
            Err($err) => $block,
        }
    };
}

pub(crate) use {ident, next, parse, unwrap};
