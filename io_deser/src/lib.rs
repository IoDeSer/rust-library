#![allow(dead_code)]

extern crate proc_macro;

use crate::enum_type::handle_enum;
use crate::fields_ordering::FieldOrder;
use crate::struct_type::handle_struct;
use proc_macro::TokenStream;
use proc_macro2::{Ident, Literal};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

use fields_processing::{create_fields_from_data, ReturnType};

mod enum_type;
mod fields_ordering;
mod fields_processing;
mod fields_renaming;
mod struct_type;


#[proc_macro_derive(IoDeSer, attributes(io_name, io_order, io_ignore, io_allow))]
/// Procedural macro which implements IoDeSer trait for Rust structs using *derive* attribute.
///
/// Works with structs and enum data types.
///
/// ## Examples
/// ```rust
/// use iodeser::*;
/// #[derive(IoDeSer)]
/// struct HtmlService{
/// 	pub api_key_string: String,
///    	pub address: String,
/// }
/// ```
pub fn opis_derive_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let input_name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = &input.generics.split_for_impl();

    let (return_type, errors) = create_fields_from_data(&input);
    let mut return_type: proc_macro::TokenStream = match return_type{
        ReturnType::Struct(s) => {
            handle_struct(s, input_name, impl_generics, ty_generics, where_clause)
        }
        ReturnType::Enum(e) => handle_enum(e, input_name, impl_generics, ty_generics, where_clause),
        ReturnType::Unit => panic!()
    }.into();

    return_type.extend(errors);
    return_type
}

//TODO escape characters in String values, for example new line should be = \n (destroy whole format)

/*

ReturnType::Struct(s) => {
                    handle_struct(s, input_name, impl_generics, ty_generics, where_clause)
                }
                ReturnType::Enum(e) => handle_enum(e, input_name, impl_generics, ty_generics, where_clause),
                ReturnType::Unit => quote!(compile_error!("Unit-like struct are not supported yet!"))
*/