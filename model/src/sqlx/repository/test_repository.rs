
use sqlx::QueryBuilder;

use crate::error::ModelError;
use crate::sqlx::entity::test::Test;
use crate::sqlx::get_datasource;

pub async fn find_by_id(id: i32) -> Result<Test, ModelError> {
    sqlx::query_as("SELECT * FROM TEST WHERE ID = $1")
        .bind(id)
        .fetch_one(&get_datasource()).await
        .or_else(|e| Err(ModelError::QueryError(e.to_string())))
}

pub async fn insert(key: String, value: String) -> Result<Test, ModelError> {
    sqlx::query_as("INSERT INTO TEST(KEY, VALUE) VALUES($1, $2) RETURNING *")
        .bind(key)
        .bind(value)
        .fetch_one(&get_datasource()).await
        .or_else(|e| Err(ModelError::QueryError(e.to_string())))
}

pub async fn insert_all(tests: Vec<Test>) -> Result<(), ModelError> {
    // query_builder_insert(tests).await;
    postgres_unnest_insert(tests).await
}

async fn query_builder_insert(tests: Vec<Test>) -> Result<(), ModelError> {
    QueryBuilder::new("INSERT INTO TEST(KEY, VALUE)")
    .push_values(
        tests,
        |mut b, test| {
            b.push_bind(test.key)
                .push_bind(test.value);
        }
    ).build()
    .execute(&get_datasource()).await
    .or(Err(ModelError::QueryError(String::from("Insert Error"))))?;

    Ok(())
}

async fn postgres_unnest_insert(tests: Vec<Test>) -> Result<(), ModelError> {
    let mut keys: Vec<String> = Vec::new();
    let mut values: Vec<String> = Vec::new();
    
    for test in tests {
        keys.push(test.key);
        values.push(test.value);
    }
    
    sqlx::query("INSERT INTO TEST(KEY, VALUE) SELECT * FROM UNNEST($1::text[], $2::text[])")
        .bind(keys)
        .bind(values)
        .execute(&get_datasource()).await
        .or(Err(ModelError::QueryError(String::from("Insert Error"))))?;

    Ok(())
}

#[cfg(test)]
mod just_test {
    use sqlx::postgres::PgPoolOptions;

    use super::*;
    use crate::sqlx::entity::test::Test;
    use crate::sqlx::set_datasource;

    async fn connect_db() {
        let db = PgPoolOptions::new()
        .max_connections(1)
        .connect("postgres://postgres:ppassword@localhost:5432/postgres").await
        .expect("Connect Error");

        set_datasource(db);
    }
    
    #[ignore]
    #[tokio::test]
    async fn bulk_insert_test() {
        connect_db().await;

        let tests: Vec<Test> = (0..3).map(|i| {
            Test {
                id: 0,
                key: format!("key{}", i),
                value: format!("value{}", i)
            }
        }).collect();

        // postgres_unnest_insert(tests).await.expect("Unnest Insert Error");
        query_builder_insert(tests).await.expect("Unnest Insert Error");
    }

    #[ignore]
    #[tokio::test]
    async fn test() {
        connect_db().await;

        let insert_test = insert(String::from("ink"), String::from("inv")).await;
        match insert_test {
            Ok(it) => {
                let find_test = find_by_id(it.id).await;
                match find_test {
                    Ok(ft) => assert_eq!(it.id, ft.id),
                    Err(e) => println!("{:?}", e)
                }
            },
            Err(e) => println!("{:?}", e)
        }
    }

}