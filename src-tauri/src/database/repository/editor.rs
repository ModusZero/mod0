use crate::database::models::shell::EditorState;
use super::Repository;

impl<'a> Repository<'a> {
    /// Persiste el estado completo de la interfaz para su recuperación en futuras sesiones.
    /// 
    /// Este método realiza un "upsert" sobre la ruta del proyecto, sobrescribiendo 
    /// cualquier configuración previa con el estado actual del frontend.
    /// 
    /// ### Parámetros
    /// * `state`: Estructura con la captura completa de pestañas, archivo activo y layout.
    pub async fn save_editor_state(&self, state: &EditorState) -> sqlx::Result<()> {
        sqlx::query!(
            r#"INSERT INTO editor_state (project_path, open_tabs, active_tab, layout_config) 
               VALUES (?, ?, ?, ?)
               ON CONFLICT(project_path) DO UPDATE SET 
               open_tabs = excluded.open_tabs,
               active_tab = excluded.active_tab,
               layout_config = excluded.layout_config"#,
            state.project_path, 
            state.open_tabs, 
            state.active_tab, 
            state.layout_config
        )
        .execute(self.pool).await?;
        Ok(())
    }

    /// Recupera el estado guardado para restaurar la interfaz al abrir un proyecto.
    /// 
    /// ### Parámetros
    /// * `project_path`: Ruta absoluta que identifica el espacio de trabajo.
    /// 
    /// ### Retorna
    /// Un `Option<EditorState>` con los datos necesarios para reconstruir la sesión de UI.
    pub async fn get_editor_state(&self, project_path: &str) -> sqlx::Result<Option<EditorState>> {
        sqlx::query_as!(
            EditorState,
            r#"SELECT 
                project_path as "project_path!", 
                open_tabs as "open_tabs!", 
                active_tab as "active_tab!", 
                layout_config as "layout_config!"
               FROM editor_state 
               WHERE project_path = ?"#,
            project_path
        )
        .fetch_optional(self.pool).await
    }

    /// Elimina la persistencia del estado de un proyecto.
    /// 
    /// Útil para procesos de limpieza o cuando el usuario desea resetear la interfaz 
    /// a los valores por defecto.
    /// 
    /// ### Parámetros
    /// * `project_path`: Ruta del proyecto a limpiar.
    pub async fn delete_editor_state(&self, project_path: &str) -> sqlx::Result<()> {
        sqlx::query!(
            "DELETE FROM editor_state WHERE project_path = ?",
            project_path
        )
        .execute(self.pool).await?;
        Ok(())
    }
}