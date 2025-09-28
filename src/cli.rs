use std::path::PathBuf;

use argh::FromArgs;

// Command line argument parsing using argh

#[derive(FromArgs, Debug)]
/// VexDoc - A fast, simple documentation generator for any programming language
pub struct VexDocArgs {
    #[argh(subcommand)]
    pub subcommands: VexDocSubcommands,
}

#[derive(FromArgs, Debug)]
#[argh(subcommand)]
/// Available VexDoc commands
pub enum VexDocSubcommands {
    Init(InitArgs),
    Generate(GenArgs),
}

#[derive(FromArgs, Debug)]
/// Initialize a new VexDoc project
#[argh(subcommand, name = "init")]
pub struct InitArgs {
    #[argh(option, default = "\".\".into()")]
    /// directory where the config file should be created (defaults to current directory)
    pub dir: PathBuf,
    // TODO: Add overwrite flag
}

#[derive(FromArgs, Debug)]
/// Generate HTML documentation from your source files
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
