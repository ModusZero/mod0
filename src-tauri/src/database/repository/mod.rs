pub mod collaboration;
pub mod intelligence;
pub mod kernel;

pub struct Repository<'a> {
    pub pool: &'a sqlx::SqlitePool,
}

impl<'a> Repository<'a> {
    pub fn new(pool: &'a sqlx::SqlitePool) -> Self {
        Self { pool }
    }
}