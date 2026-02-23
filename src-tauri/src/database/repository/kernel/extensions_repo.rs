use super::{extension::Extension, project_extension::ProjectExtension};
use super::Repository;

impl<'a> Repository<'a> {
    /// Registra una nueva extensión en el sistema global.
    /// 
    /// ### Parámetros
    /// * `ext`: Datos de la extensión, incluyendo versión y ruta local.
    pub async fn register_extension(&self, ext: &Extension) -> sqlx::Result<()> {
        sqlx::query!(
            r#"INSERT OR REPLACE INTO extension_registry (id, version, local_path, is_enabled, config_schema, manifest_cache) 
               VALUES (?, ?, ?, ?, ?, ?)"#,
            ext.id, ext.version, ext.local_path, ext.is_enabled, ext.config_schema, ext.manifest_cache
        ).execute(self.pool).await?;
        Ok(())
    }

    /// Vincula una extensión a un proyecto con una configuración específica.
    pub async fn enable_extension_for_project(
        &self, 
        project_path: &str, 
        extension_id: &str, 
        config: &str
    ) -> sqlx::Result<()> {
        sqlx::query!(
            "INSERT OR REPLACE INTO project_extensions (project_path, extension_id, config_values) VALUES (?, ?, ?)",
            project_path, extension_id, config
        ).execute(self.pool).await?;
        Ok(())
    }

    /// Recupera todas las extensiones activas para un proyecto.
    pub async fn get_project_extensions(&self, project_path: &str) -> sqlx::Result<Vec<ProjectExtension>> {
        sqlx::query_as!(
            ProjectExtension,
            r#"SELECT 
                project_path as "project_path!", 
                extension_id as "extension_id!", 
                config_values as "config_values!"
               FROM project_extensions WHERE project_path = ?"#,
            project_path
        ).fetch_all(self.pool).await
    }
}