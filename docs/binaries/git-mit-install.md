# git-mit-install

Generic usage

``` shell,script(name="run_help",expected_exit_code=0)
export HOME="/example/home/dir"
export USERPROFILE="F:\\some\\userprofile"
git-mit-install --help
```

```text,verify(script_name="run_help",target_os="!windows")
Install git-mit into a repository

Usage: git-mit-install [OPTIONS]

Options:
  -s, --scope <SCOPE>
          [default: local]

          Possible values:
          - global: The home directory
          - local:  The local folder

      --completion <COMPLETION>
          [possible values: bash, elvish, fish, powershell, zsh]

      --home-dir <HOME_DIR>
          [env: HOME=/example/home/dir]

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

On windows the home directory is different

```text,verify(script_name="run_help",target_os="windows")
Install git-mit into a repository

Usage: git-mit-install [OPTIONS]

Options:
  -s, --scope <SCOPE>
          [default: local]

          Possible values:
          - global: The home directory
          - local:  The local folder

      --completion <COMPLETION>
          [possible values: bash, elvish, fish, powershell, zsh]

      --home-dir <HOME_DIR>
          [env: USERPROFILE=F:\some\userprofile]

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

You can generate completion with

```shell,script(name="completion",expected_exit_code=0)
git-mit-install --completion bash
```
