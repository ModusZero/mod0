use std::path::PathBuf;
use crate::database::DbManager;

/// Servicio encargado de las operaciones de búsqueda avanzada en el sistema de archivos indexado.
pub struct SearchService;

impl SearchService {
    /// Realiza una búsqueda de texto completo (Full Text Search) en el contenido de los archivos.
    /// 
    /// Utiliza el motor FTS5 de SQLite para encontrar coincidencias rápidas en el código fuente.
    /// Retorna una lista de rutas (PathBuf) que contienen el término buscado.
    pub async fn find_code(db: &DbManager, query: &str) -> Result<Vec<PathBuf>, String> {
        let repo = db.repository();
        
        // El método search_code en el repositorio ya gestiona la lógica MATCH de FTS5
        repo.search_code(query)
            .await
            .map(|paths| paths.into_iter().map(PathBuf::from).collect())
            .map_err(|e| e.to_string())
    }

    /// Realiza una búsqueda rápida por nombre de archivo (Quick Open).
    /// 
    /// A diferencia de `find_code`, esta función busca en la columna 'path' del índice
    /// permitiendo encontrar archivos por su nombre o extensión sin importar el contenido.
    pub async fn find_filename(db: &DbManager, query: &str) -> Result<Vec<PathBuf>, String> {
        let pattern = format!("%{}%", query);
        
        let rows = sqlx::query!(
            r#"SELECT path as "path: String" FROM file_index WHERE path LIKE ? LIMIT 50"#,
            pattern
        )
        .fetch_all(&db.pool)
        .await
        .map_err(|e| e.to_string())?;

        // Como usamos el override de tipo, r.path será Option<String> o String dependiendo de SQLx, 
        // usamos flatten para asegurar que tenemos el valor.
        Ok(rows.into_iter().filter_map(|r| r.path).map(PathBuf::from).collect())
    }

    /// Busca habilidades (skills) registradas en la base de datos que coincidan con un tag.
    /// 
    /// Útil para que el agente recupere contexto sobre qué herramientas usar.
    pub async fn find_skills(db: &DbManager, tags: Vec<String>) -> Result<Vec<String>, String> {
        let repo = db.repository();
        repo.get_skills_by_tags(tags)
            .await
            .map(|skills| skills.into_iter().map(|s| s.name).collect())
            .map_err(|e| e.to_string())
    }
}