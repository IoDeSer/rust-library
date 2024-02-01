extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, Visibility};
use quote::quote;

#[proc_macro_derive(IoDeSer)]
pub fn opis_derive_macro(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as DeriveInput);
	let struct_name = &input.ident;


	let mut impl_tokens = quote! {
        let mut string_output = String::from("|\n");
        //string_output+=&(0..tab).map(|_| "\t").collect::<String>();
    };

	let mut tokens_from_io = quote!{};

	let mut is_first = true;

	let mut vector_field_maker = quote!{};


	let mut token_for_this_field = quote!{};

	if let syn::Data::Struct(ref data) = input.data {
		if let syn::Fields::Named(ref fields) = data.fields {
			for field in &fields.named {
				if matches!(field.vis, Visibility::Public(_)) {
					let field_type = &field.ty;


					let field_name = field.ident.as_ref();

					let field_name_str = field.ident.as_ref().unwrap().to_string();

					let tokens = quote! {
							string_output += &format!("{}{}{}->{}",
							if !#is_first { "\n" } else { "" },
								(0..tab+1).map(|_| "\t").collect::<String>(),
								#field_name_str,
								IoSerialization::next(self.#field_name, tab + 1).ser()
							);
						};


						token_for_this_field.extend(quote! {
							#field_name_str => Some(from_io!(assignment[1].to_string(),#field_type)),
							}
						);






					tokens_from_io.extend(quote!{
						//#field_name: from_io!(io_input, #field_type) , //TODO
						#field_name: #field_type::default(),
					});

					vector_field_maker.extend(quote!{
						#field_name_str,
					});

					impl_tokens.extend(tokens);
				}

				if is_first {
					is_first = false;
				}
			}
		}
	}

	let token_for_this_field = quote!{
		match found_property{
			#token_for_this_field
			_=>None
		}
	};

	vector_field_maker = quote!{vec![#vector_field_maker]};
	impl_tokens.extend(quote! {
		format!("{}\n{}|",string_output, (0..tab).map(|_| "\t").collect::<String>())
    });


	let tokens = quote! {
        impl IoDeSer<#struct_name> for #struct_name{
            fn to_io_string(self, tab: u8)->String{
                #impl_tokens
            }
            fn from_io_string(io_input:&mut String)->#struct_name{
				//println!("{:?}", &io_input);

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

					let value = #token_for_this_field;
					// if is primitive or is a string



					// if is a class or array/vector



					line_pointer=line_pointer+1;
				}

				//println!("{:?}", &io_input);
                #struct_name { #tokens_from_io }
            }
        }
    };

	tokens.into()
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