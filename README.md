# mdvi-web

> **WebAssembly version of mdvi - A browser-based Markdown viewer**

---

## About

`mdvi-web` is a WebAssembly-powered Markdown viewer that runs entirely in the browser. It's a browser adaptation of the excellent [mdvi](https://github.com/taf2/mdvi) terminal markdown viewer, rewritten using Rust and WebAssembly for performance and portability.

**Original Project:** https://github.com/taf2/mdvi
**Original Author:** taf2 (https://github.com/taf2)
**Original License:** MIT

### Why WebAssembly?

By rewriting mdvi in WebAssembly, we can:

- Run entirely in the browser with no server required
- Leverage Rust's performance for fast markdown parsing
- Deploy easily to static hosting like GitHub Pages
- Provide cross-platform compatibility without installation

## Features

- **Pure WebAssembly**: Markdown parsing powered by Rust and pulldown-cmark
- **Full Markdown Support**: Headings, lists, task lists, blockquotes, tables, footnotes, code blocks with syntax highlighting indicators
- **Vim-style Navigation**:
  - `j` / `↓` - Scroll down
  - `k` / `↑` - Scroll up
  - `Ctrl-d` - Half-page down
  - `Ctrl-u` - Half-page up
  - `PageDown` / `f` - Full page down
  - `PageUp` / `b` - Full page up
  - `g` / `Home` - Jump to top
  - `G` / `End` - Jump to bottom
  - `q` - Quit and clear content
- **Dark Theme**: Eye-friendly dark mode inspired by VS Code
- **Responsive Design**: Works on desktop and mobile devices
- **No Server Required**: Everything runs client-side

## Live Demo

Coming soon to GitHub Pages!

## Installation

### For Users

Simply open the `index.html` file in your browser, or deploy it to a web server.

### For Developers

Prerequisites:
- Rust (1.70 or later)
- Node.js (for the HTTP server)
- wasm-pack

Install wasm-pack:
```bash
cargo install wasm-pack
```

## Usage

### Building

```bash
# Build the WebAssembly module
npm run build:wasm

# Or directly with wasm-pack
wasm-pack build --target web --out-dir pkg
```

### Running Locally

```bash
# Start a local HTTP server
npm run serve

# Or use Python
python3 -m http.server 8080

# Or use Node.js http-server
npx http-server
```

Then open `http://localhost:8080` in your browser.

### Using the Viewer

1. Click "Open File" button
2. Select a Markdown file (`.md`, `.markdown`, or `.txt`)
3. Use keyboard shortcuts to navigate
4. Press `q` to quit and clear content

## Development

```bash
# Build in debug mode for faster compilation
wasm-pack build --dev --target web --out-dir pkg

# Build in release mode for optimized output
wasm-pack build --target web --out-dir pkg

# Run tests
cargo test

# Format code
cargo fmt

# Lint
cargo clippy
```

## Project Structure

```
mdvi-web/
├── src/
│   └── lib.rs          # Rust WASM module (markdown parsing)
├── pkg/                # Generated WASM files (after build)
├── index.html          # Web interface
├── Cargo.toml          # Rust dependencies
├── package.json        # Build scripts
└── README.md           # This file
```

## Technical Details

### Dependencies

- **pulldown-cmark**: Fast markdown parser
- **wasm-bindgen**: JavaScript-Rust bindings
- **wasm-pack**: Tool for building Rust WebAssembly packages

### Browser Compatibility

Works in all modern browsers that support WebAssembly:
- Chrome 57+
- Firefox 52+
- Safari 11+
- Edge 16+

## Deployment

### GitHub Pages

1. Build the project:
   ```bash
   npm run build:wasm
   ```

2. Deploy to GitHub Pages using gh-pages:
   ```bash
   npm install -g gh-pages
   gh-pages -d . -b gh-pages
   ```

3. Enable GitHub Pages in your repository settings (source: `gh-pages` branch)

### Other Static Hosting

The project can be deployed to any static hosting service:
- Netlify
- Vercel
- Cloudflare Pages
- AWS S3 + CloudFront

Simply upload all files (including the `pkg/` directory) to your hosting provider.

## Limitations

- No syntax highlighting for code blocks (planned feature)
- Image rendering is limited (images are displayed but not processed)
- No live reload from disk (browser security restriction)
- No search functionality (planned feature)

## Future Improvements

- [ ] Syntax highlighting for code blocks
- [ ] Search functionality
- [ ] Table of contents sidebar
- [ ] Multiple themes (light/dark/custom)
- [ ] Export to HTML/PDF
- [ ] Collaborative editing

## License

This project is distributed under the same MIT license as the original mdvi project.

MIT License

Copyright (c) 2026 Jerry Shen

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.

## Acknowledgments

- Original mdvi project by [taf2](https://github.com/taf2)
- [pulldown-cmark](https://github.com/raphlinus/pulldown-cmark) for the excellent markdown parser
- [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen) for Rust-WASM bindings
- The Rust WebAssembly community

## Contact

- Author: Jerry Shen
- Email: awesomejerryshen@gmail.com
- Repository: https://github.com/awesomejerryshen/mdvi-web
