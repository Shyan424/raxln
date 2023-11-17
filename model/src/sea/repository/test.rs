
use std::sync::Arc;

use sea_orm::*;

use crate::sea::data_source::get_datasource;
use crate::sea::entity::test;


pub struct TestRepository {
    db: Arc<DatabaseConnection>
}

impl TestRepository {
    pub fn new() -> Self {
        TestRepository { db: get_datasource() }
    }

    pub async fn insert(&self, test: test::Model) -> Result<(), String> {
        let test_model = test::ActiveModel {
            key: Set(test.key.to_owned()),
            value: Set(test.value.to_owned()),
            ..Default::default()
        };
        
        let back = test::Entity::insert(test_model)
            .exec(&*Arc::clone(&self.db)).await;

        match back {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("{}", e)),
        }
    }

    pub async fn find_by_key(&self, key: String) -> Result<Option<test::Model>, DbErr> {
        test::Entity::find()
            .filter(test::Column::Key.eq(key))
            .one(&*Arc::clone(&self.db)).await
    }
}

#[cfg(test)]
mod just_test {
    use sea_orm::{DatabaseConnection, ConnectionTrait, Schema};

    use crate::sea::data_source::{self, get_datasource};
    use crate::sea::entity::test;
    use crate::sea::repository::test::TestRepository;

    #[ignore]
    #[tokio::test]
    async fn ttt() {
        data_source::connect_db("sqlite::memory:").await;
        let db = get_datasource();

        create_table(&*db).await;

        let test = test::Model {
            id: 0,
            key: "123".to_string(),
            value: "xyz".to_string(),
        };

        let test_repository = TestRepository::new();

        let insert = test_repository.insert(test.clone()).await;
        if let Err(e) = insert {
            println!("{e}");
            assert!(false);
        };

        let vr = test_repository.find_by_key(String::from("123")).await;
        match vr {
            Ok(o) => {
                let o = o.unwrap();
                assert_eq!(o.key, test.key);
                assert_eq!(o.value, test.value);
            },
            Err(e) => {
                println!("{}", e);
                assert!(false);
            }
        }
    }


    async fn create_table(db: &DatabaseConnection) {
        let builder = db.get_database_backend();
        let schema = Schema::new(builder);

        let stmt = builder.build(&schema.create_table_from_entity(test::Entity));
        let _ = db.execute(stmt).await;
    }

}