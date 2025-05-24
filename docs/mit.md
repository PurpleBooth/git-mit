# Authors

This is the `git mit` part of the tool.

## Setup

To get started with this tool you'll need a git repository

``` shell,script(name="init-repo",expected_exit_code=0)
git init .
```

You'll need to install the hooks into this repository

``` shell,script(name="install-hooks",expected_exit_code=0)
git mit-install
```

## Configuring

For this example we'll be using the same configuration as you can
generate from the example command.

You can see this configuration yourself by running

``` shell,script(name="generate-example-config",expected_exit_code=0)
git-mit-config mit example
```

``` toml,verify(script_name="generate-example-config",stream=stdout)
[ae]
name = "Anyone Else"
email = "anyone@example.com"

[bt]
name = "Billie Thompson"
email = "billie@example.com"
signingkey = "0A46826A"

[se]
name = "Someone Else"
email = "someone@example.com"
```

You can use yaml too

``` yaml,file(path="git-mit.yml")
---
ae:
  name: Anyone Else
  email: anyone@example.com
bt:
  name: Billie Thompson
  email: billie@example.com
  signingkey: 0A46826A
se:
  name: Someone Else
  email: someone@example.com
```

To make keeping this file up-to-date easier there's some commands to
adding and removing users to your git repository, that can then be
output into a more permanent configuration file.

You can quickly add a new author by running

``` shell,script(name="set-signed-author",expected_exit_code=0)
git mit-config mit set jd "Jane Doe" "jd@example.com"
```

To override an existing user temporarily

``` shell,script(name="override-user-se",expected_exit_code=0)
git mit-config mit set se "Someone Else" "se@example.com"
```

You can use these straight away, you don't have to update your authors
file.

``` shell,script(name="commit-with-jd-author",expected_exit_code=0)
git mit jd
```

However, if you want to make it more permanent, you can output the
configuration with these added authors by running the generate command

``` shell,script(name="generate-config",expected_exit_code=0)
git mit-config mit generate
```

``` toml,skip()
[ae]
name = "Anyone Else"
email = "anyone@example.com"

[bt]
name = "Billie Thompson"
email = "billie@example.com"
signingkey = "0A46826A"

[jd]
name = "Jane Doe"
email = "jd@example.com"

[se]
name = "Someone Else"
email = "se@example.com"
```

To get a summary of the authors that are configured run

``` shell,script(name="list-configured-authors",expected_exit_code=0)
git mit-config mit available
```

``` text,skip()
╭─────────┬─────────────────┬────────────────────┬─────────────╮
│ Initial ┆ Name            ┆ Email              ┆ Signing Key │
╞═════════╪═════════════════╪════════════════════╪═════════════╡
│ ae      ┆ Anyone Else     ┆ anyone@example.com ┆ None        │
├╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ bt      ┆ Billie Thompson ┆ billie@example.com ┆ 0A46826A    │
├╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ jd      ┆ Jane Doe        ┆ jd@example.com     ┆ None        │
├╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ se      ┆ Someone Else    ┆ se@example.com     ┆ None        │
╰─────────┴─────────────────┴────────────────────┴─────────────╯
```

## Running the command

We can then use this by passing `-c` to the `git-mit` command.

``` shell,script(name="commit-with-external-config",expected_exit_code=0)
git mit -c "../git-mit.toml" ae bt se
```

It's exactly the same for a YAML configuration

``` shell,script(name="commit-with-yaml-config",expected_exit_code=0)
git mit -c "git-mit.yml" ae bt se
```

Or you can use the environment variables

``` shell,script(name="commit-with-env-var",expected_exit_code=0)
export GIT_MIT_AUTHORS_CONFIG="../git-mit.toml"
git mit -c "$HOME/git-mit.toml" ae bt se
```

Or just put it at the default location

``` shell,script(name="commit-default-config",expected_exit_code=0)
git mit ae bt se
```

Then next when you make a commit the `Co-authored-by` trailers will be
set of the author initials you selected.

``` shell,script(name="create-initial-commit",expected_exit_code=0)
echo "# Hello, world!" > README.md

git add .
git commit --message="Initial Commit" --quiet
git show --pretty='format:author: [%an %ae] signed-by: [%GS] 
---
%B' -q
```

``` text,verify(script_name="create-initial-commit",stream=stdout)
author: [Anyone Else anyone@example.com] signed-by: [] 
---
Initial Commit

Co-authored-by: Billie Thompson <billie@example.com>
Co-authored-by: Someone Else <se@example.com>
```

You don't need to constantly pass the config everywhere though, you can
set an environment variable.

``` shell,script(name="set-env-variable",expected_exit_code=0)
export GIT_MIT_AUTHORS_CONFIG="$HOME/git-mit.toml"
git mit se ae
```

So next time we commit

``` shell,script(name="create-second-commit",expected_exit_code=0)
echo "Lorem Ipsum" >> README.md

git commit --all --message="Second Commit" --quiet
git show --pretty='format:author: [%an %ae] signed-by: [%GS] 
---
%B' -q
```

The author configuration will be updated like this

``` text,verify(script_name="create-second-commit",stream=stdout)
author: [Someone Else se@example.com] signed-by: [] 
---
Second Commit

Co-authored-by: Anyone Else <anyone@example.com>
```

If for some reason you've already added the author we won't duplicate it

``` shell,script(name="create-duplicate-coauthor-commit",expected_exit_code=0)
echo "Lorem Ipsum" >> README.md

git commit --all --message="Second Commit

Co-authored-by: Anyone Else <anyone@example.com>
" --quiet
git show --pretty='format:author: [%an %ae] signed-by: [%GS] 
---
%B' -q
```

