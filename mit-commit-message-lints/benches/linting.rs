use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

use mit_commit::CommitMessage;

use indoc::indoc;
use mit_commit_message_lints::lints::{lib::Lints, lint, Lint};

const COMMIT_MESSAGE_WITH_NO_COMMENTS: &str = indoc!(
    "
        Update bashrc to include kubernetes completions

        This should make it easier to deploy things for the developers.
        Benchmarked with Hyperfine, no noticable performance decrease.

        Co-authored-by: Billie Thomposon <billie@example.com>
        Co-authored-by: Somebody Else <somebody@example.com>
        "
);

const MULTIPLE_TRAILERS: &str = indoc!(
    "
        Update bashrc to include kubernetes completions

        This should make it easier to deploy things for the developers.
        Benchmarked with Hyperfine, no noticable performance decrease.

        Co-authored-by: Billie Thomposon <billie@example.com>
        Co-authored-by: Somebody Else <somebody@example.com>

        # Bitte geben Sie eine Commit-Beschreibung f\u{00FC}r Ihre \u{00E4}nderungen ein. Zeilen,
        # die mit '#' beginnen, werden ignoriert, und eine leere Beschreibung
        # bricht den Commit ab.
        #
        # Datum:            Sat Jun 27 21:40:14 2020 +0200
        #
        # Auf Branch master
        #
        # Initialer Commit
        #
        # Zum Commit vorgemerkte \u{00E4}nderungen:
        #	neue Datei:     .bashrc
        #"
);

const NON_STANDARD_COMMENT_CHARACTER: &str = indoc!(
    "
        Allow the server to respond to https

        This allows the server to respond to HTTPS requests, by correcting the port binding.
        We should see a nice speed increase from this

        fixes:
        #6436
        #6437
        #6438

        ; Bitte geben Sie eine Commit-Beschreibung f\u{00FC}r Ihre \u{00C4}nderungen ein. Zeilen,
        ; die mit ';' beginnen, werden ignoriert, und eine leere Beschreibung
        ; bricht den Commit ab."
);

const NOT_VERBOSE_COMMIT: &str = indoc!(
    "
        Update bashrc to include kubernetes completions

        This should make it easier to deploy things for the developers.
        Benchmarked with Hyperfine, no noticable performance decrease.

        # Bitte geben Sie eine Commit-Beschreibung f\u{00FC}r Ihre \u{00E4}nderungen ein. Zeilen,
        # die mit '#' beginnen, werden ignoriert, und eine leere Beschreibung
        # bricht den Commit ab.
        #
        # Datum:            Sat Jun 27 21:40:14 2020 +0200
        #
        # Auf Branch master
        #
        # Initialer Commit
        #
        # Zum Commit vorgemerkte \u{00E4}nderungen:
        #	neue Datei:     .bashrc
        #"
);

const LONG_SUBJECT_ONLY_COMMIT: &str = indoc!(
    "
        Initial Commit
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
        # Auf Branch master
        #
        # Initialer Commit
        #
        # Zum Commit vorgemerkte \u{00E4}nderungen:
        #	neue Datei:     src/bodies.rs
        #	neue Datei:     src/body.rs
        #	neue Datei:     src/comment.rs
        #	neue Datei:     src/comments.rs
        #	neue Datei:     src/commit_message.rs
        #	neue Datei:     src/scissors.rs
        #	neue Datei:     src/subject.rs
        #	neue Datei:     src/trailer.rs
        #	neue Datei:     src/trailers.rs
        #
        # \u{00E4}nderungen, die nicht zum Commit vorgemerkt sind:
        #	ge\u{00E4}ndert:       src/bodies.rs
        #	ge\u{00E4}ndert:       src/body.rs
        #	ge\u{00E4}ndert:       src/comment.rs
        #	ge\u{00E4}ndert:       src/comments.rs
        #	ge\u{00E4}ndert:       src/commit_message.rs
        #	ge\u{00E4}ndert:       src/scissors.rs
        #	ge\u{00E4}ndert:       src/subject.rs
        #	ge\u{00E4}ndert:       src/trailer.rs
        #	ge\u{00E4}ndert:       src/trailers.rs
        #
        # Unversionierte Dateien:
        #	.gitignore
        #	Cargo.toml
        #	src/lib.rs
        #
        # ------------------------ >8 ------------------------
        # \u{00E4}ndern oder entfernen Sie nicht die obige Zeile.
        # Alles unterhalb von ihr wird ignoriert.
        diff --git a/src/bodies.rs b/src/bodies.rs
        new file mode 100644
        index 0000000..e69de29
        diff --git a/src/body.rs b/src/body.rs
        new file mode 100644
        index 0000000..e69de29
        diff --git a/src/comment.rs b/src/comment.rs
        new file mode 100644
        index 0000000..e69de29
        diff --git a/src/comments.rs b/src/comments.rs
        new file mode 100644
        index 0000000..e69de29
        diff --git a/src/commit_message.rs b/src/commit_message.rs
        new file mode 100644
        index 0000000..e69de29
        diff --git a/src/scissors.rs b/src/scissors.rs
        new file mode 100644
        index 0000000..e69de29
        diff --git a/src/subject.rs b/src/subject.rs
        new file mode 100644
        index 0000000..e69de29
        diff --git a/src/trailer.rs b/src/trailer.rs
        new file mode 100644
        index 0000000..e69de29
        diff --git a/src/trailers.rs b/src/trailers.rs
        new file mode 100644
        index 0000000..e69de29"
);

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

const INVALID: &str = indoc!(
    "
        add file add file add file add file add file add file add file add file add file
        Looks-like-a-trailer: But isn't

        This adds file primarily for demonstration purposes. It might not be
        useful as an actual commit, but it's very useful as a example to use in
        tests.

        This line is too long This line is too long This line is too long This line is too long

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
    let messages: &[&str] = &[
        COMMIT_MESSAGE_WITH_NO_COMMENTS,
        MULTIPLE_TRAILERS,
        NON_STANDARD_COMMENT_CHARACTER,
        NOT_VERBOSE_COMMIT,
        LONG_SUBJECT_ONLY_COMMIT,
        COMMIT_WITH_ALL_FEATURES,
        INVALID,
    ];

    messages.iter().enumerate().for_each(|(number, message)| {
        c.bench_with_input(BenchmarkId::new("lint", number), &message, |b, &message| {
            let default_lints: Lints = Lint::iterator()
                .filter(|x| x.enabled_by_default())
                .collect::<Vec<_>>()
                .into();
            b.iter(|| lint(&CommitMessage::from(*message), default_lints.clone()))
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
