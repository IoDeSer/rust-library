

use serde::{ser, Serialize};
use std::fmt::Display;
use crate::errors::IoFormatError;

impl ser::Error for crate::Error {
    fn custom<T: Display>(msg: T) ->
     Self {
        crate::Error::IoFormatError(IoFormatError{io_input:msg.to_string(),kind: msg.to_string()})
    }
}

pub struct IoSerializer{
    pub output: String,
    pub depth:i32
}

#[cfg(feature = "serde")]
pub fn to_string<T: ::serde::Serialize>(value: &T) -> crate::Result<String> {
    let mut serializer = IoSerializer {
        output: String::new(),
        depth: 0,
    };
    
    // To uruchamia całą "magię" Serde
    value.serialize(&mut serializer)?;
    
    Ok(serializer.output)
}

impl<'a> ser::Serializer for &'a mut IoSerializer {

    type Ok = (); 
    type Error = crate::Error; // Ensure this points to your Error type

    // Use Impossible for things you aren't ready to implement yet
    type SerializeSeq = ser::Impossible<(), Self::Error>;
    type SerializeTuple = ser::Impossible<(), Self::Error>;
    type SerializeTupleStruct = ser::Impossible<(), Self::Error>;
    type SerializeTupleVariant = ser::Impossible<(), Self::Error>;
    type SerializeMap = ser::Impossible<(), Self::Error>;
    type SerializeStruct = ser::Impossible<(), Self::Error>;
    type SerializeStructVariant = ser::Impossible<(), Self::Error>;

    fn serialize_bool(self, v: bool) -> Result<(), Self::Error> {
        self.output.push_str(&format!("|{}|", v));
        Ok(())
    }

    fn serialize_i32(self, v: i32) -> Result<(), Self::Error> {
        self.output.push_str(&format!("|{}|", v));
        Ok(())
    }

    fn serialize_str(self, v: &str) -> Result<(), Self::Error> {
        self.output.push_str(&format!("|{}|", v));
        Ok(())
    }

    // --- Primitives (Add your format logic here later) ---

    fn serialize_i8(self, v: i8) -> Result<(), Self::Error> { todo!() }
    fn serialize_i16(self, v: i16) -> Result<(), Self::Error> { todo!() }
    fn serialize_i64(self, v: i64) -> Result<(), Self::Error> { todo!() }
    fn serialize_u8(self, v: u8) -> Result<(), Self::Error> { todo!() }
    fn serialize_u16(self, v: u16) -> Result<(), Self::Error> { todo!() }
    fn serialize_u32(self, v: u32) -> Result<(), Self::Error> { todo!() }
    fn serialize_u64(self, v: u64) -> Result<(), Self::Error> { todo!() }
    fn serialize_f32(self, v: f32) -> Result<(), Self::Error> { todo!() }
    fn serialize_f64(self, v: f64) -> Result<(), Self::Error> { todo!() }
    fn serialize_char(self, v: char) -> Result<(), Self::Error> { todo!() }

    // --- Complex / Special Types ---

    fn serialize_bytes(self, v: &[u8]) -> Result<(), Self::Error> { todo!() }
    fn serialize_none(self) -> Result<(), Self::Error> { todo!() }
    fn serialize_some<T: ?Sized + ser::Serialize>(self, value: &T) -> Result<(), Self::Error> {
        todo!()
    }
    fn serialize_unit(self) -> Result<(), Self::Error> { todo!() }
    fn serialize_unit_struct(self, name: &'static str) -> Result<(), Self::Error> { todo!() }
    fn serialize_unit_variant(self, name: &'static str, variant_index: u32, variant: &'static str) -> Result<(), Self::Error> { todo!() }
    fn serialize_newtype_struct<T: ?Sized + ser::Serialize>(self, name: &'static str, value: &T) -> Result<(), Self::Error> { todo!() }
    fn serialize_newtype_variant<T: ?Sized + ser::Serialize>
    (self, name: &'static str, variant_index: u32, variant: &'static str, value: &T) -> Result<(), Self::Error> { todo!() }

    // --- Collections and Structs ---

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        // Zwracamy błąd, bo używamy Impossible
        Err(ser::Error::custom("Sequences not supported yet"))
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(ser::Error::custom("Tuples not supported yet"))
    }

    fn serialize_tuple_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(ser::Error::custom("Tuple structs not supported yet"))
    }

    fn serialize_tuple_variant(self, _name: &'static str, _variant_index: u32, _variant: &'static str, _len: usize) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(ser::Error::custom("Tuple variants not supported yet"))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(ser::Error::custom("Maps not supported yet"))
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct, Self::Error> {
        // Tutaj docelowo zmienisz Impossible na Self, ale narazie:
        Err(ser::Error::custom("Structs not supported yet"))
    }

    fn serialize_struct_variant(self, _name: &'static str, _variant_index: u32, _variant: &'static str, _len: usize) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(ser::Error::custom("Struct variants not supported yet"))
    }
}