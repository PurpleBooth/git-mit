use std::{collections::BTreeMap, time::Duration};

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use indoc::indoc;
use mit_commit::{CommitMessage, Trailer};
use mit_commit_message_lints::{
    external,
    mit::{get_commit_coauthor_configuration, set_commit_authors, Author},
    relates::{get_relate_to_configuration, set_relates_to, RelateTo},
};

const COMMIT_WITH_ALL_FEATURES: &str = indoc!(
    "
        Add file

        Looks-like-a-trailer: But isn't

        This adds file primarily for demonstration purposes. It might not be
        useful as an actual commit, but it's very useful as a example to use in
        tests.

        Relates-to: #128

        # Short (50 chars or less) summary of changes
        #
        # More detailed explanatory text, if necessary.  Wrap it to
        # about 72 characters or so.  In some contexts, the first
        # line is treated as the subject of an email and the rest of
        # the text as the body.  The blank line separating the
        # summary from the body is critical (unless you omit the body
        # entirely); tools like rebase can get confused if you run
        # the two together.
        #
        # Further paragraphs come after blank lines.
        #
        #   - Bullet points are okay, too
        #
        #   - Typically a hyphen or asterisk is used for the bullet,
        #     preceded by a single space, with blank lines in
        #     between, but conventions vary here

        # Bitte geben Sie eine Commit-Beschreibung f\u{00FC}r Ihre \u{00E4}nderungen ein. Zeilen,
        # die mit '#' beginnen, werden ignoriert, und eine leere Beschreibung
        # bricht den Commit ab.
        #
        # Auf Branch main
        # Ihr Branch ist auf demselben Stand wie 'origin/main'.
        #
        # Zum Commit vorgemerkte \u{00E4}nderungen:
        #	neue Datei:     file
        #
        # ------------------------ >8 ------------------------
        # \u{00E4}ndern oder entfernen Sie nicht die obige Zeile.
        # Alles unterhalb von ihr wird ignoriert.
        diff --git a/file b/file
        new file mode 100644
        index 0000000..e69de29
        "
);

/// # Panics
///
/// On test failure
pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_with_input(
        BenchmarkId::new("trailers", "commit_with_all_features"),
        &COMMIT_WITH_ALL_FEATURES,
        |b, message| {
            b.iter(|| {
                let mut config = BTreeMap::new();

                let mut vcs = external::InMemory::new(&mut config);

                set_relates_to(
                    &mut vcs,
                    &RelateTo::new("#12345678"),
                    Duration::from_secs(60 * 60),
                )
                .unwrap();

                set_commit_authors(
                    &mut vcs,
                    &[
                        &Author::new("Someone Else", "someone@example.com", None),
                        &Author::new("Anyone Else", "anyone@example.com", None),
                    ],
                    Duration::from_secs(60 * 60),
                )
                .unwrap();

                let message =
                    CommitMessage::from(String::from(*message)).add_trailer(Trailer::new(
                        "Relates-to".to_string(),
                        get_relate_to_configuration(&mut vcs).unwrap().unwrap().to(),
                    ));
                get_commit_coauthor_configuration(&mut vcs)
                    .unwrap()
                    .unwrap()
                    .iter()
                    .map(|x| {
                        Trailer::new(
                            "Co-authored-by".to_string(),
                            format!("{} <{}>", x.name(), x.email()),
                        )
                    })
                    .fold(message.clone(), |_acc, author| message.add_trailer(author))
            });
        },
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
