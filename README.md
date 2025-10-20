# Roslyn Language Server

Roslyn Language Server is an easy-to-use language server implementation built on top of the Roslyn APIs. It provides Language Server Protocol (LSP) features for C# (and other .NET languages supported by Roslyn) with a focus on simplicity, extensibility, and minimal setup.

## Goals
- Provide a lightweight, easy-to-integrate LSP server using the Roslyn compiler platform.
- Offer core editor features (completion, diagnostics, go-to-definition, hover, symbol search) out of the box.
- Make it simple to extend and customize language behaviors via Roslyn analyzers and workspace services.
- Support smooth integration with editors and IDEs that speak LSP.

## Key Features
- Diagnostics and real-time error reporting using Roslyn analyzers
- Auto-completion and signature help powered by Roslyn semantic model
- Go-to-definition, find-references, and document symbols
- Hover information and rich tooltips
- Workspace/solution awareness and project-level analysis
- Simple extension points for adding custom analyzers or code actions

## Quick Start
1. Clone the repository:
   git clone https://example.com/roslyn-language-server.git

2. Build and run the server:
   cd roslyn-language-server
   dotnet build
   dotnet run --project src/RoslynLanguageServer

3. Configure your editor to point at the running LSP endpoint (stdio, TCP, or named pipe, depending on your setup).

## Getting Started for Integrators
- The server exposes standard LSP endpoints; editors can connect over stdio or TCP.
- Configure workspace roots to enable solution and project discovery.
- Add Roslyn analyzers or custom workspace services to extend behavior.
- See the docs/ folder for sample editor configurations and protocol settings.

## Contributing
Contributions are welcome. Please open issues for bugs or feature requests and submit pull requests for enhancements. Follow the coding and test guidelines in CONTRIBUTING.md.

## License
This project is licensed under the MIT License. See LICENSE for details.
