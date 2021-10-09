use std::{io, str::FromStr};

use clap::App;
use clap_generate::{generate, generators};
use miette::{Diagnostic, SourceSpan};
use quickcheck::{Arbitrary, Gen};
use thiserror::Error;

#[allow(clippy::enum_variant_names)]
#[derive(Eq, PartialEq, Debug, Clone, clap::ArgEnum, Copy)]
pub enum Shell {
    Bash,
    Elvish,
    Fish,
    PowerShell,
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

pub fn print_completions(writer: &mut dyn io::Write, app: &mut App, shell: Shell) {
    match shell {
        Shell::Bash => generate::<generators::Bash, _>(app, app.get_name().to_string(), writer),
        Shell::Elvish => generate::<generators::Elvish, _>(app, app.get_name().to_string(), writer),
        Shell::Fish => generate::<generators::Fish, _>(app, app.get_name().to_string(), writer),
        Shell::PowerShell => {
            generate::<generators::PowerShell, _>(app, app.get_name().to_string(), writer);
        }
        Shell::Zsh => generate::<generators::Zsh, _>(app, app.get_name().to_string(), writer),
    }
}
