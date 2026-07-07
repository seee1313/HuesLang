# HuesLang Architecture

HuesLang is currently a toy systems programming language, but with the ambitious goal of becoming a production-ready tool.

## Project Structure
Here is a quick overview of the codebase to help you get started:

```tree
.
├── Cargo.toml      # Project dependencies and metadata
├── README.md       # Main documentation
└── src/
    ├── main.rs     # CLI entry point (reads file, triggers compilation)
    ├── token.rs    # Lexer: Defines tokens and extracts them from source text
    ├── ast.rs      # AST: Defines the Abstract Syntax Tree and Type system
    ├── parser.rs   # Parser: Converts a stream of tokens into an AST
    └── codegen.rs  # Backend: Generates LLVM IR from the AST
```

## Compilation Pipeline
If you want to contribute, it is helpful to understand how data flows through the compiler:

1. **Source Code** (`.hues` file) is read by `main.rs`.
2. **Lexical Analysis** (`token.rs`): The text is broken down into `Tokens` (e.g., `Let`, `Ident("x")`, `Assign`).
3. **Parsing** (`parser.rs`): Tokens are validated and assembled into an `AST` (Abstract Syntax Tree) using a Pratt parsing algorithm for expressions.
4. **Code Generation** (`codegen.rs`): The AST is traversed to generate LLVM IR (Intermediate Representation), which can then be compiled into native machine code.
