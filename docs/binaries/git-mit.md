# git-mit

Generic usage

``` shell,script(name="show-help",expected_exit_code=0)
git-mit --help
```

``` shell,verify(script_name="show-help",stream=stdout)
Set author and Co-authored trailer.

Usage: git-mit [OPTIONS] [INITIALS]...

Arguments:
  [INITIALS]...  Initials of the mit to put in the commit

Options:
  -c, --config <CONFIG>          Path to a file where mit initials, emails and names can be found
                                 [env: GIT_MIT_AUTHORS_CONFIG=] [default:
                                 $HOME/.config/git-mit/mit.toml]
  -e, --exec <EXEC>              Execute a command to generate the mit configuration, stdout will be
                                 captured and used instead of the file, if both this and the file
                                 are present, this takes precedence [env: GIT_MIT_AUTHORS_EXEC=]
  -t, --timeout <TIMEOUT>        Number of minutes to expire the configuration in [env:
                                 GIT_MIT_AUTHORS_TIMEOUT=] [default: 60]
      --completion <COMPLETION>  Shell to generate completions for [possible values: bash, elvish,
                                 fish, powershell, zsh]
  -h, --help                     Print help
  -V, --version                  Print version

COMMON TASKS:
    You can install git-mit into a new repository using

        git mit-install

    You can add a new author to that repository by running

        git mit-config mit set eg "Egg Sample" egg.sample@example.com

    You can save that author permanently by running

        git mit-config mit set eg "Egg Sample" egg.sample@example.com
        git mit-config mit generate > $HOME/.config/git-mit/mit.toml

    You can disable a lint by running

        git mit-config lint disable jira-issue-key-missing

    You can install the example authors file to the default location with

        git mit-config mit example > $HOME/.config/git-mit/mit.toml

    You can set the current author, and Co-authors by running

        git mit ae se

    You can populate the `Relates-to` trailer using

        git mit-relates-to "[#12345678]"
```

You can generate completion with

``` shell,script(name="generate-bash-completion",expected_exit_code=0)
git-mit --completion bash
```

Otherwise, you must provide an author initial

``` shell,script(name="missing-initials-error",expected_exit_code=2)
git-mit
```

``` shell,verify(script_name="missing-initials-error",stream=stderr)
error: the following required arguments were not provided:
  <INITIALS>...

Usage: git-mit <INITIALS>...

For more information, try '--help'.
```
