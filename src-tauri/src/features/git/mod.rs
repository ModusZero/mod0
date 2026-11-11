use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use serde::Serialize;

use crate::core::error::{AppError, AppResult};
use crate::infrastructure::process::run;

/// Se invoca el binario `git` del sistema: es lo que ya usa el usuario, evita compilar
/// libgit2 y mantiene el mismo comportamiento que su terminal.
fn git(cwd: &Path, args: &[&str]) -> AppResult<String> {
    Ok(String::from_utf8_lossy(&run("git", cwd, args)?).into_owned())
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangedFile {
    pub path: String,
    /// M modificado, A añadido, D borrado, U sin seguimiento.
    pub status: String,
    pub additions: u32,
    pub deletions: u32,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FilePair {
    pub path: String,
    pub status: String,
    pub original: String,
    pub modified: String,
}

pub fn is_repo(cwd: &Path) -> bool {
    git(cwd, &["rev-parse", "--is-inside-work-tree"])
        .map(|out| out.trim() == "true")
        .unwrap_or(false)
}

pub fn head_sha(cwd: &Path) -> Option<String> {
    git(cwd, &["rev-parse", "HEAD"])
        .ok()
        .map(|out| out.trim().to_string())
        .filter(|s| !s.is_empty())
}

pub fn current_branch(cwd: &Path) -> Option<String> {
    git(cwd, &["rev-parse", "--abbrev-ref", "HEAD"])
        .ok()
        .map(|out| out.trim().to_string())
        .filter(|branch| !branch.is_empty() && branch != "HEAD")
}

/// Cambios del directorio de trabajo respecto a `base` (por defecto `HEAD`), incluyendo
/// los archivos nuevos sin seguimiento.
pub fn changes(cwd: &Path, base: Option<&str>) -> AppResult<Vec<ChangedFile>> {
    let base = base.unwrap_or("HEAD");
    let mut files: BTreeMap<String, ChangedFile> = BTreeMap::new();

    // Sin commits o con una base inválida el diff falla: se ignora y solo se listan los nuevos.
    if let Ok(raw) = git(cwd, &["diff", "--name-status", "--no-renames", "-z", base]) {
        let mut tokens = raw.split('\0').filter(|t| !t.is_empty());
        while let (Some(status), Some(path)) = (tokens.next(), tokens.next()) {
            let path = path.replace('\\', "/");
            files.insert(
                path.clone(),
                ChangedFile {
                    path,
                    status: status.chars().take(1).collect(),
                    additions: 0,
                    deletions: 0,
                },
            );
        }
    }

    if let Ok(raw) = git(cwd, &["diff", "--numstat", "--no-renames", "-z", base]) {
        for record in raw.split('\0').filter(|r| !r.is_empty()) {
            let mut cols = record.splitn(3, '\t');
            let (Some(added), Some(deleted), Some(path)) = (cols.next(), cols.next(), cols.next()) else {
                continue;
            };
            if let Some(file) = files.get_mut(&path.replace('\\', "/")) {
                file.additions = added.parse().unwrap_or(0);
                file.deletions = deleted.parse().unwrap_or(0);
            }
        }
    }

    if let Ok(raw) = git(cwd, &["ls-files", "--others", "--exclude-standard", "-z"]) {
        for path in raw.split('\0').filter(|p| !p.is_empty()) {
            let path = path.replace('\\', "/");
            let additions = count_lines(&cwd.join(&path));
            files.insert(
                path.clone(),
                ChangedFile {
                    path,
                    status: "U".into(),
                    additions,
                    deletions: 0,
                },
            );
        }
    }

    Ok(files.into_values().collect())
}

fn count_lines(path: &Path) -> u32 {
    match fs::metadata(path) {
        Ok(meta) if meta.is_file() && meta.len() < 1_000_000 => fs::read(path)
            .map(|bytes| bytes.iter().filter(|b| **b == b'\n').count() as u32 + 1)
            .unwrap_or(0),
        _ => 0,
    }
}

/// Contenido original (en `base`) y actual de un archivo para mostrar el diff.
pub fn file_pair(cwd: &Path, base: Option<&str>, path: &str, status: &str) -> AppResult<FilePair> {
    let base = base.unwrap_or("HEAD");
    let spec = format!("{base}:{}", path.replace('\\', "/"));

    let original = match status {
        "A" | "U" => String::new(),
        _ => git(cwd, &["show", &spec]).unwrap_or_default(),
    };
    let modified = match status {
        "D" => String::new(),
        _ => fs::read(cwd.join(path))
            .map(|b| String::from_utf8_lossy(&b).into_owned())
            .unwrap_or_default(),
    };

    Ok(FilePair {
        path: path.to_string(),
        status: status.to_string(),
        original,
        modified,
    })
}

/// Confirma todos los cambios y devuelve el hash corto del nuevo commit.
pub fn commit_all(cwd: &Path, message: &str) -> AppResult<String> {
    if message.trim().is_empty() {
        return Err(AppError::msg("el mensaje del commit no puede estar vacío"));
    }
    git(cwd, &["add", "-A"])?;
    git(cwd, &["commit", "-m", message])?;
    Ok(git(cwd, &["rev-parse", "--short", "HEAD"])?.trim().to_string())
}

pub fn slugify(name: &str) -> String {
    let mut slug = String::new();
    for ch in name.to_lowercase().chars() {
        if ch.is_ascii_alphanumeric() {
            slug.push(ch);
        } else if !slug.ends_with('-') && !slug.is_empty() {
            slug.push('-');
        }
    }
    let slug = slug.trim_matches('-').to_string();
    if slug.is_empty() {
        "sesion".to_string()
    } else {
        slug.chars().take(40).collect()
    }
}

/// Crea un worktree hermano del proyecto y devuelve su ruta.
pub fn worktree_add(project: &Path, slug: &str, branch: &str) -> AppResult<PathBuf> {
    if !is_repo(project) {
        return Err(AppError::Git("el proyecto no es un repositorio git".into()));
    }

    let name = project.file_name().and_then(|n| n.to_str()).unwrap_or("proyecto");
    let parent = project
        .parent()
        .ok_or_else(|| AppError::msg("el proyecto no tiene carpeta padre"))?;

    let mut path = parent.join(format!("{name}-{slug}"));
    let mut n = 2;
    while path.exists() {
        path = parent.join(format!("{name}-{slug}-{n}"));
        n += 1;
    }

    let path_str = path.to_string_lossy().into_owned();
    let exists = git(
        project,
        &[
            "rev-parse",
            "--verify",
            "--quiet",
            &format!("refs/heads/{branch}"),
        ],
    )
    .is_ok();

    if exists {
        git(project, &["worktree", "add", &path_str, branch])?;
    } else {
        git(project, &["worktree", "add", "-b", branch, &path_str])?;
    }
    Ok(path)
}

pub fn worktree_remove(project: &Path, path: &Path) -> AppResult<()> {
    git(
        project,
        &["worktree", "remove", "--force", &path.to_string_lossy()],
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slugs_are_safe_for_paths_and_branches() {
        assert_eq!(slugify("Refactor: Auth/JWT!!"), "refactor-auth-jwt");
        assert_eq!(slugify("   "), "sesion");
    }
}
