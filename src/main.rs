use std::collections::HashMap;

use anyhow::Result;
use column_types::ColumnDataType;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use table::Table;

mod column_types;
mod table;

#[derive(Debug)]
struct Relationship {
    constraint_name: String,
    source_table_name: String,
    source_column_name: String,
    foreign_table_name: String,
    foreign_column_name: String,
}

#[derive(Debug, Default)]
struct Metadata {
    tables: Vec<Table>,
    relationships: HashMap<String, Vec<Relationship>>,
}

impl Metadata {
    async fn get_tables(&mut self, pool: &Pool<Postgres>) -> Result<()> {
        let recs: Vec<(String,)> =
            sqlx::query_as("SELECT tablename FROM pg_catalog.pg_tables WHERE schemaname='public'")
                .fetch_all(pool)
                .await?;

        self.tables = recs
            .iter()
            .map(|(tablename,)| Table {
                tablename: tablename.to_string(),
                columns: vec![],
            })
            .collect();

        for table in self.tables.iter_mut() {
            table.get_columns(pool).await?;
        }

        let table_names = self.tables.iter().map(|t| &t.tablename).collect::<Vec<_>>();
        self.relationships = self.get_relationships(table_names, &pool).await?;

        Ok(())
    }

    async fn get_relationships(
        &self,
        tables: Vec<&String>,
        pool: &Pool<Postgres>,
    ) -> Result<HashMap<String, Vec<Relationship>>> {
        let mut result = HashMap::new();

        for tbl in tables {
            let recs: Vec<(String, String, String, String, String)> = sqlx::query_as(
                "SELECT 
            tc.constraint_name AS constraint_name,
            tc.table_name      AS source_table_name,
            kcu.column_name    AS source_column_name,
            ccu.table_name     AS foreign_table_name,
            ccu.column_name    AS foreign_column_name
        FROM information_schema.table_constraints AS tc
            JOIN information_schema.key_column_usage AS kcu
              ON tc.constraint_name = kcu.constraint_name
                  AND tc.table_schema = kcu.table_schema
            JOIN information_schema.constraint_column_usage AS ccu
              ON ccu.constraint_name = tc.constraint_name
                  AND ccu.table_schema = tc.table_schema
        WHERE tc.constraint_type = 'FOREIGN KEY'
            AND tc.table_name = $1;",
            )
            .bind(&tbl)
            .fetch_all(pool)
            .await?;

            let relationships: Vec<Relationship> = recs
                .iter()
                .map(
                    |(
                        constraint_name,
                        source_table_name,
                        source_column_name,
                        foreign_table_name,
                        foreign_column_name,
                    )| Relationship {
                        constraint_name: constraint_name.to_string(),
                        source_table_name: source_table_name.to_string(),
                        source_column_name: source_column_name.to_string(),
                        foreign_table_name: foreign_table_name.to_string(),
                        foreign_column_name: foreign_column_name.to_string(),
                    },
                )
                .collect();

            result.insert(tbl.to_string(), relationships);
        }

        Ok(result)
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:postgres@localhost:5432/rust")
        .await?;

    let mut metadata = Metadata::default();

    metadata.get_tables(&pool).await?;

    println!("{:#?}", metadata);

    Ok(())
}
