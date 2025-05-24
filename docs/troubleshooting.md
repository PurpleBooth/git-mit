# Troubleshooting git-mit

This guide will help you diagnose and fix common issues with git-mit.

## Authors and the `git mit` command

Something is up with authors, author discovery or similar 

### Steps

1. **View all configured authors**:
   ```bash,skip()
   git mit-config mit available
   ```
   This shows all authors available to git-mit. See the [authors documentation](mit.md#configuring) for more details.

2. **Check where your authors file is located**:
   ```bash,skip()
   echo $GIT_MIT_AUTHORS_CONFIG
   ```
   If not set, git-mit uses the default location: `$HOME/.config/git-mit/mit.toml` (or `%APPDATA%\git-mit\mit.toml` on Windows)

3. **Verify your authors file exists**:
   ```bash,skip()
   cat $HOME/.config/git-mit/mit.toml
   # Or if you have a custom location:
   cat $GIT_MIT_AUTHORS_CONFIG
   ```

4. **Check Git's stored author configuration**:
   ```bash,skip()
   git config --list | grep "^mit.author"
   ```
   This shows authors stored in Git's config. Example output:
   ```
   mit.author.config.bt.email=billie@example.com
   mit.author.config.bt.name=Billie Thompson
   mit.author.config.bt.signingkey=0A46826A
   mit.author.expires=1747556921
   ```

### Technical Details

git-mit loads authors from multiple sources and merges them together:

1. **From exec command** (highest priority):
   - Set via `--exec` flag or `GIT_MIT_AUTHORS_EXEC` environment variable
   - The command output must be valid TOML or YAML format
   - The command is executed using `shell_words::split()` for proper argument parsing

2. **From authors file**:
   - Default location: `$HOME/.config/git-mit/mit.toml`
   - Custom location via `--config` flag or `GIT_MIT_AUTHORS_CONFIG` environment variable
   - Special handling for `$HOME/.config/git-mit/mit.toml` path - it's expanded to the actual home directory
   - If the file doesn't exist, an empty string is used (no error)

3. **From Git config**:
   - Authors stored in `mit.author.config.<initial>.*` entries
   - These are set when you run `git mit-config mit set <initial> <name> <email>`
   - Stored at repository level (local) or user level (global) based on `--scope`

The authors from these sources are **merged together**, with later sources overriding earlier ones if there are conflicts.

For more information on configuring authors, see the [authors documentation](mit.md).

### Extending Author Configuration Timeout

By default, author configuration expires after 60 minutes, requiring you to run `git mit <initials>` again. If you find this timeout too short, you can extend it.

#### How the Timeout Works

- The timeout is stored as a Unix timestamp in `mit.author.expires`
- When you run `git mit <initials>`, it sets the expiration to current time + timeout duration (in minutes)
- The `pre-commit` hook checks if the current time is past this expiration
- If expired, you'll see an error with the expiration time and need to run `git mit` again

Note: Setting very long timeouts reduces the benefit of the expiration feature, which ensures commit attribution stays current when developers change throughout the day.

#### Setting a Longer Timeout

1. **Using command-line flag**:
   ```bash,skip()
   git mit --timeout 480 bt se
   ```
   This sets the timeout to 480 minutes (8 hours).

2. **Using environment variable**:
   ```bash,skip()
   export GIT_MIT_AUTHORS_TIMEOUT=480
   git mit bt se
   ```
   Add this to your shell profile (`.bashrc`, `.zshrc`, etc.) to make it permanent.

3. **Check current expiration time**:
   ```bash,skip()
   git config mit.author.expires
   ```
   This shows the Unix timestamp when the current configuration expires.

## Hooks and `git mit-install` command

If co-authors aren't being added to commits or commit message validation isn't working:

### Verify Hook Installation

1. **Check if hooks are installed in your repository**:
   ```bash,skip()
   ls -la .git/hooks/ | grep mit
   ```
   You should see symbolic links for:
    - `commit-msg` → mit-commit-msg (or mit-commit-msg.exe on Windows)
    - `pre-commit` → mit-pre-commit (or mit-pre-commit.exe on Windows)
    - `prepare-commit-msg` → mit-prepare-commit-msg (or mit-prepare-commit-msg.exe on Windows)

2. **For global installations, check the template directory**:
   ```bash,skip()
   git config --global init.templatedir
   ```
   If this returns a path, check that directory contains the hooks:
   ```bash,skip()
   ls -la $(git config --global init.templatedir)/hooks/
   ```

3. **Verify hooks are executable**:
   ```bash,skip()
   ls -la .git/hooks/ | grep mit
   ```
   Look for `x` in the permissions (e.g., `-rwxr-xr-x`)

### Hook Installation Details

The installation process creates symbolic links to the git-mit binaries:

- **Windows**: Creates file symbolic links with `.exe` extension
- **Unix/Linux/macOS**: Creates standard symbolic links
- **Existing hooks**: Installation will fail if hooks already exist (unless they're already symlinks to git-mit)
- **Symlink validation**: If a symlink exists but points to the correct binary, installation succeeds silently

### Reinstall Hooks

If hooks are missing or not working:

1. **For local installation** (installs in current repository only):
   ```bash,skip()
   git mit-install
   ```

2. **For global installation** (affects all new repositories):
   ```bash,skip()
   git mit-install --scope=global
   ```
   Then reinitialize existing repositories:
   ```bash,skip()
   git init
   ```

See the [installation documentation](binaries/git-mit-install.md) for more details.

### Test Hook Execution

1. **Test the pre-commit hook**:
   ```bash,skip()
   # This should fail if no authors are set
   .git/hooks/pre-commit
   ```
   See [`mit-pre-commit`](binaries/mit-pre-commit.md) for expected behavior.

2. **Test commit message validation**:
   ```bash,skip()
   echo "test" > test-file.txt
   git add test-file.txt
   git commit -m "this is a commit message that is way too long and should definitely fail the 72 character limit check"
   ```
   This should fail with an error about the subject being too long. See [`mit-commit-msg`](binaries/mit-commit-msg.md) for details.

## Lint Configuration Not Working

If lints aren't being applied as expected:

### Understanding Lint Configuration Precedence

Lints are loaded from multiple sources with TOML files taking precedence:

1. **TOML configuration files** (highest priority):
   - `.git-mit.toml` (takes precedence if exists)
   - `.git-mit.toml.dist` (used if `.git-mit.toml` doesn't exist)
   - Located by discovering the Git repository and checking the parent of `.git` directory
   - For bare repositories, checks the repository directory itself

2. **VCS configuration** (Git config - used if no TOML config exists):
   - Read from `mit.lint.<lint-name>` entries
   - Can be set with `git mit-config lint enable/disable <lint-name>`
   - Uses the lint's default enabled state if not explicitly configured

The TOML configuration uses this format:
```toml,skip()
[mit.lint]
"lint-name" = true  # or false
```

### Debugging Lint Configuration

1. **Check current lint status**:
   ```bash,skip()
   git mit-config lint status <lint-name>
   ```

2. **Check for TOML config files**:
   ```bash,skip()
   ls -la .git-mit.toml*
   ```

3. **View Git config lint settings**:
   ```bash,skip()
   git config --list | grep "^mit.lint"
   ```

4. **Generate current configuration**:
   ```bash,skip()
   git mit-config lint generate
   ```
   This shows the effective configuration from all sources.

5. **Warning about TOML override**:
   When using `git mit-config lint enable/disable`, if a TOML file exists, you'll see:
   ```
   Warning: your config is overridden by a repository config file
   ```
   This reminds you that the TOML file takes precedence over Git config.

## Where Does git-mit Save Things?

Understanding where git-mit stores its data can help with troubleshooting:

### Author Configurations

1. **Authors file** (permanent storage):
    - Default location: `$HOME/.config/git-mit/mit.toml` (Linux/macOS) or `%APPDATA%\git-mit\mit.toml` (Windows)
    - Custom location: Set via `GIT_MIT_AUTHORS_CONFIG` environment variable
    - Format: TOML or YAML file containing author details
    - See [authors documentation](mit.md) for file format

2. **Git config** (permanent storage):
    - Stored in: `.git/config` (repository level) or `~/.gitconfig` (global)
    - Keys: `mit.author.config.<initial>.<field>` for author details
    - Set by: `git mit-config mit set [--scope=local|global] <initial> <name> <email>`

3. **Active author state** (temporary):
    - Stored in: `.git/config`
    - Keys: `user.name`, `user.email`, `user.signingkey`, `mit.author.coauthors.*`
    - Expires: Controlled by `mit.author.expires` timestamp
    - Set by: Running `git mit <initials>`

### Lint Configuration

1. **Repository level**:
    - File: `.git/config`
    - Keys: `mit.lint.<lint-name>`
    - Set by: `git mit-config lint enable/disable <lint-name>`

2. **Project level** (shared with team):
    - Files: `.git-mit.toml` or `.git-mit.toml.dist`
    - Example: Setting `not-conventional-commit = true` in `[mit.lint]` section
    - See [lint configuration documentation](lints/configuring.md)

### Relates-to Configuration

- Stored in: `.git/config`
- Keys: `mit.relate.to` and `mit.relate.expires`
- Template: `mit.relate.template` (for formatting the trailer)
- Set by: `git mit-relates-to`
- See [relates-to documentation](mit-relates-to.md)

### Hook Installation Locations

1. **Local installation** (`git mit-install` or `git mit-install --scope=local`):
    - Location: `.git/hooks/` in current repository
    - Creates symbolic links to the actual git-mit binaries

2. **Global installation** (`git mit-install --scope=global`):
    - Sets: `init.templatedir` in global Git config
    - Location: Template directory (e.g., `~/.config/git/init-template/hooks/`)
    - Applied to: New repositories when running `git init` or `git clone`

### git's Precedence of config files

git-mit uses libgit2 for Git interactions, which follows standard Git configuration precedence:

1. Repository config (`.git/config`) - highest priority
2. User config (`~/.gitconfig`)
3. System config (`/etc/gitconfig`) - lowest priority

### How git-mit Discovers git Configuration

The configuration discovery process:

1. **Repository discovery**: Uses `Repository::discover()` to find the Git repository
2. **Config loading**: Opens the repository's config or falls back to [default config](https://libgit2.org/docs/reference/v0.22.1/config/git_config_open_default.html)
3. **Author loading**: Reads from `mit.author.config.*` entries in Git config
4. **Lint settings**: Reads from `mit.lint.*` entries and `.git-mit.toml` files
5. **TOML file discovery**: Looks for `.git-mit.toml` first, then `.git-mit.toml.dist` in repository root

## Common Error Messages

### "failed to interact with git repository"

This usually means git-mit can't find or read the Git repository. Check:
- You're in a Git repository (`git status` should work)
- The `.git` directory has proper permissions
- Your Git installation is working correctly

### "could not parse author configuration"

Your authors file has invalid TOML or YAML syntax. The error will show which format failed:
- Check for missing quotes around strings
- Ensure proper indentation (YAML is indent-sensitive)
- Validate your file with a TOML/YAML validator
- The error message will indicate whether it failed parsing as TOML or YAML

### "failed to install hook"

The hook installation failed because:
- **ExistingHook**: A non-symlink hook already exists at that location
- **ExistingSymlink**: A symlink exists but points to a different location
- You don't have write permissions to the hooks directory

Remove the existing hook or check where the existing symlink points.

### "No authors set" (from pre-commit hook)

This error appears when:
- You haven't run `git mit <initials>` to set the current authors
- The author configuration has expired (check `git config mit.author.expires`)
- The author configuration was never set in this repository

### "The details of the author of this commit are stale"

This error shows:
- The exact time when the author configuration expired
- You need to run `git mit <initials>` again to refresh the configuration

## Still Having Issues?

If you're still experiencing problems:

1. **Check for error messages**: Run git commands with verbose output
2. **Verify Git version compatibility**: Ensure you're using a recent version of Git
3. **Check for conflicting hook managers**: Other tools like Husky or pre-commit might interfere
4. **File an issue**: Visit the [git-mit repository](https://github.com/PurpleBooth/git-mit) with details about your setup

For more help, consult:
- [Installation guide](binaries/git-mit-install.md)
- [Authors configuration](mit.md)
- [Lint configuration](lints/configuring.md)
- [Relates-to configuration](mit-relates-to.md)
- [Hook documentation](binaries/mit-pre-commit.md)
