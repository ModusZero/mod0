pub mod default;
pub mod internal_logic;

pub mod vcs_events {
    pub const COMMIT: &str = "vcs.commit";
    pub const ON_COMMIT: &str = "vcs.onCommit";
}