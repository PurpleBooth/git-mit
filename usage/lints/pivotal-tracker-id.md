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

git-mit -c git-mit.yml se
echo "git-mit.yml" > .gitignore
git add .
git commit -m "Add a git ignore"

```

## Default setting

Without enabling the lint can make commits without an ID

``` bash
mktemp > demo.txt
git add .
git commit -m "Enabling a lint"
```

## Enabling this specific lint

To enable it run

``` bash
git mit-config lint enable pivotal-tracker-id-missing
```

After enabling the lint you can't commit without a issue id

``` bash
mktemp > demo.txt
git add demo.txt

if git commit -m "I am not made" ; then
    echo "This never happens" 
    exit 1
fi
```

But you can with one

``` bash
mktemp > demo.txt
git add demo.txt

git commit -m "Enabled the lint

[#87654321]
"
```

## Disabling this specific lint

You can also disable the lint

``` bash
git mit-config lint disable pivotal-tracker-id-missing
```

You'll be able to commit without an ID

``` bash
mktemp > demo.txt
git add demo.txt
git commit -m "Disabling the lint"
```
