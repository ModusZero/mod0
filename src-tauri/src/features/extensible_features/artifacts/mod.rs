pub mod default;
pub mod internal_logic;

// Desde aqui hay que poder manejar como generar, editar, exportar y consultar artifacts.
// Para planear, documentar, y auditar las futuras acciones de la IA o del User
// Las peticiones llegan a este file sin distinguir entre made by user o made by AI
// simplemente ejecuta la accion dependiendo de la extension más indicada
// si no hay, usa el default que es para manejar diagramas de flujo
// Al terminarlos, deben poder persistirse como file y con su metadata correctamente
// para que sirva de contexto al context builder de la IA

pub mod artifacts_events {

}