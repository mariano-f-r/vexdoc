use std::path::PathBuf;

use argh::FromArgs;

// Command line argument parsing using the argh crate. This handles all the
// --help text generation and argument validation automatically. I chose argh
// because it's simple, fast, and generates nice help text without me having
// to write a bunch of boilerplate code.

#[derive(FromArgs, Debug)]
/// VexDoc - A fast, simple documentation generator for any programming language
/// 
/// VexDoc extracts inline documentation from your source code and generates
/// clean HTML documentation. It works with any language by using configurable
/// comment delimiters and file type filters.
/// 
/// Get started by running 'vexdoc init' to create a configuration file,
/// then 'vexdoc generate' to build your documentation.
/// 
/// I built this because I was tired of complex documentation generators that
/// take forever to set up. Sometimes you just want to document your code
/// without jumping through hoops, you know?
pub struct VexDocArgs {
    #[argh(subcommand)]
    pub subcommands: VexDocSubcommands,
}

#[derive(FromArgs, Debug)]
#[argh(subcommand)]
/// The different commands you can run with VexDoc
/// 
/// Right now there are just two commands, but I might add more
/// in the future if people ask for them. The init command sets up
/// a new project, and generate does the actual documentation work.
pub enum VexDocSubcommands {
    Init(InitArgs),
    Generate(GenArgs),
}

#[derive(FromArgs, Debug)]
/// Initialize a new VexDoc project
/// 
/// Creates a VexDoc.toml configuration file in the specified directory.
/// This file contains all the settings VexDoc needs to process your project,
/// including comment delimiters, file types, and directories to ignore.
/// 
/// After running this command, you'll need to edit the generated VexDoc.toml
/// file to match your project's programming language and structure.
/// 
/// The generated config file is pretty basic - just empty placeholders that
/// you'll need to fill in. I figured it's better to let you customize it
/// rather than guessing what you want.
#[argh(subcommand, name = "init")]
pub struct InitArgs {
    #[argh(option, default = "\".\".into()")]
    /// directory where the config file should be created (defaults to current directory)
    pub dir: PathBuf,
    // TODO: Add overwrite flag to force a new config - this would be nice to have
}

#[derive(FromArgs, Debug)]
/// Generate HTML documentation from your source files
/// 
/// This command processes your source files and generates HTML documentation
/// in the `docs/` directory. By default, it processes all files matching the
/// file extensions specified in your VexDoc.toml configuration.
/// 
/// You can also specify particular files to process, or use the --verbose
/// flag to see detailed progress information as files are processed.
/// 
/// The --quiet flag is handy when you're running this in a script or CI
/// pipeline and don't want all the progress bars cluttering up your output.
#[argh(subcommand, name = "generate")]
pub struct GenArgs {
    #[argh(option)]
    /// specific files to process (if not provided, processes all matching files)
    pub files: Vec<PathBuf>,
    #[argh(switch, short = 'v')]
    /// show detailed progress information for each file
    pub verbose: bool,
    #[argh(switch, short = 'q')]
    /// suppress progress bars and notices (useful for scripts)
    pub quiet: bool,
}
