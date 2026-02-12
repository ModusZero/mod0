pub struct McpDefinition {
    pub name: String,
    pub command: String,
    pub args: Vec<String>,
}

impl McpDefinition {
    pub fn get_core_tools(project_path: &str) -> Vec<Self> {
        vec![
            Self {
                name: "filesystem".into(),
                command: "npx".into(),
                args: vec!["-y".into(), "@modelcontextprotocol/server-filesystem".into(), project_path.into()],
            },
            Self {
                name: "terminal".into(),
                command: "uvx".into(),
                args: vec!["mcp-server-terminal".into()],
            },
            Self {
                name: "fetch".into(),
                command: "npx".into(),
                args: vec!["-y".into(), "@modelcontextprotocol/server-fetch".into()],
            },
            Self {
                name: "memory".into(),
                command: "npx".into(),
                args: vec!["-y".into(), "@modelcontextprotocol/server-memory".into()],
            },
        ]
    }
}