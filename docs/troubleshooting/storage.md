# Where Does git-mit Save Things?

Understanding where git-mit stores its data can help with
troubleshooting:

## Configuration Storage Locations

### Author Configurations

1.  **Authors file** (permanent storage):

    - Default location: `$HOME/.config/git-mit/mit.toml` (Linux/macOS)
      or `%APPDATA%\git-mit\mit.toml` (Windows)
    - Custom location: Set via `GIT_MIT_AUTHORS_CONFIG` environment
      variable
    - Format: TOML or YAML file containing author details
    - See [authors documentation](../mit.md) for file format

2.  **Git config** (permanent storage):

    - Stored in: `.git/config` (repository level) or `~/.gitconfig`
      (global)
    - Keys: `mit.author.config.<initial>.<field>` for author details
    - Set by:
      `git mit-config mit set [--scope=local|global] <initial> <name> <email>`

3.  **Active author state** (temporary):

    - Stored in: `.git/config`
    - Keys: `user.name`, `user.email`, `user.signingkey`,
      `mit.author.coauthors.*`
    - Expires: Controlled by `mit.author.expires` timestamp
    - Set by: Running `git mit <initials>`

### Lint Configuration

1.  **Repository level**:

    - File: `.git/config`
    - Keys: `mit.lint.<lint-name>`
    - Set by: `git mit-config lint enable/disable <lint-name>`

2.  **Project level** (shared with team):

    - Files: `.git-mit.toml` or `.git-mit.toml.dist`
    - Example: Setting `not-conventional-commit = true` in `[mit.lint]`
      section
    - See [lint configuration documentation](../lints/configuring.md)

### Relates-to Configuration

- Stored in: `.git/config`
- Keys: `mit.relate.to` and `mit.relate.expires`
- Template: `mit.relate.template` (for formatting the trailer)
- Set by: `git mit-relates-to`
- See [relates-to documentation](../mit-relates-to.md)

### Hook Installation Locations

1.  **Local installation** (`git mit-install` or
    `git mit-install --scope=local`):

    - Location: `.git/hooks/` in current repository
    - Creates symbolic links to the actual git-mit binaries

2.  **Global installation** (`git mit-install --scope=global`):

    - Sets: `init.templatedir` in global Git config
    - Location: Template directory (e.g.,
      `~/.config/git/init-template/hooks/`)
    - Applied to: New repositories when running `git init` or
      `git clone`

## Configuration Loading Precedence

### git's Precedence of config files

git-mit uses libgit2 for Git interactions, which follows standard Git
configuration precedence:

1.  Repository config (`.git/config`) - highest priority
2.  User config (`~/.gitconfig`)
3.  System config (`/etc/gitconfig`) - lowest priority

### Lint Configuration Precedence

Lints are loaded from multiple sources with TOML files taking
precedence:

1.  **TOML configuration files** (highest priority):

    - `.git-mit.toml` (takes precedence if exists)
    - `.git-mit.toml.dist` (used if `.git-mit.toml` doesn't exist)
    - Located by discovering the Git repository and checking the parent
      of `.git` directory
    - For bare repositories, checks the repository directory itself

2.  **VCS configuration** (Git config—used if no TOML config exists):

    - Read from `mit.lint.<lint-name>` entries
    - Can be set with `git mit-config lint enable/disable <lint-name>`
    - Uses the lint's default enabled state if not explicitly configured

The TOML configuration uses this format:

``` toml,skip()
[mit.lint]
"lint-name" = true  # or false
```

### Author Configuration Precedence

git-mit loads authors from multiple sources and merges them:

1.  **From exec command** (highest priority):

    - Set via `--exec` flag or `GIT_MIT_AUTHORS_EXEC` environment
      variable
    - The command output must be valid TOML or YAML format
    - The command is executed using `shell_words::split()` for proper
      argument parsing

2.  **From authors file**:

    - Default location: `$HOME/.config/git-mit/mit.toml`
    - Custom location via `--config` flag or `GIT_MIT_AUTHORS_CONFIG`
      environment variable
    - Special handling for `$HOME/.config/git-mit/mit.toml` path - it's
      expanded to the actual home directory
    - If the file doesn't exist, an empty string is used (no error)

3.  **From Git config**:

    - Authors stored in `mit.author.config.<initial>.*` entries
    - These are set when you run
      `git mit-config mit set <initial> <name> <email>`
    - Stored at repository level (local) or user level (global) based on
      `--scope`

The authors from these sources are **merged**, with later sources
overriding earlier ones if there are conflicts.

## Hook Installation Details

The installation process creates symbolic links to the git-mit binaries:

- **Windows**: Creates file symbolic links with `.exe` extension
- **Unix/Linux/macOS**: Creates standard symbolic links
- **Existing hooks**: Installation will fail if hooks already exist
  (unless they're already symlinks to git-mit)
- **Symlink validation**: If a symlink exists but points to the correct
  binary, installation succeeds silently
