# RS PDF

A high-performance Rust-based HTTP service for converting HTML to PDF using headless Chrome.

## Features

- Fast HTML to PDF conversion via REST API
- Configurable PDF options (margins, headers, footers, scaling, etc.)
- **Automatic Chrome download** - downloads latest stable Chrome if not found
- Optional page numbering control
- Base64 HTML input support
- Headless Chrome rendering engine
- Built with Axum for high-performance async handling

## Prerequisites

- Rust 1.70+ (edition 2021)
- Chrome/Chromium browser binary (automatically downloaded if not found)

## Installation

1. Clone the repository:
```bash
git clone https://github.com/BeesyOnGit/rs_pdf.git
cd rs_pdf
```

2. Build the project:
```bash
cargo build --release
```

3. (Optional) Set Chrome binary path:
```bash
export CHROME_PATH=/path/to/chrome
```

**Note:** If Chrome is not found at `CHROME_PATH` or the default location, the latest stable version will be automatically downloaded on first run.

## Usage

### Start the Server

```bash
cargo run --release
```

The server will start on `http://0.0.0.0:3005`

### API Endpoint

**POST** `/convert`

Converts HTML to PDF and returns the PDF file.

#### Request Body

**Minimum Request (only HTML required):**
```json
{
  "html": "<html><body><h1>Hello World</h1></body></html>"
}
```

**Full Request with All Options:**
```json
{
  "html": "<html><body><h1>Hello World</h1></body></html>",
  "pdf_options": {
    "landscape": false,
    "display_header_footer": true,
    "print_background": true,
    "scale": 1.0,
    "paper_width": 210.0,
    "paper_height": 297.0,
    "margin_top": 10.0,
    "margin_bottom": 10.0,
    "margin_left": 10.0,
    "margin_right": 10.0,
    "page_ranges": "",
    "header_template": "<div style='font-size:10px;text-align:center;width:100%;'>Header</div>",
    "footer_template": "<div style='font-size:10px;text-align:center;width:100%;'>Page <span class='pageNumber'></span> of <span class='totalPages'></span></div>",
    "prefer_css_page_size": false
  }
}
```

**Note:** Both `pdf_options` and all fields within it are completely optional. You can:
- Omit `pdf_options` entirely (uses all defaults)
- Include `pdf_options` with only the fields you want to customize
- Specify all fields for full control

#### Response

Returns a PDF file with `Content-Type: application/pdf`

#### Examples with cURL

**Minimal (no options):**
```bash
curl -X POST http://localhost:3005/convert \
  -H "Content-Type: application/json" \
  -d '{"html":"<html><body><h1>Hello World</h1></body></html>"}' \
  --output output.pdf
```

**Partial options (only what you need):**
```bash
curl -X POST http://localhost:3005/convert \
  -H "Content-Type: application/json" \
  -d '{"html":"<html><body><h1>Landscape</h1></body></html>","pdf_options":{"landscape":true,"margin_top":25.0}}' \
  --output landscape.pdf
```

**Full options (complete control):**
```bash
curl -X POST http://localhost:3005/convert \
  -H "Content-Type: application/json" \
  -d @example_request.json \
  --output custom.pdf
```

## PDF Options Flexibility

You have **three ways** to use the API:

| Approach | Example | Use Case |
|----------|---------|----------|
| **No options** | `{"html": "..."}` | Quick conversions with sensible defaults |
| **Partial options** | `{"html": "...", "pdf_options": {"landscape": true}}` | Customize only what you need |
| **Full options** | `{"html": "...", "pdf_options": {...all fields...}}` | Complete control over output |

## PDF Options Reference

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `landscape` | boolean | false | Paper orientation |
| `display_header_footer` | boolean | true | Display header and footer |
| `print_background` | boolean | true | Print background graphics |
| `scale` | number | 1.0 | Scale of the webpage rendering (0.1 - 2.0) |
| `paper_width` | number | 210.0 | Paper width in millimeters (default: A4) |
| `paper_height` | number | 297.0 | Paper height in millimeters (default: A4) |
| `margin_top` | number | 10.0 | Top margin in millimeters |
| `margin_bottom` | number | 10.0 | Bottom margin in millimeters |
| `margin_left` | number | 10.0 | Left margin in millimeters |
| `margin_right` | number | 10.0 | Right margin in millimeters |
| `page_ranges` | string | "" | Paper ranges to print (e.g., "1-5, 8, 11-13") |
| `header_template` | string | "" | HTML template for header |
| `footer_template` | string | Default footer | HTML template for footer |
| `prefer_css_page_size` | boolean | false | Use CSS-defined page size |
| `show_page_numbers` | boolean | true | Show page numbers in footer |

## Development

```bash
# Run in development mode
cargo run

# Run tests
cargo test

# Build for release
cargo build --release
```

## License

MIT License - see LICENSE file for details

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
