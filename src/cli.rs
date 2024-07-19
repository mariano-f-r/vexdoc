use std::path::PathBuf;

use argh::FromArgs;

// argh does a lot of the heavy lifting through this FromArgs macro based code

#[derive(FromArgs, Debug)]
/// A utility that allows for the generation of printable codebase documenation
pub struct VexDocArgs {
    #[argh(subcommand)]
    pub subcommands: VexDocSubcommands,
}

#[derive(FromArgs, Debug)]
#[argh(subcommand)]
pub enum VexDocSubcommands {
    Init(InitArgs),
    Generate(GenArgs),
}

#[derive(FromArgs, Debug)]
/// Generates a config file for VexDoc
#[argh(subcommand, name = "init")]
pub struct InitArgs {
    #[argh(option, default = "\".\".into()")]
    /// which directory to generate the config file in, defaulting to the current working directory
    pub dir: PathBuf,
    // TODO: Add overwrite flag to force a new config
}

#[derive(FromArgs, Debug)]
/// Generates documentation from codebase, defaulting to all files matching the extensions in the
/// configuration file
#[argh(subcommand, name = "generate")]
pub struct GenArgs {
    #[argh(option)]
    /// optionally list the specific files you want to generate documentation for
    pub files: Vec<PathBuf>,
}
