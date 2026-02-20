use crate::database::models::kernel::Project;
use chrono::{DateTime, Utc};
use super::Repository;

impl<'a> Repository<'a> {
    /// Registra un nuevo proyecto o actualiza los metadatos de uno existente.
    /// 
    /// ### Parámetros
    /// * `project`: Estructura con la ruta, estado de favorito y configuración.
    pub async fn upsert_project(&self, project: &Project) -> sqlx::Result<()> {
        sqlx::query!(
            r#"INSERT INTO project (path, last_opened, is_favorite, custom_settings) 
               VALUES (?, ?, ?, ?)
               ON CONFLICT(path) DO UPDATE SET 
               last_opened = excluded.last_opened,
               is_favorite = excluded.is_favorite,
               custom_settings = excluded.custom_settings"#,
            project.path, 
            project.last_opened, 
            project.is_favorite, 
            project.custom_settings
        )
        .execute(self.pool).await?;
        Ok(())
    }

    /// Recupera la configuración de un proyecto específico mediante su ruta.
    /// 
    /// ### Parámetros
    /// * `path`: Ruta absoluta del proyecto en el sistema de archivos.
    /// 
    /// ### Retorna
    /// Un `Option<Project>` con los datos persistidos si el proyecto existe.
    pub async fn get_project(&self, path: &str) -> sqlx::Result<Option<Project>> {
        sqlx::query_as!(
            Project,
            r#"SELECT 
                path as "path!", 
                last_opened as "last_opened!: DateTime<Utc>", 
                is_favorite as "is_favorite!: bool", 
                custom_settings 
               FROM project 
               WHERE path = ?"#,
            path
        )
        .fetch_optional(self.pool).await
    }

    /// Obtiene la lista de todos los proyectos registrados, ordenados por acceso reciente.
    /// 
    /// ### Retorna
    /// Un vector de estructuras `Project`.
    pub async fn get_all_projects(&self) -> sqlx::Result<Vec<Project>> {
        sqlx::query_as!(
            Project,
            r#"SELECT 
                path as "path!", 
                last_opened as "last_opened!: DateTime<Utc>", 
                is_favorite as "is_favorite!: bool", 
                custom_settings 
               FROM project 
               ORDER BY last_opened DESC"#
        )
        .fetch_all(self.pool).await
    }
}