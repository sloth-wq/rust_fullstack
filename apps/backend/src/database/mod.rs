pub mod core {
    use crate::config::get_config;
    use sqlx::{mysql::MySqlPool, MySql, Pool};

    pub async fn connect() -> Pool<MySql> {
        let config = get_config();
        match MySqlPool::connect(config.database_url()).await {
            Ok(v) => v,
            Err(_) => panic!("f"),
        }
    }
}
