use std::sync::Arc;

use crate::error::Error;

#[derive(Clone)]
pub struct HandlerContext {
    pub db: Arc<db::ConnectionPool>,
}

impl HandlerContext {
    pub fn new(connection_pool: db::ConnectionPool) -> Self {
        Self {
            db: Arc::new(connection_pool),
        }
    }

    pub async fn db(&self) -> Result<db::Client, Error> {
        Ok(self.db.get().await?)
    }
}
