# Author File Example

## Print the author file example

You can list all the available lints with a handy command

``` bash
set -euo pipefail
ACTUAL="$(pb-git-hooks authors example)"
EXPECTED="---
ae:
  name: Anyone Else
  email: anyone@example.com
bt:
  name: Billie Thompson
  email: billie@example.com
  signingkey: 0A46826A
se:
  name: Someone Else
  email: someone@example.com"

diff <(echo "$ACTUAL") <(echo "$EXPECTED")
```
