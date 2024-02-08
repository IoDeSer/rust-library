pub(crate) enum StructType<'a>{
    NamedFields(Vec<crate::FieldOrder<'a>>),
    Tuple(u8),
    NotStruct
}