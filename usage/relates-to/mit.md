# Authors

This is the `git mit` part of the tool.

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

## Configuring

We need a author configuration that is current

``` bash
last_commit() {
    git show --pretty="$(printf format:"author: %%an %%ae\nsigned: %%GS\ncommit:\n%%B")" -q
}

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

echo "git-mit.yml" > .gitignore
git add .
```

You can save yourself having to pass `-c` to each command by setting an
environment variable to its path. Or just put the config at the default
location.

``` bash
export GIT_MIT_AUTHORS_CONFIG="$PWD/git-mit.yml"
git mit se
```

## Setting the relates to

Without setting the variable nothing happens

``` bash
mktemp > demo.txt
git add demo.txt

git commit -m "Demo 1

Full Demo"
ACTUAL="$(last_commit)"
EXPECTED="$(printf "author: Someone Else someone@example.com\nsigned: \ncommit:\nDemo 1\nFull Demo\n\n")"

diff <(echo "$EXPECTED") <(echo "$ACTUAL")
```

When you set it, the relates-to trailer is set

``` bash
mktemp > demo.txt
git add demo.txt

git relates-to "#[#12343567]"
git commit -m "Demo 2

Full Demo"
ACTUAL="$(last_commit)"
EXPECTED="$(printf "author: Someone Else someone@example.com\nsigned: \ncommit:\nDemo 2\nFull Demo\n\nRelates-to: #[#12343567]\n")"

diff <(echo "$EXPECTED") <(echo "$ACTUAL")
```
