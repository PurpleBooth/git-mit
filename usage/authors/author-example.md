# Pivotal Tracker Lints

## Setup

You need the binaries on your path and them installed into a git
repository.

``` bash
set -euo pipefail

git init .

rm -rf .git/hooks/*
ln -s "$(command -v pb-commit-msg)" .git/hooks/commit-msg
ln -s "$(command -v pb-pre-commit)" .git/hooks/pre-commit
ln -s "$(command -v pb-prepare-commit-msg)" .git/hooks/prepare-commit-msg
```

## List available lints

You can list all the available lints with a handy command

``` bash
ACTUAL="$(pb-git-hooks authors example)"
EXPECTED="---
bt:
  name: Billie Thompson
  email: billie@example.com
  signingkey: 0A46826A
se:
  name: Someone Else
  email: someone@example.com
ae:
  name: Anyone Else
  email: anyone@example.com"

diff <(printf "$ACTUAL") <(printf "$EXPECTED")
```
