use anyhow::Result;
use sqlx::{postgres::PgPoolOptions, FromRow, Pool, Postgres};

// pg data types
#[derive(Debug)]
enum ColumnDataType {
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
    fn from_string(s: &str) -> Self {
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

#[derive(sqlx::FromRow, Debug)]
struct Column {
    name: String,
    dtype: ColumnDataType,
    nullable: bool,
}

#[derive(Debug)]
struct Table {
    tablename: String,

    columns: Vec<Column>,
}

impl Table {
    async fn get_columns(&mut self, pool: &Pool<Postgres>) -> Result<()> {
        let raw_columns: Vec<(String, String, bool)> = sqlx::query_as(
            "SELECT column_name as name, data_type as dtype, is_nullable::bool as nullable FROM information_schema.columns WHERE table_name = $1",
        )
        .bind(&self.tablename)
        .fetch_all(pool)
        .await?;

        let columns: Vec<Column> = raw_columns
            .iter()
            .map(|(name, dtype, nullable)| Column {
                name: name.to_string(),
                dtype: ColumnDataType::from_string(dtype),
                nullable: *nullable,
            })
            .collect();

        self.columns = columns;

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:postgres@localhost:5432/rust")
        .await?;

    let recs: Vec<(String,)> =
        sqlx::query_as("SELECT tablename FROM pg_catalog.pg_tables WHERE schemaname='public'")
            .fetch_all(&pool)
            .await?;

    let mut tables: Vec<Table> = recs
        .iter()
        .map(|(tablename,)| Table {
            tablename: tablename.to_string(),
            columns: vec![],
        })
        .collect();

    for table in &mut tables {
        table.get_columns(&pool).await?;

        println!("Table - {:?}", table.tablename);
        for column in &table.columns {
            println!("\t{:?}", column);
        }
    }

    Ok(())
}
