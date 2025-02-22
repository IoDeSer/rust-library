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
mod unit_struct;


#[proc_macro_derive(IoDeSer, attributes(io_name, io_order, io_ignore, io_allow))]
/// Procedural macro which implements IoDeSer trait for Rust structs using *derive* attribute.
///
/// Works with structs and enum data types.
///
/// ## Examples
/// By default, IoDeSer serializes only public fields in the order they appear in the object definition.
/// 
/// **Private fields are required to implement trait [Default].**
/// 
/// ```rust
/// use iodeser::*;
/// #[derive(IoDeSer)]
/// struct HtmlService{
///     api_key_string: String,
///     pub address: String,
/// }
/// ```
/// 
/// In this example, macro "to_io!" will serialize only field "address" and ignore "api_key_string".
/// It is intended behaviour to not serialize private fields, as they should not be exposed externally.
/// To serialize private fields use "io_allow" attribute. 
/// 
/// Similarly, public fields - that are noramlly serialized - 
/// can be ignored during serialization using "io_ignore" attribute.
/// 
/// **Ignored fields are required to implement trait [Default].**
/// 
/// ```rust
/// use iodeser::*;
/// #[derive(IoDeSer)]
/// struct HtmlService{
///     #[io_allow]
///     api_key_string: String,
///     #[io_ignore]
///     pub address: String,
/// }
/// ```
/// 
/// Now, only "api_key_string" will be serialized.
/// 
/// -------------------
/// 
/// To rename field in serialized string or the deserialize field of different name that the object's definition have use "io_name" attribute with **String** argument.
/// 
/// Next, to change the order of fields in serialized string use "io_order" attribute with **i16** argument.
/// 
/// ```rust
/// use iodeser::*;
/// #[derive(IoDeSer)]
/// struct HtmlService{
///     #[io_name("key")] #[io_order(2)]
///     pub api_key_string: String,
///     #[io_name("website")] #[io_order(1)]
///     pub address: String,
/// }
/// ```
pub fn opis_derive_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let input_name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = &input.generics.split_for_impl();

    let (return_type, compile_errors) = create_fields_from_data(&input);
    let mut return_type: proc_macro::TokenStream = match return_type{
        ReturnType::Struct(s) => {
            handle_struct(s, input_name, impl_generics, ty_generics, where_clause)
        }
        ReturnType::Enum(e) => handle_enum(e, input_name, impl_generics, ty_generics, where_clause),
        ReturnType::Unit => unit_struct::handle_unit(input_name, impl_generics, ty_generics, where_clause)
    }.into();

    // add errors to token return
    return_type.extend(compile_errors);
    return_type
}