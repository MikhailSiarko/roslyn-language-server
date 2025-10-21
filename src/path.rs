#![allow(dead_code)]

use anyhow::{Result, anyhow};
use std::path::{Path, PathBuf};

pub enum Binary {
    Exe(PathBuf),
    Dll(PathBuf),
}

enum Env {
    Gnu,
    Musl,
    None,
}

enum Arch {
    X64,
    Arm64,
}

enum Os {
    Windows,
    Linux,
    MacOS,
    Neutral,
}

pub fn get_binary(server_path: &Path) -> Result<Binary> {
    if !server_path.exists() {
        return Err(anyhow!(
            "The specified language server path does not exist: {:?}",
            server_path.display()
        ));
    }

    let target = match (os(), arch(), env()) {
        (Os::Windows, Arch::X64, _) => "win-x64",
        (Os::Windows, Arch::Arm64, _) => "win-arm64",
        (Os::Linux, Arch::X64, Env::Gnu) => "linux-x64",
        (Os::Linux, Arch::X64, Env::Musl) => "linux-x64-musl",
        (Os::Linux, Arch::Arm64, Env::Gnu) => "linux-arm64",
        (Os::Linux, Arch::Arm64, Env::Musl) => "linux-arm64-musl",
        (Os::MacOS, Arch::X64, _) => "osx-x64",
        (Os::MacOS, Arch::Arm64, _) => "osx-arm64",
        _ => "neutral",
    };

    let path = server_path
        .join("content")
        .join("LanguageServer")
        .join(target);

    let bin = match os() {
        Os::Windows => Binary::Exe(path.join("Microsoft.CodeAnalysis.LanguageServer.exe")),
        Os::Linux => Binary::Exe(path.join("Microsoft.CodeAnalysis.LanguageServer")),
        Os::Neutral | Os::MacOS => {
            Binary::Dll(path.join("Microsoft.CodeAnalysis.LanguageServer.dll"))
        }
    };

    Ok(bin)
}

#[allow(unreachable_code)]
const fn os() -> Os {
    #[cfg(target_os = "windows")]
    return Os::Windows;

    #[cfg(target_os = "linux")]
    return Os::Linux;

    #[cfg(target_os = "macos")]
    return Os::MacOS;

    Os::Neutral
}

#[allow(unreachable_code)]
const fn arch() -> Arch {
    #[cfg(target_arch = "x86_64")]
    return Arch::X64;

    return Arch::Arm64;
}

#[allow(unreachable_code)]
const fn env() -> Env {
    #[cfg(target_env = "gnu")]
    return Env::Gnu;

    #[cfg(target_env = "musl")]
    return Env::Musl;

    Env::None
}
