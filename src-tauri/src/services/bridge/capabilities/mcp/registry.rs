pub struct McpDefinition {
    pub name: String,
    pub command: String,
    pub args: Vec<String>,
}

impl McpDefinition {
    pub fn discover(name: &str, project_path: &str) -> Option<Self> {
        // 1. Intentar encontrar herramientas core
        let core = Self::get_core_tools(project_path);
        if let Some(d) = core.into_iter().find(|it| it.name == name) {
            return Some(d);
        }

        // 2. TODO: Lógica de Discovery en el Workspace
        // Aquí podrías buscar un archivo "modus.mcp.json" en project_path
        
        None
    }

    fn get_core_tools(project_path: &str) -> Vec<Self> {
        vec![
            Self {
                name: "filesystem".into(),
                command: "npx".into(),
                args: vec!["-y".into(), "@modelcontextprotocol/server-filesystem".into(), project_path.into()],
            },
            Self {
                name: "fetch".into(),
                command: "npx".into(),
                args: vec!["-y".into(), "@modelcontextprotocol/server-fetch".into()],
            }
        ]
    }
}