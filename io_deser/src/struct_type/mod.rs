use std::fmt::{Display, Formatter};
use proc_macro2::{Ident, TokenStream, Literal};
use quote::{quote, ToTokens};
use syn::{Field, ImplGenerics, Type, TypeGenerics, WhereClause};
use crate::fields_renaming::parse_fields_naming;
use std::fmt::Write;

pub(crate) struct TupleType<'a>{
    pub object_type: &'a Type,
    pub is_public: bool
}

pub(crate) enum StructType<'a> {
    NamedFields{publics: Vec<crate::FieldOrder<'a>>, privates: Vec<&'a Field>},
    Tuple(Vec<TupleType<'a>>),
}

impl <'a> Display for StructType<'a>{
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", match self{
			StructType::NamedFields { publics, privates } => format!("{:?}\t{}", publics,privates.len()),
			StructType::Tuple(f) => {
				let mut ret = String::new();
				for x in f{
					let _ = writeln!(ret, "{:?}", &x.object_type.into_token_stream());
				}
				ret
			},
		})
	}
}



pub(crate) enum IterType<'a> {
    Field(crate::FieldOrder<'a>),
    Type(TupleType<'a>),
}

impl <'a> Display for IterType<'a>{
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", match self{
			IterType::Field(f) => format!("{:?}", f),
			IterType::Type(t) => format!("{:?}", t.object_type.into_token_stream())
		})
	}
}

impl<'a> Iterator for StructType<'a> {
    type Item = IterType<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            StructType::NamedFields { publics, privates: _ } =>{
				if publics.is_empty(){
					return None;
				}

				Some(IterType::Field(publics.remove(0)))
			},
            StructType::Tuple(f) => {
				if f.is_empty(){
					return None;
				}
				Some(IterType::Type(f.remove(0)))
			},
		}
    }

    fn count(self) -> usize
        where
            Self: Sized,
    {
        match self {
            StructType::NamedFields { publics, privates:_ } => publics.len(),
            StructType::Tuple(f)=> f.len(),
        }
    }
}


