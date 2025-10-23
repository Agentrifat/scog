use std::fs;

use anyhow::Context;
use clap::Parser;
use clap_complete::Shell;

use crate::builder::{build_command_from_spec, print_completions};

mod builder;
mod cli;

fn shell_generator(shell: cli::Shell) -> Shell {
    match shell {
        cli::Shell::Bash => Shell::Bash,
        cli::Shell::Zsh => Shell::Zsh,
        cli::Shell::Fish => Shell::Fish,
    }
}

fn main() -> anyhow::Result<()> {
    let cli = cli::Cli::parse();
    let subcommand = cli
        .command
        .unwrap_or(cli::Subcommand::Generate { shell: vec![] });
    let shells = match subcommand {
        cli::Subcommand::Generate { shell } => shell,
    };

    let spec = fs::read_to_string(cli.spec).context("Failed to read spec file")?;

    let mut command = build_command_from_spec(&spec)?;

    let shells = if shells.is_empty() {
        (&[cli::Shell::Bash, cli::Shell::Zsh, cli::Shell::Fish]).into_iter()
    } else {
        (&shells).into_iter()
    };

    for shell in shells {
        print_completions(shell_generator(*shell), &mut command);
    }

    Ok(())
}
