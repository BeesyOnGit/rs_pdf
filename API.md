# API Documentation

## Endpoint

### POST /convert

Converts HTML content to a PDF document.

**URL:** `http://localhost:3005/convert`

**Method:** `POST`

**Content-Type:** `application/json`

---

## Request Body

### Required Fields

| Field | Type | Description |
|-------|------|-------------|
| `html` | string | The HTML content to convert to PDF |

### Optional Fields

| Field | Type | Description |
|-------|------|-------------|
| `pdf_options` | object | Configuration options for PDF generation (see below). **Completely optional** - omit entirely to use all defaults |

---

## PDF Options

**The entire `pdf_options` object is optional.** You can:
- Omit it completely to use all default values
- Include only specific fields you want to customize
- Include all fields for full control

All fields within `pdf_options` are also optional and will use defaults if not specified.

| Field | Type | Default | Range/Constraints | Description |
|-------|------|---------|-------------------|-------------|
| `landscape` | boolean | `false` | - | Paper orientation (false = portrait, true = landscape) |
| `display_header_footer` | boolean | `true` | - | Whether to display header and footer |
| `print_background` | boolean | `true` | - | Whether to print background colors and images |
| `scale` | number | `1.0` | 0.1 - 2.0 | Scale of the webpage rendering |
| `paper_width` | number | `210.0` | > 0 | Paper width in millimeters |
| `paper_height` | number | `297.0` | > 0 | Paper height in millimeters |
| `margin_top` | number | `10.0` | ≥ 0 | Top margin in millimeters |
| `margin_bottom` | number | `10.0` | ≥ 0 | Bottom margin in millimeters |
| `margin_left` | number | `10.0` | ≥ 0 | Left margin in millimeters |
| `margin_right` | number | `10.0` | ≥ 0 | Right margin in millimeters |
| `page_ranges` | string | `""` | - | Paper ranges to print, e.g., "1-5, 8, 11-13" |
| `header_template` | string | `""` | - | HTML template for the print header |
| `footer_template` | string | Default footer | - | HTML template for the print footer |
| `prefer_css_page_size` | boolean | `false` | - | Whether to prefer page size as defined by CSS |
| `show_page_numbers` | boolean | `true` | - | Whether to show page numbers in the footer |

### Default Footer Template

```html
<div style="width:100%;font-size:10px;color:#555;text-align:center;">
  Page <span class="pageNumber"></span> of <span class="totalPages"></span>
</div>
```

### Header/Footer Template Variables

The following variables can be used in header and footer templates:

- `<span class="pageNumber"></span>` - Current page number
- `<span class="totalPages"></span>` - Total number of pages
- `<span class="date"></span>` - Formatted print date
- `<span class="title"></span>` - Document title
- `<span class="url"></span>` - Document URL

---

## Response

**Success (200 OK):**
- **Content-Type:** `application/pdf`
- **Body:** Binary PDF data

**Error (500 Internal Server Error):**
- **Content-Type:** `text/plain`
- **Body:** Error message describing what went wrong

---

## Usage Patterns

### Pattern 1: No Options (Use All Defaults)
Simply send HTML without any `pdf_options`:
```json
{
  "html": "<html><body><h1>Hello</h1></body></html>"
}
```

### Pattern 2: Partial Options (Most Common)
Send only the options you want to customize. All other options use defaults:
```json
{
  "html": "<html><body><h1>Hello</h1></body></html>",
  "pdf_options": {
    "landscape": true,
    "margin_top": 25.0
  }
}
```

### Pattern 3: Full Options (Complete Control)
Specify every option explicitly:
```json
{
  "html": "<html><body><h1>Hello</h1></body></html>",
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
    "header_template": "",
    "footer_template": "",
    "prefer_css_page_size": false
  }
}
```

---

## Examples

### 1. Basic Conversion (Minimal Request)

```bash
curl -X POST http://localhost:3005/convert \
  -H "Content-Type: application/json" \
  -d '{
    "html": "<html><body><h1>Hello World</h1></body></html>"
  }' \
  --output output.pdf
```

### 2. Full Custom Options

```bash
curl -X POST http://localhost:3005/convert \
  -H "Content-Type: application/json" \
  -d '{
    "html": "<!DOCTYPE html><html><head><title>Invoice</title></head><body><h1>Invoice #12345</h1></body></html>",
    "pdf_options": {
      "landscape": false,
      "display_header_footer": true,
      "print_background": true,
      "scale": 1.0,
      "paper_width": 210.0,
      "paper_height": 297.0,
      "margin_top": 15.0,
      "margin_bottom": 15.0,
      "margin_left": 12.5,
      "margin_right": 12.5,
      "header_template": "<div style=\"font-size:10px;text-align:center;width:100%;\">Company Name</div>",
      "footer_template": "<div style=\"font-size:10px;text-align:center;width:100%;\">Page <span class=\"pageNumber\"></span></div>"
    }
  }' \
  --output invoice.pdf
```

