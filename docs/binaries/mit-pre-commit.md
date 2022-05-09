# mit-pre-commit

> Note: This is a hook binary, you will probably never interact with it directly

Generic usage

``` shell,script(expected_exit_code=0)
mit-pre-commit --help
```

``` shell,verify(stream=stdout)
mit-pre-commit 5.12.59
Billie Thompson <billie+mit-pre-commit@billiecodes.com>
Run first, before you even type in a commit message. It's used to inspect the snapshot that's about
to be committed.

USAGE:
    mit-pre-commit [OPTIONS]

OPTIONS:
        --completion <completion>    [possible values: bash, elvish, fish, powershell, zsh]
    -h, --help                       Print help information
    -V, --version                    Print version information
```

You can generate completion with

``` shell,script(expected_exit_code=0)
mit-pre-commit --completion bash
```

Otherwise you need to have configured some authors

``` shell,script(expected_exit_code=1)
mit-pre-commit
```

``` shell,verify(stream=stderr)
Error: mit_pre_commit::errors::stale_author_error

  × No authors set
  help: Can you set who's currently coding? It's nice to get and give
        the right credit. You can fix this by running `git mit` then the
        initials of whoever is coding for example: `git mit bt` or `git mit
        bt se`

```

