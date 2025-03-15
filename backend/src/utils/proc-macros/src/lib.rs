extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{DeriveInput, Ident};

fn impl_get_struct_fields(ast: DeriveInput) -> TokenStream {
    // get field identifiers

    let ident = ast.ident; 

    println!("\nIDENT: {}\n", &ident);

    let fields: Vec<Ident> = match ast.data {
        syn::Data::Struct(data) => data.fields.into_iter().filter_map(|field| field.ident).collect(),
        _ => panic!("Only structs are supported"),
    };

    let field_idents_as_strings: Vec<String> = fields.iter().map(|field| field.to_string()).collect();

    // generate the impl
    quote::quote! {
        impl GetStructFields for #ident {
            fn get_struct_fields() -> Vec<&'static str> {
                vec![#(#field_idents_as_strings),*]
            }
        }
    }.into()
}


#[proc_macro_derive(GetStructFields)]
pub fn get_struct_fields(input: TokenStream) -> TokenStream {
    // parse the input
    let ast: DeriveInput = syn::parse(input).unwrap();

    impl_get_struct_fields(ast)
}