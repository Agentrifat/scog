use std::path::PathBuf;

use clap::{Parser, Subcommand as ClapSubcommand, ValueEnum};
// TODO: for the sake of dogfooding, maybe we should define our CLI through YAML and build it with our own builder :P

#[derive(Parser)]
#[command(name = "scog")]
#[command(about = "scog generates shell completions for your CLI.", long_about = None)]
#[command(version = "0.1.0")]
pub struct Cli {
    /// The path to the spec file
    pub spec: PathBuf,

    #[command(subcommand)]
    pub command: Option<Subcommand>,
}

#[derive(ClapSubcommand)]
pub enum Subcommand {
    // /// Interactively
    // Test {
    //     /// lists test values
    //     #[arg(short, long)]
    //     list: bool,
    // },
    /// Generate shell completions
    Generate {
        /// The shell(s) to generate completions for. If none is specified, we'll assume you want completions for all shells.
        #[arg(short, long, value_name = "SHELL")]
        shell: Vec<Shell>,
    },
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Shell {
    /// Bash
    Bash,
    /// Zsh
    Zsh,
    /// Fish
    Fish,
}
