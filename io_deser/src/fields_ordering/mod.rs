use syn::{Attribute, Expr, Field};
use std::cmp::Ordering;
use proc_macro2::Ident;
use std::fmt::{Debug, Formatter};

pub struct FieldOrder<'a>{
    pub field:&'a Field,
    place:i16,
}

impl <'a> Debug for FieldOrder<'a>{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{} [{}]", &self.field.ident.as_ref().unwrap().to_string(), self.place )
    }
}


impl Eq for FieldOrder<'_> {}

impl PartialEq<Self> for FieldOrder<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.place.eq(&other.place)
    }
}

impl PartialOrd<Self> for FieldOrder<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.place.partial_cmp(&other.place)
    }
}

impl Ord for FieldOrder<'_>{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.place.cmp(&other.place)
    }
}


impl <'a> FieldOrder<'a>{
    pub(crate) fn new(field:&'a Field, struct_name: &Ident)->FieldOrder<'a>{
        let field_name_str = field.ident.as_ref().unwrap().to_string();

        for attribute in &field.attrs{
            match attribute.path.get_ident().unwrap().to_string().as_str(){
                "io_order"=>{
                    if attribute.tokens.is_empty(){
                        panic!("The 'io_order' macro in the struct '{}' for the field '{}' expects exactly one Integer argument, but none were provided.", struct_name, field_name_str)
                    }

                    return match try_from_ordering_ident(&attribute, &struct_name, &field_name_str){
                        Ok(order)=> FieldOrder{ field, place: order },
                        Err(_)=> FieldOrder{field, place: try_from_ordering_integer(&attribute, &struct_name,&field_name_str)}
                    };
                }
                _=>{}
            }
        }
        FieldOrder{ field, place: 0 }
    }
}

fn try_from_ordering_integer(attr: &Attribute, struct_name: &Ident,field_name_str:&str)->i16 {
    let field_order_expression = attr.parse_args::<Expr>()
        .expect(&format!("The 'io_order' macro in the struct '{}' for the field '{}' expected exactly one Integer argument (check: '{}'), but more were provided or in the wrong format.",
                         struct_name.to_string(),
                         field_name_str,
                         &attr.tokens));

    // Ewaluacja Expr jako Integer
    match field_order_expression {
        Expr::Lit(expr_lit) => {
            if let syn::Lit::Int(lit_int) = expr_lit.lit {
                lit_int.base10_parse::<i16>()
                    .expect(&format!("Failed to parse the integer value of 'io_order' macro for struct '{}' and field '{}'", struct_name, field_name_str))
            } else {
                panic!("The 'io_order' macro in the struct '{}' for the field '{}' expects an Integer argument, but a different literal type was provided (check: {}).", struct_name, field_name_str, &attr.tokens);
            }
        }
        _ => {
            panic!("The 'io_order' macro in the struct '{}' for the field '{}' expects an Integer argument, but a different expression type was provided (check: {}).", struct_name, field_name_str, &attr.tokens);
        }
    }
}


fn try_from_ordering_ident(attr: &Attribute, struct_name: &Ident,field_name_str:&str)->std::result::Result<i16, syn::Error>{
    match attr.parse_args::<Ident>(){
        Ok(ordering) =>{
            match ordering.to_string().as_str(){
                "FIRST" | "first" => Ok(<i16>::MIN),
                "LAST" | "last" => Ok(<i16>::MAX),
                _=> panic!("The 'io_order' macro in the struct '{}' for the field '{}' expects exactly one Integer argument or Idents 'FIRST' or 'LAST'.", struct_name, field_name_str)
            }
        },
        Err(e) =>Err(e)
    }
}