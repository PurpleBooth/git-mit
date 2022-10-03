use miette::{Diagnostic, SourceOffset, SourceSpan};
use serde_yaml::Error as YamlDeserializeError;
use thiserror::Error;
use toml::de::Error as TomlDeserializeError;

#[derive(Error, Debug, Diagnostic)]
#[error("could not convert author configuration to toml")]
#[diagnostic(
    url("https://github.com/PurpleBooth/git-mit/issues/new"),
    code(mit_commit_message_lints::mit::lib::authors::deserialise_authors_error),
    help("please report this error on our issue tracker, this is a bug")
)]
pub struct SerializeAuthorsError(#[from] pub toml::ser::Error);

#[derive(Error, Debug, Diagnostic)]
#[error("could not parse author configuration")]
#[diagnostic(
code(mit_commit_message_lints::mit::lib::authors::serialise_authors_error),
help("`git mit-config mit example` can show you an example of what it should look like, or you can generate one using `git mit-config mit generate` after setting up some authors with `git mit-config mit set`"),
)]
pub struct DeserializeAuthorsError {
    #[source_code]
    pub(crate) src: String,
    #[label("invalid in toml: {toml_message}")]
    pub(crate) toml_span: SourceSpan,
    #[label("invalid in yaml: {yaml_message}")]
    pub(crate) yaml_span: SourceSpan,

    pub(crate) yaml_message: String,
    pub(crate) toml_message: String,
}

impl DeserializeAuthorsError {
    pub(crate) fn new(
        input: &str,
        yaml_error: &YamlDeserializeError,
        toml_error: &TomlDeserializeError,
    ) -> Self {
        Self {
            src: input.to_string(),
            toml_span: (Self::span_from_toml_err(toml_error, input), 0).into(),
            yaml_span: (Self::span_from_yaml_err(yaml_error, input), 0).into(),
            yaml_message: String::new(),
            toml_message: String::new(),
        }
    }

    pub fn span_from_toml_err(err: &TomlDeserializeError, input: &str) -> usize {
        err.line_col()
            .map_or(SourceOffset::from(0), |(line, col)| {
                SourceOffset::from_location(input, line, col)
            })
            .offset()
    }

    pub fn span_from_yaml_err(err: &YamlDeserializeError, input: &str) -> usize {
        err.location()
            .map_or(SourceOffset::from(0), |location| {
                SourceOffset::from_location(input, location.line(), location.column())
            })
            .offset()
    }
}
