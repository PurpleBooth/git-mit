# git-mit-relates-to

Generic usage

``` shell,script(name="show-help",expected_exit_code=0)
git-mit-relates-to --help
```

``` shell,verify(script_name="show-help",stream=stdout)
Set Relates-to trailer.

Usage: git-mit-relates-to [OPTIONS] [ISSUE_NUMBER]

Arguments:
  [ISSUE_NUMBER]  The issue number or other string to place into the Relates-to trailer

Options:
  -t, --timeout <TIMEOUT>        Number of minutes to expire the configuration in [env:
                                 GIT_MIT_RELATES_TO_TIMEOUT=] [default: 60]
      --completion <COMPLETION>  [possible values: bash, elvish, fish, powershell, zsh]
  -h, --help                     Print help
  -V, --version                  Print version
```

You can generate completion with

``` shell,script(name="generate-bash-completion",expected_exit_code=0)
git-mit-relates-to --completion bash
```

Otherwise you need an issue number

``` shell,script(name="missing-issue-number-error",expected_exit_code=2)
git-mit-relates-to
```

``` shell,verify(script_name="missing-issue-number-error",stream=stderr)
error: the following required arguments were not provided:
  <ISSUE_NUMBER>

Usage: git-mit-relates-to <ISSUE_NUMBER>

For more information, try '--help'.
```
