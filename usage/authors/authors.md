# Authors

This is the `git authors` part of the tool.

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
bt:
    name: Billie Thompson
    email: billie@example.com
    signingkey: "$KEY"
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
mktemp > demo.txt
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
    git show --pretty="$(printf format:"author: %%an %%ae\nsigned: %%GS\ncommit:\n%%B")" -q
}

mktemp > demo.txt
git add demo.txt

git authors ae
git commit -m "Demo 2

Full Demo"

ACTUAL="$(last_commit)"
EXPECTED="author: Anyone Else anyone@example.com
signed: 
commit:
Demo 2

Full Demo"
diff <(echo "$ACTUAL") <(echo "$EXPECTED")
```

You can set multiple people as the author and it sets the trailer

``` bash
mktemp > demo.txt
git add demo.txt

git authors se ae
git commit -m "Demo 3

Full Demo"
ACTUAL="$(last_commit)"
EXPECTED="$(printf "author: Someone Else someone@example.com\nsigned: \ncommit:\nDemo 3\nFull Demo\n\nCo-authored-by: Anyone Else <anyone@example.com>\n")"

diff <(echo "$EXPECTED") <(echo "$ACTUAL")
```

We even support mobbing

``` bash
mktemp > demo.txt
git add demo.txt

git authors bt se ae
git commit -m "Demo 5

Full Demo"
ACTUAL="$(last_commit)"
EXPECTED="$(printf "author: Billie Thompson billie@example.com\nsigned: \ncommit:\nDemo 5\nFull Demo\n\nCo-authored-by: Someone Else <someone@example.com>\n\nCo-authored-by: Anyone Else <anyone@example.com>\n")"

diff <(echo "$EXPECTED") <(echo "$ACTUAL")
```

And you can sign your commits

``` bash
mktemp > demo.txt
git add demo.txt

git authors bt
git commit -S -m "Demo 6

Full Demo"
ACTUAL="$(last_commit)"
EXPECTED="$(printf "author: Billie Thompson billie@example.com\nsigned: Billie Thompson <billie@example.com>\ncommit:\nDemo 6\nFull Demo\n\n")"

diff <(echo "$EXPECTED") <(echo "$ACTUAL")
```
