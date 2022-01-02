//! Tooling for generating completions

use std::{io, str::FromStr};

use clap::App;
use clap_complete;
use miette::{Diagnostic, SourceSpan};
use quickcheck::{Arbitrary, Gen};
use thiserror::Error;

/// The shell the user has selected
#[allow(clippy::enum_variant_names)]
#[derive(Eq, PartialEq, Debug, Clone, clap::ArgEnum, Copy)]
pub enum Shell {
    /// Generate for bash
    Bash,
    /// Generate for elvish
    Elvish,
    /// Generate for Fish
    Fish,
    /// Generate for powershell
    PowerShell,
    /// Generate for zsh
    Zsh,
}

impl Arbitrary for Shell {
    fn arbitrary(g: &mut Gen) -> Self {
        *g.choose(&[
            Self::Bash,
            Self::Elvish,
            Self::Fish,
            Self::PowerShell,
            Self::Zsh,
        ])
        .unwrap()
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        let options = [
            Self::Bash,
            Self::Elvish,
            Self::Fish,
            Self::PowerShell,
            Self::Zsh,
        ];
        let index = options.iter().position(|other| self.eq(other));

        match index {
            None | Some(0) => quickcheck::empty_shrinker(),
            Some(index) => options
                .get(index - 1)
                .map_or(quickcheck::empty_shrinker(), |item| {
                    quickcheck::single_shrinker(*item)
                }),
        }
    }
}

impl FromStr for Shell {
    type Err = ShellFromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "bash" => Ok(Self::Bash),
            "fish" => Ok(Self::Fish),
            "elvish" => Ok(Self::Elvish),
            "powershell" => Ok(Self::PowerShell),
            "zsh" => Ok(Self::Zsh),
            _ => Err(ShellFromStrError {
                source_code: s.to_string(),
                underline: (0, s.len()).into(),
            }),
        }
    }
}

impl From<Shell> for String {
    fn from(shell: Shell) -> Self {
        match shell {
            Shell::Bash => "bash",
            Shell::Elvish => "elvish",
            Shell::Fish => "fish",
            Shell::PowerShell => "powershell",
            Shell::Zsh => "zsh",
        }
        .to_string()
    }
}

/// Error when we could not parse a shell from the given string
#[derive(Debug, Eq, PartialEq, Error, Diagnostic)]
#[error("could not parse a shell from the given string")]
#[diagnostic(
    url(docsrs),
    help("valid shells are: bash, elvish, fish, powershell, and zsh")
)]
pub struct ShellFromStrError {
    #[source_code]
    source_code: String,
    #[label("unknown shell")]
    underline: SourceSpan,
}

/// Print completion for the given shell
pub fn print_completions(writer: &mut dyn io::Write, app: &mut App<'_>, shell: Shell) {
    match shell {
        Shell::Bash => clap_complete::generate(
            clap_complete::shells::Bash,
            app,
            app.get_name().to_string(),
            writer,
        ),
        Shell::Elvish => {
            clap_complete::generate(
                clap_complete::shells::Elvish,
                app,
                app.get_name().to_string(),
                writer,
            );
        }
        Shell::Fish => clap_complete::generate(
            clap_complete::shells::Fish,
            app,
            app.get_name().to_string(),
            writer,
        ),
        Shell::PowerShell => {
            clap_complete::generate(
                clap_complete::shells::PowerShell,
                app,
                app.get_name().to_string(),
                writer,
            );
        }
        Shell::Zsh => clap_complete::generate(
            clap_complete::shells::Zsh,
            app,
            app.get_name().to_string(),
            writer,
        ),
    }
}
