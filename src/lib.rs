pub mod cli;
pub mod docgen;
pub mod errors;

use crate::cli::{VexDocArgs, VexDocSubcommands};
use crate::docgen::{document, DocGenConfig};
use crate::errors::SubcommandError;

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
            println!("Beginning documentation");
            if genargs.files.len() == 0 {
                let files = conf.get_files()?;
                document(conf, files)?;
            } else {
                document(conf, genargs.files)?;
            }
        }
    }
    Ok(())
}
