use std::error::Error;
use std::fmt;
use std::io;
use std::path::PathBuf;

// please compiler i beg you to turn this into simple assembly

// objective, minimize use of box<dyn error>

#[derive(Debug)]
pub enum SubcommandError {
    InitError(io::Error),
    FileReadError(io::Error),
    GenerationError(Box<dyn Error>),
    // perhaps add a path field to specify which path
    GenerationWriteError(io::Error),
    UserError {
        causes: String,
        source: Option<Box<dyn Error>>,
        kind: UserErrorKind,
        file: PathBuf,
    },
}

#[derive(Debug)]
pub enum UserErrorKind {
    Config,
    Annotations,
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
            // i can explain myself: the required signature for source() is an option to a static
            // reference to an Error trait object, but since it takes &self, I have to first deref
            // the box once to get just the box, and then twice to get the actual error, but since
            // error doesn't implement sized i must put it behind a reference again.
            Self::GenerationError(e) => Some(&**e),
            Self::GenerationWriteError(e) => Some(e),
            Self::UserError {
                causes: _,
                source: cause,
                kind: _,
                file: _,
            } => match cause {
                // this is the only way i got it to compile. would use into_inner but its not
                // stabilized yet
                Some(e) => Some(&**e),
                None => None,
            },
        }
    }
}

impl fmt::Display for SubcommandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InitError(_) => {
                write!(f, "couldn't create config file")
            }
            Self::FileReadError(_) => {
                write!(f, "couldn't read code files")
            }
            // reduce usage of this as much as possible
            Self::GenerationError(_) => {
                write!(f, "problem while running generation subcommand")
            }
            Self::GenerationWriteError(_) => {
                write!(f, "couldn't write to documentation files")
            }
            Self::UserError {
                causes: _,
                source: _,
                kind: source,
                file,
            } => match source {
                UserErrorKind::Config => write!(
                    f,
                    "failed to read config file at {} due to incorrect config",
                    file.display()
                ),
                UserErrorKind::Annotations => write!(
                    f,
                    "failed to write documentation for {} due to incorrect annotations",
                    file.display()
                ),
            },
        }
    }
}
