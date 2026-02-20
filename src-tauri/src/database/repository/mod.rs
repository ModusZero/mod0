pub mod artifact;
pub mod editor;
pub mod extension;
pub mod file_index;
pub mod history;
pub mod project;
pub mod session;
pub mod skill;
pub mod task;
pub mod thought;

pub struct Repository<'a> {
    pub pool: &'a sqlx::SqlitePool,
}

impl<'a> Repository<'a> {
    pub fn new(pool: &'a sqlx::SqlitePool) -> Self {
        Self { pool }
    }
}