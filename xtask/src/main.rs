/*
 * Created on Mon Jul 17 2023
 *
 * Copyright (c) storycraft. Licensed under the Apache Licence 2.0.
 */

pub mod dist;

use std::error::Error;

use clap::{Parser, Subcommand};
use dist::dist;

#[derive(Parser)]
#[command(name = "Storyboard xtask")]
#[command(author, version, about, long_about = None)]
struct System {
    #[command(subcommand)]
    command: SubCommand,
}

#[derive(Subcommand)]
enum SubCommand {
    /// Build package artifact with resources
    Dist {
        #[arg(allow_hyphen_values = true)]
        /// Cargo arguments
        cargo_args: Vec<String>,
    },
}

fn main() -> Result<(), Box<dyn Error>> {
    let system = System::parse();

    match system.command {
        SubCommand::Dist { cargo_args } => dist(cargo_args)?,
    }

    Ok(())
}
