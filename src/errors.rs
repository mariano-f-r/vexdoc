use std::error::Error;
use std::fmt;
use std::io;
use std::path::PathBuf;

// Error handling for VexDoc

#[derive(Debug)]
pub enum SubcommandError {
    /// Failed to create the initial VexDoc.toml config file
    InitError(io::Error),
    /// Couldn't read a source file or the config file
    FileReadError(io::Error),
    /// Something went wrong during the documentation generation process
    GenerationError(Box<dyn Error + Send + Sync>),
    /// Failed to write the generated HTML files to disk
    GenerationWriteError(io::Error),
    /// User error - usually configuration or annotation problems
    UserError {
        causes: String,
        source: Option<Box<dyn Error + Send + Sync>>,
        kind: UserErrorKind,
        file: PathBuf,
    },
}

#[derive(Debug)]
pub enum UserErrorKind {
    /// Configuration file problems (invalid TOML, missing fields, etc.)
    Config,
    /// Documentation annotation problems (missing summary, malformed blocks, etc.)
    Annotations,
}

/// Represents a validation error with its corresponding suggestion
#[derive(Debug)]
pub struct ValidationError {
    pub message: String,
    pub suggestion: String,
}

impl ValidationError {
    pub fn new(message: String, suggestion: String) -> Self {
        Self { message, suggestion }
    }
}

// impl SubcommandError {
//     pub fn solution(&self) -> Option<&String> {
//         if let Self::UserError {
//             causes: solution,
//             source: _,
//             kind: _,
//             file: _,
//         } = self
//         {
//             Some(solution)
//         } else {
//             None
//         }
//     }
// }

impl Error for SubcommandError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InitError(e) => Some(e),
            Self::FileReadError(e) => Some(e),
            // Deref the box twice to get the actual error
            Self::GenerationError(e) => Some(&**e),
            Self::GenerationWriteError(e) => Some(e),
            Self::UserError {
                causes: _,
                source: cause,
                kind: _,
                file: _,
            } => match cause {
                Some(e) => Some(&**e),
                None => None,
            },
        }
    }
}

impl fmt::Display for SubcommandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InitError(e) => {
                write!(f, "Failed to create config file: {}. Try running 'vexdoc init' in a writable directory.", e)
            }
            Self::FileReadError(e) => {
                write!(f, "Failed to read files: {}. Check file permissions and paths.", e)
            }
            Self::GenerationError(e) => {
                write!(f, "Documentation generation failed: {}. Check your configuration and file contents.", e)
            }
            Self::GenerationWriteError(e) => {
                write!(f, "Failed to write documentation files: {}. Check write permissions in the docs/ directory.", e)
            }
            Self::UserError {
                causes,
                source: _,
                kind,
                file,
            } => match kind {
                UserErrorKind::Config => write!(
                    f,
                    "Configuration error in {}: {}\n\nSuggested fixes:\n{}",
                    file.display(),
                    self.get_solution_hint(),
                    causes
                ),
                UserErrorKind::Annotations => write!(
                    f,
                    "Annotation error in {}: {}\n\nSuggested fixes:\n{}",
                    file.display(),
                    self.get_solution_hint(),
                    causes
                ),
            },
        }
    }
}

impl SubcommandError {
    fn get_solution_hint(&self) -> &'static str {
        match self {
            Self::InitError(_) => "Make sure you have write permissions in the current directory",
            Self::FileReadError(_) => "Verify file paths and permissions",
            Self::GenerationError(_) => "Check your VexDoc.toml configuration",
            Self::GenerationWriteError(_) => "Ensure the docs/ directory is writable",
            Self::UserError { kind, .. } => match kind {
                UserErrorKind::Config => "Fix the configuration file format",
                UserErrorKind::Annotations => "Check your documentation block syntax",
            },
        }
    }
}
