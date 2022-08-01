# eoflint

Check `No newline at end of file`.

# Usage

```
$ cargo install eoflint
$ printf "text\n" >/tmp/a
$ printf "text" >/tmp/b
$ eoflint /tmp/a /tmp/b
/tmp/b: no newline at end of file
```
