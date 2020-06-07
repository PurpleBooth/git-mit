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
    email: anyone@example.com" > authors.yml

git-authors -c authors.yml ae se
echo "git-authors.yml" > .gitignore
git add .
git commit -m "Add a git ignore"
```

You can use whatever editor you want, but you do need to use an editor.

``` bash
export EDITOR="bash -c \"cat message \$1 > appended && mv appended \\\"\\\$1\\\"\" -- "
echo "message" >> .gitignore
```

## Default setting

This lint is enabled by default, with it on you can't commit duplicated
trailers. The two trailers we check for `Co-authored-by` and
`Signed-of-by`

### `Co-authored-by`

``` bash
echo "Hello, world!" > demo.txt
git add demo.txt

echo "I am not made

Co-authored-by: Someone Else <someone@example.com>
" > message

if git commit ; then
    echo "This never happens" 
    exit 1
fi
```

### `Signed-of-by`

``` bash
echo "Hello, world!" > demo.txt
git add demo.txt

echo "I am not made

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
echo "Hello, world!" > demo.txt
git add demo.txt

echo "Another example

Signed-off-by: Anyone Else <anyone@example.com>
" > message
git commit -s

echo "Another example

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
echo "Hello, world!" > demo.txt
git add demo.txt

echo "I am not made

Signed-off-by: Anyone Else <anyone@example.com>
" > message

if git commit -s ; then
    echo "This never happens" 
    exit 1
fi
```
