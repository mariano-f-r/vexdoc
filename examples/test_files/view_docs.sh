#!/bin/bash

echo "🚀 VexDoc Documentation Viewer"
echo "=============================="

# Generate documentation first
echo "📚 Generating documentation..."
../../target/release/vexdoc generate --quiet

echo "✅ Generated files:"
ls -la docs/*.html | awk '{print "  📄 " $9 " (" $5 " bytes)"}'

echo
echo "🌐 Opening documentation in browser..."

# Open each HTML file in the browser
for file in docs/*.html; do
    if [ -f "$file" ]; then
        echo "  Opening $(basename "$file")"
        open "$file"
    fi
done

echo
echo "🎉 Documentation opened! Check your browser tabs."
echo
echo "📁 Files are located at:"
echo "  $(pwd)/docs/"
