# git-mit-install

Generic usage

``` shell,script(expected_exit_code=0)
git-mit-install --help
```

``` shell,verify(stream=stdout)
git-mit-install 5.12.38
Billie Thompson <billie+git-mit-install@billiecodes.com>
Install git-mit into a repository

USAGE:
    git-mit-install [OPTIONS]

OPTIONS:
        --completion <completion>    [possible values: bash, elvish, fish, powershell, zsh]
    -h, --help                       Print help information
    -s, --scope <scope>              [default: local] [possible values: local, global]
    -V, --version                    Print version information
```

You can generate completion with

``` shell,script(expected_exit_code=0)
git-mit-install --completion bash
```
