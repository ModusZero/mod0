pub mod indexer;
pub mod agent;
pub mod skills;
pub mod projects;

pub struct Repository<'a> {
    pub pool: &'a sqlx::SqlitePool,
}

impl<'a> Repository<'a> {
    pub fn new(pool: &'a sqlx::SqlitePool) -> Self {
        Self { pool }
    }
}