extern crate proc_macro;

use proc_macro2::{Ident, Literal};
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, Visibility, TypeGenerics, WhereClause, ImplGenerics};
use quote::quote;
use crate::fields_ordering::FieldOrder;
use crate::fields_renaming::parse_fields_naming;

mod fields_ordering;
mod fields_renaming;


fn create_fields_from_data<'a>(input: &'a DeriveInput)->Vec::<FieldOrder<'a>>{
	let mut fields_order = Vec::<FieldOrder>::new();

	// iterate through all fields
	if let syn::Data::Struct(ref data) = input.data {
		if let syn::Fields::Named(ref fields) = data.fields {
			for field in &fields.named {
				if matches!(field.vis, Visibility::Public(_)) {

					fields_order.push(FieldOrder::new(field, &input.ident));

				}
			}
		}
	}

	fields_order.sort();
	fields_order
}


#[proc_macro_derive(IoDeSer, attributes(io_name, io_order))]
pub fn opis_derive_macro(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as DeriveInput);
	let struct_name = &input.ident;
	let (impl_generics, ty_generics, where_clause) = &input.generics.split_for_impl();

	let fields_order = create_fields_from_data(&input);


	let mut to_io_string_tokens_implementation = quote!{};
	let mut vector_field_maker = quote!{};
	let mut tokens_from_io = quote!{};


	for (index_of,field_o) in fields_order.iter().enumerate() {
		let field = field_o.field;
		let field_type = &field.ty;
		let field_name = field.ident.as_ref();
		let field_name_str = field.ident.as_ref().unwrap().to_string();


		let (field_name_setter, option_field_file_name) = parse_fields_naming(&field,struct_name);


		// vector with real field name and otional renaming    vec![(in_rust_real_name, optional_renaming), (...)]
		vector_field_maker.extend(quote!{
			(#field_name_str, #option_field_file_name),
		});


		// field initialization inside struct definition with: its_name: deserialized_io_string
		tokens_from_io.extend(quote! {
			#field_name: from_io!(variable_and_io_str_value[#index_of as usize][1], #field_type)? ,
		});


		to_io_string_tokens_implementation.extend(
			quote! {
					string_output += &format!("{}{}{}->{}",
						if #index_of > 0 { "\n" } else { "" },
						(0..tab+1).map(|_| "\t").collect::<String>(),
						#field_name_setter,
						IoSerialization::next(&self.#field_name, tab + 1).ser()
					);
			}
		);

	}

	// final token initialization of vector with field names / optional renamings
	vector_field_maker = quote!{vec![#vector_field_maker]};


	to_io_string_tokens_implementation = quote!{
			fn to_io_string(&self, tab: u8)->String{
				let mut string_output = String::from("|\n");
                #to_io_string_tokens_implementation
				format!("{}\n{}|",string_output, (0..tab).map(|_| "\t").collect::<String>())
            }
	};


	implement_iodeser_trait(struct_name,
							to_io_string_tokens_implementation,
							vector_field_maker,
							tokens_from_io,
							impl_generics,ty_generics, where_clause).into()
}



fn implement_iodeser_trait(struct_name: &Ident, to_io_string_tokens_implementation:proc_macro2::TokenStream
						   , vector_field_maker:proc_macro2::TokenStream, tokens_from_io:proc_macro2::TokenStream,
						   impl_generics: &ImplGenerics, ty_generics:&TypeGenerics, where_clause: &Option<&WhereClause>)->proc_macro2::TokenStream{
	quote! {
        impl #impl_generics IoDeSer for #struct_name #ty_generics #where_clause {


			#to_io_string_tokens_implementation


            fn from_io_string(io_input:&mut String)->iodeser::Result<Self>{

				// DELETE TABULATOR
				let mut ret = String::new();
				let lines: Vec<&str> = io_input.lines().collect();

				for line in lines {
					if line.len() > 1 {
						ret += &format!("{}\n", &line[1..]);
					}
				}

				*io_input = ret.trim().to_string();
				// DELETE TABULATOR


				let mut variable_and_io_str_value = Vec::new();

				let lines:Vec<&str> = io_input.lines().collect();
				let mut line_pointer = 0;

				let fields:Vec<(&str, Option<&str>)> = #vector_field_maker;

				while line_pointer < lines.len(){
					let current_line = lines[line_pointer];
					let assignment:Vec<&str> = current_line.split("->").collect();

					if assignment.len() == 0{
						continue;
					}

					let variable_name = assignment[0].trim().to_string();

					let mut found_property = "";

					for f in fields.iter(){
						let original_name = f.0;
						let custom_name = f.1;
						match custom_name{
							Some(name)=>{
								if variable_name.eq(name){
									found_property = name;
								}
							},
							None=>{
								if variable_name.eq(original_name){
									found_property = original_name;
								}
							}
						}
					}



					if found_property==""{
						return Err(iodeser::errors::Error::FieldNotFoundError(iodeser::errors::FieldNotFoundError::new(variable_name, stringify!(#struct_name).to_string())));
					}



					// primitive type
					if assignment[1].len() > 1{
						variable_and_io_str_value.push(vec![found_property.to_string(), assignment[1].to_string()]);
					}else{ // class / array / vector
						line_pointer=line_pointer+1;

						if lines[line_pointer] == "|"{
							line_pointer=line_pointer+1;
							variable_and_io_str_value.push(vec![found_property.to_string(), "|\n\t\n|".to_string()]);
							continue;
						}

						let new_object_start = line_pointer;

						while lines[line_pointer] != "|" {
							line_pointer = line_pointer+1;
						}

						let new_object_end = line_pointer;
						let mut new_object_string = String::from("|\n");

						for l2 in new_object_start..new_object_end {
							new_object_string += &format!("{}\n", lines[l2]);
						}

						new_object_string+="\n|";

						variable_and_io_str_value.push(vec![found_property.to_string(), new_object_string]);
					}

					line_pointer=line_pointer+1;
				}



                Ok(#struct_name { #tokens_from_io })
            }
        }
    }
}