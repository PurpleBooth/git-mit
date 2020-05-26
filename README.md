# pb-git-hooks

My git commit hooks in binary form

## Usage

### Lint list

  - **duplicated-trailers** - Detect duplicated `Signed-off-by` and
    `Co-authored-by` Trailers. *Default: `enabled`*
  - **pivotal-tracker-id-missing** - Detect missing Pivotal Tracker Id
    *Default: `disabled`*

### Enabling Lints

``` shell
pb-git-hooks lint enable duplicated-trailers
```

### Disabling Lints

``` shell
pb-git-hooks lint disable duplicated-trailers
```

### Setting Authors and Co-Authors

``` shell
git authors bt
```

## Installing

First tap my homebrew repo

``` shell
brew tap PurpleBooth/repo
```

Next install the binary

``` shell
brew install PurpleBooth/repo/pb-git-hooks
```

You can also download the [latest
release](https://github.com/PurpleBooth/pb-git-hooks/releases/latest)
and run it.

## Configuration

### Adding to a repository

``` shell
ln -s "$(command -v pb-commit-msg)" .git/hooks/commit-msg
ln -s "$(command -v pb-pre-commit)" .git/hooks/pre-commit
ln -s "$(command -v pb-prepare-commit-msg)" .git/hooks/prepare-commit-msg
```

### Authors Configuration

If you want to use the author part create yourself a configuration and
save it into a file

``` yaml
---
bt:
    name: Billie Thompson
    email: billie@example.com
se:
    name: Someone Else
    email: someone@example.com
```

### Environment Variables

  - **GIT\_AUTHORS\_EXEC** A command to execute to generate the author
    configuration
  - **GIT\_AUTHORS\_CONFIG** The location of a author file *Default:
    `$HOME/.config/git-authors/authors.yml`*
  - **GIT\_AUTHORS\_TIMEOUT** How long to wait before you need to run
    git authors again *Default: `60`*
