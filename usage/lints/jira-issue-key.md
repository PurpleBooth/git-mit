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

We need a author configuration that is current

``` bash

echo "---
bt:
    name: Billie Thompson
    email: billie@example.com
    signingkey: 0A46826A
se:
    name: Someone Else
    email: someone@example.com
ae:
    name: Anyone Else
    email: anyone@example.com" > authors.yml

git-authors -c authors.yml se
echo "git-authors.yml" > .gitignore
git add .
git commit -m "Add a git ignore"

```

## Default setting

Without enabling the lint can make commits without an key

``` bash
echo "$(mktemp)" > demo.txt
git add .
git commit -m "Enabling a lint"
```

## Enabling this specific lint

To enable it run

``` bash
pb-git-hooks lint enable jira-issue-key-missing
```

After enabling the lint you can't commit without a issue key

``` bash
echo "$(mktemp)" > demo.txt
git add demo.txt

if git commit -m "I am not made" ; then
    echo "This never happens" 
    exit 1
fi
```

But you can with one

``` bash
echo "$(mktemp)" > demo.txt
git add demo.txt

git commit -m "Enabled the lint

JRA-123
"
```

## Disabling this specific lint

You can also disable the lint

``` bash
pb-git-hooks lint disable jira-issue-key-missing
```

You'll be able to commit without an key

``` bash
echo "$(mktemp)" > demo.txt
git add demo.txt
git commit -m "Disabling the lint"
```
