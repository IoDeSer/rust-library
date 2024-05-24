use std::fmt::{Display, Formatter};
use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use syn::{Type};
use crate::enum_type::{EnumType};



pub(crate) enum ReturnType<'a>{
	Struct(StructType<'a>),
	Enum(EnumType<'a>)
}

pub(crate) enum StructType<'a> {
    NamedFields(Vec<crate::FieldOrder<'a>>),
    Tuple(Vec<&'a Type>),
}

impl <'a> Display for StructType<'a>{
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", match self{
			StructType::NamedFields(f) => format!("{:?}", f),
			StructType::Tuple(t) => {
				let mut ret = String::new();
				for x in t{
					ret += &format!("{:?}\n", &x.into_token_stream());
				}
				ret
			},
		})
	}
}



pub(crate) enum IterType<'a> {
    Field(crate::FieldOrder<'a>),
    Type(&'a Type),
}

impl <'a> Display for IterType<'a>{
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", match self{
			IterType::Field(f) => format!("{:?}", f),
			IterType::Type(t) => format!("{:?}", t.into_token_stream())
		})
	}
}

impl<'a> Iterator for StructType<'a> {
    type Item = IterType<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            StructType::NamedFields(f) =>{
				if f.is_empty(){
					return None;
				}

				Some(IterType::Field(f.remove(0)))
			},
            StructType::Tuple(t) => {
				if t.is_empty(){
					return None;
				}
				Some(IterType::Type(t.remove(0)))
			},
		}
    }

    fn count(self) -> usize
        where
            Self: Sized,
    {
        match self {
            StructType::NamedFields(f) => f.len(),
            StructType::Tuple(t) => t.len(),
        }
    }
}

pub(crate) fn de_from_struct_type(is_tuple_struct:bool, _vector_field_maker:proc_macro2::TokenStream, struct_name: &Ident)->TokenStream{
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
							new_object_string += &format!("{}\n", lines[l2]);
						}

						new_object_string+="\n|";

						variable_and_io_str_value.push( new_object_string);
					}

					line_pointer=line_pointer+1;
				}
        }
    }
}
