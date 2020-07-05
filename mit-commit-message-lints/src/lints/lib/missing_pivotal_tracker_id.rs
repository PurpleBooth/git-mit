use indoc::indoc;
use mit_commit::CommitMessage;
use regex::Regex;

use crate::lints::lib::problem::Code;
use crate::lints::lib::Problem;

pub(crate) const CONFIG: &str = "pivotal-tracker-id-missing";

const HELP_MESSAGE: &str = indoc!(
    "
    You can fix this by adding the Id in one of the styles below to the commit message
    [Delivers #12345678]
    [fixes #12345678]
    [finishes #12345678]
    [#12345884 #12345678]
    [#12345884,#12345678]
    [#12345678],[#12345884]
    This will address [#12345884]"
);

const ERROR: &str = "Your commit message is missing a Pivotal Tracker Id";

lazy_static! {
    static ref RE: Regex = Regex::new(
        r"(?i)\[(((finish|fix)(ed|es)?|complete[ds]?|deliver(s|ed)?) )?#\d+([, ]#\d+)*]"
    )
    .unwrap();
}

pub(crate) fn lint(commit: &CommitMessage) -> Option<Problem> {
    if commit.matches_pattern(&RE) {
        None
    } else {
        Some(Problem::new(
            ERROR.into(),
            HELP_MESSAGE.into(),
            Code::PivotalTrackerIdMissing,
        ))
    }
}

#[cfg(test)]
mod tests_has_missing_pivotal_tracker_id {
    #![allow(clippy::wildcard_imports)]

    use indoc::indoc;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn with_id() {
        test_has_missing_pivotal_tracker_id(
            indoc!(
                "
                An example commit

                This is an example commit

                [#12345678]
                # Some comment
                "
            ),
            &None,
        );
    }

    fn test_has_missing_pivotal_tracker_id(message: &str, expected: &Option<Problem>) {
        let actual = &lint(&CommitMessage::from(message));
        assert_eq!(
            actual, expected,
            "Message {:?} should have returned {:?}, found {:?}",
            message, expected, actual
        );
    }

    #[test]
    fn multiple_ids() {
        test_has_missing_pivotal_tracker_id(
            indoc!(
                "
                An example commit

                This is an example commit

                [#12345678,#87654321]
                # some comment
                "
            ),
            &None,
        );
        test_has_missing_pivotal_tracker_id(
            indoc!(
                "
                An example commit

                This is an example commit

                [#12345678,#87654321,#11223344]
                # some comment
                "
            ),
            &None,
        );
        test_has_missing_pivotal_tracker_id(
            indoc!(
                "
                An example commit

                This is an example commit

                [#12345678 #87654321 #11223344]
                # some comment
                "
            ),
            &None,
        );
    }

    #[test]
    fn id_with_fixed_state_change() {
        test_has_missing_pivotal_tracker_id(
            indoc!(
                "
                An example commit

                This is an example commit

                [fix #12345678]
                # some comment
                "
            ),
            &None,
        );
        test_has_missing_pivotal_tracker_id(
            indoc!(
                "
                An example commit

                This is an example commit

                [FIX #12345678]
                # some comment
                "
            ),
            &None,
        );
        test_has_missing_pivotal_tracker_id(
            indoc!(
                "
                An example commit

                This is an example commit

                [Fix #12345678]
                # some comment
                "
            ),
            &None,
        );
        test_has_missing_pivotal_tracker_id(
            indoc!(
                "
                An example commit

                This is an example commit

                [fixed #12345678]
                # some comment
                "
            ),
            &None,
        );
        test_has_missing_pivotal_tracker_id(
            indoc!(
                "
                An example commit

                This is an example commit

                [fixes #12345678]
                # some comment
                "
            ),
            &None,
        );
    }

    #[test]
    fn id_with_complete_state_change() {
        test_has_missing_pivotal_tracker_id(
            indoc!(
                "
                An example commit

                This is an example commit

                [complete #12345678]
                # some comment
                "
            ),
            &None,
        );

        test_has_missing_pivotal_tracker_id(
            indoc!(
                "
                An example commit

                This is an example commit

                [completed #12345678]
                # some comment
                "
            ),
            &None,
        );

        test_has_missing_pivotal_tracker_id(
            indoc!(
                "
                An example commit

                This is an example commit

                [Completed #12345678]
                # some comment
                "
            ),
            &None,
        );

        test_has_missing_pivotal_tracker_id(
            indoc!(
                "
                An example commit

                This is an example commit

                [completes #12345678]
                # some comment
                "
            ),
            &None,
        );
    }

    #[test]
    fn id_with_finished_state_change() {
        test_has_missing_pivotal_tracker_id(
            indoc!(
                "
                An example commit

                This is an example commit

                [finish #12345678]
                # some comment
                "
            ),
            &None,
        );

        test_has_missing_pivotal_tracker_id(
            indoc!(
                "
                An example commit

                This is an example commit

                [finished #12345678]
                # some comment
                "
            ),
            &None,
        );
        test_has_missing_pivotal_tracker_id(
            indoc!(
                "
                An example commit

                This is an example commit

                [finishes #12345678]
                # some comment
                "
            ),
            &None,
        );
    }

    #[test]
    fn id_with_delivered_state_change() {
        test_has_missing_pivotal_tracker_id(
            indoc!(
                "
                An example commit

                This is an example commit

                [deliver #12345678]
                # some comment
                "
            ),
            &None,
        );

        test_has_missing_pivotal_tracker_id(
            indoc!(
                "
                An example commit

                This is an example commit

                [delivered #12345678]
                # some comment
                "
            ),
            &None,
        );
        test_has_missing_pivotal_tracker_id(
            indoc!(
                "
                An example commit

                This is an example commit

                [delivers #12345678]
                # some comment
                "
            ),
            &None,
        );
    }

    #[test]
    fn id_with_state_change_and_multiple_ids() {
        test_has_missing_pivotal_tracker_id(
            indoc!(
                "
                An example commit

                This is an example commit

                [fix #12345678 #12345678]
                # some comment
                "
            ),
            &None,
        );
    }

    #[test]
    fn id_with_prefixed_text() {
        test_has_missing_pivotal_tracker_id(
            indoc!(
                "
                An example commit

                This is an example commit

                Finally [fix #12345678 #12345678]
                "
            ),
            &None,
        );
    }

    #[test]
    fn invalid_state_change() {
        test_has_missing_pivotal_tracker_id(
            indoc!(
                "
                An example commit

                This is an example commit

                [fake #12345678]
                # some comment
                "
            ),
            &Some(Problem::new(
                ERROR.into(),
                HELP_MESSAGE.into(),
                Code::PivotalTrackerIdMissing,
            )),
        );
    }

    #[test]
    fn missing_id_with_square_brackets() {
        test_has_missing_pivotal_tracker_id(
            indoc!(
                "
                An example commit

                This is an example commit
                "
            ),
            &Some(Problem::new(
                ERROR.into(),
                HELP_MESSAGE.into(),
                Code::PivotalTrackerIdMissing,
            )),
        );

        test_has_missing_pivotal_tracker_id(
            indoc!(
                "
            An example commit

            This is an example commit

            [#]
            # some comment
            "
            ),
            &Some(Problem::new(
                ERROR.into(),
                HELP_MESSAGE.into(),
                Code::PivotalTrackerIdMissing,
            )),
        );
    }
}