### 3. Landscape Document

```bash
curl -X POST http://localhost:3005/convert \
  -H "Content-Type: application/json" \
  -d '{
    "html": "<html><body><h1>Wide Report</h1></body></html>",
    "pdf_options": {
      "landscape": true,
      "paper_width": 297.0,
      "paper_height": 210.0
    }
  }' \
  --output landscape.pdf
```

### 4. Specific Page Ranges

```bash
curl -X POST http://localhost:3005/convert \
  -H "Content-Type: application/json" \
  -d '{
    "html": "<html><body><h1>Page 1</h1><div style=\"page-break-after:always;\"></div><h1>Page 2</h1><div style=\"page-break-after:always;\"></div><h1>Page 3</h1></body></html>",
    "pdf_options": {
      "page_ranges": "1,3"
    }
  }' \
  --output specific_pages.pdf
```

### 5. No Headers/Footers

```bash
curl -X POST http://localhost:3005/convert \
  -H "Content-Type: application/json" \
  -d '{
    "html": "<html><body><h1>Clean Document</h1></body></html>",
    "pdf_options": {
      "display_header_footer": false
    }
  }' \
  --output no_header_footer.pdf
```

### 6. Without Page Numbers

```bash
curl -X POST http://localhost:3005/convert \
  -H "Content-Type: application/json" \
  -d '{
    "html": "<html><body><h1>Document Without Page Numbers</h1></body></html>",
    "pdf_options": {
      "show_page_numbers": false
    }
  }' \
  --output no_page_numbers.pdf
```

### 7. Using with JavaScript/Node.js

```javascript
const axios = require('axios');
const fs = require('fs');

async function convertToPdf() {
  try {
    const response = await axios.post('http://localhost:3005/convert', {
      html: '<html><body><h1>Hello from Node.js</h1></body></html>',
      pdf_options: {
        landscape: false,
        print_background: true,
        margin_top: 12.5,
        margin_bottom: 12.5
      }
    }, {
      responseType: 'arraybuffer'
    });

    fs.writeFileSync('output.pdf', response.data);
    console.log('PDF created successfully!');
  } catch (error) {
    console.error('Error:', error.message);
  }
}

convertToPdf();
```

### 8. Using with Python

```python
import requests

def convert_to_pdf():
    url = 'http://localhost:3005/convert'
    data = {
        'html': '<html><body><h1>Hello from Python</h1></body></html>',
        'pdf_options': {
            'landscape': False,
            'print_background': True,
            'margin_top': 12.5,
            'margin_bottom': 12.5
        }
    }
    
    response = requests.post(url, json=data)
    
    if response.status_code == 200:
        with open('output.pdf', 'wb') as f:
            f.write(response.content)
        print('PDF created successfully!')
    else:
        print(f'Error: {response.text}')

convert_to_pdf()
```

---

## Common Paper Sizes (millimeters)

| Size | Width | Height | Orientation |
|------|-------|--------|-------------|
| A4 | 210.0 | 297.0 | Portrait |
| A4 | 297.0 | 210.0 | Landscape |
| A3 | 297.0 | 420.0 | Portrait |
| A3 | 420.0 | 297.0 | Landscape |
| A5 | 148.0 | 210.0 | Portrait |
| US Letter | 215.9 | 279.4 | Portrait |
| US Letter | 279.4 | 215.9 | Landscape |
| US Legal | 215.9 | 355.6 | Portrait |

---

## Error Handling

The API returns descriptive error messages for common issues:

- **Invalid Chrome path:** "Invalid Chrome path: ..."
- **Browser launch failure:** "Failed to launch browser: ..."
- **Navigation failure:** "Failed to navigate: ..."
- **PDF generation failure:** "Failed to generate PDF: ..."

All errors return HTTP status code 500 with a plain text error message.

---

## Performance Tips

1. **Pre-optimize HTML:** Minimize CSS and JavaScript before sending
2. **Reduce external resources:** Inline CSS/JS when possible
3. **Optimize images:** Use compressed images or data URIs
4. **Disable unnecessary features:** Set `display_header_footer: false` if not needed
5. **Use appropriate margins:** Smaller margins = faster rendering

---

## Limitations

- Maximum HTML size depends on available system memory
- Complex CSS animations may not render correctly in PDF
- JavaScript is executed during rendering but PDF is static
- Some web fonts may require proper system configuration
