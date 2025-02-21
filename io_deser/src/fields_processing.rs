use proc_macro::TokenStream;
use syn::{DataStruct, DeriveInput, FieldsNamed, Visibility};
use quote::quote_spanned;
use syn::spanned::Spanned;

use crate::{
    enum_type::{create_from_enum, EnumType},
    struct_type::{StructType, TupleType},
    FieldOrder,
};

pub(crate) fn create_fields_from_data(input: &DeriveInput) -> (ReturnType, TokenStream) {
    let mut errors = Vec::new();

    if let syn::Data::Struct(ref data) = input.data {
        (process_struct(data, input, &mut errors), errors.into_iter().collect())
    } else if let syn::Data::Enum(ref data) = input.data {
        (ReturnType::Enum(create_from_enum(data)), errors.into_iter().collect())
    } else {
        panic!();
        // quote!{
        //     compile_error!("This data type is not supported by IoDeSer attribute.")
        // }.into())
    }
}

pub(crate) enum ReturnType<'a> {
    Struct(StructType<'a>),
    Enum(EnumType<'a>),
    Unit
}

fn process_struct<'a>(struct_data: &'a DataStruct, input: &'a DeriveInput, errors: &mut Vec<TokenStream>) -> ReturnType<'a> {
    match &struct_data.fields {
        syn::Fields::Named(fields_named) => process_named(fields_named, input, errors), // struct X{}
        syn::Fields::Unnamed(fields_unnamed) => process_unnamed(fields_unnamed), // struct X()
        syn::Fields::Unit => ReturnType::Unit, // struct X;
    }
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

fn process_named<'a>(fields: &'a FieldsNamed, input: &'a DeriveInput, errors: &mut Vec<TokenStream>) -> ReturnType<'a> {
    let (public_fields, private_fields) = sort_field_by_visibility(fields, input, errors);

    ReturnType::Struct(StructType::NamedFields {
        publics: public_fields,
        privates: private_fields,
    })
}

fn sort_field_by_visibility<'a>(fields: &'a FieldsNamed, input: &'a DeriveInput, errors: &mut Vec<TokenStream>) -> (Vec<FieldOrder<'a>>, Vec<&'a syn::Field>) {
    let (mut public_fields, private_fields): (Vec<FieldOrder<'_>>, Vec<_>) =
        fields
        .named
        .iter()
        .fold(

            // init public and private with empty array
            (Vec::new(), Vec::new()),

            // apply function, that pushes to public/private based on the field visibility
            |(public, private), field| {
                classify_fields(field, input, (public, private), errors)
            },
        );

    public_fields.sort();

    (public_fields, private_fields)
}

fn classify_fields<'a>(field: &'a syn::Field, input: &'a DeriveInput, mut fields: (Vec<FieldOrder<'a>>, Vec<&'a syn::Field>), errors: &mut Vec<TokenStream>) -> (Vec<FieldOrder<'a>>, Vec<&'a syn::Field>) {

    // .0 for public | .1 for private
    if let Visibility::Public(_) = field.vis {

        if is_public_ignored(field){
            check_attributes_errors(field, false, true, errors);
            fields.1.push(field);
        }else{
            check_attributes_errors(field, true, true, errors);
            fields.0.push(FieldOrder::new(field, &input.ident));
        }
    } else {

        if is_private_allowed(field){
            check_attributes_errors(field, true, false, errors);
            fields.0.push(FieldOrder::new(field, &input.ident));
        }else{
            check_attributes_errors(field, false, false, errors);
            fields.1.push(field);
        }
    }

    fields
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

fn check_attributes_errors(field: &syn::Field, should_be_serialized: bool, is_public: bool, errors: &mut Vec<TokenStream>){
    if is_public{
        if field_has_attribute(field, "io_allow"){
            errors.push(quote_spanned!{
                field.span() =>
                compile_error!("Public fields should not have attibute #[io_allow].");
            }.into());
        }
    }else{
        if field_has_attribute(field, "io_ignore"){
            errors.push(quote_spanned!{
                field.span() =>
                compile_error!("Private fields should not have attibute #[io_ignore].");
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
                errors.push(quote_spanned!{
                    field.span() => // TODO attrubite span, not field
                    compile_error!("Private fields should not have attibute #[io_order]");
                }.into());
            }else{
                errors.push(quote_spanned!{
                    field.span() =>
                    compile_error!("Private fields should not have attibute #[io_order]");
                }.into());
            }
        }

        // renaming is unnecesarry
        if field_has_attribute(field, "io_name"){
            if is_public{
                errors.push(quote_spanned!{
                    field.span() =>
                    compile_error!("Ignored fields should not have attibute #[io_name]");
                }.into());
            }else{
                errors.push(quote_spanned!{
                    field.span() =>
                    compile_error!("Private fields should not have attibute #[io_name]");
                }.into());
            }
        }
    }
}