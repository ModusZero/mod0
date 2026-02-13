CREATE VIRTUAL TABLE IF NOT EXISTS file_index USING fts5(
    path UNINDEXED, 
    content,
    tokenize='unicode61'
);

CREATE TABLE IF NOT EXISTS file_metadata (
    path TEXT PRIMARY KEY,
    last_modified DATETIME NOT NULL,
    size INTEGER NOT NULL,
    file_type TEXT, -- 'text', 'binary'
    language_id TEXT, -- Para que el LSP sepa qué servidor levantar (ej: 'rust', 'typescript')
    project_path TEXT NOT NULL,
    FOREIGN KEY(project_path) REFERENCES project(path)
);

CREATE TABLE IF NOT EXISTS editor_state (
    project_path TEXT PRIMARY KEY,
    open_tabs TEXT, -- JSON array de rutas
    active_tab TEXT,
    layout_config TEXT, -- JSON con la posición de los paneles
    FOREIGN KEY(project_path) REFERENCES project(path)
);

CREATE TABLE IF NOT EXISTS project (
    path TEXT PRIMARY KEY,
    last_opened DATETIME NOT NULL,
    is_favorite BOOLEAN DEFAULT 0,
    custom_settings TEXT
);

CREATE TABLE IF NOT EXISTS skill (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    code TEXT NOT NULL,
    description TEXT,
    tags TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS session (
    id TEXT PRIMARY KEY,
    project_path TEXT NOT NULL,
    session_type TEXT NOT NULL, -- 'user_chat', 'agent_task', 'terminal'
    name TEXT,                   -- Ej: "Fix login bug"
    branch_name TEXT,            -- Opcional, para cuando el agente crea una rama
    worktree_path TEXT,          -- RUTA FÍSICA si se usa un git worktree
    status TEXT NOT NULL,        -- 'active', 'merging', 'completed', 'aborted'
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY(project_path) REFERENCES project(path)
);

CREATE TABLE IF NOT EXISTS chat_history (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    project_path TEXT NOT NULL,
    session_id TEXT NOT NULL, 
    role TEXT NOT NULL, 
    content TEXT NOT NULL,
    artifact_data TEXT, 
    timestamp DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY(session_id) REFERENCES session(id),
    FOREIGN KEY(project_path) REFERENCES project(path)
);

CREATE TABLE IF NOT EXISTS terminal_history (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    project_path TEXT NOT NULL,
    session_id TEXT NOT NULL,
    command TEXT NOT NULL,
    exit_code INTEGER,
    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY(session_id) REFERENCES session(id),
    FOREIGN KEY(project_path) REFERENCES project(path)
);

CREATE TABLE IF NOT EXISTS thought_node (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    session_id TEXT NOT NULL,
    parent_id INTEGER,
    node_type TEXT NOT NULL, -- Decision, Action, Observation
    content TEXT NOT NULL,
    metadata TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY(session_id) REFERENCES session(id),
    FOREIGN KEY(parent_id) REFERENCES thought_node(id)
);

CREATE TABLE IF NOT EXISTS execution_tasks (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    session_id TEXT NOT NULL,
    title TEXT NOT NULL,
    status TEXT NOT NULL, -- Todo, In Progress, Testing, Done
    tdd_status TEXT NOT NULL, -- Red, Green, Refactor
    position INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY(session_id) REFERENCES session(id)
);