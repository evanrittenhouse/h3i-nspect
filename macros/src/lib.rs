use h3i::actions::h3::Action;
use proc_macro::TokenStream;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
};

#[proc_macro_attribute]
pub fn test_case(args: TokenStream, item: TokenStream) -> TokenStream {
    expand(args, item)
}

fn expand(args: TokenStream, item: TokenStream) -> TokenStream {
    // let args = parse_macro_input!(args as H3iArgs);

    item
}

struct H3iArgs {
    actions: Vec<Action>,
}
