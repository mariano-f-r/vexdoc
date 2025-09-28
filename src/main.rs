//! VexDoc - The main entry point
//! 
//! This is where everything starts. We parse command line arguments,
//! run the appropriate subcommand, and handle any errors that come up.
//! Pretty straightforward stuff, really.

use std::error::Error;
use std::io;
use std::process;

use vexdoc::{cli::VexDocArgs, errors::SubcommandError, run};

/// The main function - where the magic happens
/// 
/// This is pretty much just a wrapper around the actual logic in lib.rs.
/// We parse the command line args, run the command, and if something
/// goes wrong, we print a nice error message and exit with code 1.
/// 
/// The error handling here is a bit verbose, but I wanted to make sure
/// users get helpful messages when things go wrong. Nothing worse than
/// a cryptic error message when you're trying to generate docs!
fn main() {
    let args: VexDocArgs = argh::from_env();
    let mut exit_code = 0;
    
    // Run the actual command and see what happens
    if let Err(err) = run(args) {
        exit_code = 1;
        
        // Debug info for developers (you can ignore this)
        dbg!(err.source());
        dbg!(&err);
        // Handle different types of errors with user-friendly messages
        match &err {
            SubcommandError::InitError(ref e) => match e.kind() {
                io::ErrorKind::AlreadyExists => {
                    eprintln!("vexdoc: could not create new config file: {}", e);
                    eprintln!("Delete the existing config file to generate a new one");
                }
                io::ErrorKind::NotFound => {
                    eprintln!(
                        "vexdoc: could not create new config file at requested location: {}",
                        e
                    );
                }
                _ => eprintln!("vexdoc: could not create new config file: {}", e),
            },
            SubcommandError::FileReadError(ref e) => {
                eprintln!("vexdoc: could not read config file: {}", e,);
            }
            SubcommandError::GenerationError(ref e) => {
                eprintln!("vexdoc: could not generate documentation for files: {}", e);
            }
            SubcommandError::GenerationWriteError(ref e) => {
                eprintln!("vexdoc: {}: {}", &err, e);
            }
            SubcommandError::UserError {
                causes,
                source: _,
                kind: _,
                file: _,
            } => {
                eprintln!("vexdoc: an error has occurred at runtime: {}", &err);
                for i in causes.lines() {
                    eprintln!("caused by: {}", i);
                }
            }
        }
    }
    process::exit(exit_code);
}
