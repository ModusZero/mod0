pub mod skills_repo;
pub mod files_repo;

pub struct Repository<'a> {
    pub pool: &'a sqlx::SqlitePool,
}

impl<'a> Repository<'a> {
    pub fn new(pool: &'a sqlx::SqlitePool) -> Self {
        Self { pool }
    }
}