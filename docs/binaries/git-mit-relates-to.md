# git-mit-relates-to

Generic usage

``` shell,script(expected_exit_code=0)
git-mit-relates-to --help
```

``` shell,verify(stream=stdout)
git-mit-relates-to 5.12.95
Billie Thompson <billie+git-mit-relates-to@billiecodes.com>
Set Relates-to trailer.

USAGE:
    git-mit-relates-to [OPTIONS] [ISSUE_NUMBER]

ARGS:
    <ISSUE_NUMBER>    The issue number or other string to place into the Relates-to trailer

OPTIONS:
        --completion <COMPLETION>    [possible values: bash, elvish, fish, powershell, zsh]
    -h, --help                       Print help information
    -t, --timeout <TIMEOUT>          Number of minutes to expire the configuration in [env:
                                     GIT_MIT_RELATES_TO_TIMEOUT=] [default: 60]
    -V, --version                    Print version information
```

You can generate completion with

``` shell,script(expected_exit_code=0)
git-mit-relates-to --completion bash
```

Otherwise you need an issue number

``` shell,script(expected_exit_code=2)
git-mit-relates-to
```

``` shell,verify(stream=stderr)
error: The following required arguments were not provided:
    <ISSUE_NUMBER>

USAGE:
    git-mit-relates-to <ISSUE_NUMBER>

For more information try --help
```
