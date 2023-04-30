use anyhow::Result;
use sqlx::{Pool, Postgres};

use crate::column_types::ColumnDataType;

#[derive(sqlx::FromRow, Debug)]
pub struct Column {
    pub name: String,
    pub dtype: ColumnDataType,
    pub nullable: bool,
}

#[derive(Debug)]
pub struct Table {
    pub tablename: String,
    pub columns: Vec<Column>,
}

impl Table {
    pub async fn get_columns(&mut self, pool: &Pool<Postgres>) -> Result<()> {
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
