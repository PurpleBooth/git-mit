# Lint Configuration Not Working

If lints aren't being applied as expected:

## Debugging Lint Configuration

1.  **Check current lint status**:

    ``` bash,skip()
    git mit-config lint status <lint-name>
    ```

2.  **Check for TOML config files**:

    ``` bash,skip()
    ls -la .git-mit.toml*
    ```

3.  **View Git config lint settings**:

    ``` bash,skip()
    git config --list | grep "^mit.lint"
    ```

4.  **Generate current configuration**:

    ``` bash,skip()
    git mit-config lint generate
    ```

    This shows the effective configuration from all sources.

5.  **Warning about TOML override**: When using
    `git mit-config lint enable/disable`, if a TOML file exists, you'll
    see:

        Warning: your config is overridden by a repository config file

    This reminds you that the TOML file takes precedence over Git
    config.
