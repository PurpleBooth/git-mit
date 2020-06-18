use crate::errors::MitCommitMsgError::PbCommitMessageLints;
use mit_commit_message_lints::errors::MitCommitMessageLintsError;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub(crate) enum MitCommitMsgError {
    CommitPathMissing,
    PbCommitMessageLints(MitCommitMessageLintsError),
    Io(String, String),
}

impl MitCommitMsgError {
    pub(crate) fn new_io(location: String, error: &std::io::Error) -> MitCommitMsgError {
        MitCommitMsgError::Io(location, format!("{}", error))
    }
}

impl Display for MitCommitMsgError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PbCommitMessageLints(error) => write!(f, "{}", error),
            MitCommitMsgError::Io(file_source, error) => write!(
                f,
                "Failed to read git config from `{}`:\n{}",
                file_source, error
            ),
            MitCommitMsgError::CommitPathMissing => write!(f, "Expected file path name",),
        }
    }
}

impl From<MitCommitMessageLintsError> for MitCommitMsgError {
    fn from(err: MitCommitMessageLintsError) -> Self {
        PbCommitMessageLints(err)
    }
}

impl Error for MitCommitMsgError {}
