# mit-commit-msg

> Note: This is a hook binary, you will probably never interact with it directly

Generic usage

``` shell,script(expected_exit_code=0)
mit-commit-msg --help
```

``` shell,verify(stream=stdout)
mit-commit-msg 5.12.95
Billie Thompson <billie+mit-commit-msg@billiecodes.com>
Validate the commit message that a user has input

USAGE:
    mit-commit-msg [OPTIONS] [COMMIT_FILE_PATH]

ARGS:
    <COMMIT_FILE_PATH>    Path to a temporary file that contains the commit message written by
                          the developer

OPTIONS:
        --completion <COMPLETION>
            [possible values: bash, elvish, fish, powershell, zsh]

        --copy-message-to-clipboard <copy-message-to-clipboard>
            On lint failure copy the message to clipboard [env: GIT_MIT_COPY_MESSAGE_TO_CLIPBOARD=]
            [default: true]

    -h, --help
            Print help information

    -V, --version
            Print version information
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
error: The following required arguments were not provided:
    <COMMIT_FILE_PATH>

USAGE:
    mit-commit-msg <COMMIT_FILE_PATH>

For more information try --help
```

