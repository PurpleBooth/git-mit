# git-mit

*git-mit* started as a reimplementation of *git-duet*. It was an excuse
for me to learn Rust. It's a bit more than that now, with lints for
issues.

## Usage

### Preparing the repository

This works via git hooks, so you need these hooks to be present in the
git repository you're using to use them.

```shell,script(name="1", expected_exit_code=0)
git init .
git mit-install
```

This works by symlinking in your repositories hooks directory. You can
do this automatically by adding them to your [init
template](https://git-scm.com/docs/git-init#_template_directory). This
is the template that git uses to create the `.git` directory when you
run `git init`.

```shell,skip()
mkdir -p "$HOME/.config/git/init-template/hooks"
ln -s "$(command -v mit-commit-msg)" "$HOME/.config/git/init-template/hooks/commit-msg"
ln -s "$(command -v mit-pre-commit)" "$HOME/.config/git/init-template/hooks/pre-commit"
ln -s "$(command -v mit-prepare-commit-msg)" "$HOME/.config/git/init-template/hooks/prepare-commit-msg"
git config --global init.templatedir "$HOME/.config/git/init-template"
```

You can also run this on an existing repository, to set up an already
checked out repository. You can re-initialise all of your repositories,
recursively from the home directory using this command.

```shell,skip()
find "$HOME" -type d -name .git -exec sh -c 'git init "$1"/..' -- {} \;
```

### Lint list

```shell,script(name="lint-list", expected_exit_code=0)
git mit-config lint available
```

```text,verify(script_name="lint-list", stream=stdout)
duplicated-trailers
pivotal-tracker-id-missing
jira-issue-key-missing
github-id-missing
subject-not-separated-from-body
subject-longer-than-72-characters
subject-line-not-capitalized
subject-line-ends-with-period
body-wider-than-72-characters
```

With only lints that ensure git will work properly enabled by default

```shell,script(name="lint-list", expected_exit_code=0)
git mit-config lint enabled
```

```text,verify(script_name="lint-list", stream=stdout)
duplicated-trailers
subject-not-separated-from-body
subject-longer-than-72-characters
body-wider-than-72-characters
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

```toml,file(path=".git-mit.toml.dist")
[mit.lint]
"pivotal-tracker-id-missing" = true
```

With this you can enable lints

```shell,script(name="7", expected_exit_code=0)
git mit-config lint status pivotal-tracker-id-missing
```

```text,verify(script_name="7", stream=stdout)
pivotal-tracker-id-missing	enabled
```

You can read more about this on the [configuring
page](docs/lints/configuring.md)

### Append issue number

In projects it nice to help out your co-workers by linking the commits
you're making back to issues in the backlog. This can get a bit tedious
to remember though, so here's a command to reduce the amount of typing.

Say you've just made this awesome `README.md` for Pivotal Tracker ID
`[#12321513]`

```markdown,file(path="README.md")
# The Best Readme

This is the best readme
```

If you run

```shell,script(name="2", expected_exit_code=0)
git mit-relates-to "[#12321513]"
```

Next time you commit

```shell,script(name="3", expected_exit_code=0)
git add README.md
git mit bt
git commit -m "Wrote a great README"
```

the commit message will contain the ID

```shell,script(name="4", expected_exit_code=0)
git show --pretty='format:author: [%an %ae] signed-by: [%GS] 
---
%B' -q
```

```text,verify(script_name="4", stream=stdout)
author: [Billie Thompson billie@example.com] signed-by: [] 
---
Wrote a great README

Relates-to: [#12321513]
```

Read more about this at the [relates to page](docs/mit-relates-to.md)

### Setting Authors and Co-Authors

Pairing is a great way to program, and it's even better when you give
credit, you can give credit with the mit command

Configure your authors like the example by creating a config at 
`$HOME/.config/git-mit/mit.toml`


```shell,script(name="3")
git-mit-config mit example
```

```toml,verify(script_name="3", stream=stdout)
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

```shell,script(name="6", expected_exit_code=0)
git mit ae bt se
```

Then next when you make a commit the `Co-authored-by` trailers will be
set of the author initials you selected.

```shell,script(name="7", expected_exit_code=0)
echo "# Hello, world!" > README.md

git add .
git commit --message="Initial Commit" --quiet
git show --pretty='format:author: [%an %ae] signed-by: [%GS] 
---
%B' -q
```

```text,verify(script_name="7", stream=stdout)
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

You can install this with brew\!

```shell,skip()
brew install PurpleBooth/repo/git-mit
```

You can also download the [latest
release](https://github.com/PurpleBooth/git-mit/releases/latest) and run
it.

### Completions

We generate completions for `fish`,`zsh`, `bash`, and `elvish`. They're
installed with the homebrew package. You don't need to do anything to
activate them.
