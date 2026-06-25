# git-mit-install --uninstall

See [git-mit-install](./git-mit-install.md) for installation usage.

You can uninstall git-mit hooks from a repository using the `--uninstall` flag.

This removes the symlinks that `git mit-install` created in the local repository's hooks directory.

## Local uninstall

First, let's install git-mit into a fresh repository so we have something to uninstall.

``` shell,script(name="local-setup",expected_exit_code=0)
git init local-repo
cd local-repo
git mit-install
```

After installing, the hooks should be present.

``` shell,script(name="verify-local-installed",expected_exit_code=0)
cd local-repo
test -L .git/hooks/prepare-commit-msg && echo "prepare-commit-msg present"
test -L .git/hooks/pre-commit && echo "pre-commit present"
test -L .git/hooks/commit-msg && echo "commit-msg present"
```

Now uninstall from the local repository.

``` shell,script(name="local-uninstall",expected_exit_code=0)
cd local-repo
git mit-install --uninstall
```

After uninstalling, the hooks should be gone.

``` shell,script(name="verify-local-uninstalled",expected_exit_code=0)
cd local-repo
test ! -e .git/hooks/prepare-commit-msg && echo "prepare-commit-msg removed"
test ! -e .git/hooks/pre-commit && echo "pre-commit removed"
test ! -e .git/hooks/commit-msg && echo "commit-msg removed"
```

## Uninstall is idempotent

Running uninstall when hooks are not installed should succeed without error.

``` shell,script(name="idempotent-uninstall",expected_exit_code=0)
cd local-repo
git mit-install --uninstall
```

## Global uninstall

You can also uninstall git-mit from the global template directory.

``` shell,script(name="global-setup",expected_exit_code=0)
git init global-repo
cd global-repo
git mit-install --scope=global
```

Now uninstall globally.

``` shell,script(name="global-uninstall",expected_exit_code=0)
git mit-install --uninstall --scope=global
```

The template hooks should be removed.

``` shell,script(name="verify-global-uninstalled",expected_exit_code=0)
test ! -e "$HOME/.config/git/init-template/hooks/prepare-commit-msg" && echo "global prepare-commit-msg removed"
test ! -e "$HOME/.config/git/init-template/hooks/pre-commit" && echo "global pre-commit removed"
test ! -e "$HOME/.config/git/init-template/hooks/commit-msg" && echo "global commit-msg removed"
```
