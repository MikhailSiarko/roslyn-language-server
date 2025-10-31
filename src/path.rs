use anyhow::{Context, Result, anyhow};
use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};
use url::Url;

pub async fn get_logs_path(server_path: &Path) -> Result<PathBuf> {
    let logs_path = server_path
        .parent()
        .map(|p| p.join("logs"))
        .ok_or_else(|| anyhow!("Unable to determine logs path"))?;

    if !logs_path.exists() {
        tokio::fs::create_dir_all(&logs_path)
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
