use anyhow::{Result, anyhow, bail};
use faccess::{self, PathExt};
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

    let bin = match server_path.extension().and_then(|e| e.to_str()) {
        Some("exe") => Binary::Exe(server_path),
        Some("dll") => Binary::Dll(server_path),
        Some(_) if server_path.executable() => Binary::Exe(server_path),
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
            .expect("Unable to create logs directory");
    }

    Ok(logs_path)
}

#[cfg(test)]
mod tests {
    use crate::path::Binary;

    use super::get_binary;
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
    fn get_binary_returns_error_when_path_not_exist() {
        let tmp = TempDir::new().unwrap();
        create_dirs(&tmp);
        let path = tmp.path().join("one").join("one.e");

        let result = get_binary(&path);

        assert!(result.is_err());
    }

    #[test]
    fn get_binary_returns_ok_when_path_exists() {
        let tmp = TempDir::new().unwrap();
        create_dirs(&tmp);
        let path = tmp.path().join("two").join("lsp.exe");

        let result = get_binary(&path);

        assert!(result.is_ok());
    }

    #[test]
    fn get_binary_returns_error_when_path_exists_but_dir() {
        let tmp = TempDir::new().unwrap();
        create_dirs(&tmp);
        let path = tmp.path().join("two");

        let result = get_binary(&path);

        assert!(result.is_err());
    }

    #[test]
    fn get_binary_returns_dll_when_path_exists() {
        let tmp = TempDir::new().unwrap();
        create_dirs(&tmp);
        let path = tmp.path().join("two").join("lsp.dll");

        let Ok(binary) = get_binary(&path) else {
            panic!("Error returned!");
        };

        match binary {
            Binary::Exe(_) => panic!("Exe returned!"),
            _ => (),
        }
    }

    #[test]
    fn get_binary_returns_exe_when_path_exists() {
        let tmp = TempDir::new().unwrap();
        create_dirs(&tmp);
        let path = tmp.path().join("two").join("lsp.exe");

        let Ok(binary) = get_binary(&path) else {
            panic!("Error returned!");
        };

        match binary {
            Binary::Dll(_) => panic!("Exe returned!"),
            _ => (),
        }
    }
}
