# git-mit-install

Generic usage

``` shell,script(name="run-help",expected_exit_code=0)
export HOME="/example/home/dir"
export USERPROFILE="F:\\some\\userprofile"
git-mit-install --help
```

``` text,verify(script_name="run-help",target_os="!windows")
Install git-mit into a repository

Usage: git-mit-install [OPTIONS]

Options:
  -s, --scope <SCOPE>
          Possible values:
          - global: The home directory
          - local:  The local folder
          
          [default: local]

      --completion <COMPLETION>
          [possible values: bash, elvish, fish, powershell, zsh]

      --home-dir <HOME_DIR>
          [env: HOME=/example/home/dir]

      --uninstall
          Uninstall git-mit hooks instead of installing them

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

On windows the home directory is different

``` text,verify(script_name="run-help",target_os="windows")
Install git-mit into a repository

Usage: git-mit-install [OPTIONS]

Options:
  -s, --scope <SCOPE>
          Possible values:
          - global: The home directory
          - local:  The local folder
          
          [default: local]

      --completion <COMPLETION>
          [possible values: bash, elvish, fish, powershell, zsh]

      --home-dir <HOME_DIR>
          [env: USERPROFILE=F:\some\userprofile]

      --uninstall
          Uninstall git-mit hooks instead of installing them

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

You can generate completion with

``` shell,script(name="generate-bash-completion",expected_exit_code=0)
git-mit-install --completion bash
```

## Uninstalling

To remove git-mit hooks from a repository, see the
[uninstall documentation](./git-mit-install-uninstall.md).
