pub mod fs {
    pub const ON_WILL_SAVE: &str = "fs.onWillSave";
    pub const ON_DID_SAVE: &str = "fs.onDidSave";
    pub const ON_DID_CREATE: &str = "fs.onDidCreate";
    pub const ON_DID_DELETE: &str = "fs.onDidDelete";
}

pub mod terminal {
    pub const ON_DATA: &str = "terminal.onData";
    pub const ON_COMMAND_EXECUTED: &str = "terminal.onCommandExecuted";
}

pub mod ai {
    pub const ON_GENERATE: &str = "ai.onGenerate";
    pub const PROVIDE_CONTEXT: &str = "ai.provideContext";
}