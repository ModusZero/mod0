pub mod pty;
pub mod persistence;

use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct TerminalOutput {
    pub session_id: String,
    pub data: String,
}