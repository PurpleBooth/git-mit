use crate::lints::CommitMessage;
use mit_commit::CommitMessage as NgCommitMessage;

impl From<CommitMessage> for NgCommitMessage {
    fn from(old: CommitMessage) -> Self {
        NgCommitMessage::from(format!("{}", old))
    }
}

impl From<NgCommitMessage> for CommitMessage {
    fn from(new: NgCommitMessage) -> Self {
        CommitMessage::new(new.into())
    }
}

#[cfg(test)]
mod tests {
    use crate::lints::CommitMessage;
    use indoc::indoc;
    use mit_commit::CommitMessage as NgCommitMessage;
    use pretty_assertions::assert_eq;

    #[test]
    fn can_convert_from_a_old_commit_to_a_new_one() {
        let old = CommitMessage::new(
            indoc!(
                "
            Example commit message

            This is an example commit message
            "
            )
            .into(),
        );
        let actual: NgCommitMessage = old.into();

        assert_eq!(
            actual,
            NgCommitMessage::from(indoc!(
                "
            Example commit message

            This is an example commit message
            "
            ))
        );
    }

    #[test]
    fn can_convert_from_a_new_commit_to_an_old_one() {
        let new = NgCommitMessage::from(indoc!(
            "
            Example commit message

            This is an example commit message
            "
        ));
        let actual: CommitMessage = new.into();

        assert_eq!(
            actual,
            CommitMessage::new(
                indoc!(
                    "
                Example commit message

                This is an example commit message
                "
                )
                .into()
            )
        );
    }
}
