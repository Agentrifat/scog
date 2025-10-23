use std::{
    fmt::Write,
    fs,
    io::BufWriter,
    path::{Path, PathBuf},
};

use anyhow::Context;
use clap::{Command, Parser};
use clap_complete::Generator;
use termimad::{Alignment, FmtText, MadSkin};

use crate::{builder::build_command_from_spec, shell::Shell};

mod builder;
mod cli;
mod shell;

fn write_completions(
    shells: &[Shell],
    output: &Path,
    bin_name: &str,
    command: &Command,
) -> Result<Vec<PathBuf>, anyhow::Error> {
    fs::create_dir_all(output)
        .with_context(|| format!("Failed to create directory {}", output.display()))?;
    let mut files_written = Vec::with_capacity(shells.len());

    for shell in shells {
        let path = output.join(shell.completions_filename(bin_name));
        let file = fs::File::create(&path)
            .with_context(|| format!("Failed to create file {}", path.display()))?;
        let mut writer = BufWriter::new(file);

        shell.generator().generate(command, &mut writer);

        files_written.push(path);
    }

    Ok(files_written)
}

fn main() -> anyhow::Result<()> {
    let cli::Cli {
        spec: spec_path,
        output,
        shell,
    } = cli::Cli::parse();

    let output = output.unwrap_or_else(|| PathBuf::from("./completions"));

    let spec = fs::read_to_string(&spec_path)
        .with_context(|| format!("Failed to read spec file {}", spec_path.display()))?;

    let command = build_command_from_spec(&spec)?;
    let bin_name = command.get_name();

    let shells = if shell.is_empty() {
        Shell::all()
    } else {
        &shell
    };

    let files_written = write_completions(shells, &output, bin_name, &command)?;

    display_message(bin_name, shells, &files_written)
}

fn display_message(
    bin_name: &str,
    shells: &[Shell],
    files_written: &[PathBuf],
) -> Result<(), anyhow::Error> {
    let mut text = String::with_capacity(2048);

    writeln!(text, "## Generated completions for *{bin_name}*:")?;
    for file in files_written {
        writeln!(text, "    - {}:", file.display())?;
    }

    writeln!(text, "## Installation:\n")?;

    for shell in shells {
        shell.installation_guide(bin_name, &mut text)?;
    }

    let mut skin = MadSkin::default();
    skin.code_block.align = Alignment::Center;
    let (width, _) = termimad::terminal_size();

    let mut text = FmtText::from(&skin, &text, Some(width as _));
    text.set_rendering_width(text.content_width());
    println!("{}", text);

    Ok(())
}
