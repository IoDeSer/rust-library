extern crate proc_macro;


use proc_macro2::Ident;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, Visibility};
use quote::quote;



#[proc_macro_derive(IoDeSer)]
pub fn opis_derive_macro(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as DeriveInput);
	let struct_name = &input.ident;


	let mut to_io_string_tokens_implementation = quote!{};
	let mut vector_field_maker = quote!{};
	let mut tokens_from_io = quote!{};

	let mut is_first = true;
	let mut index_of = 0;

	if let syn::Data::Struct(ref data) = input.data {
		if let syn::Fields::Named(ref fields) = data.fields {
			for field in &fields.named {
				if matches!(field.vis, Visibility::Public(_)) {
					let field_type = &field.ty;


					let field_name = field.ident.as_ref();

					let field_name_str = field.ident.as_ref().unwrap().to_string();








					//TODO handle: what if #field_name will have missmatch with variable_and_io_str_value vector
					// (different order of fields in io string, than in result struct)
					tokens_from_io.extend(quote!{
						#field_name: from_io!(variable_and_io_str_value[#index_of as usize][1], #field_type) , //TODO
					});

					vector_field_maker.extend(quote!{
						#field_name_str,
					});


					to_io_string_tokens_implementation.extend(
						quote! {
							string_output += &format!("{}{}{}->{}",
							if !#is_first { "\n" } else { "" },
								(0..tab+1).map(|_| "\t").collect::<String>(),
								#field_name_str,
								IoSerialization::next(&self.#field_name, tab + 1).ser()
							);
						}
					);

				}

				if is_first {
					is_first = false;
				}
				index_of = index_of + 1;
			}
		}
	}



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
							tokens_from_io).into()
}



fn implement_iodeser_trait(struct_name: &Ident, to_io_string_tokens_implementation:proc_macro2::TokenStream
						   , vector_field_maker:proc_macro2::TokenStream, tokens_from_io:proc_macro2::TokenStream)->proc_macro2::TokenStream{
	quote! {
        impl IoDeSer for #struct_name{


			#to_io_string_tokens_implementation



            fn from_io_string(io_input:&mut String)->#struct_name{

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

				let fields = #vector_field_maker;

				while line_pointer < lines.len(){
					let current_line = lines[line_pointer];
					let assignment:Vec<&str> = current_line.split("->").collect();

					if assignment.len() == 0{
						continue;
					}

					let variable_name = assignment[0].trim().to_string();

					let mut found_property = "";

					for f in fields.iter(){
						if variable_name.eq(f){
							found_property = f;
						}
					}


					if found_property==""{
						panic!("Field '{}' was not found in struct '{}'",variable_name, stringify!(#struct_name));
					}

					// TODO this for now only works for primitive values
					variable_and_io_str_value.push(vec![found_property.to_string(), assignment[1].to_string()]);
					// if is primitive or is a string



					// if is a class or array/vector



					line_pointer=line_pointer+1;
				}


				// TODO make 'variable_and_io_str_value' order match fields = #vector_field_maker order

                #struct_name { #tokens_from_io }
            }
        }
    }
}

fn is_array_type(field_type: &syn::Type) -> bool {
	matches!(field_type, syn::Type::Array(_))
}

fn is_slice_type(field_type: &syn::Type) -> bool {
	if let syn::Type::Reference(reference) = field_type {
		if let syn::Type::Slice(_) = &*reference.elem {
			// Jest to referencja do slice'a
			return true;
		}
	}
	false
}