use std::path::{Path, PathBuf};

use crate::config::AppConfig;
use crate::core::error::{AppError, AppResult};
use crate::features::sessions::model::Session;
use crate::features::sessions::profile::CliProfile;
use crate::features::terminal::pty::PtySpec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShellKind {
    Posix,
    PowerShell,
    Cmd,
}

/// Qué proceso abrir en el PTY y qué texto escribirle para arrancar la CLI.
pub struct LaunchPlan {
    pub spec: PtySpec,
    pub input: String,
}

pub struct LaunchInput<'a> {
    pub session: &'a Session,
    pub profile: &'a CliProfile,
    pub config: &'a AppConfig,
    pub hook_url: Option<String>,
    pub settings_file: Option<PathBuf>,
    pub resume: bool,
    pub prompt: Option<&'a str>,
}

fn kind_of(program: &str) -> ShellKind {
    let name = program
        .rsplit(['/', '\\'])
        .next()
        .unwrap_or(program)
        .to_lowercase();
    if name.contains("pwsh") || name.contains("powershell") {
        ShellKind::PowerShell
    } else if name.starts_with("cmd") {
        ShellKind::Cmd
    } else {
        ShellKind::Posix
    }
}

fn default_args(kind: ShellKind) -> Vec<String> {
    match kind {
        ShellKind::PowerShell => vec!["-NoLogo".into()],
        ShellKind::Cmd => Vec::new(),
        ShellKind::Posix => vec!["-l".into()],
    }
}

fn in_path(executable: &str) -> bool {
    std::env::var_os("PATH")
        .map(|paths| std::env::split_paths(&paths).any(|dir| dir.join(executable).is_file()))
        .unwrap_or(false)
}

/// Elige la shell: la del usuario si la configuró; PowerShell 7 o 5 en Windows; `$SHELL` en el resto.
pub fn detect_shell(config: &AppConfig) -> (String, ShellKind, Vec<String>) {
    if let Some(custom) = config.shell.as_ref().map(|s| s.trim()).filter(|s| !s.is_empty()) {
        let kind = kind_of(custom);
        return (custom.to_string(), kind, default_args(kind));
    }

    if cfg!(windows) {
        let program = if in_path("pwsh.exe") {
            "pwsh.exe"
        } else {
            "powershell.exe"
        };
        return (
            program.to_string(),
            ShellKind::PowerShell,
            default_args(ShellKind::PowerShell),
        );
    }

    let program = std::env::var("SHELL")
        .ok()
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "/bin/bash".into());
    let kind = kind_of(&program);
    (program, kind, default_args(kind))
}

fn needs_quotes(arg: &str) -> bool {
    arg.is_empty()
        || !arg.chars().all(|c| {
            c.is_ascii_alphanumeric()
                || matches!(
                    c,
                    '_' | '@' | '%' | '+' | '=' | ':' | ',' | '.' | '/' | '\\' | '-'
                )
        })
}

/// Escapa un argumento para la shell indicada.
pub fn quote(arg: &str, kind: ShellKind) -> String {
    if !needs_quotes(arg) {
        return arg.to_string();
    }
    match kind {
        ShellKind::Posix => format!("'{}'", arg.replace('\'', "'\\''")),
        ShellKind::PowerShell => format!("'{}'", arg.replace('\'', "''")),
        ShellKind::Cmd => format!("\"{}\"", arg.replace('"', "\\\"")),
    }
}

pub fn command_line(tokens: &[String], kind: ShellKind) -> String {
    let mut parts: Vec<String> = tokens.iter().map(|token| quote(token, kind)).collect();

    // En PowerShell una ruta entre comillas solo se ejecuta con el operador de llamada.
    if kind == ShellKind::PowerShell && tokens.first().is_some_and(|first| needs_quotes(first)) {
        parts.insert(0, "&".to_string());
    }
    parts.join(" ")
}

pub fn plan(input: LaunchInput) -> AppResult<LaunchPlan> {
    let LaunchInput {
        session,
        profile,
        config,
        hook_url,
        settings_file,
        resume,
        prompt,
    } = input;

    if !Path::new(&session.cwd).is_dir() {
        return Err(AppError::msg(format!(
            "la carpeta de trabajo no existe: {}",
            session.cwd
        )));
    }

    let (program, kind, args) = detect_shell(config);

    let mut env = vec![
        ("TERM".to_string(), "xterm-256color".to_string()),
        ("COLORTERM".to_string(), "truecolor".to_string()),
        ("MOD0_SESSION_ID".to_string(), session.id.clone()),
    ];
    if let Some(url) = hook_url {
        env.push(("MOD0_HOOK_URL".to_string(), url));
    }
    env.extend(profile.env.iter().map(|(k, v)| (k.clone(), v.clone())));

    let mut tokens: Vec<String> = Vec::new();
    if !profile.command.trim().is_empty() {
        tokens.push(profile.command.trim().to_string());
        tokens.extend(profile.args.iter().cloned());

        if let Some(file) = settings_file {
            tokens.push("--settings".into());
            tokens.push(file.to_string_lossy().into_owned());
        }

        match (
            &session.claude_session_id,
            resume && !profile.resume_args.is_empty(),
        ) {
            (Some(cli_session), true) => {
                tokens.extend(
                    profile
                        .resume_args
                        .iter()
                        .map(|arg| arg.replace("{session}", cli_session)),
                );
            }
            _ => {
                // Se aplana a una línea: así es seguro en cualquier shell.
                if let Some(text) = prompt.map(|p| p.split_whitespace().collect::<Vec<_>>().join(" ")) {
                    if !text.is_empty() {
                        tokens.push(text);
                    }
                }
            }
        }
    }

    let input = if tokens.is_empty() {
        String::new()
    } else {
        format!("{}\r", command_line(&tokens, kind))
    };

    Ok(LaunchPlan {
        spec: PtySpec {
            program,
            args,
            cwd: session.cwd.clone(),
            env,
            rows: 30,
            cols: 120,
        },
        input,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quotes_per_shell() {
        assert_eq!(quote("simple-arg", ShellKind::Posix), "simple-arg");
        assert_eq!(quote("it's ok", ShellKind::Posix), "'it'\\''s ok'");
        assert_eq!(quote("it's ok", ShellKind::PowerShell), "'it''s ok'");
        assert_eq!(quote("a b", ShellKind::Cmd), "\"a b\"");
    }

    #[test]
    fn powershell_uses_call_operator_for_quoted_commands() {
        let tokens = vec!["C:\\Program Files\\cli.exe".to_string(), "--x".to_string()];
        assert_eq!(
            command_line(&tokens, ShellKind::PowerShell),
            "& 'C:\\Program Files\\cli.exe' --x"
        );
        assert_eq!(
            command_line(&["claude".to_string()], ShellKind::PowerShell),
            "claude"
        );
    }

    #[test]
    fn detects_shell_kind_from_the_program_name() {
        assert_eq!(
            kind_of("C:\\Program Files\\PowerShell\\7\\pwsh.exe"),
            ShellKind::PowerShell
        );
        assert_eq!(kind_of("cmd.exe"), ShellKind::Cmd);
        assert_eq!(kind_of("/usr/bin/zsh"), ShellKind::Posix);
    }
}
