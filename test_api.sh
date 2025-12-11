#!/bin/bash

# Example script to test the RS PDF API

echo "Testing RS PDF API - All Usage Patterns..."

# Pattern 1: No options at all
echo -e "\n1. Pattern 1 - No options (all defaults):"
curl -X POST http://localhost:3005/convert \
  -H "Content-Type: application/json" \
  -d '{
    "html": "<!DOCTYPE html><html><head><title>Simple Test</title></head><body><h1>Hello World</h1><p>This is a simple test.</p></body></html>"
  }' \
  --output simple_test.pdf

if [ -f simple_test.pdf ]; then
    echo "✓ simple_test.pdf created successfully"
else
    echo "✗ Failed to create simple_test.pdf"
fi

# Pattern 2: Partial options
echo -e "\n2. Pattern 2 - Partial options (landscape + margins only):"
curl -X POST http://localhost:3005/convert \
  -H "Content-Type: application/json" \
  -d @example_partial.json \
  --output partial_test.pdf

if [ -f partial_test.pdf ]; then
    echo "✓ partial_test.pdf created successfully"
else
    echo "✗ Failed to create partial_test.pdf"
fi

# Pattern 3: Full options
echo -e "\n3. Pattern 3 - Full custom options:"
curl -X POST http://localhost:3005/convert \
  -H "Content-Type: application/json" \
  -d @example_request.json \
  --output custom_test.pdf

if [ -f custom_test.pdf ]; then
    echo "✓ custom_test.pdf created successfully"
else
    echo "✗ Failed to create custom_test.pdf"
fi

# Additional example
echo -e "\n4. Additional example - Custom header/footer:"
curl -X POST http://localhost:3005/convert \
  -H "Content-Type: application/json" \
  -d '{
    "html": "<!DOCTYPE html><html><head><style>body { font-family: Arial; padding: 20px; }</style></head><body><h1>Landscape Document</h1><p>This PDF is in landscape orientation with custom margins.</p></body></html>",
    "pdf_options": {
      "landscape": true,
      "margin_top": 1.0,
      "margin_bottom": 1.0,
      "margin_left": 1.5,
      "margin_right": 1.5
    }
  }' \
  --output landscape_test.pdf

if [ -f landscape_test.pdf ]; then
    echo "✓ landscape_test.pdf created successfully"
else
    echo "✗ Failed to create landscape_test.pdf"
fi

echo -e "\nAll tests completed!"
