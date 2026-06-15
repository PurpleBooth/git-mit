# Common Error Messages

Here are solutions for frequently encountered error messages:

## "failed to interact with git repository"

This usually means git-mit can't find or read the Git repository. Check:

- You're in a Git repository (`git status` should work)
- The `.git` directory has proper permissions
- Your Git installation is working correctly

## "could not parse author configuration"

Your authors file has invalid TOML or YAML syntax. The error will show
which format failed:

- Check for missing quotes around strings
- Ensure proper indentation (YAML is indent-sensitive)
- Validate your file with a TOML/YAML validator
- The error message will indicate whether it failed parsing as TOML or
  YAML

## "failed to install hook"

The hook installation failed because:

- **ExistingHook**: A hook already exists at that location (a regular file,
  or a symlink/wrapper that is not git-mit's)
- You don't have write permissions to the hooks directory

Remove the existing hook and run `git mit-install` again.

## "No authors set" (from pre-commit hook)

This error appears when:

- You haven't run `git mit <initials>` to set the current authors
- The author configuration has expired (check
  `git config mit.author.expires`)
- The author configuration was never set in this repository

## "The details of the author of this commit are stale"

This error shows:

- The exact time when the author configuration expired
- You need to run `git mit <initials>` again to refresh the
  configuration
