# Configuring Lints

Some lints are not appropriate in some situations. For example, you
probably don't want the `github-id-missing` lint if you're using Jira.

## Setup

As always, we need a working it repository, with the hooks installed.

``` shell,script(name="init-repo",expected_exit_code=0)
git init .
git mit-install
```

## Default lints

Some lints are enabled by default

If we run

``` shell,script(name="list-enabled-lints",expected_exit_code=0)
git mit-config lint enabled
```

You can see what's enabled by default.

``` text,verify(script_name="list-enabled-lints",stream=stdout)
╭───────────────────────────────────┬─────────╮
│ Lint                              ┆ Status  │
╞═══════════════════════════════════╪═════════╡
│ duplicated-trailers               ┆ enabled │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ subject-not-separated-from-body   ┆ enabled │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ subject-longer-than-72-characters ┆ enabled │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ body-wider-than-72-characters     ┆ enabled │
╰───────────────────────────────────┴─────────╯
```

## Toggling lints

To disable a lint we run

``` shell,script(name="disable-subject-length-lint",expected_exit_code=0)
git mit-config lint disable subject-longer-than-72-characters
```

and then run

``` shell,script(name="check-subject-length-lint-status",expected_exit_code=0)
git mit-config lint status subject-longer-than-72-characters
```

We will see that it's now disabled

``` text,verify(script_name="check-subject-length-lint-status",stream=stdout)
╭───────────────────────────────────┬──────────╮
│ Lint                              ┆ Status   │
╞═══════════════════════════════════╪══════════╡
│ subject-longer-than-72-characters ┆ disabled │
╰───────────────────────────────────┴──────────╯
```

If we run

``` shell,script(name="enable-subject-length-lint",expected_exit_code=0)
git mit-config lint enable subject-longer-than-72-characters
```

We can see that it's back

``` shell,script(name="verify-subject-length-lint-enabled",expected_exit_code=0)
git mit-config lint status subject-longer-than-72-characters
```

``` text,verify(script_name="verify-subject-length-lint-enabled",stream=stdout)
╭───────────────────────────────────┬─────────╮
│ Lint                              ┆ Status  │
╞═══════════════════════════════════╪═════════╡
│ subject-longer-than-72-characters ┆ enabled │
╰───────────────────────────────────┴─────────╯
```

These are written to the `./.git/config` file.

## Project level lint configuration

Sometimes you're working with a group of people, and want share the
setup for git-mit with everyone, saving them having to set it up.

We can do this with a `.git-mit.toml.dist`

``` toml,file(path=".git-mit.toml.dist")
[mit.lint]
"pivotal-tracker-id-missing" = true
```

With this you can enable lints

``` shell,script(name="check-pivotal-lint-status",expected_exit_code=0)
git mit-config lint status pivotal-tracker-id-missing
```

``` text,verify(script_name="check-pivotal-lint-status",stream=stdout)
╭────────────────────────────┬─────────╮
│ Lint                       ┆ Status  │
╞════════════════════════════╪═════════╡
│ pivotal-tracker-id-missing ┆ enabled │
╰────────────────────────────┴─────────╯
```

You can generate this file for your current settings by running

``` shell,script(name="generate-lint-config",expected_exit_code=0)
git mit-config lint generate
```

``` toml,verify(script_name="generate-lint-config",stream=stdout)
[mit.lint]
body-wider-than-72-characters = true
duplicated-trailers = true
github-id-missing = false
jira-issue-key-missing = false
not-conventional-commit = false
not-emoji-log = false
pivotal-tracker-id-missing = true
subject-line-ends-with-period = false
subject-line-not-capitalized = false
subject-longer-than-72-characters = true
subject-not-separated-from-body = true
```

I'd recommend you commit this file, then locally if someone wants to
tweak something, that they then create a `.git-mit.toml` and add it to
the `.gitignore`.

``` toml,file(path=".git-mit.toml")
[mit.lint]
"pivotal-tracker-id-missing" = false
```

This file will override the `.git-mit.toml.dist`

``` shell,script(name="check-pivotal-lint-override",expected_exit_code=0)
git mit-config lint status pivotal-tracker-id-missing
```

``` text,verify(script_name="check-pivotal-lint-override",stream=stdout)
╭────────────────────────────┬──────────╮
│ Lint                       ┆ Status   │
╞════════════════════════════╪══════════╡
│ pivotal-tracker-id-missing ┆ disabled │
╰────────────────────────────┴──────────╯
```

You configure the authors [separately](../mit.md). This is so you don't
end up committing people's emails into a public repository.
