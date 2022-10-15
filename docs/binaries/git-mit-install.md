# git-mit-install

Generic usage

``` shell,script(expected_exit_code=0)
git-mit-install --help
```

``` shell,verify(stream=stdout)
git-mit-install 5.12.94
Billie Thompson <billie+git-mit-install@billiecodes.com>
Install git-mit into a repository

USAGE:
    git-mit-install [OPTIONS]

OPTIONS:
        --completion <COMPLETION>    [possible values: bash, elvish, fish, powershell, zsh]
    -h, --help                       Print help information
    -s, --scope <SCOPE>              [default: local] [possible values: global, local]
    -V, --version                    Print version information
```

You can generate completion with

``` shell,script(expected_exit_code=0)
git-mit-install --completion bash
```
