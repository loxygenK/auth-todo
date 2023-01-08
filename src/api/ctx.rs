use sea_orm::DatabaseConnection;

pub struct Context {
    db: DatabaseConnection
}

impl Context {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}
