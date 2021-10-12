# git-mit-install

Generic usage

``` shell,script(expected_exit_code=0)
git-mit-install --help
```

``` shell,verify(stream=stdout)
git-mit-install 5.10.3

Billie Thompson <billie+git-mit-install@billiecodes.com>

Install git-mit into a repository

USAGE:
    git-mit-install [OPTIONS]

FLAGS:
    -h, --help       Print help information
    -V, --version    Print version information

OPTIONS:
        --completion <completion>    [possible values: bash, elvish, fish, powershell, zsh]
    -s, --scope <scope>              [default: local] [possible values: local, global]
```

You can generate completion with

``` shell,script(expected_exit_code=0)
git-mit-install --completion bash
```
