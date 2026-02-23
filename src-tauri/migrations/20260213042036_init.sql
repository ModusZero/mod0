CREATE TABLE IF NOT EXISTS project (
    path TEXT PRIMARY KEY,
    last_opened DATETIME NOT NULL,
    is_favorite BOOLEAN DEFAULT 0,
    custom_settings TEXT
);

CREATE VIRTUAL TABLE IF NOT EXISTS file_index USING fts5(
    path UNINDEXED, 
    project_path UNINDEXED, 
    content,
    tokenize='unicode61'
);

CREATE TABLE IF NOT EXISTS file_metadata (
    path TEXT PRIMARY KEY,
    last_modified DATETIME NOT NULL,
    size INTEGER NOT NULL,
    file_type TEXT,
    language_id TEXT,
    project_path TEXT NOT NULL,
    FOREIGN KEY(project_path) REFERENCES project(path) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS extension (
    id TEXT PRIMARY KEY,
    version TEXT NOT NULL,
    local_path TEXT NOT NULL,
    is_enabled BOOLEAN DEFAULT 1,
    config_schema TEXT,
    manifest_cache TEXT
);

CREATE TABLE IF NOT EXISTS project_extension (
    project_path TEXT,
    extension_id TEXT,
    config_values TEXT,
    PRIMARY KEY(project_path, extension_id),
    FOREIGN KEY(project_path) REFERENCES project(path) ON DELETE CASCADE,
    FOREIGN KEY(extension_id) REFERENCES extension(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS skill (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    code TEXT NOT NULL,
    description TEXT,
    tags TEXT,
    usage_count INTEGER DEFAULT 0,
    last_used DATETIME,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS session (
    id TEXT PRIMARY KEY,
    project_path TEXT NOT NULL,
    session_type TEXT NOT NULL,
    name TEXT,
    branch_name TEXT,
    worktree_path TEXT,
    status TEXT NOT NULL,
    is_ghost_session BOOLEAN DEFAULT 0,
    base_commit_hash TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY(project_path) REFERENCES project(path) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS thought_node (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    session_id TEXT NOT NULL,
    parent_id INTEGER,
    node_type TEXT NOT NULL,
    content TEXT NOT NULL,
    status TEXT DEFAULT 'pending',
    metadata TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY(session_id) REFERENCES session(id) ON DELETE CASCADE,
    FOREIGN KEY(parent_id) REFERENCES thought_node(id) ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS artifact (
    id TEXT PRIMARY KEY,
    session_id TEXT NOT NULL,
    thought_node_id INTEGER,
    artifact_type TEXT NOT NULL,
    content TEXT NOT NULL,
    version INTEGER DEFAULT 1,
    checksum TEXT,
    status TEXT DEFAULT 'draft',
    FOREIGN KEY(session_id) REFERENCES session(id) ON DELETE CASCADE,
    FOREIGN KEY(thought_node_id) REFERENCES thought_node(id) ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS task (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    session_id TEXT NOT NULL,
    skill_id INTEGER,
    title TEXT NOT NULL,
    status TEXT NOT NULL,
    tdd_status TEXT NOT NULL,
    error_log TEXT,
    position INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY(session_id) REFERENCES session(id) ON DELETE CASCADE,
    FOREIGN KEY(skill_id) REFERENCES skill(id) ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS editor_state (
    project_path TEXT PRIMARY KEY,
    open_tabs TEXT,
    active_tab TEXT,
    layout_config TEXT,
    FOREIGN KEY(project_path) REFERENCES project(path) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS chat_history (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    project_path TEXT NOT NULL,
    session_id TEXT NOT NULL, 
    role TEXT NOT NULL,
    content TEXT NOT NULL,
    timestamp DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY(session_id) REFERENCES session(id) ON DELETE CASCADE,
    FOREIGN KEY(project_path) REFERENCES project(path) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS terminal_history (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    project_path TEXT NOT NULL,
    session_id TEXT NOT NULL,
    command TEXT NOT NULL,
    exit_code INTEGER,
    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY(session_id) REFERENCES session(id) ON DELETE CASCADE,
    FOREIGN KEY(project_path) REFERENCES project(path) ON DELETE CASCADE
);