# Configuring Lints

Some lints are not appropriate in some situations. For example you
probably don't want the `github-id-missing` lint if you're using Jira.

## Setup

As always we need a working it repository the, with the hooks installed.

```shell,script(name="1", expected_exit_code=0)
git init .
git mit-install
```

## Default lints

Some lints are enabled by default

If we run

```shell,script(name="2", expected_exit_code=0)
git mit-config lint enabled
```

You can see what's enabled by default.

```text,verify(script_name="2", stream=stdout)
duplicated-trailers
subject-not-separated-from-body
subject-longer-than-72-characters
body-wider-than-72-characters
```

## Toggling lints

To disable a lint we run

```shell,script(name="4", expected_exit_code=0)
git mit-config lint disable subject-longer-than-72-characters
```

and then run

```shell,script(name="5", expected_exit_code=0)
git mit-config lint status subject-longer-than-72-characters
```

We will see that it's now disabled

```text,verify(script_name="5", stream=stdout)
subject-longer-than-72-characters	disabled
```

If we run

```shell,script(name="6", expected_exit_code=0)
git mit-config lint enable subject-longer-than-72-characters
```

We can see that it's back

```shell,script(name="6", expected_exit_code=0)
git mit-config lint status subject-longer-than-72-characters
```

```text,verify(script_name="6", stream=stdout)
subject-longer-than-72-characters	enabled
```

This are written to the `./.git/config` file.

## Project level lint configuration

Sometimes you're working with a group of people and you want them to
have the same configuration as you, so the git history looks lovely and
tidy.

We can do this with a `.git-mit.toml.dist`

```toml,file(path=".git-mit.toml.dist")
[mit.lint]
"pivotal-tracker-id-missing" = true
```

With this you can enable lints

```shell,script(name="7", expected_exit_code=0)
git mit-config lint status pivotal-tracker-id-missing
```

```text,verify(script_name="7", stream=stdout)
pivotal-tracker-id-missing	enabled
```

Alternatively you can see this in the enabled command from the start

```shell,script(name="2", expected_exit_code=0)
git mit-config lint enabled
```

You can see what's enabled by default.

```text,verify(script_name="2", stream=stdout)
duplicated-trailers
pivotal-tracker-id-missing
subject-not-separated-from-body
subject-longer-than-72-characters
body-wider-than-72-characters
```

I'd recommend you commit this file, then locally if someone wants to
tweak something, that they then create a `.git-mit.toml` and add it to
the `.gitignore`.

```toml,file(path=".git-mit.toml")
[mit.lint]
"pivotal-tracker-id-missing" = false
```

This file will override the `.git-mit.toml.dist`

```shell,script(name="8", expected_exit_code=0)
git mit-config lint status pivotal-tracker-id-missing
```

```text,verify(script_name="8", stream=stdout)
pivotal-tracker-id-missing	disabled
```
