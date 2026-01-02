# StyleSense

StyleSense is a real-time, customizable code style enforcer for VS Code, powered by a Rust-based LSP server and Tree-sitter parsing. It detects style violations, provides instant feedback with yellow squiggles, and offers one-click auto-fixes based on user-defined rules.

## Features

StyleSense brings the following features to VS Code:

- **Real-time Style Enforcement**: Detects style violations as you type
- **Visual Feedback**: Highlights style issues with yellow squiggles
- **Language Support**: Currently works with C and C++ files
- **Customizable Rules**: Define your own coding style preferences

## Current Implementation

The current version includes:

- Basic Language Server Protocol (LSP) integration
- Style checking for spacing around operators (e.g., `=`, `>`, etc.)
- Visual feedback for style violations
- Integration with VS Code's diagnostics system

## Requirements

- VS Code 1.98.0 or higher
- Rust toolchain (for building the LSP server)

## Extension Settings

This extension contributes the following settings:

* `stylesense.enabled`: Enable/disable this extension
* `stylesense.trace.server`: Controls the verbosity of the LSP server logs

## Development

1. Clone the repository
2. Build the LSP server:
   ```
   cd lsp
   cargo build
   ```
3. Build the extension:
   ```
   npm install
   npm run compile
   ```
4. Press F5 to open a new window with the extension loaded

## Testing

See the `TEST_PLAN.md` file for detailed testing procedures.

## License

This project is licensed under the MIT License - see the LICENSE file for details.