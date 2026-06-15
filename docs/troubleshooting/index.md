# Troubleshooting git-mit

This guide will help you diagnose and fix common issues with git-mit.

- [Common error messages](error-messages.md) — solutions for frequently
  encountered error messages
- [Authors and the `git mit` command](authors.md) — issues with author
  configuration
- [Hooks and `git mit-install`](hooks.md) — co-authors missing or
  commit validation not working
- [Lint configuration](lint-configuration.md) — lints not being applied
  as expected
- [Where git-mit saves things](storage.md) — configuration storage
  locations and loading precedence

## Still Having Issues?

If you're still experiencing problems:

1.  **Check for error messages**: Run git commands with verbose output
2.  **Verify Git version compatibility**: Ensure you're using a recent
    version of Git
3.  **Check for conflicting hook managers**: Other tools like Husky or
    pre-commit might interfere
4.  **File an issue**: Visit the [git-mit
    repository](https://github.com/PurpleBooth/git-mit) with details
    about your setup

For more help, consult:

- [Installation guide](../binaries/git-mit-install.md)
- [Authors configuration](../mit.md)
- [Lint configuration](../lints/configuring.md)
- [Relates-to configuration](../mit-relates-to.md)
- [Hook documentation](../binaries/mit-pre-commit.md)
