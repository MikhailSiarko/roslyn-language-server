# Roslyn Language Server

## Work in Progress

A small wrapper around the language server used by the official C# Visual Studio Code extension (Microsoft.CodeAnalysis.LanguageServer). It launches and configures that language server so it can be used from editors other than VS Code (for example, Helix or Neovim). The wrapper handles starting the server with the right arguments and exposes a standard Language Server Protocol (LSP) endpoint that other editors can connect to.

## Usage

- Purpose: run Microsoft.CodeAnalysis.LanguageServer for an arbitrary editor by supplying the executable and the solution/projects to load.
- Behavior: the wrapper launches the provided Microsoft.CodeAnalysis.LanguageServer executable, passes the configured solution and projects to preload, and proxies LSP traffic over stdio or the chosen transport so non-VS Code editors can talk to the Roslyn language server.

## Required arguments

- cmd — an absolute path to a Microsoft.CodeAnalysis.LanguageServer executable to run.
- working_dir — an absolute path to a project directory, if solution and projects are not provided, the directory will be scanned for a sln or csproj files. (Optional, ignored if any of solution or projects provided)

## Example

- CLI (repeatable project paths):
  ./roslyn-ls-wrapper --cmd "/absolute/path/to/Microsoft.CodeAnalysis.LanguageServer" --working_dir "/absolute/path/to/working_dir"

## Notes

- All paths must be absolute so the server can resolve project references reliably.
- The wrapper does not modify the language server binary; it only starts it with the requested arguments and exposes an LSP endpoint usable from any editor that can talk LSP (Helix, Neovim + nvim-lspconfig, etc.).

## Inspired by

- [SofusA csharp-language-server](https://github.com/SofusA/csharp-language-server)
- [seblyng roslyn.nvim](https://github.com/seblyng/roslyn.nvim)
