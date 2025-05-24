# git-mit-config

Generic usage

``` shell,script(name="show-main-help",expected_exit_code=0)
git-mit-config --help
```

``` shell,verify(script_name="show-main-help",stream=stdout)
A command for enabling and disabling git lints

Usage: git-mit-config [OPTIONS] [COMMAND]

Commands:
  lint        Manage active lints
  mit         Manage mit configuration
  relates-to  Manage relates-to settings
  help        Print this message or the help of the given subcommand(s)

Options:
      --completion <COMPLETION>  [possible values: bash, elvish, fish, powershell, zsh]
  -h, --help                     Print help
  -V, --version                  Print version
```

``` shell,script(name="show-lint-help",expected_exit_code=0)
git-mit-config lint --help
```

``` shell,verify(script_name="show-lint-help",stream=stdout)
Manage active lints

Usage: git-mit-config lint <COMMAND>

Commands:
  generate   Generate the config file for your current settings
  available  List the available lints
  enabled    List the enabled lints
  status     Get the status of a lint
  enable     Enable a lint
  disable    Disable a lint
  help       Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

``` shell,script(name="show-mit-help",expected_exit_code=0)
git-mit-config mit --help
```

``` shell,verify(script_name="show-mit-help",stream=stdout)
Manage mit configuration

Usage: git-mit-config mit <COMMAND>

Commands:
  set                      Update or add an initial in the mit configuration
  non-clean-behaviour      Get the current behavior when the repository is mid-rebase or merge
  set-non-clean-behaviour  Set the current behavior when the repository is mid-rebase or merge
  generate                 Generate a file version of available authors
  available                List available authors
  example                  Print example mit toml file
  help                     Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

``` shell,script(name="show-relates-to-help",expected_exit_code=0)
git-mit-config relates-to --help
```

``` shell,verify(script_name="show-relates-to-help",stream=stdout)
Manage relates-to settings

Usage: git-mit-config relates-to <COMMAND>

Commands:
  template  Use a template for the relates-to trailer
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

You can generate completion with

``` shell,script(name="generate-bash-completion",expected_exit_code=0)
git-mit-config --completion bash
```

Otherwise an error will be generated

``` shell,script(name="missing-subcommand-error",expected_exit_code=1)
git-mit-config
```
``` shell,verify(script_name="missing-subcommand-error",stream=stderr)
Error: git_mit_config::errors::unrecognised_lint_command

  × unrecognised subcommand
  help: try `git mit-config --help`

```
