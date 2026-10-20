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
# Build the WASM module for development
wasm-pack build --release --out-dir pkg
```

## Development Server

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

## Installation

### Quick Start — Download the Zip

1. Go to the **Releases** page on GitHub
2. Download `tala-diagram-app.zip` from the latest release
3. Extract it into your web directory:

```bash
unzip tala-diagram-app.zip -d /var/www/html/diagrams/
```

That's it — open `index.html` in a browser or point your web server at the extracted folder.

### Building from Source

#### Local Build

To package the app for deployment to any static web server:

```bash
./build-release.sh
```

This creates a `release/` directory and a `tala-diagram-app.zip` archive:

```
release/
├── index.html
└── pkg/
    ├── tala_diagram_app.js
    ├── tala_diagram_app_bg.js
    └── tala_diagram_app_bg.wasm
```

The release package is approximately **80KB** and has no server-side dependencies.

#### GitHub Actions Release

Push a version tag to automatically build and publish a release:

```bash
git tag v0.1.0
git push origin v0.1.0
```

The GitHub Actions workflow (`.github/workflows/release.yml`) will:

1. Build the WASM module
2. Package the release files into a zip archive
3. Create a GitHub Release with `tala-diagram-app.zip` as a downloadable artifact

You can also trigger the workflow manually from the **Actions** tab → **Release** → **Run workflow**.

### Deploying the Release

Extract the zip into your web directory, or copy the `release/` folder to any static web server:

**nginx:**
```nginx
server {
    listen 80;
    server_name diagrams.example.com;
    root /path/to/extracted-folder;
    index index.html;
}
```

**Apache:**
```apache
DocumentRoot "/path/to/extracted-folder"
```

**GitHub Pages:**
```bash
# Push release/ contents to a gh-pages branch
gh-pages-publish release/
```

**Python (quick test):**
```bash
cd release && python3 -m http.server 8080
```

**Any static hosting** (Netlify, Vercel, S3 + CloudFront, etc.):
Simply upload the extracted files.

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
├── build-release.sh    # Release packaging script
├── index.html          # Web application
├── src/
│   └── lib.rs          # Rust library with WASM bindings
├── pkg/                # Generated WASM output (after build, gitignored)
│   ├── tala_diagram_app.js
│   ├── tala_diagram_app_bg.js
│   ├── tala_diagram_app_bg.wasm
│   └── tala_diagram_app.d.ts
└── release/            # Production release (after ./build-release.sh, gitignored)
    ├── index.html
    └── pkg/
        ├── tala_diagram_app.js
        ├── tala_diagram_app_bg.js
        └── tala_diagram_app_bg.wasm
```

## Dependencies

- **wasm-bindgen**: Rust ↔ JavaScript interop
- **wasm-bindgen-futures**: Async/await support for WASM
- **js-sys**: JavaScript type bindings
- **web-sys**: Web API bindings
- **@d2lang/d2**: D2 compiler with TALA layout (loaded from CDN)

## License

MPL-2.0 (same as D2/TALA)
