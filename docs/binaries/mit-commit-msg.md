# mit-commit-msg

> Note: This is a hook binary, you will probably never interact with it
> directly

Generic usage

``` shell,script(name="show-help",expected_exit_code=0)
mit-commit-msg --help
```

``` shell,verify(script_name="show-help",stream=stdout)
Validate the commit message that a user has input

Usage: mit-commit-msg [OPTIONS] [COMMIT_FILE_PATH]

Arguments:
  [COMMIT_FILE_PATH]
          Path to a temporary file that contains the commit message written by the developer
          
          When omitted the hook falls back to `<gitdir>/COMMIT_EDITMSG`, which is useful when the
          hook is invoked via a hook manager like lefthook that does not forward git's positional
          argument.

Options:
      --copy-message-to-clipboard
          On lint failure copy the message to clipboard
          
          [env: GIT_MIT_COPY_MESSAGE_TO_CLIPBOARD=]

      --completion <COMPLETION>
          [possible values: bash, elvish, fish, powershell, zsh]

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

You can generate completion with

``` shell,script(name="generate-bash-completion",expected_exit_code=0)
mit-commit-msg --completion bash
```

When no commit file path is provided — for example when a hook manager like
[lefthook](https://github.com/evilmartians/lefthook) invokes the hook
without forwarding git's positional argument — the hook falls back to
reading the commit message from `.git/COMMIT_EDITMSG`. See the
[lefthook integration guide](../lefthook.md) for setup instructions.

``` shell,script(name="no-argument-fallback",expected_exit_code=0)
git init .
printf 'Add a feature\n' > .git/COMMIT_EDITMSG
mit-commit-msg
```