The author configuration will be updated like this

``` text,verify(script_name="create-duplicate-coauthor-commit",stream=stdout)
author: [Someone Else se@example.com] signed-by: [] 
---
Second Commit

Co-authored-by: Anyone Else <anyone@example.com>
```

## Rebases

It might be preferable not to do this on rebase, you can disable this happening on rebase by running

``` shell,script(name="set-non-clean-behavior-no-change",expected_exit_code=0)
git mit-config mit set-non-clean-behaviour no-change
git mit-config mit non-clean-behaviour
```

To get the current behavior run

``` text,verify(script_name="set-non-clean-behavior-no-change",stream=stdout)
no-change
```

lets say we have two diverging branches

``` shell,script(name="create-rebase-branches",expected_exit_code=0)
echo "Lorem Ipsum" >> new-so-no-conflicts.md
git switch -c rebase-demo-branch
git switch -
git commit --all --message="Diverging commit" --quiet
git switch -
```

Now you can rebase changes without adding any additional trailers

``` shell,script(name="verify-rebase-no-change",expected_exit_code=0)
git mit bt se
echo "Lorem Ipsum" >> README.md

git commit --all --message="Rebase behaviour
" --quiet
git show --pretty='format:author: [%an %ae] signed-by: [%GS] 
---
%B' -q
```

``` text,verify(script_name="verify-rebase-no-change",stream=stdout)
author: [Billie Thompson billie@example.com] signed-by: [] 
---
Rebase behaviour

Co-authored-by: Someone Else <se@example.com>
```

Then if you rebase the commit stays the same

``` shell,script(name="rebase-with-add-to-behavior",expected_exit_code=0)
git mit bt ae
git rebase --reset-author-date "-"
```

``` shell,script(name="git-mit-config-mit-non-clean-behaviour",expected_exit_code=0)
git show --pretty='format:author: [%an %ae] signed-by: [%GS] 
---
%B' -q
```

``` text,verify(script_name="git-mit-config-mit-non-clean-behaviour",stream=stdout)
author: [Billie Thompson billie@example.com] signed-by: [] 
---
Rebase behaviour

Co-authored-by: Someone Else <se@example.com>
```

The default setting is to modify the co-authored by.

``` shell,script(name="set-non-clean-behavior-add-to",expected_exit_code=0)
git mit-config mit set-non-clean-behaviour add-to
git mit-config mit non-clean-behaviour
```
``` text,verify(script_name="set-non-clean-behavior-add-to",stream=stdout)
add-to
```

Now you can rebase changes without adding any additional trailers
``` shell,script(name="verify-rebase-add-to",expected_exit_code=0)
git mit bt ae
git rebase --reset-author-date "-"
```

``` shell,script(name="git-mit-config-mit-non-clean-behaviour-check",expected_exit_code=0)
git show --pretty='format:author: [%an %ae] signed-by: [%GS] 
---
%B' -q
```

``` text,verify(script_name="git-mit-config-mit-non-clean-behaviour-check",stream=stdout)
author: [Billie Thompson billie@example.com] signed-by: [] 
---
Rebase behaviour

Co-authored-by: Someone Else <se@example.com>
Co-authored-by: Anyone Else <anyone@example.com>
```

## Signed Commits

The command also works with signed commits

The `bt` user has a valid gpg key.

``` shell,script(name="show-commit-with-multiple-coauthors",expected_exit_code=0)
git mit bt
```

``` shell,script(name="show-signed-commit-with-gpg",expected_exit_code=0)
echo "Delores Et" >> README.md

git commit --all --gpg-sign --message="Third Commit" --quiet
git show --pretty='format:author: [%an %ae] signed-by: [%GS] 
---
%B' -q
```

``` text,verify(script_name="show-signed-commit-with-gpg",stream=stdout)
author: [Billie Thompson billie@example.com] signed-by: [Billie Thompson <billie@example.com>] 
---
Third Commit
```

## Errors

If your authors file is broken like the one below (or for any other
reason)

``` toml,file(path="broken.toml")
Hello, I am a broken file
```

You'll get an error when you run the command

``` shell,script(name="error-invalid-config-file",expected_exit_code=1)
git mit -c "broken.toml" ae bt se
```

``` text,verify(script_name="error-invalid-config-file",stream=stderr)
Error: mit_commit_message_lints::mit::lib::authors::serialise_authors_error

  × could not parse author configuration
   ╭────
 1 │ Hello, I am a broken file
   · ▲    ▲
   · │    ╰── invalid in toml: 
   · ╰── invalid in yaml: 
   ╰────
  help: `git mit-config mit example` can show you an example of what it should
        look like, or you can generate one using `git mit-config mit generate`
        after setting up some authors with `git mit-config mit set`

```

Same applies for `git mit-config mit generate`

``` shell,script(name="error-generate-invalid-config",expected_exit_code=1)
git mit-config mit generate -c "broken.toml"
```

``` text,verify(script_name="error-generate-invalid-config",stream=stderr)
Error: mit_commit_message_lints::mit::lib::authors::serialise_authors_error

  × could not parse author configuration
   ╭────
 1 │ Hello, I am a broken file
   · ▲    ▲
   · │    ╰── invalid in toml: 
   · ╰── invalid in yaml: 
   ╰────
  help: `git mit-config mit example` can show you an example of what it should
        look like, or you can generate one using `git mit-config mit generate`
        after setting up some authors with `git mit-config mit set`

```
