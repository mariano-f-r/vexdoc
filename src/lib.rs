//! # VexDoc
/*startsummary
A fast documentation generator that extracts inline comments and generates HTML docs.
endsummary*/
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
