use syn::{DataEnum, Fields, FieldsNamed, FieldsUnnamed};
use proc_macro2::{Ident, Span, TokenStream};
use syn::{TypeGenerics, WhereClause, ImplGenerics};
use quote::quote;

pub(crate) type EnumType<'a> = Vec<EnumTypes<'a>>;


pub(crate) enum EnumTypes<'a> {
    Unit { name: String },
    Unnamed { name: String, fields: &'a FieldsUnnamed },
    Named { name: String, fields: &'a FieldsNamed },
}

#[allow(dead_code)]
impl<'a> EnumTypes<'a> {
    #[allow(dead_code)]
    fn named_from_io(name: &String, enum_name: &Ident, fields: &FieldsNamed)->TokenStream{
        let n = syn::Ident::new(&name, Span::call_site());
        let len = name.len() + 2;

        let mut fields_token_creator = quote!();
        let mut types_token = quote!();

        let mut iterator:usize = 0;
        for field in &fields.named {
            let field_name = field.ident.clone().unwrap().to_string();
            fields_token_creator.extend(quote!(#field_name,));

            let n2 = field.ident.clone().unwrap();
            let typ = field.ty.clone();
            types_token.extend(quote!{
                #n2: <#typ>::from_io_string(&mut variable_and_io_str_value[#iterator])?,
            });


            iterator+=1;
        }

        let fields_name_token_creator = quote!(let fields_name = vec![#fields_token_creator];);


        quote!(
            {
            use std::fmt::Write;

            let temp = &io_input[#len..];
            *io_input = temp.to_string();

            let mut ret = String::new();
            for line in io_input.lines().filter(|line| line.len() > 1) {
                ret.push_str(&line[1..]);
                ret.push('\n');
            }
            *io_input = ret.trim().to_string();

            let lines:Vec<&str> = io_input.lines().collect();
            let mut line_pointer = 0;

            #fields_name_token_creator //fields_name : Vec<String>
            let mut variable_and_io_str_value = vec![];
            while line_pointer < lines.len(){

                let current_line = lines[line_pointer];
					let assignment:Vec<&str> = current_line.split("->").collect();

					if assignment.len() == 0{
						continue;
					}

                let variable_name = assignment[0].trim().to_string();

                let mut found_property = "";

                for f in &fields_name{
                    if variable_name.eq(f){
                        found_property = f;
                    }
                }


                if found_property==""{
                    return Err(iodeser::Error::field_not_found(variable_name, stringify!(#enum_name).to_string())
                        .into());
                }


                // primitive type
					if assignment[1].len() > 1{
						variable_and_io_str_value.push(assignment[1].to_string());
					}else{ // class / array / vector
						line_pointer=line_pointer+1;

						if lines[line_pointer] == "|"{
							line_pointer=line_pointer+1;
							variable_and_io_str_value.push("|\n\t\n|".to_string());
							continue;
						}

						let new_object_start = line_pointer;

						while lines[line_pointer] != "|" {
							line_pointer = line_pointer+1;
						}

						let new_object_end = line_pointer;
						let mut new_object_string = String::from("|\n");

						for l2 in new_object_start..new_object_end {
                            let _ = writeln!(new_object_string, "{}\n", lines[l2]).expect("UNHANDLED ERROR WHILE WRITE!-ING TO STRING 'new_object_string'");
						}

						new_object_string+="\n|";

						variable_and_io_str_value.push( new_object_string);
					}

                line_pointer=line_pointer+1;
            }

            //#ender_token
            #enum_name::#n {#types_token}
        }
        )
    }

    #[allow(dead_code)]
    fn unnamed_from_io(name: &String, enum_name: &Ident, fields: &FieldsUnnamed)->TokenStream{
        let n = syn::Ident::new(&name, Span::call_site());
        let len = name.len() + 2;
        let types = fields.unnamed.iter().map(|x| &x.ty).collect::<Vec<&syn::Type>>();
        let mut types_token = quote!();
        let mut iterator:usize = 0;
        for typ in types {

            types_token.extend(quote!{
                <#typ>::from_io_string(&mut objects[#iterator].to_string())?, 
            });

            iterator+=1;
        }


        quote!(
            let temp = &io_input[#len..];
            *io_input = temp.to_string();

            let mut ret = String::new();
            for line in io_input.lines().filter(|line| line.len() > 1) {
                ret.push_str(&line[1..]);
                ret.push('\n');
            }
            *io_input = ret.trim().to_string();

            let mut objects: Vec<&str> = io_input.split_terminator("\n+\n").collect();
            if objects.is_empty(){
                if io_input.is_empty(){
                    objects = Vec::new();
                } else{
                    objects = vec![io_input];
                }
            }

            #enum_name::#n(#types_token)
        )
    }

    fn quote_from_unit(name: String, enum_name: &Ident) -> TokenStream {
        let n = syn::Ident::new(&name, Span::call_site());
        quote! {
            #enum_name::#n => {
                let _ = write!(buffer,"{}{}->|||", (0..tab+1).map(|_| "\t").collect::<String>(),#name);
            } //ok use of format
        }
    }

    fn quote_from_unnamed(name: String, fields: &FieldsUnnamed, enum_name: &Ident) -> TokenStream {
        let n = syn::Ident::new(&name, Span::call_site());
        let mut variable_number = 0;
        let mut variables_token = quote!();
        let mut variables_to_io_token = quote!();

        for _ in &fields.unnamed {
            let new_field_ident = Ident::new(&format!("temp_ident{}", variable_number), Span::call_site()); // ok use of format
            variables_token.extend(quote!(#new_field_ident ,));


            if variable_number == 0 {
                variables_to_io_token.extend(quote!(
                    {
                        let mut buffer2 = (0..tab+2).map(|_| "\t").collect::<String>();// todo, remove buffer2, and let variables_to_io_token assign to buffer(1)

                        #new_field_ident.to_io_string(tab+2,&mut  buffer2);
                        buffer2
                    }
                ));
            } else {
                variables_to_io_token.extend(quote!(
                    +"\n"+&(0..tab+2).map(|_| "\t").collect::<String>()+"+\n"+&(0..tab+2).map(|_| "\t").collect::<String>()+
                    {
                        let mut buffer2 = String::new();// todo, remove buffer2, and let variables_to_io_token assign to buffer(1)

                        #new_field_ident.to_io_string(tab+2,&mut buffer2);
                        &buffer2.clone()
                    }
                ));
            }

            variable_number += 1;
        }


        quote! {
            #enum_name::#n( #variables_token ) => { 

                let _ = write!(buffer, "{}{}->|\n{}\n{}|", (0..tab+1).map(|_| "\t").collect::<String>(), #name, #variables_to_io_token ,&(0..tab+1).map(|_| "\t").collect::<String>());
        }
    }

    }

    fn quote_from_named(name: String, fields: &FieldsNamed, enum_name: &Ident) -> TokenStream {
        let mut tokens_names_impl = quote!();
        let n = syn::Ident::new(&name, Span::call_site());
        let mut tokens = quote!();

        let mut iterator = 0;

        for field in &fields.named {
            let n = field.ident.clone().unwrap().to_string();
            let n_ident = field.ident.clone().unwrap();

            if iterator > 0 {
                tokens.extend(
                    quote! {
                        let _ = write!(buffer, "\n{}", &_tab_more);
                    }
                );

                tokens_names_impl.extend(quote!(,));
            }

            tokens_names_impl.extend(quote!(#n_ident));


            tokens.extend(
                quote! {
                    let _ = write!(buffer, "{}->", &#n);
                    #n_ident.to_io_string(tab+2,buffer);
                }
            );

            iterator += 1;
        }

        quote! {
            #enum_name::#n{ #tokens_names_impl } => {
                    let _tab = (0..tab+1).map(|_| "\t").collect::<String>();
                    let _tab_more = (0..tab+2).map(|_| "\t").collect::<String>();
                    let _ = write!(buffer,"{}{}->|\n{}", &_tab, #name, &_tab_more);
                    #tokens
                    let _ = write!(buffer, "\n{}|", &_tab);
            }
        }
    }

    fn quote_from_enum(&self, enum_name: &Ident) -> TokenStream {
        match self {
            EnumTypes::Unit { name } => EnumTypes::quote_from_unit(name.to_string(), enum_name),
            EnumTypes::Unnamed { name, fields } => EnumTypes::quote_from_unnamed(name.to_string(), fields, enum_name),
            EnumTypes::Named { name, fields } => EnumTypes::quote_from_named(name.to_string(), fields, enum_name),
        }
    }
}


pub(crate) fn create_from_enum(data: &DataEnum) -> Vec<EnumTypes> {
    let mut enum_types = vec![];

    for variant in &data.variants {
        match &variant.fields {
            Fields::Named(named) => enum_types.push(EnumTypes::Named { name: variant.ident.to_string().clone(), fields: named }),
            Fields::Unnamed(unnamed) => enum_types.push(EnumTypes::Unnamed { name: variant.ident.to_string().clone(), fields: unnamed }),
            Fields::Unit => enum_types.push(EnumTypes::Unit { name: variant.ident.to_string().clone() }),
        }
    }

    enum_types
}

pub(crate) fn handle_enum(enums_fields: EnumType, enum_name: &Ident,
                          impl_generics: &ImplGenerics, ty_generics: &TypeGenerics, where_clause: &Option<&WhereClause>) -> proc_macro2::TokenStream {

    let to_io_token_implementation = to_io_token_implementation(&enums_fields, enum_name);
    let from_io_token_implementation = from_io_token_implementation(&enums_fields, enum_name);


    quote! {
		#[automatically_derived]
        impl #impl_generics IoDeSer for #enum_name #ty_generics #where_clause {

            #to_io_token_implementation

            #from_io_token_implementation
        }
    }
}

fn from_io_token_implementation(enums_fields: &EnumType, enum_name: &Ident) -> TokenStream {
    let mut vector_fields_token = quote!();
    let mut iterator = 0;

    for enum_field in enums_fields {
        let codition_token = if iterator == 0{
            quote!(if)
        }else{
            quote!(else if)
        };

        match enum_field {
            EnumTypes::Unit { name } => {
                let n = syn::Ident::new(&name, Span::call_site());
                vector_fields_token.extend(quote!(
                #codition_token #name == enum_selected_field{
                    #enum_name::#n
                }
            ))}
            EnumTypes::Unnamed { name, fields } => {
                let from_io_token = EnumTypes::unnamed_from_io(name, enum_name, fields);
                vector_fields_token.extend(quote!(
                #codition_token #name == enum_selected_field{
                        #from_io_token
                }
            ))}
            EnumTypes::Named { name, fields } => {
                let from_io_token = EnumTypes::named_from_io(name, enum_name, fields);

                vector_fields_token.extend(quote!(
                #codition_token #name == enum_selected_field{
                        #from_io_token
                }
            ))}
        }

        iterator+=1;
    }


    quote! {
        fn from_io_string(io_input:&mut String)->iodeser::Result<Self>{
            // DELETE TABULATOR


            if !io_input.starts_with('|') || !io_input.ends_with('|') {
                return Err(iodeser::Error::io_format(io_input.clone(),"String lacks vertical bars at the beginning or end".to_string())
                    .into());
            }

            let mut previous_was_newline = true;
            io_input.retain(|c| {

                if previous_was_newline{
                    previous_was_newline = false;
                    return false;
                }

                if c=='\n'{
                    previous_was_newline = true;
                    return true;
                }

                true
            });

            // Remove the first and last vertical bars
            io_input.remove(0); // Remove the first '|'
            io_input.pop(); // Remove the last '|'

            // DELETE TABULATOR
            let enum_selected_field = io_input.split("->").next().unwrap(); //TODO delete unwrap

            return Ok(#vector_fields_token
            else{
                panic!("field not found in enum") // TODO better error
            });
        }
    }
}

fn to_io_token_implementation(enums_fields: &EnumType, enum_name: &Ident) -> TokenStream {
    let enum_match_statement: TokenStream = enums_fields
        .iter()
        .map(|enum_field| enum_field.quote_from_enum(enum_name))
        .collect();

    quote!(
        fn to_io_string(&self, tab: u8, buffer: &mut String){
            {
                use std::fmt::Write;
                let _ = write!(buffer, "|\n");
                match &self {
                    #enum_match_statement
                };
                let _ = write!(buffer, "\n{}|", (0..tab).map(|_| "\t").collect::<String>());
            }
        }
    )
}