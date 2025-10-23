use anyhow::Context;
use clap::{Arg, ArgAction, Command, ValueHint};
use serde::Deserialize;

pub fn build_command_from_spec(spec: &str) -> anyhow::Result<Command> {
    let spec: CliSpec = serde_saphyr::from_str(spec).context("Failed to parse YAML")?;
    let mut cmd = build_command_recursive(&spec);
    cmd.build();
    cmd.set_bin_name(&spec.name);
    Ok(cmd)
}

fn build_command_recursive(spec: &CliSpec) -> Command {
    let mut cmd = Command::new(&spec.name);

    if let Some(about) = &spec.about {
        cmd = cmd.about(about);
    }

    for arg_spec in &spec.args {
        let mut arg = Arg::new(&arg_spec.name);

        if let Some(short) = arg_spec.short {
            arg = arg.short(short);
        }
        if let Some(long) = &arg_spec.long {
            arg = arg.long(long);
        }
        if let Some(help) = &arg_spec.help {
            arg = arg.help(help);
        }
        if let Some(true) = arg_spec.required {
            arg = arg.required(true);
        }

        if let Some(false) = arg_spec.takes_value {
            arg = arg.action(ArgAction::SetTrue);
        } else {
            arg = arg.action(ArgAction::Set);

            if let Some(values) = &arg_spec.possible_values {
                arg = arg.value_parser(values.to_owned());
            }

            if let Some(hint) = &arg_spec.value_hint {
                arg = arg.value_hint(parse_value_hint(hint));
            }
        }

        cmd = cmd.arg(arg);
    }

    if let Some(subcommands) = &spec.subcommands {
        for subcmd_spec in subcommands {
            cmd = cmd.subcommand(build_command_recursive(&subcmd_spec));
        }
    }

    cmd
}

#[derive(Deserialize)]
struct CliSpec {
    name: String,
    about: Option<String>,
    #[serde(default)]
    args: Vec<ArgSpec>,
    subcommands: Option<Vec<CliSpec>>,
}

#[derive(Deserialize)]
struct ArgSpec {
    name: String,
    short: Option<char>,
    long: Option<String>,
    help: Option<String>,
    required: Option<bool>,
    takes_value: Option<bool>,
    value_hint: Option<String>,
    possible_values: Option<Vec<String>>,
}

fn parse_value_hint(hint: &str) -> ValueHint {
    match hint {
        "FilePath" => ValueHint::FilePath,
        "DirPath" => ValueHint::DirPath,
        "AnyPath" => ValueHint::AnyPath,
        "ExecutablePath" => ValueHint::ExecutablePath,
        "CommandName" => ValueHint::CommandName,
        "CommandString" => ValueHint::CommandString,
        "CommandWithArguments" => ValueHint::CommandWithArguments,
        "Username" => ValueHint::Username,
        "Hostname" => ValueHint::Hostname,
        "Url" => ValueHint::Url,
        "EmailAddress" => ValueHint::EmailAddress,
        "Other" => ValueHint::Other,
        other => {
            eprintln!("Unknown value hint: {}. Using default (Unknown).", other);
            ValueHint::Unknown
        }
    }
}
