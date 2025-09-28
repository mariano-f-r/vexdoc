//! VexDoc - The main entry point

use std::error::Error;
use std::io;
use std::process;

use vexdoc::{cli::VexDocArgs, errors::SubcommandError, run};

/// Main function - parses args and runs the appropriate subcommand
fn main() {
    let args: VexDocArgs = argh::from_env();
    let mut exit_code = 0;
    
    if let Err(err) = run(args) {
        exit_code = 1;
        dbg!(err.source());
        dbg!(&err);
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
