# Enabling and Disabling via git-mit.toml

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

## You can create a `.git-mit.toml.dist`

This is at the root of your repository

### Enabling a lint

``` bash
mktemp > demo.txt
git add demo.txt

echo "
[mit.lint]
\"pivotal-tracker-id-missing\" = true
" > .git-mit.toml.dist

git-mit-config lint enabled

if git commit -m "Missing ID" ; then
    echo "This never happens" 
    exit 1
fi
```

### Overriding `.git-mit.toml.dist` with `.git-mit.toml`

You can also create a `.git-mit.toml` which takes precedence over
`.git-mit.toml.dist`

``` bash
mktemp > demo.txt
git add demo.txt

echo "
[mit.lint]
" > .git-mit.toml

git commit -m "No ID"
```

### Disabling a lint

You can also enforce a lint being off

``` bash
mktemp > demo.txt

echo "
[mit.lint]
\"duplicated-trailers\" = false
" > .git-mit.toml

git commit -a -m "$(printf "Another example\n\nDisabling this specific lint - Co-authored\nCo-authored-by: Someone Else <someone@example.com>Co-authored-by: Someone Else <someone@example.com>")"
```
