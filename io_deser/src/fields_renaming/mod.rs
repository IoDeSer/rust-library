use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::Field;
use crate::quote;
use crate::Ident;
use crate::Literal;

pub(crate) fn parse_fields_naming(field: &Field, struct_name: &Ident) -> (TokenStream, TokenStream){
    let field_name_str = field.ident.as_ref().unwrap().to_string();

    // default values, if field was not named using #[io_name(...)]
    let mut field_name_setter = quote!{#field_name_str};
    let mut option_field_file_name = quote!{None};


    // helper attributes for changing order and name
    for attribute in &field.attrs{
        match attribute.path().get_ident().unwrap().to_string().as_str(){
            "io_name"=>{
                if attribute.to_token_stream().is_empty(){
                    panic!("The 'io_name' macro in the struct '{}' for the field '{}' expects exactly one String argument, but none were provided.", struct_name, field_name_str)
                }

                let new_field_name = attribute.parse_args::<Literal>()
                    .expect(&format!("The 'io_name' macro in the struct '{}' for the field '{}' expected exactly one String argument (check: '{}'), but more were provided or in the wrong format.",
                                     struct_name.to_string(),
                                     field_name_str,
                                     &attribute.to_token_stream()));

                field_name_setter = quote!{#new_field_name};
                option_field_file_name = quote!{Some(#new_field_name)};
            }
            _=>{}
        }
    }


    (field_name_setter, option_field_file_name)
}