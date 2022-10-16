# git-mit-install

Generic usage

``` shell,script(expected_exit_code=0)
git-mit-install --help
```

``` shell,verify(stream=stdout)
Install git-mit into a repository

Usage: git-mit-install [OPTIONS]

Options:
  -s, --scope <SCOPE>
          [default: local]

          Possible values:
          - global: The home directory
          - local:  The local folder

      --completion <COMPLETION>
          [possible values: bash, elvish, fish, powershell, zsh]

  -h, --help
          Print help information (use `-h` for a summary)

  -V, --version
          Print version information
```

You can generate completion with

``` shell,script(expected_exit_code=0)
git-mit-install --completion bash
```
