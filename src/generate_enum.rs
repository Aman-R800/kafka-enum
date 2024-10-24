use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Fields, Ident, ItemEnum, Variant};

pub fn generate(input: &mut ItemEnum, variants: &Vec<String>) -> TokenStream {

    for topic in variants.iter() {
        input.variants.push(Variant{
            ident: Ident::new(topic, Span::call_site()),
            attrs: Vec::new(),
            fields: Fields::Unit,
            discriminant: None
        });
    }

    quote! {
        #input
    }
}

pub fn generate_as_str_impl(variants: Vec<String>, string_reprs: Vec<String>, input: &ItemEnum) -> TokenStream {
    assert_eq!(variants.len(), string_reprs.len(), "Vectors should have the same length");

    let match_arms: Vec<TokenStream> = variants.iter().zip(string_reprs.iter()).map(|(variant, repr)| {
        let variant_ident = syn::Ident::new(variant, proc_macro2::Span::call_site());
        quote! {
            Self::#variant_ident => #repr,
        }
    }).collect();

    let enum_ident = &input.ident;

    let generated = quote! {
        impl #enum_ident {
            pub fn as_str(&self) -> &'static str {
                match self {
                    #(#match_arms)*
                }
            }
        }
    };

    generated
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;

    #[test]
    fn test_expand_variants() {
        // Define an enum using the macro
        let input = quote! {
            pub enum Example {}
        };

        let mut input_item = syn::parse2::<ItemEnum>(input).unwrap();

        // Use the macro to expand the enum
        let expanded = syn::parse2::<ItemEnum>(generate(&mut input_item, &vec!["test".to_string()])).unwrap();

        // Expected output
        let expected = {
            let temp = quote! {
                pub enum Example {
                    test
                }
            };

            syn::parse2::<ItemEnum>(temp).unwrap()
        };


        // Assert that the expanded output matches the expected output
        assert!(expanded == expected);
    }
}
