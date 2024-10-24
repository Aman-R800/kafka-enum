mod generate_enum;
mod get_components;

use get_components::{get_json, get_topics, get_topics_str};
use syn::ItemEnum;
use proc_macro::TokenStream;
use generate_enum::{generate, generate_as_str_impl};
use quote::quote;
use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn topics(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let inp_clone = input.clone();
    let mut parsed_enum = parse_macro_input!(inp_clone as ItemEnum);
    
    let json = get_json();
    let topics = get_topics(&json);
    let topic_str = get_topics_str(&json);

    let enum_definition = generate(&mut parsed_enum, &topics);
    let enum_impl = generate_as_str_impl(topics, topic_str, &parsed_enum);

    quote! {
        #enum_definition
        #enum_impl
    }.into()
    
}

