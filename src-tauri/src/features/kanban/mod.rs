use serde_json::Value;

use crate::core::error::AppResult;
use crate::core::state::Paths;
use crate::infrastructure::storage;

/// El tablero se guarda por proyecto. La forma del JSON la define el frontend
/// (`kanban-types.ts`); el backend solo lo persiste de forma atómica.
pub fn load(paths: &Paths, project: &str) -> AppResult<Value> {
    let file = paths.board_file(&storage::stable_key(project));
    Ok(storage::read_json(&file)?.unwrap_or(Value::Null))
}

pub fn save(paths: &Paths, project: &str, board: &Value) -> AppResult<()> {
    storage::write_json_atomic(&paths.board_file(&storage::stable_key(project)), board)
}
