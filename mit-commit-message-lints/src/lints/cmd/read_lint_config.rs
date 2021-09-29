use std::{
    collections::{BTreeMap, BTreeSet},
    convert::TryInto,
};

use miette::{Diagnostic, Result, SourceOffset, SourceSpan};
use mit_lint::{Lint, Lints, CONFIG_KEY_PREFIX};
use thiserror::Error as ThisError;

use crate::external::Vcs;

/// # Errors
///
/// If we fail to parse the toml
///
/// # Panics
///
/// Will panic if the lint prefix isn't delimited by dots. This should never
/// happen as it's a constant
pub fn read_from_toml_or_else_vcs(config: &str, vcs: &mut dyn Vcs) -> Result<Lints> {
    let vcs_lints = try_from_vcs(vcs)?;
    // contains PB  // contains lint // contains config
    let config: BTreeMap<String, BTreeMap<String, BTreeMap<String, bool>>> = toml::from_str(config)
        .map_err(|x| SerialiseLintError {
            src: config.to_string(),
            message: x.to_string(),
            span: x.line_col().map_or(
                SourceSpan::new(0.into(), SourceOffset::from(0)),
                |(line, col)| {
                    SourceSpan::new(
                        SourceOffset::from_location(config, line, col),
                        SourceOffset::from(0),
                    )
                },
            ),
        })?;

    let lint_prefix = CONFIG_KEY_PREFIX.split('.').collect::<Vec<_>>();
    let namespace = (*lint_prefix.get(0).unwrap()).to_string();

    let config = match config.get(&namespace) {
        None => return Ok(vcs_lints),
        Some(lints) => lints,
    };

    let group = (*lint_prefix.get(1).unwrap()).to_string();

    let lint_names = match config.get(&group) {
        None => return Ok(vcs_lints),
        Some(lints) => lints,
    };

    let to_add: Lints = lint_names
        .iter()
        .filter_map(|(key, value)| if *value { Some(key as &str) } else { None })
        .collect::<Vec<&str>>()
        .try_into()?;

    let to_remove: Lints = lint_names
        .iter()
        .filter_map(|(key, value)| if *value { None } else { Some(key as &str) })
        .collect::<Vec<&str>>()
        .try_into()?;

    Ok(vcs_lints.subtract(&to_remove).merge(&to_add))
}

#[derive(ThisError, Debug, Diagnostic)]
#[error("could not parse lint configuration")]
#[diagnostic(
    url(docsrs),
    code(mit_commit_message_lints::lints::cmd::read_lint_config::serialise_lint_error),
    help("you can generate an example using `git mit-config lint generate`")
)]
struct SerialiseLintError {
    #[source_code]
    src: String,
    #[label("invalid in toml: {message}")]
    span: SourceSpan,
    message: String,
}

/// Create lints from the VCS configuration
///
/// # Errors
/// If reading from the VCS fails
fn try_from_vcs(config: &mut dyn Vcs) -> Result<Lints> {
    Ok(Lints::new(
        Lint::all_lints()
            .filter_map(|lint| {
                get_config_or_default(config, lint, lint.enabled_by_default()).transpose()
            })
            .collect::<Result<BTreeSet<Lint>>>()?,
    ))
}

fn get_config_or_default(config: &dyn Vcs, lint: Lint, default: bool) -> Result<Option<Lint>> {
    Ok(config
        .get_bool(&lint.config_key())?
        .or(Some(default))
        .filter(|lint_value| lint_value == &true)
        .map(|_| lint))
}
