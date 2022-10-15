# mit-prepare-commit-msg

> Note: This is a hook binary, you will probably never interact with it directly

Generic usage

``` shell,script(expected_exit_code=0)
mit-prepare-commit-msg --help
```

``` shell,verify(stream=stdout)
mit-prepare-commit-msg 5.12.93
Billie Thompson <billie+mit-prepare-commit-msg@billiecodes.com>
This hook is invoked by git-commit right after preparing the default log message, and before the
editor is started.

USAGE:
    mit-prepare-commit-msg [OPTIONS] [ARGS]

ARGS:
    <COMMIT_MESSAGE_PATH>      The name of the file that contains the commit log message
    <COMMIT_MESSAGE_SOURCE>    The commit message, and can be: message (if a -m or -F option
                               was given to git); template (if a -t option was given or the
                               configuration option commit.template is set in git); merge (if
                               the commit is a merge or a .git/MERGE_MSG file exists); squash
                               (if a .git/SQUASH_MSG file exists); or commit
    <COMMIT_SHA>               Commit SHA-1 (if a -c, -C or --amend option was given to git)

OPTIONS:
        --completion <COMPLETION>
            [possible values: bash, elvish, fish, powershell, zsh]

    -h, --help
            Print help information

        --relates-to-exec <RELATES_TO_EXEC>
            A command to execute to get the value for the relates to trailer [env:
            GIT_MIT_RELATES_TO_EXEC=]

        --relates-to-template <RELATES_TO_TEMPLATE>
            A template to apply to the relates to trailer [env: GIT_MIT_RELATES_TO_TEMPLATE=]

    -V, --version
            Print version information
```

You can generate completion with

``` shell,script(expected_exit_code=0)
mit-prepare-commit-msg --completion bash
```

Otherwise you need an commit message path

``` shell,script(expected_exit_code=2)
mit-prepare-commit-msg
```

``` shell,verify(stream=stderr)
error: The following required arguments were not provided:
    <COMMIT_MESSAGE_PATH>

USAGE:
    mit-prepare-commit-msg <COMMIT_MESSAGE_PATH>

For more information try --help
```


