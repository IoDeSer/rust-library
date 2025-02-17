#![allow(dead_code)]

extern crate proc_macro;

use proc_macro2::{Ident, Literal};
use proc_macro::TokenStream;
use struct_type::TupleType;
use syn::{parse_macro_input, DeriveInput, Visibility};
use quote::quote;
use crate::enum_type::{create_from_enum, EnumType, handle_enum};
use crate::fields_ordering::FieldOrder;
use crate::struct_type::{handle_struct, StructType};

mod fields_ordering;
mod fields_renaming;
mod struct_type;
mod enum_type;

pub(crate) enum ReturnType<'a>{
    Struct(StructType<'a>),
    Enum(EnumType<'a>)
}

#[inline]
fn create_fields_from_data(input: &DeriveInput) -> ReturnType {
    if let syn::Data::Struct(ref data) = input.data {
        if let syn::Fields::Named(ref fields) = data.fields {
            let mut fields_order = fields.named
                .iter()
                .filter_map(|field| {
                    if let Visibility::Public(_) = field.vis {
                        Some(FieldOrder::new(field, &input.ident))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();
            fields_order.sort();

            let private_fields = fields.named
                .iter()
                .filter_map(|field| {
                    if let Visibility::Public(_) = field.vis {
                        None
                    } else {
                        Some(field)
                    }
                })
                .collect::<Vec<_>>();
            ReturnType::Struct(StructType::NamedFields{publics: fields_order, privates: private_fields})
        } else if let syn::Fields::Unnamed(ref unnamed) = data.fields {
            ReturnType::Struct(
                StructType::Tuple(
                    unnamed.unnamed
                        .iter()
                        .filter_map(|f| {
                            if let Visibility::Public(_) = f.vis {
                                Some(TupleType{object_type: &f.ty, is_public: true})
                            } else {
                                Some(TupleType{object_type: &f.ty, is_public: false})
                            }
                        })
                        .collect::<Vec<TupleType>>()
                )
                
            )
        } else {
            panic!("This data type is not supported by IoDeSer attribute")
        }
    } else if let syn::Data::Enum(ref data) = input.data {
        ReturnType::Enum(create_from_enum(data))
    } else {
        panic!("This data type is not supported by IoDeSer attribute")
    }
}


#[proc_macro_derive(IoDeSer, attributes(io_name, io_order))]
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


    match create_fields_from_data(&input) {
        ReturnType::Struct(s) =>
            handle_struct(s, input_name, impl_generics, ty_generics, where_clause),
        ReturnType::Enum(e) =>
            handle_enum(e, input_name, impl_generics, ty_generics, where_clause)
    }.into()
}


//TODO escape characters in String values, for example new line should be = \n (destroy whole format)