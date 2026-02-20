use crate::database::models::bridge::{ExecutionTask, TaskStatus, TddStatus};
use super::Repository;

impl<'a> Repository<'a> {
    /// Inserta una nueva tarea o actualiza una existente (Upsert).
    /// 
    /// ### Parámetros
    /// * `task`: Referencia a la tarea. Si el `id` es 0 o nulo (autoincremento), se insertará.
    /// 
    /// ### Comportamiento
    /// En caso de conflicto con el ID, actualiza el estado, el estado TDD, la posición y el log de errores.
    pub async fn upsert_task(&self, task: &ExecutionTask) -> sqlx::Result<()> {
        sqlx::query!(
            r#"INSERT INTO execution_tasks (id, session_id, skill_id, title, status, tdd_status, position, error_log) 
               VALUES (?, ?, ?, ?, ?, ?, ?, ?)
               ON CONFLICT(id) DO UPDATE SET 
               status = excluded.status, 
               tdd_status = excluded.tdd_status, 
               position = excluded.position,
               error_log = excluded.error_log"#,
            task.id, task.session_id, task.skill_id, task.title, 
            task.status, task.tdd_status, task.position, task.error_log
        ).execute(self.pool).await?;
        Ok(())
    }

    /// Recupera todas las tareas de una sesión ordenadas por su posición jerárquica.
    /// 
    /// ### Parámetros
    /// * `session_id`: ID de la sesión vinculada.
    /// 
    /// ### Retorna
    /// Un vector de `ExecutionTask` con mapeo de tipos correcto y forzado de no-nulidad.
    pub async fn get_session_tasks(&self, session_id: &str) -> sqlx::Result<Vec<ExecutionTask>> {
        sqlx::query_as!(
            ExecutionTask,
            r#"SELECT
                id as "id!",
                session_id as "session_id!",
                skill_id,
                title as "title!",
                status as "status!: TaskStatus",
                tdd_status as "tdd_status!: TddStatus",
                error_log,
                position as "position!"
            FROM execution_tasks 
            WHERE session_id = ? 
            ORDER BY position ASC"#,
            session_id
        ).fetch_all(self.pool).await
    }

    /// Actualiza exclusivamente el estado vital de una tarea (e.g. Active -> Completed).
    /// 
    /// ### Parámetros
    /// * `id`: ID de la tarea.
    /// * `status`: Nuevo estado basado en el enum `TaskStatus`.
    pub async fn update_task_status(&self, id: i64, status: TaskStatus) -> sqlx::Result<()> {
        sqlx::query!(
            "UPDATE execution_tasks SET status = ? WHERE id = ?",
            status, id
        )
        .execute(self.pool).await?;
        Ok(())
    }

    /// Actualiza el resultado de las pruebas TDD y registra logs de error si existen.
    /// 
    /// ### Parámetros
    /// * `id`: ID de la tarea.
    /// * `tdd_status`: Cadena que representa el resultado del test (e.g. "passed", "failed").
    /// * `error_log`: Contenido opcional del error capturado en terminal.
    pub async fn update_task_tdd(&self, id: i64, tdd_status: &str, error_log: Option<String>) -> sqlx::Result<()> {
        sqlx::query!(
            "UPDATE execution_tasks SET tdd_status = ?, error_log = ? WHERE id = ?",
            tdd_status, error_log, id
        )
        .execute(self.pool).await?;
        Ok(())
    }
}