use mit_commit_message_lints::errors::MitCommitMessageLintsError;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub(crate) enum MitPreCommitError {
    PbCommitMessageLintsError(MitCommitMessageLintsError),
    Io(String, String),
}

impl Display for MitPreCommitError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MitPreCommitError::PbCommitMessageLintsError(error) => write!(f, "{}", error),
            MitPreCommitError::Io(file_source, error) => write!(
                f,
                "Failed to read from config from `{}`:\n{}",
                file_source, error
            ),
        }
    }
}

impl From<MitCommitMessageLintsError> for MitPreCommitError {
    fn from(from: MitCommitMessageLintsError) -> Self {
        MitPreCommitError::PbCommitMessageLintsError(from)
    }
}

impl Error for MitPreCommitError {}

impl MitPreCommitError {
    pub(crate) fn new_io(source: String, error: &std::io::Error) -> MitPreCommitError {
        MitPreCommitError::Io(source, format!("{}", error))
    }
}
