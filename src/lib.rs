//! # VexDoc
//! 
//! A documentation generator that extracts inline comments from your codebase
//! and turns them into clean HTML documentation. Think of it as a lightweight
//! alternative to tools like Sphinx or JSDoc, but with a focus on simplicity
//! and speed.
//! 
//! ## Why VexDoc?
//! 
//! Most documentation generators are either too complex for simple projects or
//! too slow for large codebases. VexDoc strikes a balance - it's fast enough
//! to run on every commit, simple enough to set up in minutes, and flexible
//! enough to work with any programming language.
//! 
//! ## Getting Started
//! 
//! The workflow is straightforward:
//! 
//! ```bash
//! # Set up a new project
//! vexdoc init
//! 
//! # Edit VexDoc.toml to match your language
//! # Then generate docs
//! vexdoc generate
//! ```
//! 
//! ## Configuration
//! 
//! VexDoc uses a simple TOML file for configuration. Here's what you need:
//! 
//! ```toml
//! # Single-line comment marker (like // or #)
//! inline_comments = "//"
//! 
//! # Multi-line comment delimiters
//! multi_comments = ["/*", "*/"]
//! 
//! # Directories to skip
//! ignored_dirs = ["target", "node_modules", ".git"]
//! 
//! # File types to process
//! file_extensions = ["rs", "py", "c", "h"]
//! ```
//! 
//! ## Writing Documentation
//! 
//! The format is intentionally simple. Just wrap your code with special comments:
//! 
//! ```rust
//! //! My Awesome Function
//! /*startsummary
//! This function does something really cool. It takes some input,
//! processes it through several steps, and returns a useful result.
//! 
//! The function handles edge cases gracefully and provides clear
//! error messages when things go wrong.
//! endsummary*/
//! fn my_awesome_function(input: &str) -> Result<String, Error> {
//!     // Implementation here
//!     Ok(input.to_uppercase())
//! }
//! // ENDVEXDOC
//! ```
//! 
//! That's it! VexDoc will find this block, extract the title and description,
//! and generate a nice HTML page with syntax highlighting.

pub mod cli;
pub mod docgen;
pub mod errors;

use crate::cli::{VexDocArgs, VexDocSubcommands};
use crate::docgen::{document, DocGenConfig};
use crate::errors::SubcommandError;

/// Runs the main VexDoc application logic
/// 
/// This function handles the core workflow - whether that's initializing
/// a new project or generating documentation from existing files. It's
/// the central dispatch function that routes commands to the appropriate
/// handlers.
/// 
/// The function is designed to be called from the main binary, but it's
/// also exposed as part of the library API for testing and integration
/// purposes.
/// 
/// # Arguments
/// 
/// * `args` - Parsed command line arguments containing the subcommand
///            and any relevant options
/// 
/// # Returns
/// 
/// * `Ok(())` - Everything worked as expected
/// * `Err(SubcommandError)` - Something went wrong (file not found,
///                            invalid config, etc.)
/// 
/// # Examples
/// 
/// ```no_run
/// use vexdoc::{run, cli::VexDocArgs};
/// use argh::FromArgs;
/// 
/// // This is essentially what happens in main()
/// let args = VexDocArgs::from_args(&["vexdoc", "generate"]);
/// if let Err(e) = run(args) {
///     eprintln!("Error: {}", e);
///     std::process::exit(1);
/// }
/// ```
pub fn run(args: VexDocArgs) -> Result<(), SubcommandError> {
    match args.subcommands {
        VexDocSubcommands::Init(initargs) => {
            // TODO: figure out how to avoid clone.
            if let Err(e) = DocGenConfig::create_config(initargs.dir.clone()) {
                return Err(SubcommandError::InitError(e));
            }
            // output should come after the action, so that error propagation happens before we
            // tell the user anything
            println!(
                "Created new, empty configuration file in {}",
                initargs.dir.display()
            );
        }
        VexDocSubcommands::Generate(genargs) => {
            let conf = DocGenConfig::read_config()?;
            if !genargs.quiet {
                println!("Beginning documentation");
            }
            if genargs.files.len() == 0 {
                let files = conf.get_files()?;
                document(conf, files, genargs.verbose, genargs.quiet)?;
            } else {
                document(conf, genargs.files, genargs.verbose, genargs.quiet)?;
            }
        }
    }
    Ok(())
}
