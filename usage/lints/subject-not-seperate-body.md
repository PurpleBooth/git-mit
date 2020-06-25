# Subject Not seperate from body

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
  email: someone@example.com" > git-mit.toml

git-mit -c git-mit.toml ae se
echo "git-mit.toml" > .gitignore
git add .
```

## Default setting

This lint is enabled by default, with it on you can't commit without a
gutter between the subject and the body.

``` bash
mktemp > demo.txt
git add demo.txt


if git commit -m "Example poorly formed commit
Notice how there's no gap here" ; then
    echo "This never happens" 
    exit 1
fi
```

You must put a gutter in like this

``` bash
mktemp > demo.txt
git add demo.txt


git commit -m "$(printf "Example well formed commit\n\nNotice how there's a gap here")"
```

## Disabling this specific lint

You can also disable the lint

``` bash
git mit-config lint disable subject-not-separated-from-body
```

You'll be able to commit with a poorly formed commit

``` bash
mktemp > demo.txt
git add demo.txt

git commit -m "Example poorly formed commit\
Notice how there's no gap here"
```

## Enabling this lint again

To enable it run

``` bash
git mit-config lint enable subject-not-separated-from-body
```

Then the lints are enabled again

``` bash
mktemp > demo.txt
git add demo.txt


if git commit -m "Example poorly formed commit
Notice how there's no gap here" ; then
    echo "This never happens" 
    exit 1
fi
```
