use anyhow::{Result, anyhow, bail};
use std::path::{Path, PathBuf};

pub enum Binary<'a> {
    Exe(&'a Path),
    Dll(&'a Path),
}

pub fn get_binary<'a>(server_path: &'a Path) -> Result<Binary<'a>> {
    if !server_path.exists() {
        bail!(
            "The specified language server path does not exist: {:?}",
            server_path.display()
        );
    }

    let bin = match server_path.extension().and_then(|e| e.to_str()) {
        Some("exe") => Binary::Exe(server_path),
        Some("dll") => Binary::Dll(server_path),
        _ => Binary::Exe(server_path),
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
            .expect("Unable to create logs directory");
    }

    Ok(logs_path)
}
