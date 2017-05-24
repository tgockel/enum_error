#![crate_type="proc-macro"]

extern crate syn;
#[macro_use]
extern crate quote;
extern crate proc_macro;

use proc_macro::TokenStream;

/// Emit an `std::fmt::Display` implementation for an enum type.
#[proc_macro_derive(EnumDisplay)]
pub fn emit_enum_display(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_derive_input(&s).unwrap();

    let name = &ast.ident;

    let toks = quote! {
        impl ::std::fmt::Display for #name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "{:?}", self)
            }
        }
    };

    toks.parse().unwrap()
}

/// Emit an `std::error::Error` implementation for an enum type. This is most useful in conjunction
/// with `Debug` and `EnumDisplay`.
#[proc_macro_derive(EnumError)]
pub fn emit_enum_error(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_derive_input(&s).unwrap();

    let name = &ast.ident;

    let variants = match ast.body {
        syn::Body::Enum(ref v) => v,
        _                      => panic!("EnumError only works for enum types")
    };

    let mut serializations = Vec::new();

    for variant in variants {
        let ident = &variant.ident;
        let ident_str = format!("{}", ident);
        serializations.push(quote! {
            &#name::#ident => #ident_str
        });
    }

    let toks = quote! {
        impl ::std::error::Error for #name {
            fn description(&self) -> &str {
                match self {
                    #(#serializations),*
                }
            }
        }
    };

    toks.parse().unwrap()
}
