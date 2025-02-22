use proc_macro2::{Ident, TokenStream};
use syn::{TypeGenerics, WhereClause, ImplGenerics};
use quote::quote;

pub(crate) fn handle_unit(
    name: &Ident,
    impl_generics: &ImplGenerics,
    ty_generics: &TypeGenerics,
    where_clause: &Option<&WhereClause>,
) -> TokenStream {

	let n = name.to_string();

    quote! {
		#[automatically_derived]
        impl #impl_generics IoDeSer for #name #ty_generics #where_clause {


			fn to_io_string(&self, tab: u8, buffer: &mut String){
				{
					use std::fmt::Write;
					let _ = write!(buffer,"|||");
				}
            }


            fn from_io_string(io_input:&mut String)->iodeser::Result<Self>{

				if !io_input.starts_with('|') || !io_input.ends_with('|') {
					return Err(iodeser::Error::io_format (
						io_input.clone(),
						"String lacks vertical bars at the beginning or end".to_string(),
					).into());
				}

                if io_input == "|||"{
					return Ok(#name);
				}

				return Err(iodeser::Error::io_format (
					io_input.clone(),
					format!("Enum struct \"{}\" could not be deserialized. It should be represented by \"|||\". Perhaps there was an error while serializing or reading buffer?", #n),
				).into())
            }
        }
    }
}
