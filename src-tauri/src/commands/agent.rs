use tauri::State;
use crate::database::DbManager;
use crate::kernel::filesystem::search::SearchService;

/// Descubre capacidades del agente filtrando por múltiples etiquetas.
/// Ejemplo desde JS: invoke('discover_agent_skills', { tags: ['git', 'branch'] })
#[tauri::command]
pub async fn discover_agent_skills(
    db: State<'_, DbManager>, 
    tags: Vec<String>
) -> Result<Vec<String>, String> {
    SearchService::find_skills(&db, tags).await
}