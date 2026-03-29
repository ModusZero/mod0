pub mod default;
pub mod internal_logic;

pub mod ai_events {
    pub const ON_GENERATE: &str = "ai.onGenerate";
    pub const PROVIDE_CONTEXT: &str = "ai.provideContext";
}

// El proceso debe ser
// 1. LLega una request hacia la IA
// 2. Si es de un chat persiste el chat, si no, ignora ese paso
// 3. Construye el contexto para inyectar al modelo
// 4. Envia el request al modelo (Extensible)
// 5. Persiste los pensamientos
// 6. Devuelve la respuesta

// Debe permitir multi agent, multi tenant, multi thread

// TODO Aqui hay que implementar todo lo relacionado con el chat, el thinking graph 
// (como un registro de pensamientos), la pantalla de manejo de contexto y uso y manejo de skills.