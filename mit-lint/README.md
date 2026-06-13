# mit-lint

Lint commit messages

## Example

```rust
use mit_commit::CommitMessage;
use mit_lint::{Code, lint, Problem, Lints, Lint};

let message: String = "x".repeat(73).clone().into();

let expected = vec![Problem::new(
    "Your subject is longer than 72 characters".into(),
    "It's important to keep the subject of the commit less than 72 characters \
     because when you look at the git log, that's where it truncates the message. \
     This means that people won't get the entirety of the information in your commit.\n\n\
     Please keep the subject line 72 characters or under".into(),
    Code::SubjectLongerThan72Characters,
    &message.clone().into(),
    Some(vec![(String::from("Too long"), 72, 1)]),
    Some(
        "https://git-scm.com/book/en/v2/Distributed-Git-Contributing-to-a-Project#_commit_guidelines"
            .parse()
            .unwrap()
    ),
)];

let actual = lint(
&CommitMessage::from(message.clone()),
&Lints::new(
vec![Lint::SubjectLongerThan72Characters]
.into_iter()
.collect()
)
);

assert_eq!(
    actual,
    expected,
    "Expected {:?}, found {:?}",
    expected,
    actual
);
```

## Docs

Read more at [Docs.rs](https://docs.rs/mit-lint/)

## Source

The source code is available on [Codeberg](https://codeberg.org/PurpleBooth/mit-lint)
