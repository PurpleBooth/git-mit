use std::collections::BTreeSet;

use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use mit_commit::CommitMessage;
use mit_lint::{Lint, Lints, async_lint};
use tokio::runtime::Runtime;

const COMMIT_WITH_ALL_FEATURES: &str = "Add file

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
#   neue Datei:     file
#
# ------------------------ >8 ------------------------
# \u{00E4}ndern oder entfernen Sie nicht die obige Zeile.
# Alles unterhalb von ihr wird ignoriert.
diff --git a/file b/file
new file mode 100644
index 0000000..e69de29
";

/// Run the benchmark
///
/// # Arguments
///
/// * `c` - The Criterion instance used to configure and run the benchmarks
///
/// # Returns
///
/// This function doesn't return a value; it configures the Criterion benchmarks
///
/// # Panics
///
/// Panics if tokio fails to start
pub fn criterion_benchmark(c: &mut Criterion) {
    let tokio = Runtime::new().unwrap();
    let lints = Lint::all_lints().collect::<Vec<_>>();

    for enabled_lint in &lints {
        let mut btree_lints = BTreeSet::new();
        btree_lints.insert(*enabled_lint);
        let enabled_lints = Lints::new(btree_lints);
        c.bench_with_input(
            BenchmarkId::new("commit_with_all_features", (*enabled_lint).name()),
            &(COMMIT_WITH_ALL_FEATURES, enabled_lints),
            |b, (message, enabled_lints)| {
                let commit = CommitMessage::from(*message);
                b.to_async(&tokio)
                    .iter(|| async_lint(&commit, enabled_lints));
            },
        );
    }

    let all_lints = Lints::available();
    c.bench_with_input(
        BenchmarkId::new("commit_with_all_features", "all"),
        &(COMMIT_WITH_ALL_FEATURES, all_lints),
        |b, (message, all_lints)| {
            let commit = CommitMessage::from(*message);
            b.to_async(&tokio).iter(|| async_lint(&commit, all_lints));
        },
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
