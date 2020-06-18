use mit_commit_message_lints::errors::MitCommitMessageLintsError;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub(crate) enum MitPrepareCommitMessageError {
    PbCommitMessageLintsError(MitCommitMessageLintsError),
    Io(String, String),
    MissingCommitFilePath,
}

impl Display for MitPrepareCommitMessageError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MitPrepareCommitMessageError::PbCommitMessageLintsError(error) => {
                write!(f, "{}", error)
            }
            MitPrepareCommitMessageError::MissingCommitFilePath => {
                write!(f, "Expected commit file path")
            }
            MitPrepareCommitMessageError::Io(file_source, error) => write!(
                f,
                "Failed to read author config from `{}`:\n{}",
                file_source, error
            ),
        }
    }
}

impl From<MitCommitMessageLintsError> for MitPrepareCommitMessageError {
    fn from(from: MitCommitMessageLintsError) -> Self {
        MitPrepareCommitMessageError::PbCommitMessageLintsError(from)
    }
}

impl Error for MitPrepareCommitMessageError {}

impl MitPrepareCommitMessageError {
    pub(crate) fn new_io(source: String, error: &std::io::Error) -> MitPrepareCommitMessageError {
        MitPrepareCommitMessageError::Io(source, format!("{}", error))
    }
}
