use std::path::PathBuf;

use clap::Parser;

use crate::shell::Shell;
// TODO: for the sake of dogfooding, maybe we should define our CLI through YAML and build it with our own builder :P

#[derive(Parser)]
#[command(name = "scog")]
#[command(about = "scog generates shell completions for your CLI.", long_about = None)]
#[command(version = "0.1.0")]
pub struct Cli {
    /// In case you want to specify a different output directory for the completions.
    /// Defaults to `./completions/`.
    #[arg(short, long, value_name = "PATH")]
    pub output: Option<PathBuf>,

    /// The shell(s) to generate completions for. If none is specified, we'll assume you want completions for all shells.
    // #[arg(short, long, value_name = "SHELL")]
    pub shell: Vec<Shell>,

    /// The path to the spec file
    pub spec: PathBuf,
}
