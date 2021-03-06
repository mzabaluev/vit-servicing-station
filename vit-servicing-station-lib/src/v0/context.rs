use crate::db;
use std::sync::Arc;
use tokio::sync::RwLock;

pub type SharedContext = Arc<RwLock<Context>>;

#[derive(Clone)]
pub struct Context {
    pub db_connection_pool: db::DBConnectionPool,
    pub block0_path: String,
    pub block0: Vec<u8>,
}

impl Context {
    pub fn new(
        db_connection_pool: db::DBConnectionPool,
        block0_path: &str,
        block0: Vec<u8>,
    ) -> Self {
        Self {
            db_connection_pool,
            block0_path: block0_path.to_string(),
            block0,
        }
    }
}

pub fn new_shared_context(
    db_connection_pool: db::DBConnectionPool,
    block0_path: &str,
) -> SharedContext {
    let block0 = std::fs::read(block0_path).unwrap_or_default();
    let context = Context::new(db_connection_pool, block0_path, block0);
    Arc::new(RwLock::new(context))
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::db;

    pub fn new_in_memmory_db_test_shared_context() -> SharedContext {
        let pool = db::load_db_connection_pool("").unwrap();
        let block0: Vec<u8> = vec![1, 2, 3, 4, 5];
        Arc::new(RwLock::new(Context::new(pool, "", block0)))
    }

    pub fn new_test_shared_context(db_url: &str, block0_path: &str) -> SharedContext {
        let pool = db::load_db_connection_pool(db_url).unwrap();
        new_shared_context(pool, block0_path)
    }
}
