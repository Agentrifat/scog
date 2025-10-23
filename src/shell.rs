use std::fmt;

use clap::ValueEnum;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum Shell {
    /// Bash
    Bash,
    /// Zsh
    Zsh,
    /// Fish
    Fish,
}

impl Shell {
    pub fn all() -> &'static [Shell] {
        &[Shell::Bash, Shell::Zsh, Shell::Fish]
    }
    pub fn generator(&self) -> clap_complete::Shell {
        match self {
            Shell::Bash => clap_complete::Shell::Bash,
            Shell::Zsh => clap_complete::Shell::Zsh,
            Shell::Fish => clap_complete::Shell::Fish,
        }
    }

    pub fn completions_filename(&self, bin_name: &str) -> String {
        match self {
            Shell::Bash => format!("{bin_name}.bash"),
            Shell::Zsh => format!("_{bin_name}"),
            Shell::Fish => format!("{bin_name}.fish"),
        }
    }

    pub fn installation_guide<W: fmt::Write>(&self, bin_name: &str, w: &mut W) -> fmt::Result {
        match self {
            Shell::Bash => {
                write!(w, "### Bash\n\n")?;

                write!(
                    w,
                    "```shell\nsource completions/{bin_name}.bash\n# Or copy to: ~/.local/share/bash-completion/completions/\n```\n"
                )?;
            }
            Shell::Zsh => {
                write!(w, "### Zsh\n\n")?;

                write!(
                    w,
                    "```shell\n# 1. Create a completions directory if it doesn't exist\nmkdir -p ~/.zsh/completions\n\n# 2. Copy the completion file there\ncp completions/_{bin_name} ~/.zsh/completions/_{bin_name}\n\n# 3. Add to ~/.zshrc (if not already present)\nfpath=(~/.zsh/completions $fpath)\n\n# 4. Initialize completions\nautoload -Uz compinit\ncompinit\n```\n"
                )?;
            }
            Shell::Fish => {
                write!(w, "### Fish\n\n")?;

                write!(
                    w,
                    "```shell\ncp completions/myapp.fish ~/.config/fish/completions/\n```\n"
                )?;
            }
        }

        Ok(())
    }
}
