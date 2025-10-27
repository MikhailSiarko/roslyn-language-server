use anyhow::{Context, Result, anyhow, bail};
use faccess::{self, PathExt};
use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};
use url::Url;

pub fn cmd(server_path: &Path) -> Result<String> {
    if !server_path.exists() {
        bail!(
            "The specified language server path does not exist: {:?}",
            server_path.display()
        );
    }

    let Ok(metadata) = std::fs::metadata(server_path) else {
        bail!(
            "Failed to retrieve path metadata: {:?}",
            server_path.display()
        )
    };

    if metadata.is_dir() {
        bail!(
            "The specified language server path is not a path to a file: {:?}",
            server_path.display()
        );
    }

    let cmd = server_path.display().to_string();
    let bin = match server_path.extension().and_then(|e| e.to_str()) {
        Some("exe") => cmd,
        Some("dll") => format!("dotnet exec {cmd}"),
        Some(_) if server_path.executable() => cmd,
        _ => bail!("No language server executable found"),
    };

    Ok(bin)
}

pub async fn get_logs_path(server_path: &Path) -> Result<PathBuf> {
    let logs_path = server_path
        .parent()
        .map(|p| p.join("logs"))
        .ok_or_else(|| anyhow!("Unable to determine logs path"))?;

    if !logs_path.exists() {
        smol::fs::create_dir_all(&logs_path)
            .await
            .context("Unable to create logs directory")?;
    }

    Ok(logs_path)
}

pub fn find_solution_file(workspace_path: &Path) -> Option<String> {
    find_extension(
        workspace_path,
        &vec![OsStr::new("sln"), OsStr::new("slnx"), OsStr::new("slnf")],
    )
    .filter_map(|s| Url::from_file_path(s.as_path()).map(|u| u.to_string()).ok())
    .next()
}

pub fn find_projects_files(workspace_path: &Path) -> Vec<String> {
    find_extension(workspace_path, &vec![OsStr::new("csproj")])
        .filter_map(|p| Url::from_file_path(p.as_path()).map(|p| p.to_string()).ok())
        .collect()
}

fn find_extension(root_path: &Path, ext: &Vec<&'static OsStr>) -> impl Iterator<Item = PathBuf> {
    let mut found_paths: Vec<(usize, PathBuf)> = ignore::Walk::new(root_path)
        .filter_map(|res| res.ok())
        .filter_map(|d| path_for_file_with_extension(&d, ext).map(|p| (d.depth(), p)))
        .collect();

    found_paths.sort_by(|(depth_a, path_a), (depth_b, path_b)| {
        depth_a.cmp(depth_b).then_with(|| path_a.cmp(path_b))
    });

    found_paths
        .into_iter()
        .map(|(_, p)| p.canonicalize())
        .filter_map(|r| r.ok())
}

fn path_for_file_with_extension(
    dir: &ignore::DirEntry,
    ext: &Vec<&'static OsStr>,
) -> Option<PathBuf> {
    if dir.path().is_file() && dir.path().extension().is_some_and(|e| ext.contains(&e)) {
        return Some(dir.path().to_path_buf());
    }
    None
}

#[cfg(test)]
mod tests {
    use super::cmd;
    use std::fs;
    use tempfile::TempDir;

    fn create(path: &std::path::Path) {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(path, b"").unwrap();
    }

    fn create_dirs(tmp: &TempDir) {
        let root = tmp.path();

        // root/
        //   one.a
        //   two/
        //     lsp.exe
        //     lsp.dll
        //     lsp
        create(&root.join("one.a"));
        create(&root.join("two").join("lsp.exe"));
        create(&root.join("two").join("lsp.dll"));
        create(&root.join("two").join("lsp"));
    }

    #[test]
    fn cmd_returns_error_when_path_not_exist() {
        let tmp = TempDir::new().unwrap();
        create_dirs(&tmp);
        let path = tmp.path().join("one").join("one.e");

        let result = cmd(&path);

        assert!(result.is_err());
    }

    #[test]
    fn cmd_returns_ok_when_path_exists() {
        let tmp = TempDir::new().unwrap();
        create_dirs(&tmp);
        let path = tmp.path().join("two").join("lsp.exe");

        let result = cmd(&path);

        assert!(result.is_ok());
    }

    #[test]
    fn cmd_returns_error_when_path_exists_but_dir() {
        let tmp = TempDir::new().unwrap();
        create_dirs(&tmp);
        let path = tmp.path().join("two");

        let result = cmd(&path);

        assert!(result.is_err());
    }

    #[test]
    fn cmd_returns_dll_when_path_exists() {
        let tmp = TempDir::new().unwrap();
        create_dirs(&tmp);
        let path = tmp.path().join("two").join("lsp.dll");

        let Ok(_) = cmd(&path) else {
            panic!("Error returned!");
        };
    }

    #[test]
    fn cmd_returns_exe_when_path_exists() {
        let tmp = TempDir::new().unwrap();
        create_dirs(&tmp);
        let path = tmp.path().join("two").join("lsp.exe");

        let Ok(_) = cmd(&path) else {
            panic!("Error returned!");
        };
    }
}
