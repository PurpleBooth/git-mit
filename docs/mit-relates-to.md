# Relates to

This is the `git mit-relates-to` part of the tool.

## Setup

In order to get started with this tool you'll need a git repository

``` shell,script(name="init-repo",expected_exit_code=0)
git init .
```

You'll need to install the hooks into this repository

``` shell,script(name="install-hooks",expected_exit_code=0)
git mit-install
```

## Running the command

In projects, it is nice to help out your co-workers by linking the
commits you're making back to issue in the backlog. Be very easy to
forget though, so here's a command to automate it.

Say you've just made this awesome `README.md` for Pivotal Tracker ID
`[#12321513]`

``` markdown,file(path="README.md")
# The Best Readme

This is the best readme
```

If you run

``` shell,script(name="set-relates-to-configuration",expected_exit_code=0)
git mit-relates-to "[#12321513]"
```

Next time you commit

``` shell,script(name="commit-with-relates-to",expected_exit_code=0)
git add README.md
git mit bt
git commit -m "Wrote a great README"
```

the commit message will contain the ID

``` shell,script(name="show-log-with-relates-to-trailer",expected_exit_code=0)
git show --pretty='format:author: [%an %ae] signed-by: [%GS] 
---
%B' -q
```

``` text,verify(script_name="show-log-with-relates-to-trailer",stream=stdout)
author: [Billie Thompson billie@example.com] signed-by: [] 
---
Wrote a great README

Relates-to: [#12321513]
```

We don't duplicate the ID if you manually type in the trailer

``` shell,script(name="commit-without-relates-to",expected_exit_code=0)
echo "Some change" >> README.md
git add README.md
git mit bt
git commit -m "Wrote a great README

Relates-to: [#12321513]
"
```

the commit message will contain the ID

``` shell,script(name="show-log-without-relates-to",expected_exit_code=0)
git show --pretty='format:author: [%an %ae] signed-by: [%GS] 
---
%B' -q
```

``` text,verify(script_name="show-log-without-relates-to",stream=stdout)
author: [Billie Thompson billie@example.com] signed-by: [] 
---
Wrote a great README

Relates-to: [#12321513]
```

This times out after 60 minutes, and is configurable with the

`GIT_MIT_RELATES_TO_TIMEOUT` by environment variable.

``` shell,script(name="set-relates-to-trailer",expected_exit_code=0)
export GIT_MIT_RELATES_TO_TIMEOUT=120
git mit-relates-to "[#12321513]"
```

Would set the timeout to 2 hours (or 120 minutes).

You can also populate this value from a script, allowing you to, for
example, query an API and pull your current ticket from there.

``` shell,script(name="set-trailer-with-shell-script",expected_exit_code=0)
echo "Something else" >> README.md
git add README.md
export GIT_MIT_RELATES_TO_EXEC="echo [#88553322]"
git commit -m "Another great addition"
```

the commit message will contain the ID

``` shell,script(name="show-log-with-shell-set-trailer",expected_exit_code=0)
git show --pretty='format:author: [%an %ae] signed-by: [%GS] 
---
%B' -q
```

``` text,verify(script_name="show-log-with-shell-set-trailer",stream=stdout)
author: [Billie Thompson billie@example.com] signed-by: [] 
---
Another great addition

Relates-to: [#88553322]
```

You could use a script like this to populate the current Pivotal Tracker
ID. You need `curl` and `jq` for this command to work.

It assumes you have you Pivotal Tracker API Key in the environment
variable `PIVOTAL_TRACKER_TOKEN` and also that you have the project ID
in a file at the root of your repository called
`.pivotal_tracker_project`

``` shell,skip()
echo "[#$(curl --silent -X GET -H "X-TrackerToken: $PIVOTAL_TRACKER_TOKEN" "https://www.pivotaltracker.com/services/v5/projects/$(cat .pivotal_tracker_project)/stories?filter=state:started+owner:$(curl --silent "https://www.pivotaltracker.com/services/v5/me?fields=%3Adefault" -H "X-TrackerToken: $PIVOTAL_TRACKER_TOKEN" | jq -r .initials)" | jq .[0].id)]"
```

This would become

``` shell,skip()
export GIT_MIT_RELATES_TO_EXEC="bash -c 'echo \"[#\$(curl --silent -X GET -H \"X-TrackerToken: \$PIVOTAL_TRACKER_TOKEN\" \"https://www.pivotaltracker.com/services/v5/projects/\$(cat .pivotal_tracker_project)/stories?filter=state:started+owner:\$(curl --silent \"https://www.pivotaltracker.com/services/v5/me?fields=%3Adefault\" -H \"X-TrackerToken: \$PIVOTAL_TRACKER_TOKEN\" | jq -r .initials)\" | jq .[0].id)]\"'"
```

## Templating

It is possible to do some light templating of this command. This makes
it possible to avoid typing so much, or skip awkward characters.

``` shell,script(expected_exit_code=0)
git mit-config relates-to template "[#{value}]"
```

The templating uses
[TinyTemplate](https://docs.rs/tinytemplate/latest/tinytemplate/), with
a single `value` variable available, containing the value passed to the
command

Next time you commit after running relates to

``` shell,script(name="set-relates-to-template",expected_exit_code=0)
git mit-relates-to "12321513"
echo "Something else" >> README.md
git add README.md
git mit bt
git commit -m "More awesome additions"
```

the commit message will contain the ID

``` shell,script(name="show-log-with-templated-relates-to",expected_exit_code=0)
git show --pretty='format:author: [%an %ae] signed-by: [%GS] 
---
%B' -q
```

``` text,verify(script_name="set-relates-to-template",stream=stdout)
author: [Billie Thompson billie@example.com] signed-by: [] 
---
More awesome additions

Relates-to: [#12321513]
```

Alternatively you can set the template via an environment variable

``` shell,script(expected_exit_code=0)
git mit-relates-to "12321513"
echo "Something else" >> README.md
git add README.md
git mit bt
GIT_MIT_RELATES_TO_TEMPLATE="JIR-{value}" git commit -m "More awesome additions"
```

``` shell,script(expected_exit_code=0)
git show --pretty='format:author: [%an %ae] signed-by: [%GS] 
---
%B' -q
```

``` text,verify(script_name="set-relates-to-env-template",stream=stdout)
author: [Billie Thompson billie@example.com] signed-by: [] 
---
More awesome additions

Relates-to: JIR-12321513
```
