use super::artifact::{Artifact, ArtifactType};
use super::Repository;

impl<'a> Repository<'a> {
    /// Registra o actualiza un artefacto utilizando la lógica de conflicto sobre el ID.
    ///
    /// ### Parámetros
    /// * `art`: Estructura que contiene los datos del artefacto (UI, esquemas, etc.).
    pub async fn save_artifact(&self, art: &Artifact) -> sqlx::Result<()> {
        sqlx::query!(
            r#"INSERT OR REPLACE INTO artifact (id, session_id, thought_node_id, artifact_type, content, version, checksum, status) 
               VALUES (?, ?, ?, ?, ?, ?, ?, ?)"#,
            art.id,
            art.session_id,
            art.thought_node_id,
            art.artifact_type,
            art.content,
            art.version,
            art.checksum,
            art.status
        )
        .execute(self.pool)
        .await?;
        Ok(())
    }

    /// Busca un artefacto específico en la base de datos.
    ///
    /// ### Parámetros
    /// * `id`: Identificador único del artefacto.
    ///
    /// ### Retorna
    /// Un `Option<Artifact>` con el mapeo de tipos y nulabilidad verificado.
    pub async fn get_artifact(&self, id: &str) -> sqlx::Result<Option<Artifact>> {
        sqlx::query_as!(
            Artifact,
            r#"SELECT 
                id as "id!", 
                session_id as "session_id!", 
                thought_node_id, 
                artifact_type as "artifact_type: ArtifactType", 
                content as "content!", 
                version as "version!: i64", 
                checksum, 
                status as "status!"
            FROM artifact WHERE id = ?"#,
            id
        )
        .fetch_optional(self.pool)
        .await
    }

    /// Obtiene la lista completa de artefactos producidos en una sesión.
    ///
    /// ### Parámetros
    /// * `session_id`: Identificador de la sesión de trabajo.
    ///
    /// ### Retorna
    /// Un vector de artefactos ordenados por versión de forma descendente.
    pub async fn get_artifacts_by_session(&self, session_id: &str) -> sqlx::Result<Vec<Artifact>> {
        sqlx::query_as!(
            Artifact,
            r#"SELECT 
                id as "id!", 
                session_id as "session_id!", 
                thought_node_id, 
                artifact_type as "artifact_type: ArtifactType", 
                content as "content!", 
                version as "version!: i64", 
                checksum, 
                status as "status!"
            FROM artifact 
            WHERE session_id = ?
            ORDER BY version DESC"#,
            session_id
        )
        .fetch_all(self.pool)
        .await
    }

    /// Actualiza la relación entre un artefacto y un nodo de pensamiento específico.
    ///
    /// ### Parámetros
    /// * `artifact_id`: Identificador del artefacto a modificar.
    /// * `thought_id`: ID del nodo de pensamiento que generó esta versión.
    pub async fn link_to_thought(&self, artifact_id: &str, thought_id: i64) -> sqlx::Result<()> {
        sqlx::query!(
            "UPDATE artifact SET thought_node_id = ? WHERE id = ?",
            thought_id,
            artifact_id
        )
        .execute(self.pool)
        .await?;
        Ok(())
    }
}