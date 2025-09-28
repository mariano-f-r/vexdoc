# VexDoc Examples

This directory contains example files to test VexDoc functionality.

## Quick Test

```bash
cd examples/test_files
./test.sh
```

## What's Included

- `VexDoc.toml` - Configuration file
- `sample.rs` - Rust example with documentation
- `sample.py` - Python example with documentation  
- `test.sh` - Simple test script

## Generated Output

After running the test, you'll find HTML documentation in the `docs/` directory:
- `sample-rs.html` - Generated from the Rust file
- `sample-py.html` - Generated from the Python file

## Manual Testing

You can also test manually:

```bash
# Generate documentation
../../target/release/vexdoc generate

# Generate with verbose output
../../target/release/vexdoc generate --verbose

# Generate quietly
../../target/release/vexdoc generate --quiet
```

## View Results

Open the HTML files in your browser to see the generated documentation with syntax highlighting and clean formatting.
