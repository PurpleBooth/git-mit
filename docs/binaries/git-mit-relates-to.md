# git-mit-relates-to

Generic usage

``` shell,script(expected_exit_code=0)
git-mit-relates-to --help
```

``` shell,verify(stream=stdout)
git-mit-relates-to 5.12.68
Billie Thompson <billie+git-mit-relates-to@billiecodes.com>
Set Relates-to trailer.

USAGE:
    git-mit-relates-to [OPTIONS] [issue-number]

ARGS:
    <issue-number>    The issue number or other string to place into the Relates-to trailer

OPTIONS:
        --completion <completion>    [possible values: bash, elvish, fish, powershell, zsh]
    -h, --help                       Print help information
    -t, --timeout <timeout>          Number of minutes to expire the configuration in [env:
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
    <issue-number>

USAGE:
    git-mit-relates-to <issue-number>

For more information try --help
```
