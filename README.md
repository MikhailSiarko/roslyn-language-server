# Roslyn Language Server

[![Rust](https://github.com/MikhailSiarko/roslyn-language-server/actions/workflows/rust.yml/badge.svg)](https://github.com/MikhailSiarko/roslyn-language-server/actions/workflows/rust.yml)

A small wrapper around the language server used by the official C# Visual Studio Code extension (Microsoft.CodeAnalysis.LanguageServer). It launches and configures that language server so it can be used from editors other than VS Code (for example, Helix or Neovim). The wrapper handles starting the server with the right arguments and exposes a standard Language Server Protocol (LSP) endpoint that other editors can connect to.

## Usage
- Purpose: run Microsoft.CodeAnalysis.LanguageServer for an arbitrary editor by supplying the executable and the solution/projects to load.
- Behavior: the wrapper launches the provided Microsoft.CodeAnalysis.LanguageServer executable, passes the configured solution and projects to preload, and proxies LSP traffic over stdio or the chosen transport so non-VS Code editors can talk to the Roslyn language server.

## Required arguments
- cmd — absolute path to a Microsoft.CodeAnalysis.LanguageServer executable to run.
- solution_path — absolute path to the .sln file to load. (Optional)
- project_paths — one or more absolute paths to project files (.csproj) to load. This can be provided as a repeatable flag (e.g. --project_paths /p1 --project_paths /p2) or as a comma-separated list, depending on how you start the wrapper. (Optional, nut ignored if solution_path is provided)

## Example
- CLI (repeatable project paths):
  ./roslyn-ls-wrapper --cmd "/absolute/path/to/Microsoft.CodeAnalysis.LanguageServer" --solution_path "/absolute/path/to/MySolution.sln" --project_paths "/absolute/path/to/ProjectA/ProjectA.csproj" --project_paths "/absolute/path/to/ProjectB/ProjectB.csproj"

## Notes
- All paths must be absolute so the server can resolve project references reliably.
- The wrapper does not modify the language server binary; it only starts it with the requested arguments and exposes an LSP endpoint usable from any editor that can talk LSP (Helix, Neovim + nvim-lspconfig, etc.).
