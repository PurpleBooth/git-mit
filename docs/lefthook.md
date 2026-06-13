# Lefthook integration

[lefthook](https://github.com/evilmartians/lefthook) is a popular git hook
manager. When lefthook takes over a repository's hooks it replaces the hook
scripts in `.git/hooks` with its own runner. Lefthook then invokes the
configured commands but, unlike git, does **not** forward the positional
arguments (such as the commit-message file path) to those commands.

This means that when lefthook runs `mit-commit-msg` and
`mit-prepare-commit-msg` the commit-message path argument is missing. The
hooks fall back to `<gitdir>/COMMIT_EDITMSG` so co-author trailers and lints
continue to work.

## Setup

We need a git repository with the hooks installed.

```shell,script(name="init-repo",expected_exit_code=0)
git init .
```

```shell,script(name="install-hooks",expected_exit_code=0)
git mit-install
```

```shell,script(name="set-author",expected_exit_code=0)
git mit bt se
```

## Configuring lefthook

We tell lefthook to run all three git-mit hooks. No `{1}` argument forwarding
is needed.

```yaml,file(path="lefthook.yml")
---
pre-commit:
  commands:
    git-mit:
      run: mit-pre-commit

prepare-commit-msg:
  commands:
    git-mit:
      run: mit-prepare-commit-msg

commit-msg:
  commands:
    git-mit:
      run: mit-commit-msg
```

Installing lefthook takes over the hooks.

```shell,script(name="lefthook-install",expected_exit_code=0)
lefthook install
```

## Committing through lefthook

When we commit, lefthook intercepts each hook and runs the git-mit binaries
without forwarding the commit-message path. The co-author trailers still
appear because the hooks fall back to `COMMIT_EDITMSG`.

```shell,script(name="lefthook-commit",expected_exit_code=0)
echo "# Hello, world!" > README.md

git add .
git commit --message="Add a feature" --quiet
git show --pretty='format:author: [%an %ae] signed-by: [%GS]
---
%B' -q
```

```text,verify(script_name="lefthook-commit",stream=stdout)
author: [Billie Thompson billie@example.com] signed-by: []
---
Add a feature

Co-authored-by: Someone Else <someone@example.com>
```
