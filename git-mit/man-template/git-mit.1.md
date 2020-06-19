% GIT-MIT(1) Version 3.12.0 | The git-mit suite of git hooks

NAME
====

**git-mit** — Set author and Co-authored trailer.

SYNOPSIS
========

| git-mit [-h|--help|-v|--version]
| git-mit [OPTIONS] <initials>...

DESCRIPTION
===========

Set author and Co-authored trailer.

Arguments
---------

\<initials\>...

:   Initials of the author to put in the commit

Options
-------

-h, --help

:   Prints help information

-V, --version

:   Prints version information

-e, --exec \<command\>

:  Execute a command to generate the author configuration, stdout will be captured and used instead of the file, if both this and the file is present, this takes precedence 

-c, --config \<file\>

:  Path to a file where author initials, emails and names can be found 

-t, --timeout \<timeout\>

:  Number of minutes to expire the configuration in 

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

ENVIRONMENT
===========

**GIT_MIT_AUTHORS_EXEC**

:   Execute a command to generate the author configuration, stdout will be captured and used instead of the file, if both this and the file is present, this takes precedence

**GIT_MIT_AUTHORS_CONFIG**

:   Path to a file where author initials, emails and names can be found

**GIT_MIT_AUTHORS_TIMEOUT**

:   Number of minutes to expire the configuration in

BUGS
====

See GitHub Issues: <https://github.com/PurpleBooth/git-mit/issues>

AUTHOR
======

Billie Thompson <billie+git-mit@billiecodes.com>

SEE ALSO
========

**git-mit-config(1)**, **mit-commit-msg(1)**, **mit-pre-commit(1)**, **mit-prepare-commit-msg(1)**
