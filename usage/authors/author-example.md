# Author File Example

## Setup

You need the binaries on your path and them installed into a git
repository.

``` bash
git init .

rm -rf .git/hooks/*
ln -s "$(command -v pb-commit-msg)" .git/hooks/commit-msg
ln -s "$(command -v pb-pre-commit)" .git/hooks/pre-commit
ln -s "$(command -v pb-prepare-commit-msg)" .git/hooks/prepare-commit-msg
```

## Print the author file example

You can list all the available lints with a handy command

``` bash
set -euo pipefail
ACTUAL="$(pb-git-hooks authors example)"
EXPECTED="---
ae:
  name: Anyone Else
  email: anyone@example.com
bt:
  name: Billie Thompson
  email: billie@example.com
  signingkey: 0A46826A
se:
  name: Someone Else
  email: someone@example.com"

diff <(echo "$ACTUAL") <(echo "$EXPECTED")
```
