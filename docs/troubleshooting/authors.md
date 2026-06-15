# Authors and the `git mit` command

If you're having issues with author configuration:

## Troubleshooting Steps

1.  **View all configured authors**:

    ``` bash,skip()
    git mit-config mit available
    ```

    This shows all authors available to git-mit. See the [authors
    documentation](../mit.md#configuring) for more details.

2.  **Check where your authors file is located**:

    ``` bash,skip()
    echo $GIT_MIT_AUTHORS_CONFIG
    ```

    If not set, git-mit uses the default location:
    `$HOME/.config/git-mit/mit.toml` (or `%APPDATA%\git-mit\mit.toml` on
    Windows)

3.  **Verify your authors file exists**:

    ``` bash,skip()
    cat $HOME/.config/git-mit/mit.toml
    # Or if you have a custom location:
    cat $GIT_MIT_AUTHORS_CONFIG
    ```

4.  **Check Git's stored author configuration**:

    ``` bash,skip()
    git config --list | grep "^mit.author"
    ```

    This shows authors stored in Git's config. Example output:

        mit.author.config.bt.email=billie@example.com
        mit.author.config.bt.name=Billie Thompson
        mit.author.config.bt.signingkey=0A46826A
        mit.author.expires=1747556921

## Extending Author Configuration Timeout

By default, author configuration expires after 60 minutes. If you find
this timeout too short, you can extend it:

1.  **Using command-line flag**:

    ``` bash,skip()
    git mit --timeout 480 bt se
    ```

    This sets the timeout to 480 minutes (8 hours).

2.  **Using environment variable**:

    ``` bash,skip()
    export GIT_MIT_AUTHORS_TIMEOUT=480
    git mit bt se
    ```

    Add this to your shell profile (`.bashrc`, `.zshrc`, etc.) to make
    it permanent.

3.  **Check current expiration time**:

    ``` bash,skip()
    git config mit.author.expires
    ```

    This shows the Unix timestamp when the current configuration
    expires.
