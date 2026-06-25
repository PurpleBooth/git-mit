# Hooks and `git mit-install` command

If co-authors aren't being added to commits or commit message validation
isn't working:

## Verify Hook Installation

1.  **Check if hooks are installed in your repository**:

    ``` bash,skip()
    ls -la .git/hooks/ | grep mit
    ```

    You should see git-mit's hooks for:

    - `commit-msg`
    - `pre-commit`
    - `prepare-commit-msg`

    On Unix these are symlinks to the mit binaries; on Windows they are small
    `#!/bin/sh` wrappers that invoke them.

2.  **For global installations, check the template directory**:

    ``` bash,skip()
    git config --global init.templatedir
    ```

    If this returns a path, check that directory contains the hooks:

    ``` bash,skip()
    ls -la $(git config --global init.templatedir)/hooks/
    ```

3.  **Verify hooks are executable**:

    ``` bash,skip()
    ls -la .git/hooks/ | grep mit
    ```

    Look for `x` in the permissions (e.g., `-rwxr-xr-x`)

## Reinstall Hooks

If hooks are missing or not working:

1.  **For local installation** (installs in current repository only):

    ``` bash,skip()
    git mit-install
    ```

2.  **For global installation** (affects all new repositories):

    ``` bash,skip()
    git mit-install --scope=global
    ```

    Then reinitialize existing repositories:

    ``` bash,skip()
    git init
    ```

See the [installation documentation](../binaries/git-mit-install.md) for
more details, or the [uninstall documentation](../binaries/git-mit-install-uninstall.md)
to remove git-mit hooks.

## Test Hook Execution

1.  **Test the pre-commit hook**:

    ``` bash,skip()
    # This should fail if no authors are set
    .git/hooks/pre-commit
    ```

    See [`mit-pre-commit`](../binaries/mit-pre-commit.md) for expected
    behavior.

2.  **Test commit message validation**:

    ``` bash,skip()
    echo "test" > test-file.txt
    git add test-file.txt
    git commit -m "this is a commit message that is way too long and should definitely fail the 72 character limit check"
    ```

    This should fail with an error about the subject being too long. See
    [`mit-commit-msg`](../binaries/mit-commit-msg.md) for details.
