// pg data types
#[derive(Debug)]
pub enum ColumnDataType {
    Int,
    String,
    Bool,
    Date,
    DateTime,
    Time,
    Float,
    Double,
    Decimal,
    Real,
    Binary,
    Json,
    Jsonb,
    Uuid,
    Array,
    Integer,
    Boolean,
    Text,
    Other(String),
}

impl ColumnDataType {
    pub fn from_string(s: &str) -> Self {
        match s {
            "int4" => ColumnDataType::Int,
            "int8" => ColumnDataType::Int,
            "varchar" => ColumnDataType::String,
            "bool" => ColumnDataType::Bool,
            "date" => ColumnDataType::Date,
            "timestamp" => ColumnDataType::DateTime,
            "time" => ColumnDataType::Time,
            "float4" => ColumnDataType::Float,
            "float8" => ColumnDataType::Double,
            "real" => ColumnDataType::Real,
            "text" => ColumnDataType::Text,
            "integer" => ColumnDataType::Integer,
            "boolean" => ColumnDataType::Boolean,
            "numeric" => ColumnDataType::Decimal,
            "bytea" => ColumnDataType::Binary,
            "json" => ColumnDataType::Json,
            "jsonb" => ColumnDataType::Jsonb,
            "uuid" => ColumnDataType::Uuid,
            "int4[]" => ColumnDataType::Array,
            _ => ColumnDataType::Other(s.to_string()),
        }
    }
}
