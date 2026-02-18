pub mod agent;
pub mod files;
pub mod history;
pub mod session;
pub mod workspace;

pub struct Repository<'a> {
    pub pool: &'a sqlx::SqlitePool,
}

impl<'a> Repository<'a> {
    pub fn new(pool: &'a sqlx::SqlitePool) -> Self {
        Self { pool }
    }
}