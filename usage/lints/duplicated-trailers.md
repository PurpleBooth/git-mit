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

git-authors -c git-authors.yml ae se
echo "git-authors.yml" > .gitignore
git add .
```

You can use whatever editor you want, but you do need to use an editor.

``` bash
export EDITOR="bash -c \"cat message \\\"\\\$1\\\" > appended && mv appended \\\"\\\$1\\\" && rm message\" -- "
echo "message" >> .gitignore
git add .
git commit -m "Duplicated Trailer Lints: Setup"
```

## Default setting

This lint is enabled by default, with it on you can't commit duplicated
trailers. The two trailers we check for `Co-authored-by` and
`Signed-of-by`

### `Co-authored-by`

``` bash
mktemp > demo.txt
git add demo.txt

echo "Duplicated Trailer Lints

Default setting - Co-Author

Co-authored-by: Someone Else <someone@example.com>
" > message

if git commit ; then
    echo "This never happens" 
    exit 1
fi
```

### `Signed-of-by`

``` bash
mktemp > demo.txt
git add demo.txt

echo "I am not made

Duplicated Trailer Lints

Default setting - Signed-off

Signed-off-by: Anyone Else <anyone@example.com>
" > message

if git commit -s ; then
    echo "This never happens" 
    exit 1
fi
```

## Disabling this specific lint

You can also disable the lint

``` bash
pb-git-hooks lint disable duplicated-trailers
```

You'll be able to commit without an ID

``` bash
mktemp > demo.txt
git add demo.txt

echo "Another example

Disabling this specific lint - Signed-off

Signed-off-by: Anyone Else <anyone@example.com>
" > message
git commit -s

mktemp > demo.txt
git add demo.txt
echo "Another example

Disabling this specific lint - Co-authored

Co-authored-by: Someone Else <someone@example.com>
" > message
git commit
```

## Enabling this lint again

To enable it run

``` bash
pb-git-hooks lint enable duplicated-trailers
```

Then the lints are enabled again

``` bash
mktemp > demo.txt
git add demo.txt

echo "I am not made

Signed-off-by: Anyone Else <anyone@example.com>
" > message

if git commit -s ; then
    echo "This never happens" 
    exit 1
fi
```
