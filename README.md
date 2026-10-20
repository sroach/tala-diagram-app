# TALA Diagram App

A Rust-based web application that uses the **TALA layout engine** (via D2) to render diagrams from text, compiled to WebAssembly.

## Overview

This project provides a Rust library compiled to WebAssembly that bridges to the `@d2lang/d2` JavaScript/WASM library, enabling client-side diagram rendering with the TALA layout engine. TALA is an orthogonal layout engine designed specifically for software architecture diagrams.

## Features

- **TALA Layout Engine**: Uses D2's TALA layout for beautiful architecture diagrams
- **Multiple Layout Engines**: Switch between TALA, Dagre, and ELK layouts
- **Sketch Mode**: Optional hand-drawn sketch style
- **Dark Mode Support**: Automatic light/dark theme switching
- **Sample Diagrams**: Built-in samples for quick testing
- **SVG Export**: Download rendered diagrams as SVG files
- **Keyboard Shortcuts**: Ctrl+Enter to render

## Prerequisites

- Rust 1.70+ (install from [rustup.rs](https://rustup.rs/))
- wasm-pack (install with `cargo install wasm-pack`)
- A modern web browser

## Build

```bash
# Build the WASM module
wasm-pack build --target web

# This creates the pkg/ directory with:
# - pkg/tala_diagram_app.js
# - pkg/tala_diagram_app_bg.wasm
# - pkg/tala_diagram_app.d.ts
```

## Run

After building, serve the project root with a local HTTP server:

```bash
# Using Python
python3 -m http.server 8080

# Using Node.js
npx serve .

# Using Rust
cargo install static-web-server
static-web-server --port 8080
```

Then open `http://localhost:8080/index.html` in your browser.

## Usage

1. Enter D2 diagram source in the left panel, or click a sample button
2. Select a layout engine (TALA, Dagre, or ELK)
3. Optionally enable Sketch mode
4. Click "Render" or press Ctrl+Enter
5. View the result in the right panel
6. Click "Download SVG" to save the diagram

## D2 Syntax Examples

### Basic Diagram
```
cloud: {shape: cloud}
server: {shape: cylinder}
database: {shape: database}

cloud -> server
server -> database
```

### Architecture Diagram
```
Frontend: {shape: hexagon}
API Gateway: {shape: trapezoid}
Auth Service
User Service
Database: {shape: cylinder}

Frontend -> API Gateway
API Gateway -> Auth Service
API Gateway -> User Service
Auth Service -> Database
User Service -> Database
```

### Sequence Diagram
```
sequence: {
  shape: sequence_diagram
  User -> API
  API -> Database
  Database -> API: Result
  API -> User: Response
}
```

## API

The Rust library exposes the following functions to JavaScript:

- `render_d2_to_svg(source, layout)` → `RenderResult` - Render D2 source to SVG
- `sample_diagram()` → `String` - Returns a basic sample diagram
- `sample_architecture()` → `String` - Returns an architecture sample
- `sample_sequence()` → `String` - Returns a sequence diagram sample

### RenderResult
- `svg` (String) - The rendered SVG string
- `error` (String) - Error message if rendering failed
- `has_error` (boolean) - Whether an error occurred

## Project Structure

```
tala-diagram-app/
├── Cargo.toml          # Rust package configuration
├── index.html          # Web application
├── src/
│   └── lib.rs          # Rust library with WASM bindings
└── pkg/                # Generated WASM output (after build)
    ├── tala_diagram_app.js
    ├── tala_diagram_app_bg.wasm
    └── tala_diagram_app.d.ts
```

## Dependencies

- **wasm-bindgen**: Rust ↔ JavaScript interop
- **wasm-bindgen-futures**: Async/await support for WASM
- **js-sys**: JavaScript type bindings
- **web-sys**: Web API bindings
- **@d2lang/d2**: D2 compiler with TALA layout (loaded from CDN)

## License

MPL-2.0 (same as D2/TALA)
