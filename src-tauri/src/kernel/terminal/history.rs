use crate::database::{DbManager, models::history::TerminalHistory};
use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref ANSI_ESCAPE: Regex = Regex::new(r"[\u001b\u009b][\[()#;?]*(?:[0-9]{1,4}(?:;[0-9]{0,4})*)?[0-9A-ORZcf-nqry=><]").unwrap();
}

pub struct HistoryService;

impl HistoryService {
    pub async fn log_command(
        db: DbManager,
        session_id: String, 
        project_path: String, 
        data: String
    ) {
        let clean_cmd = ANSI_ESCAPE.replace_all(&data, "").trim().to_string();
        if !clean_cmd.is_empty() {
            let repo = db.repository();
            let _ = repo.add_terminal_entry(
                &project_path, 
                &session_id, 
                &clean_cmd, 
                None
            ).await;
        }
    }

    pub async fn get_history(
        db: DbManager,
        session_id: String, 
    ) -> Result<Vec<TerminalHistory>, String> {
        db.repository()
            .get_session_terminal(&session_id)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn clear_history(
        db: DbManager,
        session_id: String,
    ) -> Result<(), String> {
        let repo = db.repository();
        
        repo.clear_terminal_history(&session_id)
            .await
            .map_err(|e| e.to_string())?;
            
        Ok(())
    }
}