# mit-prepare-commit-msg

> Note: This is a hook binary, you will probably never interact with it
> directly

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
          
          When omitted the hook falls back to `<gitdir>/COMMIT_EDITMSG`, which is useful when the
          hook is invoked via a hook manager like lefthook that does not forward git's positional
          argument.

  [COMMIT_MESSAGE_SOURCE]
          The commit message, and can be: message (if a -m or -F option was given to git); template
          (if a -t option was given or the configuration option commit.template is set in git);
          merge (if the commit is a merge or a `.git/MERGE_MSG` file exists); squash (if a
          `.git/SQUASH_MSG` file exists); or commit

  [COMMIT_SHA]
          Commit SHA-1 (if a -c, -C or --amend option was given to git)

Options:
      --relates-to-exec <RELATES_TO_EXEC>
          A command to execute to get the value for the "relates to" trailer
          
          [env: GIT_MIT_RELATES_TO_EXEC=]

      --relates-to-template <RELATES_TO_TEMPLATE>
          A template to apply to the "relates to" trailer
          
          [env: GIT_MIT_RELATES_TO_TEMPLATE=]

      --non-clean-behaviour-option <NON_CLEAN_BEHAVIOUR_OPTION>
          What to do when we rebase

          Possible values:
          - add-to:    Change the commit message to include the current author
          - no-change: Do not change the commit message
          
          [env: GIT_MIT_SET_NON_CLEAN_BEHAVIOUR=]

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

When no commit message path is provided — for example when a hook manager
like [lefthook](https://github.com/evilmartians/lefthook) invokes the hook
without forwarding git's positional argument — the hook falls back to
reading from `.git/COMMIT_EDITMSG`. See the
[lefthook integration guide](../lefthook.md) for setup instructions.

``` shell,script(name="no-argument-fallback",expected_exit_code=0)
git init --quiet .
git mit bt se
printf 'Add a feature\n' > .git/COMMIT_EDITMSG
mit-prepare-commit-msg
cat .git/COMMIT_EDITMSG
```

``` text,verify(script_name="no-argument-fallback",stream=stdout)
Add a feature


Co-authored-by: Someone Else <someone@example.com>
```
