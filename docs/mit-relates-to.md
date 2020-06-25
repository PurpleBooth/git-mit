# Relates to

This is the `git mit-relates-to` part of the tool.

## Setup

In order to get started with this tool you'll need a git repostory

```shell,script(name="1", expected_exit_code=0)
git init .
```

You'll need to install the hooks into this repository

```shell,script(name="2", expected_exit_code=0)
git mit-install
```

## Running the command

In projects it nice to help out your co-workers by linking the commits you're making back to issues in the backlog. This can get a bit tedious to remember though, so here's a command to reduce the amount of typing.

Say you've just made this awesome `README.md` for Pivotal Tracker ID `[#12321513]`

```markdown,file(path="README.md")
# The Best Readme

This is the best readme
```

If you run 

```shell,script(name="2", expected_exit_code=0)
git mit-relates-to "[#12321513]"
```

Next time you commit

```shell,script(name="3", expected_exit_code=0)
git add README.md
git mit bt
git commit -m "Wrote a great README"
```

the commit message will contain the ID

```shell,script(name="4", expected_exit_code=0)
git show --pretty='format:author: [%an %ae] signed-by: [%GS] 
---
%B' -q
```

```text,verify(script_name="4", stream=stdout)
author: [Billie Thompson billie@example.com] signed-by: [] 
---
Wrote a great README

Relates-to: [#12321513]
```

This times out after 60 minuites, and is configurable with the `GIT_MIT_RELATES_TO_TIMEOUT` by environment variable.


```shell,script(name="5", expected_exit_code=0)
export GIT_MIT_RELATES_TO_TIMEOUT=120
git mit-relates-to "[#12321513]"
```

Would set the timeout to 2 hours (or 120 minutes).
