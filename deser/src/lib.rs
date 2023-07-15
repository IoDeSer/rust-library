extern crate proc_macro;
use proc_macro::TokenStream;

use syn::{parse_macro_input, Data, DeriveInput, Fields, Visibility};
use quote::quote;
use quote::ToTokens;

#[proc_macro_derive(IoDeSer)]
pub fn opis_derive_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;

    let mut fields_hash = std::collections::HashMap::<String, String>::new();
    let mut field_arr = Vec::<String>::new();

    if let syn::Data::Struct(ref data) = input.data {
        if let syn::Fields::Named(ref fields) = data.fields {
            for field in &fields.named {


                if matches!(field.vis, Visibility::Public(_)){
                    let field_type = field.ty.to_token_stream().to_string();
                    field_arr.push(field_type);
                    let field_name = field.ident.as_ref().unwrap().to_string();
                    field_arr.push(field_name);
                }

            }
        }
    }

    let fields_str = field_arr.join(" ");

    let tokens = quote! {
        impl IoDeSer<#struct_name> for #struct_name{
            fn to_io_string(self)->String{
                "e".to_string()
            }
            fn from_io_string(io_input:String)->#struct_name{
                todo!()
            }
        }
    };

    tokens.into()
}




/// Example of user-defined [procedural macro attribute][1].
///
/// [1]: https://doc.rust-lang.org/reference/procedural-macros.html#attribute-macros
#[proc_macro_attribute]
pub fn my_attribute(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let tokens = quote! {
        #input

        struct Hello;
    };

    tokens.into()
}
