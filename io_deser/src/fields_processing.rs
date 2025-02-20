use proc_macro::TokenStream;
use syn::{DataStruct, DeriveInput, FieldsNamed, Visibility};
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;

use crate::{
    enum_type::{create_from_enum, EnumType},
    struct_type::{StructType, TupleType},
    FieldOrder,
};

pub(crate) fn create_fields_from_data(input: &DeriveInput) -> Result<ReturnType, TokenStream> {
    if let syn::Data::Struct(ref data) = input.data {
        Ok(process_struct(data, input)?)
    } else if let syn::Data::Enum(ref data) = input.data {
        Ok(ReturnType::Enum(create_from_enum(data)))
    } else {
        Err(quote!{
            compile_error!("This data type is not supported by IoDeSer attribute.")
        }.into())
    }
}

pub(crate) enum ReturnType<'a> {
    Struct(StructType<'a>),
    Enum(EnumType<'a>),
    Unit
}

fn process_struct<'a>(struct_data: &'a DataStruct, input: &'a DeriveInput) -> Result<ReturnType<'a>, TokenStream> {
    Ok(match &struct_data.fields {
        syn::Fields::Named(fields_named) => process_named(fields_named, input)?, // struct X{}
        syn::Fields::Unnamed(fields_unnamed) => process_unnamed(fields_unnamed), // struct X()
        syn::Fields::Unit => ReturnType::Unit, // struct X;
    })
}

fn process_unnamed<'a>(fields: &'a syn::FieldsUnnamed) -> ReturnType<'a> {
    ReturnType::Struct(StructType::Tuple(
        fields
            .unnamed
            .iter()
            .filter_map(|f| {
                if let Visibility::Public(_) = f.vis {
                    Some(TupleType {
                        object_type: &f.ty,
                        is_public: true,
                    })
                } else {
                    Some(TupleType {
                        object_type: &f.ty,
                        is_public: false,
                    })
                }
            })
            .collect::<Vec<TupleType>>(),
    ))
}

fn process_named<'a>(fields: &'a FieldsNamed, input: &'a DeriveInput) -> Result<ReturnType<'a>, TokenStream> {
    let (public_fields, private_fields) = sort_field_by_visibility(fields, input)?;

    Ok(ReturnType::Struct(StructType::NamedFields {
        publics: public_fields,
        privates: private_fields,
    }))
}

fn sort_field_by_visibility<'a>(fields: &'a FieldsNamed, input: &'a DeriveInput) -> Result<(Vec<FieldOrder<'a>>, Vec<&'a syn::Field>), TokenStream> {
    let (mut public_fields, private_fields): (Vec<FieldOrder<'_>>, Vec<_>) =
        fields
        .named
        .iter()
        .try_fold(

            // init public and private with empty array
            (Vec::new(), Vec::new()),

            // apply function, that pushes to public/private based on the field visibility
            |(public, private), field| {
                classify_fields(field, input, (public, private))
            },
        )?;

    public_fields.sort();

    Ok((public_fields, private_fields))
}

fn classify_fields<'a>(field: &'a syn::Field, input: &'a DeriveInput, mut fields: (Vec<FieldOrder<'a>>, Vec<&'a syn::Field>)) -> Result<(Vec<FieldOrder<'a>>, Vec<&'a syn::Field>),TokenStream> {

    // .0 for public | .1 for private
    if let Visibility::Public(_) = field.vis {

        if is_public_ignored(field){
            check_attributes_errors(field, false, true)?;
            fields.1.push(field);
        }else{
            check_attributes_errors(field, true, true)?;
            fields.0.push(FieldOrder::new(field, &input.ident));
        }
    } else {

        if is_private_allowed(field){
            check_attributes_errors(field, true, false)?;
            fields.0.push(FieldOrder::new(field, &input.ident));
        }else{
            check_attributes_errors(field, false, false)?;
            fields.1.push(field);
        }
    }

    Ok(fields)
}

fn is_public_ignored(public_field: &syn::Field) -> bool {
    field_has_attribute(public_field, "io_ignore")
}

fn is_private_allowed(private_field: &syn::Field) -> bool {
    field_has_attribute(private_field, "io_allow")
}

fn field_has_attribute(field: &syn::Field, attribute_name: &str) -> bool {
    field.attrs.iter().any(
        |attr| attr.path().is_ident(attribute_name)
    )
}

fn check_attributes_errors(field: &syn::Field, should_be_serialized: bool, is_public: bool)->Result<(), TokenStream>{
    if is_public{
        if field_has_attribute(field, "io_allow"){
            return Err(quote_spanned!{
                field.span() =>
                compile_error!("Public fields should not have attibute #[io_allow].")
            }.into());
        }
    }else{
        if field_has_attribute(field, "io_ignore"){
            return Err(quote_spanned!{
                field.span() =>
                compile_error!("Private fields should not have attibute #[io_ignore].")
            }.into());
        }
    }


    if should_be_serialized{ 
        // allowed private or public


    }else{ 
        // should not be serialized: private or ignored public

        // ordering is unnecessary
        if field_has_attribute(field, "io_order"){
            if is_public{
                return Err(quote_spanned!{
                    field.span() =>
                    compile_error!("Private fields should not have attibute #[io_order]")
                }.into());
            }else{
                return Err(quote_spanned!{
                    field.span() =>
                    compile_error!("Private fields should not have attibute #[io_order]")
                }.into());
            }
        }

        // renaming is unnecesarry
        if field_has_attribute(field, "io_name"){
            if is_public{
                return Err(quote_spanned!{
                    field.span() =>
                    compile_error!("Ignored fields should not have attibute #[io_name]")
                }.into());
            }else{
                return Err(quote_spanned!{
                    field.span() =>
                    compile_error!("Private fields should not have attibute #[io_name]")
                }.into());
            }
        }
    }

    Ok(())
}