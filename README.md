# VexDoc

(for real this time, no Python)

VexDoc is my second attempt at program that generates HTML documentation for code based on annotations written inside the code.
This version is written in Rust for portability and performance, and also works more like a Unix-style command.
Among other things, I took inspiration from commands like `git`, `cat`, `python3`, and `cargo`.

## First Run

To begin using VexDoc, first create a config file by running `vexdoc init`.
This will create a config file in the current working directory called `VexDoc.toml`.

Fill out this file using the configuration details found below.

Then once you are done, you can document the file types marked in the config using `vexdoc generate`.

## Installation

Executables should be available in the releases tab.
Download an appropriate executable, rename it to `vexdoc`, or `vexdoc.exe` depending on your platform, then simply move it onto $PATH

## Configuration

VexDoc has a similar configuration to VexDocPy, with some minor differences:

| Key | Value |
|---------|--------|
| `inline_comments` | This value is what VexDoc looks for when starting and reading the title of a documentation block, and ending a documentation block |
| `multi_comments` | This value(s) are used by VexDoc to determine where the description for a documentation block starts and ends, as well as containing said description |
| `ignored_dirs`   | These are directories that VexDoc ignores. They can be anywhere, including in the middle of the file tree. |
| `file_extensions` | These are the extensions of the files VexDoc will target, written without the leading dot: ie, "py", "rs", "h", etc |

`ignored_dirs` and `file_extensions` are both case-sensitive.

Here is a sample config:
```toml
inline_comments = "//"
multi_comments = ["/*", "*/"]
ignored_dirs = ["tests", "headers"]
file_extensions = ["c","h"]
```

## Writing the Documentation

For this example, let's consider an imaginary file: `fizz.py`.
`fizz.py` contains the following:
```python
def foo():
  print("Foo!")

def bar():
  print("Bar!")
```

VexDoc extracts the documentation from a file in so-called "documentation blocks".
These blocks contain a title, a summary of the code (the actual documentation), and the code being documented.
To create a documentation block, write a single line comment at the start followed by an exclamation mark.
Then write the title of the documentation block.
At the end of the area that you want to document this, write the single line comment followed by "ENDVEXDOC".

This is what `fizz.py` looks like after doing this for the `foo` function:
```python
#! The Foo Function
def foo():
  print("Foo!")
# ENDVEXDOC

def bar():
  print("Bar!")
```

Finally, create a multiline comment starting with "startsummary" (no space this time) and ending with "endsummary". 
Within these 2 lines, you can write the actual documentation.
`fizz.py` will now look like this:
```python
#! The Foo Function
"""startsummary
The foo function prints Foo!
endsummary"""
def foo():
  print("Foo!")
# ENDVEXDOC

def bar():
  print("Bar!")
```

You can repeat this as many times as you want per file for as many files as needed.
Once you are done, simply rerun the script, and documentation will be generated in the `docs/` folder.
