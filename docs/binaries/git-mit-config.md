# git-mit-config

Generic usage

``` shell,script(expected_exit_code=0)
git-mit-config --help
```

``` shell,verify(stream=stdout)
git-mit-config 5.12.32
Billie Thompson <billie+git-mit-config@billiecodes.com>
A command for enabling and disabling git lints

USAGE:
    git-mit-config [OPTIONS] [SUBCOMMAND]

OPTIONS:
        --completion <completion>    [possible values: bash, elvish, fish, powershell, zsh]
    -h, --help                       Print help information
    -V, --version                    Print version information

SUBCOMMANDS:
    help          Print this message or the help of the given subcommand(s)
    lint          Manage active lints
    mit           Manage mit configuration
    relates-to    Manage relates-to settings
```

``` shell,script(expected_exit_code=0)
git-mit-config lint --help
```

``` shell,verify(stream=stdout)
git-mit-config-lint 
Manage active lints

USAGE:
    git-mit-config lint <SUBCOMMAND>

OPTIONS:
    -h, --help    Print help information

SUBCOMMANDS:
    available    List the available lints
    disable      Disable a lint
    enable       Enable a lint
    enabled      List the enabled lints
    generate     Generate the config file for your current settings
    help         Print this message or the help of the given subcommand(s)
    status       Get status of a lint
```

``` shell,script(expected_exit_code=0)
git-mit-config mit --help
```

``` shell,verify(stream=stdout)
git-mit-config-mit 
Manage mit configuration

USAGE:
    git-mit-config mit <SUBCOMMAND>

OPTIONS:
    -h, --help    Print help information

SUBCOMMANDS:
    available    List available authors
    example      Print example mit toml file
    generate     Generate a file version of available authors
    help         Print this message or the help of the given subcommand(s)
    set          Update or add an initial in the mit configuration
```

``` shell,script(expected_exit_code=0)
git-mit-config relates-to --help
```

``` shell,verify(stream=stdout)
git-mit-config-relates-to 
Manage relates-to settings

USAGE:
    git-mit-config relates-to <SUBCOMMAND>

OPTIONS:
    -h, --help    Print help information

SUBCOMMANDS:
    help        Print this message or the help of the given subcommand(s)
    template    Use a template for the relates-to trailer
```

You can generate completion with

``` shell,script(expected_exit_code=0)
git-mit-config --completion bash
```

Otherwise an error will be generated

``` shell,script(expected_exit_code=1)
git-mit-config
```
``` shell,verify(stream=stderr)
Error: git_mit_config::errors::unrecognised_lint_command

  × unrecognised subcommand
  help: try `git mit-config --help`

```
