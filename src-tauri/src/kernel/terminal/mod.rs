pub mod pty;
pub mod manager;
pub mod history;

pub use manager::TerminalManager;
use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct TerminalOutput {
    pub session_id: String,
    pub data: String,
}