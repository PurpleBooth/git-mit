% { bin | upper }(1) Version { version } | The git-mit suite of git hooks

NAME
====

**{ bin }** — { about }

SYNOPSIS
========

{ usage }

DESCRIPTION
===========

{ about }

Arguments
---------

```
{positionals|unescape}
```

Options
-------

```
{unified|unescape}
```

FILES
=====

*~/.config/git-mit/mit.yml*

:   Configuration file listing authors

```yaml
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

Common Tasks
============

You can install git-mit into a new repository using

```shell script
git mit-install
```

You can add a new author to that repository by running

```shell script
git mit-config mit set eg "Egg Sample" egg.sample@example.com
```

You can save that author permanently by running

```shell script
git mit-config mit set eg "Egg Sample" egg.sample@example.com
git mit-config mit generate > $HOME/.config/git-mit/mit.yml
```

You can disable a lint by running

```shell script
git mit-config lint disable jira-issue-key-missing
```

You can install the example authors file to the default location with

```shell script
git mit-config mit example > $HOME/.config/git-mit/mit.yml
```

BUGS
====

See GitHub Issues: <https://github.com/PurpleBooth/git-mit/issues>

AUTHOR
======

{ author }

SEE ALSO
========

**git-mit(1)**, **git-mit-config(1)**, **mit-commit-msg(1)**, **mit-pre-commit(1)**, **mit-prepare-commit-msg(1)**
