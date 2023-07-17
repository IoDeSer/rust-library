extern crate proc_macro;

use proc_macro::TokenStream;

use syn::{parse_macro_input, DeriveInput, Visibility, Field};
use quote::quote;

#[proc_macro_derive(IoDeSer)]
pub fn opis_derive_macro(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as DeriveInput);
	let struct_name = &input.ident;


	let mut impl_tokens = quote! {
        let mut string_output = String::from("|\n");
        //string_output+=&(0..tab).map(|_| "\t").collect::<String>();
    };

	let mut is_first = true;

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
					impl_tokens.extend(tokens);
				}

				if is_first {
					is_first = false;
				}
			}
		}
	}
	impl_tokens.extend(quote! {
		format!("{}\n{}|",string_output, (0..tab).map(|_| "\t").collect::<String>())
    });


	let tokens = quote! {
        impl IoDeSer<#struct_name> for #struct_name{
            fn to_io_string(self, tab: u8)->String{
                #impl_tokens
            }
            fn from_io_string(io_input:String)->#struct_name{
                todo!()
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
