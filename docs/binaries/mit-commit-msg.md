# mit-commit-msg

> Note: This is a hook binary, you will probably never interact with it directly

Generic usage

``` shell,script(expected_exit_code=0)
mit-commit-msg --help
```

``` shell,verify(stream=stdout)
Validate the commit message that a user has input

Usage: mit-commit-msg [OPTIONS] [COMMIT_FILE_PATH]

Arguments:
  [COMMIT_FILE_PATH]  Path to a temporary file that contains the commit message written by the
                      developer

Options:
      --copy-message-to-clipboard  On lint failure copy the message to clipboard [env:
                                   GIT_MIT_COPY_MESSAGE_TO_CLIPBOARD=]
      --completion <COMPLETION>    [possible values: bash, elvish, fish, powershell, zsh]
  -h, --help                       Print help
  -V, --version                    Print version
```

You can generate completion with

``` shell,script(expected_exit_code=0)
mit-commit-msg --completion bash
```

Otherwise you need an the commit file path

``` shell,script(expected_exit_code=2)
mit-commit-msg
```

``` shell,verify(stream=stderr)
error: the following required arguments were not provided:
  <COMMIT_FILE_PATH>

Usage: mit-commit-msg <COMMIT_FILE_PATH>

For more information, try '--help'.
```