pub(crate) fn handle_struct(fields_order: StructType, struct_name: &Ident,
				 impl_generics: &ImplGenerics, ty_generics: &TypeGenerics, where_clause: &Option<&WhereClause>) -> proc_macro2::TokenStream {
	let mut to_io_string_tokens_implementation = quote! {};
	let mut _vector_field_maker = quote! {};
	let mut _struct_return_definition = quote! {};
	let mut index_of = 0;

	let is_tuple_struct = match &fields_order {
		StructType::NamedFields { publics:_, privates:_ } => false,
		StructType::Tuple(f) => {
            // lenght of the elements with property is_public == false
            let public_len = f.iter().filter(|x| x.is_public).count();
			_vector_field_maker = quote! {#public_len};
			true
		}
	};



    // create deserializer for private fields (using $type::default())
    // only if fields_order is NamedFields
    if let StructType::NamedFields { publics:_, privates } = &fields_order {
        for private_field in privates{
            let field_type = &private_field.ty;
			let field_name = private_field.ident.as_ref();

            _struct_return_definition.extend(quote! {
                #field_name: #field_type::default(),
            });
        }
    } 

    // public fields
	for field_type in fields_order {
		match field_type {

			// IF NAMED STRUCT
			IterType::Field(f) => {
				let field = f.field;
				let field_type = &field.ty;
				let field_name = field.ident.as_ref();
				let field_name_str = field.ident.as_ref().unwrap().to_string();

				let (field_name_setter, option_field_file_name) = parse_fields_naming(&field, struct_name);


				// vector with real field name and otional renaming    vec![(in_rust_real_name, optional_renaming), (...)]
				_vector_field_maker.extend(quote! {
					(#field_name_str, #option_field_file_name),
				});


				// field initialization inside struct definition with: its_name: deserialized_io_string
				_struct_return_definition.extend(quote! {
					#field_name: <#field_type>::from_io_string(&mut variable_and_io_str_value[#index_of as usize])?,
				});


				to_io_string_tokens_implementation.extend(
					quote! {
						{
						use std::fmt::Write;

						let _ = write!(buffer, "{}{}{}->",
							if #index_of > 0 { "\n" } else { "" },
							(0..tab+1).map(|_| "\t").collect::<String>(),
							#field_name_setter,					
						);
						self.#field_name.to_io_string(tab + 1, buffer);
					}
					}
				);
			} // IF NAMED STRUCT

			// IF STRUCT TUPLE
			IterType::Type(t) => {
				let field_type = t.object_type;

                if t.is_public{
                    _struct_return_definition.extend(quote! {
						<#field_type>::from_io_string(&mut variable_and_io_str_value[#index_of as usize])?, 
                    });

                    let _suffix = Literal::usize_unsuffixed(index_of);
                    to_io_string_tokens_implementation.extend(
                        quote! {
                        {
                            use std::fmt::Write;

                            if #index_of > 0{
                                let _ = write!(buffer, "\n{}+\n{}",(0..tab+1).map(|_| "\t").collect::<String>(),(0..tab+1).map(|_| "\t").collect::<String>());
                            }else{
                                let _ = write!(buffer, "{}", (0..tab+1).map(|_| "\t").collect::<String>());
                            }
                            self.#_suffix.to_io_string(tab+1, buffer);
                        }
                        }
                    );
                }else{
                    _struct_return_definition.extend(quote! {
                        #field_type::default(),
                    });
                }
			}
		}


		index_of = index_of + 1;
	}


	// final token initialization of vector with field names / optional renamings

	_struct_return_definition = match is_tuple_struct {
		false => {
			_vector_field_maker = quote! {vec![#_vector_field_maker]};

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
							impl_generics, ty_generics, where_clause,
							_deserialization_implementation).into()
}


fn implement_iodeser_trait(struct_name: &Ident, to_io_string_tokens_implementation: proc_macro2::TokenStream
						   , _struct_return_definition: proc_macro2::TokenStream,
						   impl_generics: &ImplGenerics, ty_generics: &TypeGenerics, where_clause: &Option<&WhereClause>,
						   _deserialization_implementation: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
	quote! {
		#[automatically_derived]
        impl #impl_generics IoDeSer for #struct_name #ty_generics #where_clause {


			fn to_io_string(&self, tab: u8, buffer: &mut String){
				{
					use std::fmt::Write;
					let _ = write!(buffer, "|\n");
					#to_io_string_tokens_implementation
					let _ = write!(buffer,"\n{}|", (0..tab).map(|_| "\t").collect::<String>()); // todo, apply format to buffer
				}
            }


            fn from_io_string(io_input:&mut String)->iodeser::Result<Self>{
				if !io_input.starts_with('|') || !io_input.ends_with('|') {
					return Err(iodeser::Error::io_format (
						io_input.clone(),
						"String lacks vertical bars at the beginning or end".to_string(),
					).into());
				}


				// DELETE TABULATOR
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


				
				let mut variable_and_io_str_value = Vec::<String>::new();
				#_deserialization_implementation

                Ok(#_struct_return_definition)
            }
        }
    }
}

fn de_from_struct_type(is_tuple_struct:bool, _vector_field_maker:proc_macro2::TokenStream, struct_name: &Ident)->TokenStream{
    if is_tuple_struct{
        quote!{
            let mut objects: Vec<&str> = io_input.split_terminator("\n+\n").collect();
                if objects.is_empty(){
                    if io_input.is_empty(){
                        objects = Vec::new();
                    } else{
                        objects = vec![io_input];
                    }
                }

			if &#_vector_field_maker != &objects.len(){
                    return Err(iodeser::Error::length_error(objects.len(),#_vector_field_maker).into());
			}

            for o in objects{
                variable_and_io_str_value.push(o.to_string());
            }
        }
    }else {
        quote!{
			{
			use std::fmt::Write;
				let lines:Vec<&str> = io_input.lines().collect();
				let mut line_pointer = 0;

				let fields:Vec<(&str, Option<&str>)> = #_vector_field_maker;

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
						return Err(iodeser::Error::field_not_found(variable_name, stringify!(#struct_name).to_string())
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
							let _ = writeln!(new_object_string,"{}", lines[l2]);
						}

						new_object_string+="\n|";

						variable_and_io_str_value.push( new_object_string);
					}

					line_pointer=line_pointer+1;
				}
        }
	}
    }
}
