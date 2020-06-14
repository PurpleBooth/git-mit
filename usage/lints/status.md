# Pivotal Tracker Lints

## Setup

You need the binaries on your path and them installed into a git
repository.

``` bash
set -euo pipefail

git init .

rm -rf .git/hooks/*
ln -s "$(command -v mit-commit-msg)" .git/hooks/commit-msg
ln -s "$(command -v mit-pre-commit)" .git/hooks/pre-commit
ln -s "$(command -v mit-prepare-commit-msg)" .git/hooks/prepare-commit-msg
```

## List available lints

You can list all the available lints with a handy command

``` bash
ACTUAL="$(git mit-config lint available)"
EXPECTED="duplicated-trailers
pivotal-tracker-id-missing
jira-issue-key-missing"

diff <(printf "$ACTUAL") <(printf "$EXPECTED")
```

## List enabled lints

You can list all the enabled lints with a handy command

``` bash
ACTUAL="$(git mit-config lint enabled)"
EXPECTED="duplicated-trailers"

diff <(printf "$ACTUAL") <(printf "$EXPECTED")
```

## Get Status of single lint

You can get the status of a single lint either enabled

``` bash
ACTUAL="$(git mit-config lint status duplicated-trailers)"
EXPECTED="$(printf "duplicated-trailers\tenabled\n")"

diff <(printf "$ACTUAL") <(printf "$EXPECTED")
```

or disabled

``` bash
ACTUAL="$(git mit-config lint status jira-issue-key-missing)"
EXPECTED="$(printf "jira-issue-key-missing\tdisabled\n")"

diff <(printf "$ACTUAL") <(printf "$EXPECTED")
```
