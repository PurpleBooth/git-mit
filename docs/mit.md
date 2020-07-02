# Authors

This is the `git mit` part of the tool.

## Setup

In order to get started with this tool you'll need a git repostory

```shell,script(name="1", expected_exit_code=0)
git init .
```

You'll need to install the hooks into this repository

```shell,script(name="2", expected_exit_code=0)
git mit-install
```

## Configuring

For this example we'll be using the same configuration as you can
generate from the example command.

You can see this configuration yourself by running

```shell,script(name="3")
git-mit-config mit example
```

```toml,verify(script_name="3", stream=stdout)
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

```yaml,file(path="git-mit.yml")
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

## Running the command

We can then use this by passing `-c` to the `git-mit` command.

```shell,script(name="4", expected_exit_code=0)
git mit -c "../git-mit.toml" ae bt se
```

It's exactly the same for a YAML configuration

```shell,script(name="4", expected_exit_code=0)
git mit -c "git-mit.yml" ae bt se
```

Or you can use the environment variables

```shell,script(name="5", expected_exit_code=0)
export GIT_MIT_AUTHORS_CONFIG="../git-mit.toml"
git mit -c "$HOME/git-mit.toml" ae bt se
```

Or just put it at the default location

```shell,script(name="6", expected_exit_code=0)
git mit ae bt se
```

Then next when you make a commit the `Co-authored-by` trailers will be
set of the author initials you selected.

```shell,script(name="7", expected_exit_code=0)
echo "# Hello, world!" > README.md

git add .
git commit --message="Initial Commit" --quiet
git show --pretty='format:author: [%an %ae] signed-by: [%GS] 
---
%B' -q
```

```text,verify(script_name="7", stream=stdout)
author: [Anyone Else anyone@example.com] signed-by: [] 
---
Initial Commit

Co-authored-by: Billie Thompson <billie@example.com>
Co-authored-by: Someone Else <someone@example.com>
```

You don't need to constantly pass the config everywhere though, you can
set an environment variable.

```shell,script(name="8", expected_exit_code=0)
export GIT_MIT_AUTHORS_CONFIG="$HOME/git-mit.toml"
git mit se ae
```

So next time we commit

```shell,script(name="9", expected_exit_code=0)
echo "Lorem Ipsum" >> README.md

git commit --all --message="Second Commit" --quiet
git show --pretty='format:author: [%an %ae] signed-by: [%GS] 
---
%B' -q
```

The author configuration will be updated like this

```text,verify(script_name="9", stream=stdout)
author: [Someone Else someone@example.com] signed-by: [] 
---
Second Commit

Co-authored-by: Anyone Else <anyone@example.com>
```

The command also works with signed commits

The `bt` user has a valid gpg key.


```shell,script(name="10", expected_exit_code=0)
git mit bt
```

```shell,script(name="10", expected_exit_code=0)
echo "Delores Et" >> README.md

git commit --all --gpg-sign --message="Third Commit" --quiet
git show --pretty='format:author: [%an %ae] signed-by: [%GS] 
---
%B' -q
```

```text,verify(script_name="10", stream=stdout)
author: [Billie Thompson billie@example.com] signed-by: [Billie Thompson <billie@example.com>] 
---
Third Commit
```
