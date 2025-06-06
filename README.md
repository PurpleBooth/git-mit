<p align="center">
    <img alt="git-mit" width="50%" src="./logo/logo.png">
</p>

*git-mit* is a suite of git hooks. It's aimed to make pair programming,
adding issue numbers to your commits, and following good commit message
practices something that happens without thinking about it.

## Usage

### Preparing the repository

This works via git hooks, so you need these hooks to be present in the
git repository you're using to use them.

``` shell,script(name="initialize-repository",expected_exit_code=0)
git init .
git mit-install
```

This works by creating a symlink in your repositories hooks directory.
You can do this automatically by adding them to your [init
template](https://git-scm.com/docs/git-init#_template_directory). This
is the template that git uses to create the `.git` directory when you
run `git init`.

``` shell,skip()
git mit-install --scope=global
```

You can also run this on an existing repository, to set up an already
checked out repository. You can re-initialise all of your repositories,
recursively from the home directory using this command.

``` shell,skip()
find "$HOME" -type d -name .git -exec sh -c 'git init "$1"/..' -- {} \;
```

### Listing Lints

``` shell,script(name="list-available-lints",expected_exit_code=0)
git mit-config lint available
```

``` text,verify(script_name="list-available-lints",stream=stdout)
╭───────────────────────────────────┬──────────╮
│ Lint                              ┆ Status   │
╞═══════════════════════════════════╪══════════╡
│ duplicated-trailers               ┆ enabled  │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌┤
│ pivotal-tracker-id-missing        ┆ disabled │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌┤
│ jira-issue-key-missing            ┆ disabled │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌┤
│ github-id-missing                 ┆ disabled │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌┤
│ subject-not-separated-from-body   ┆ enabled  │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌┤
│ subject-longer-than-72-characters ┆ enabled  │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌┤
│ subject-line-not-capitalized      ┆ disabled │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌┤
│ subject-line-ends-with-period     ┆ disabled │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌┤
│ body-wider-than-72-characters     ┆ enabled  │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌┤
│ not-conventional-commit           ┆ disabled │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌┤
│ not-emoji-log                     ┆ disabled │
╰───────────────────────────────────┴──────────╯
```

With only lints that ensure git will work properly enabled by default

``` shell,script(name="list-enabled-lints",expected_exit_code=0)
git mit-config lint enabled
```

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

You can read more details about this, with examples on the [lints
page](docs/lints/index.md)

### Centralising lint config

You can add a `.git-mit.toml` or `.git-mit.toml.dist` to the root of
your repository, and we will read it and try to enable the correct lints
(with `.git-mit.toml` taking precedence).

I recommend you commit `.git-mit.toml.dist` and add `.git-mit.toml` to
your `.gitignore` to allow easy local reconfiguration

For example

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

You can read more about this on the [configuring
page](docs/lints/configuring.md)

### Append issue number

In projects, it is nice to help out your co-workers by linking the
commits you're making back to the issue in the backlog. This can get a
bit tedious to remember though, so here's a command to reduce the amount
of typing.

Say you've just made this awesome `README.md` for Pivotal Tracker ID
`[#12321513]`

``` markdown,file(path="README.md")
# The Best Readme

This is the best readme
```

If you run

``` shell,script(name="set-relates-to-trailer",expected_exit_code=0)
git mit-relates-to "[#12321513]"
```

Next time you commit

``` shell,script(name="stage-and-set-authors",expected_exit_code=0)
git add README.md
git mit bt
git commit -m "Wrote a great README"
```

the commit message will contain the ID

``` shell,script(name="show-commit-with-relates-to",expected_exit_code=0)
git show --pretty='format:author: [%an %ae] signed-by: [%GS] 
---
%B' -q
```

``` text,verify(script_name="show-commit-with-relates-to",stream=stdout)
author: [Billie Thompson billie@example.com] signed-by: [] 
---
Wrote a great README

Relates-to: [#12321513]
```

Read more about this at the [relates to page](docs/mit-relates-to.md)

### Setting Authors and Co-authors

Pairing is a great way to program, and it's even better when you give
credit, you can give credit with the mit command

Configure your authors like the example by creating a config at
`$HOME/.config/git-mit/mit.toml`

``` shell,script(name="generate-example-config",expected_exit_code=0)
git-mit-config mit example
```

``` toml,verify(script_name="generate-example-config",stream=stdout)
[ae]
name = "Anyone Else"
email = "anyone@example.com"

[bt]
name = "Billie Thompson"
email = "billie@example.com"
signingkey = "0A46826A"

[se]
name = "Someone Else"
email = "someone@example.com"
```

And you can run

``` shell,script(name="set-multiple-coauthors",expected_exit_code=0)
git mit ae bt se
```

Then next when you make a commit the `Co-authored-by` trailers will be
set of the author initials you selected.

``` shell,script(name="show-coauthored-commit",expected_exit_code=0)
echo "# Hello, world!" > README.md

git add .
git commit --message="Initial Commit" --quiet
git show --pretty='format:author: [%an %ae] signed-by: [%GS] 
---
%B' -q
```

``` text,verify(script_name="show-coauthored-commit",stream=stdout)
author: [Anyone Else anyone@example.com] signed-by: [] 
---
Initial Commit

Co-authored-by: Billie Thompson <billie@example.com>
Co-authored-by: Someone Else <someone@example.com>
Relates-to: [#12321513]
```

Notice how the "Relates-to" tag is here even though we didn't trigger
it? It's from the example higher on the page, git-mit remembers your
author and ticket number for 60 min

For more information on this see the [mit page](docs/mit.md)

## Installing

You can install this with brew! This is the preferred method of
installing.

``` shell,skip()
brew install PurpleBooth/repo/git-mit
```

You can use Cargo too, though this won't install the completions

``` shell,skip()
cargo install git-mit
cargo install git-mit-config
cargo install git-mit-install
cargo install git-mit-relates-to
cargo install mit-commit-msg
cargo install mit-pre-commit
cargo install mit-prepare-commit-msg
```

You can also download the [latest
release](https://github.com/PurpleBooth/git-mit/releases/latest) and run
it.

There is a script to download the latest release:

- [Windows](./installer.ps1)
- [Linux/Mac OS](./installer.sh)

### Completions

We generate completions for `fish`,`zsh`, and `bash`. They're installed
with the homebrew package. You don't need to do anything to activate
them.

Each binary also has a command to generate completion.

## Docs

### Common Tasks

- [Using the pair programming part of the tool](./docs/mit.md)
- [Using the issue number inserting part of the
  tool](./docs/mit-relates-to.md)
- [Configuring lints](./docs/lints/configuring.md)
- [Lint list](./docs/lints/index.md)
- [Troubleshooting](./docs/troubleshooting.md)

### Usage

- [git-mit](./docs/binaries/git-mit.md)
- [git-mit-config](./docs/binaries/git-mit-config.md)
- [git-mit-install](./docs/binaries/git-mit-install.md)
- [git-mit-relates-to](./docs/binaries/git-mit-relates-to.md)
- [Hook: mit-commit-msg](./docs/binaries/mit-commit-msg.md)
- [Hook: mit-pre-commit](./docs/binaries/mit-pre-commit.md)
- [Hook:
  mit-prepare-commit-msg](./docs/binaries/mit-prepare-commit-msg.md)
