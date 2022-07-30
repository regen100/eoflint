# eoflint

Check `No newline at end of file`.

# Usage

```
$ cargo build --release
$ printf "text\n" >/tmp/a
$ printf "text" >/tmp/b
$ target/release/eoflint /tmp/a /tmp/b
/tmp/b: no newline at end of file
```
