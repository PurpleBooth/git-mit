# git-mit

This started out as a reimplementation of git-duet. It was an excuse for
me to learn Rust. It's a bit more than that now, with lints for issues.

## Usage

### Lint list

#### Trailers

Lints relating to trailers

  - **duplicated-trailers** - Detect duplicated `Signed-off-by` and
    `Co-authored-by` Trailers. *Default: `enabled`*

#### Git Manual Style

The style from the git manual, that directly affect the experience of
git beyond aesthetics.

  - **subject-not-separated-from-body** - If there is a body, enforce a
    gap between it and the subject. *Default: `enabled`*
  - **subject-longer-than-72-characters** - After 72 characters, git
    will truncate commit messages in the history view, this prevents
    that *Default: `enabled`*

#### Git Manual Style Extended

The style from the git manual, that do not affect the experience of git
beyond aesthetics

  - **subject-line-not-capitalized** - Detect a subject line that is not
    capitalised *Default: `disabled`*

#### Commit Messages

  - **pivotal-tracker-id-missing** - Detect missing Pivotal Tracker Id
    *Default: `disabled`*
  - **jira-issue-key-missing** - Detect missing Jira Issue Key *Default:
    `disabled`*
  - **github-id-missing** - Detect missing GitHub Id *Default:
    `disabled`*

### Enabling Lints

``` shell
git mit-config lint enable duplicated-trailers
```

### Disabling Lints

``` shell
git mit-config lint disable duplicated-trailers
```

### Centralising lint config

You can add a `.git-mit.toml` or `.git-mit.toml.dist` to the root of
your repository and we will read it and try to enable the correct lints
(with `.git-mit.toml` taking precedence).

I recommend you commit `.git-mit.toml.dist` and `.gitignore`
`.git-mit.toml` to allow easy local reconfiguration

``` toml
[mit.lint]
"pivotal-tracker-id-missing" = true
```

### Append issue number

You can append an issue number to your commits in a trailer

``` shell
git mit-relates-to "#21346578"
```

Will append

    Relates-to: #21346578

To your commit messsage

### Setting Authors and Co-Authors

Just set the author

``` shell
git mit bt
```

Set the author and co-author trailer

``` shell
git mit bt se
```

If you're mobbing

``` shell
git mit bt se ae
```

## Installing

First tap my homebrew repo

``` shell
brew tap PurpleBooth/repo
```

Next install the binary

``` shell
brew install PurpleBooth/repo/git-mit
```

You can also download the [latest
release](https://github.com/PurpleBooth/git-mit/releases/latest) and run
it.

## Configuration

### Adding to a repository

``` shell
ln -s "$(command -v mit-commit-msg)" .git/hooks/commit-msg
ln -s "$(command -v mit-pre-commit)" .git/hooks/pre-commit
ln -s "$(command -v mit-prepare-commit-msg)" .git/hooks/prepare-commit-msg
```

You can make git link these for you automatically by adding them to your
[init template](https://git-scm.com/docs/git-init#_template_directory).
This is the template that git uses to create the `.git` directory when
you run `git init`.

``` shell
mkdir -p "$HOME/.config/git/init-template/hooks"
ln -s "$(command -v mit-commit-msg)" "$HOME/.config/git/init-template/hooks/commit-msg"
ln -s "$(command -v mit-pre-commit)" "$HOME/.config/git/init-template/hooks/pre-commit"
ln -s "$(command -v mit-prepare-commit-msg)" "$HOME/.config/git/init-template/hooks/prepare-commit-msg"
git config --global init.templatedir "$HOME/.config/git/init-template"
```

You can also run this on an existing repository, to set up an already
checked out repository. You can re-initialise all of your repositories,
recursively from the home directory using this command.

``` shell
find "$HOME" -type d -name .git -exec sh -c 'git init "$1"/..' -- {} \;
```

### Authors Configuration

If you want to use the author part create yourself a configuration and
save it into a file

``` toml
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

you can use yaml too

``` yaml
---
ae:
  name: Anyone Else
  email: anyone@example.com
bt:
  name: Billie Thompson
  email: billie@example.com
  signingkey: 0A46826A
se:
  name: Someone Else
  email: someone@example.com
```

### More examples

You can see more examples in the [usage
directory](https://github.com/PurpleBooth/git-mit/tree/main/usage)

### Environment Variables

  - **GIT\_MIT\_AUTHORS\_EXEC** A command to execute to generate the
    author configuration
  - **GIT\_MIT\_AUTHORS\_CONFIG** The location of a author file
    *Default: `$HOME/.config/git-mit/mit.yml`*
  - **GIT\_MIT\_AUTHORS\_TIMEOUT** How long to wait before you need to
    run git mit again *Default: `60`*
  - **GIT\_MIT\_RELATES\_TO\_TIMEOUT** How long to wait before you need
    to run git relates-to again *Default: `60`*
