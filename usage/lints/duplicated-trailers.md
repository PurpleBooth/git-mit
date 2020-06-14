# Duplicated Trailer Lints

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

We need a author configuration that is current

``` bash

export GNUPGHOME="$(mktemp -d)"
cat >foo <<EOF
%echo Generating a basic OpenPGP key
%no-protection
Key-Type: DSA
Key-Length: 1024
Subkey-Type: ELG-E
Subkey-Length: 1024
Name-Real: Billie Thompson
Name-Email: billie@example.com
Expire-Date: 0
# Do a commit here, so that we can later print "done" :-)
%commit
%echo done
EOF
gpg --batch --generate-key foo
KEY="$(gpg --with-colons --fingerprint billie@example.com | awk -F: '$1 == "fpr" {print $10;}' | head -n 1)"
rm foo
git config --local --add --bool commit.gpgsign false

echo "---
ae:
  name: Anyone Else
  email: anyone@example.com
bt:
  name: Billie Thompson
  email: billie@example.com
  signingkey: $KEY
se:
  name: Someone Else
  email: someone@example.com" > git-mit.yml

git-mit -c git-mit.yml ae se
echo "git-mit.yml" > .gitignore
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
git mit-config lint disable duplicated-trailers
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
git mit-config lint enable duplicated-trailers
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
