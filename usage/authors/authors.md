# Duplicated Trailer Lints

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

## Configuring

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
    email: anyone@example.com" > git-authors.yml

echo "git-authors.yml" > .gitignore
git add .
```

You can save yourself having to pass `-c` to each command by setting an
environment variable to its path. Or just put the config at the default
location.

``` bash
export GIT_AUTHORS_CONFIG="$PWD/git-authors.yml"
```

## Setting author

You can't commit without an author set

``` bash
echo "$(mktemp)" > demo.txt
git add demo.txt

if git commit -m "Demo 1

Full Demo" ; then
    echo "This never happens" 
    exit 1
fi
```

You can set a single person as the author

``` bash
last_commit() {
    git show --pretty="$(printf format:"author: %%an %%ae\ncommit:\n%%B")" -q
}

echo "$(mktemp)" > demo.txt
git add demo.txt

git authors ae
git commit -m "Demo 2

Full Demo"

ACTUAL="$(last_commit)"
EXPECTED="author: Anyone Else anyone@example.com
commit:
Demo 2

Full Demo"
diff <(echo "$ACTUAL") <(echo "$EXPECTED")
```

You can set multiple people as the author and it sets the trailer

``` bash
echo "$(mktemp)" > demo.txt
git add demo.txt

git authors se ae
git commit -m "Demo 3

Full Demo"
ACTUAL="$(last_commit)"
EXPECTED="$(printf "author: Someone Else someone@example.com\ncommit:\nDemo 3\nFull Demo\n\nCo-authored-by: Anyone Else <anyone@example.com>\n")"

diff <(echo "$EXPECTED") <(echo "$ACTUAL")
```

We even support mobbing


``` bash
echo "$(mktemp)" > demo.txt
git add demo.txt

git authors se ae bt
git commit -m "Demo 4

Full Demo"
ACTUAL="$(last_commit)"
EXPECTED="$(printf "author: Someone Else someone@example.com\ncommit:\nDemo 4\nFull Demo\n\nCo-authored-by: Anyone Else <anyone@example.com>\n\nCo-authored-by: Billie Thompson <billie@example.com>\n")"

diff <(echo "$EXPECTED") <(echo "$ACTUAL")
```
