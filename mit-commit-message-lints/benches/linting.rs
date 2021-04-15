use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

use mit_commit::CommitMessage;

use indoc::indoc;
use mit_commit_message_lints::lints::{lint, Lint, Lints};
use std::collections::BTreeSet;

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

pub fn criterion_benchmark(c: &mut Criterion) {
    let lints = Lint::iterator().collect::<Vec<_>>();

    for enabled_lint in &lints {
        let mut btree_lints = BTreeSet::new();
        btree_lints.insert(*enabled_lint);
        let enabled_lints = Lints::new(btree_lints);
        c.bench_with_input(
            BenchmarkId::new("commit_with_all_features", (*enabled_lint).name()),
            &(COMMIT_WITH_ALL_FEATURES, enabled_lints),
            |b, (message, enabled_lints)| {
                b.iter(|| lint(&CommitMessage::from(*message), enabled_lints.clone()))
            },
        );
    }

    let all_lints = Lints::available();
    c.bench_with_input(
        BenchmarkId::new("commit_with_all_features", "all"),
        &(COMMIT_WITH_ALL_FEATURES, all_lints.clone()),
        |b, (message, all_lints)| {
            b.iter(|| lint(&CommitMessage::from(*message), all_lints.clone()))
        },
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
