use std::sync::{OnceLock, Arc};

use sea_orm::{DatabaseConnection, ConnectOptions, Database};


static DB: OnceLock<Arc<DatabaseConnection>> = OnceLock::new();

// https://stackoverflow.com/questions/67650879/how-to-use-lazy-static-with-async-await-initializer/67758135#67758135
pub async fn connect_db(url: &str) {
    if let None = DB.get() {
        let option = ConnectOptions::new(url);
        let db = Database::connect(option).await.unwrap();
        db.ping().await.expect("Db Connect Error");
    
        DB.set(Arc::new(db)).expect("Db Store Error");
    }
}

pub fn get_datasource() -> Arc<DatabaseConnection> {
    Arc::clone(DB.get().expect("Db Need Init"))
}