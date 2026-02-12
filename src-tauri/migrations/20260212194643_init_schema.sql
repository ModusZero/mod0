CREATE VIRTUAL TABLE IF NOT EXISTS file_index USING fts5(
    path UNINDEXED, 
    content,
    tokenize='unicode61'
);

CREATE TABLE IF NOT EXISTS skills (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    code TEXT NOT NULL,
    description TEXT,
    tags TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS projects (
    path TEXT PRIMARY KEY,
    last_opened DATETIME NOT NULL,
    is_favorite BOOLEAN DEFAULT 0,
    custom_settings TEXT
);

CREATE TABLE IF NOT EXISTS agent_sessions (
    id TEXT PRIMARY KEY,
    project_path TEXT NOT NULL,
    branch_name TEXT NOT NULL,
    status TEXT NOT NULL, -- Active, Committed, Aborted
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY(project_path) REFERENCES projects(path)
);

CREATE TABLE IF NOT EXISTS thought_nodes (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    session_id TEXT NOT NULL,
    parent_id INTEGER,
    node_type TEXT NOT NULL, -- Decision, Action, Observation
    content TEXT NOT NULL,
    metadata TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY(session_id) REFERENCES agent_sessions(id),
    FOREIGN KEY(parent_id) REFERENCES thought_nodes(id)
);

CREATE TABLE IF NOT EXISTS execution_tasks (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    session_id TEXT NOT NULL,
    title TEXT NOT NULL,
    status TEXT NOT NULL, -- Todo, In Progress, Testing, Done
    tdd_status TEXT NOT NULL, -- Red, Green, Refactor
    position INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY(session_id) REFERENCES agent_sessions(id)
);

CREATE TABLE IF NOT EXISTS chat_history (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    project_path TEXT NOT NULL,
    session_id TEXT, -- Relación opcional con la sesión del agente
    role TEXT NOT NULL, 
    content TEXT NOT NULL,
    artifact_data TEXT,
    timestamp DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY(project_path) REFERENCES projects(path)
);