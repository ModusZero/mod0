use std::path::PathBuf;

use serde::Serialize;
use tauri::State;

use crate::core::error::AppResult;
use crate::core::state::AppState;
use crate::features::git::{self, ChangedFile, FilePair};
use crate::infrastructure::process::blocking;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GitInfo {
    pub is_repo: bool,
    pub branch: Option<String>,
}

#[tauri::command]
pub async fn git_info(_state: State<'_, AppState>, path: String) -> AppResult<GitInfo> {
    blocking(move || {
        let path = PathBuf::from(path);
        Ok(GitInfo {
            is_repo: git::is_repo(&path),
            branch: git::current_branch(&path),
        })
    })
    .await
}

/// Archivos que la sesión cambió respecto a su commit base.
#[tauri::command]
pub async fn review_changes(state: State<'_, AppState>, session_id: String) -> AppResult<Vec<ChangedFile>> {
    let session = state.sessions.session(&session_id)?;
    blocking(move || git::changes(&PathBuf::from(&session.cwd), session.base_sha.as_deref())).await
}

#[tauri::command]
pub async fn review_file(
    state: State<'_, AppState>,
    session_id: String,
    path: String,
    status: String,
) -> AppResult<FilePair> {
    let session = state.sessions.session(&session_id)?;
    blocking(move || {
        git::file_pair(
            &PathBuf::from(&session.cwd),
            session.base_sha.as_deref(),
            &path,
            &status,
        )
    })
    .await
}

/// Confirma todos los cambios de la sesión y la marca como revisada.
#[tauri::command]
pub async fn review_commit(
    state: State<'_, AppState>,
    session_id: String,
    message: String,
) -> AppResult<String> {
    let session = state.sessions.session(&session_id)?;
    let sha = blocking(move || git::commit_all(&PathBuf::from(&session.cwd), &message)).await?;

    state.sessions.set_reviewed(&session_id, true)?;
    state.sessions.refresh_changes(&session_id);
    Ok(sha)
}
