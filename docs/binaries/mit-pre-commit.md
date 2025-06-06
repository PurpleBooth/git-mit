# mit-pre-commit

> Note: This is a hook binary, you will probably never interact with it
> directly

Generic usage

``` shell,script(name="show-help",expected_exit_code=0)
mit-pre-commit --help
```

``` shell,verify(script_name="show-help",stream=stdout)
Run first, before you even type in a commit message. It's used to inspect the snapshot that's about
to be committed.

Usage: mit-pre-commit [OPTIONS]

Options:
      --completion <COMPLETION>  [possible values: bash, elvish, fish, powershell, zsh]
  -h, --help                     Print help
  -V, --version                  Print version
```

You can generate completion with

``` shell,script(name="generate-bash-completion",expected_exit_code=0)
mit-pre-commit --completion bash
```

Otherwise, you need to have configured some authors

``` shell,script(name="no-authors-configured-error",expected_exit_code=1)
mit-pre-commit
```

``` shell,verify(script_name="no-authors-configured-error",stream=stderr)
Error: mit_pre_commit::errors::stale_author_error

  × No authors set
  help: Can you set who's currently coding? It's nice to get and give the
        right credit. You can fix this by running `git mit` then the initials
        of whoever is coding for example: `git mit bt` or `git mit bt se`

```
