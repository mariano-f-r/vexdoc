#!/bin/bash
echo "🚀 Testing VexDoc"
echo "================="
echo "📚 Generating documentation..."
../../target/release/vexdoc generate --verbose
echo "✅ Done! Check the docs/ directory for generated HTML files."
