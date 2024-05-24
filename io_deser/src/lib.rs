#![allow(dead_code)]

extern crate proc_macro;

use proc_macro2::{Ident, Literal};
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, Visibility, TypeGenerics, WhereClause, ImplGenerics, Type};
use quote::{quote};
use crate::enum_type::{create_from_enum, handle_enum};
use crate::fields_ordering::FieldOrder;
use crate::fields_renaming::parse_fields_naming;
use crate::struct_type::{StructType, IterType, de_from_struct_type, ReturnType};

mod fields_ordering;
mod fields_renaming;
mod struct_type;
mod enum_type;

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
			ReturnType::Struct(StructType::NamedFields(fields_order))
		} else if let syn::Fields::Unnamed(ref unnamed) = data.fields{
			ReturnType::Struct(
				StructType::Tuple(
					unnamed.unnamed
					.iter()
					.filter_map(|f|{
						if let Visibility::Public(_) = f.vis{
							Some(&f.ty)
						}else {
							None
						}
					})
					.collect::<Vec<&Type>>()
				)
			)
		}
		else {
			panic!("IoDeSer attibute is for structs only")
		}
	}else if let syn::Data::Enum(ref data) = input.data{
		ReturnType::Enum(create_from_enum(data))
	}
	else {
		panic!("IoDeSer attibute is for structs only")
	}
}


#[proc_macro_derive(IoDeSer, attributes(io_name, io_order))]
/// Procedural macro which implements IoDeSer trait for Rust structs using *derive* attribute.
///
/// ## Examples
/// ```rust
/// use iodeser::*;
/// #[derive(IoDeSer)]
/// struct HtmlService{
/// 	pub api_key_string: String,
///		pub address: String,
/// }
/// ```
pub fn opis_derive_macro(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as DeriveInput);
	let struct_name = &input.ident;
	let (impl_generics, ty_generics, where_clause) = &input.generics.split_for_impl();

	// TODO struct type to handle tuple type



	match create_fields_from_data(&input){
		ReturnType::Struct(s) => {
			handle_struct(s, struct_name, impl_generics, ty_generics, where_clause)
		}
		ReturnType::Enum(e) => {
			handle_enum(e, struct_name, impl_generics, ty_generics, where_clause)
		}
	}.into()
}


fn handle_struct(fields_order: StructType, struct_name: &Ident,
				 impl_generics: &ImplGenerics, ty_generics:&TypeGenerics, where_clause: &Option<&WhereClause>)->proc_macro2::TokenStream{

	let mut to_io_string_tokens_implementation = quote!{};
	let mut _vector_field_maker = quote!{};
	let mut _struct_return_definition = quote!{};
	let mut index_of = 0;

	let is_tuple_struct = match &fields_order{
		StructType::NamedFields(_) => false,
		StructType::Tuple(t) => {
			let l = t.len();
			_vector_field_maker = quote!{#l};
			true
		}
	};


	for field_type in fields_order{
		match field_type{

			// IF NAMED STRUCT
			IterType::Field(f) => {

				let field = f.field;
				let field_type = &field.ty;
				let field_name = field.ident.as_ref();
				let field_name_str = field.ident.as_ref().unwrap().to_string();

				let (field_name_setter, option_field_file_name) = parse_fields_naming(&field,struct_name);



				// vector with real field name and otional renaming    vec![(in_rust_real_name, optional_renaming), (...)]
				_vector_field_maker.extend(quote!{
					(#field_name_str, #option_field_file_name),
				});


				// field initialization inside struct definition with: its_name: deserialized_io_string
				_struct_return_definition.extend(quote! {
					#field_name: from_io!(variable_and_io_str_value[#index_of as usize], #field_type)? ,
				});


				to_io_string_tokens_implementation.extend(
					quote! {
						string_output += &format!("{}{}{}->{}",
							if #index_of > 0 { "\n" } else { "" },
							(0..tab+1).map(|_| "\t").collect::<String>(),
							#field_name_setter,
							self.#field_name.to_io_string(tab + 1)
						);
					}
				);

			} // IF NAMED STRUCT

			// IF STRUCT TUPLE
			IterType::Type(t) => {



				let field_type = t;

				_struct_return_definition.extend(quote! {
					from_io!(variable_and_io_str_value[#index_of as usize], #field_type)? ,
				});

				let _suffix = Literal::usize_unsuffixed(index_of);
				to_io_string_tokens_implementation.extend(
					quote! {
						string_output += &format!("{}{}",
							if #index_of > 0 {
								format!("\n{}+\n{}", (0..tab+1).map(|_| "\t").collect::<String>(),(0..tab+1).map(|_| "\t").collect::<String>())
							} else { // for first element
								format!("{}", (0..tab+1).map(|_| "\t").collect::<String>())
							},
							self.#_suffix.to_io_string(tab+1)
						);
					}
				);

			}
		}


		index_of=index_of+1;
	}

	// final token initialization of vector with field names / optional renamings



	_struct_return_definition = match is_tuple_struct{
		false => {
			_vector_field_maker = quote!{vec![#_vector_field_maker]};

			quote! {
				#struct_name {#_struct_return_definition}
			}
		}
		true => {
			quote! {
				#struct_name (#_struct_return_definition)
			}
		}

	};

	let _deserialization_implementation = de_from_struct_type(is_tuple_struct, _vector_field_maker, struct_name);


	implement_iodeser_trait(struct_name,
							to_io_string_tokens_implementation,
							_struct_return_definition,
							impl_generics,ty_generics, where_clause,
							_deserialization_implementation).into()
}


fn implement_iodeser_trait(struct_name: &Ident, to_io_string_tokens_implementation:proc_macro2::TokenStream
						   , _struct_return_definition:proc_macro2::TokenStream,
						   impl_generics: &ImplGenerics, ty_generics:&TypeGenerics, where_clause: &Option<&WhereClause>,
						   _deserialization_implementation:proc_macro2::TokenStream)->proc_macro2::TokenStream{
	quote! {
		#[automatically_derived]
        impl #impl_generics IoDeSer for #struct_name #ty_generics #where_clause {


			fn to_io_string(&self, tab: u8)->String{
				let mut string_output = String::from("|\n");
                #to_io_string_tokens_implementation
				format!("{}\n{}|",string_output, (0..tab).map(|_| "\t").collect::<String>())
            }


            fn from_io_string(io_input:&mut String)->iodeser::Result<Self>{
				// DELETE TABULATOR

				if !io_input.starts_with('|') || !io_input.ends_with('|') {
					return Err(iodeser::Error::io_format (
						io_input.clone(),
						"String lacks vertical bars at the beginning or end".to_string(),
					).into());
				}


				let mut ret = String::new();
				for line in io_input.lines().filter(|line| line.len() > 1) {
					ret.push_str(&line[1..]);
					ret.push('\n');
				}
				*io_input = ret.trim().to_string();
				// DELETE TABULATOR

				let mut variable_and_io_str_value = Vec::<String>::new();
				#_deserialization_implementation

                Ok(#_struct_return_definition)
            }
        }
    }
}


//TODO escape characters in String values, for example new line should be = \n (destroy whole format)