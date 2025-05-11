# mit-prepare-commit-msg

> Note: This is a hook binary, you will probably never interact with it directly

Generic usage

``` shell,script(name="show-help",expected_exit_code=0)
mit-prepare-commit-msg --help
```

``` shell,verify(script_name="show-help",stream=stdout)
This hook is invoked by git-commit right after preparing the default log message, and before the
editor is started.

Usage: mit-prepare-commit-msg [OPTIONS] [COMMIT_MESSAGE_PATH] [COMMIT_MESSAGE_SOURCE] [COMMIT_SHA]

Arguments:
  [COMMIT_MESSAGE_PATH]
          The name of the file that contains the commit log message

  [COMMIT_MESSAGE_SOURCE]
          The commit message, and can be: message (if a -m or -F option was given to git); template
          (if a -t option was given or the configuration option commit.template is set in git);
          merge (if the commit is a merge or a `.git/MERGE_MSG` file exists); squash (if a
          `.git/SQUASH_MSG` file exists); or commit

  [COMMIT_SHA]
          Commit SHA-1 (if a -c, -C or --amend option was given to git)

Options:
      --relates-to-exec <RELATES_TO_EXEC>
          A command to execute to get the value for the relates to trailer
          
          [env: GIT_MIT_RELATES_TO_EXEC=]

      --relates-to-template <RELATES_TO_TEMPLATE>
          A template to apply to the relates to trailer
          
          [env: GIT_MIT_RELATES_TO_TEMPLATE=]

      --non-clean-behaviour-option <NON_CLEAN_BEHAVIOUR_OPTION>
          A template to apply to the relates to trailer
          
          [env: GIT_MIT_SET_NON_CLEAN_BEHAVIOUR=]

          Possible values:
          - add-to:    Change the commit message to include the current author
          - no-change: Do not change the commit message

      --completion <COMPLETION>
          [possible values: bash, elvish, fish, powershell, zsh]

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

You can generate completion with
``` shell,script(name="generate-mit-prepare-commit-msg-completion",expected_exit_code=0)
mit-prepare-commit-msg --completion bash
```

Otherwise you need an commit message path

``` shell,script(name="missing-commit-path-error",expected_exit_code=2)
mit-prepare-commit-msg
```

``` shell,verify(script_name="missing-commit-path-error",stream=stderr)
error: the following required arguments were not provided:
  <COMMIT_MESSAGE_PATH>

Usage: mit-prepare-commit-msg <COMMIT_MESSAGE_PATH> [COMMIT_MESSAGE_SOURCE] [COMMIT_SHA]

For more information, try '--help'.
```


