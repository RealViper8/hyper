# Standard file I/O

File access follows a Python-like surface: `open`, modes, and `with` for automatic close. Underneath, handles are buffered streams (read-ahead and write-behind) so tight loops amortize syscalls.

```hyper
with open("file.txt", "w") as f:
    f.write("first line\n")

with open("file.txt", "r") as f:
    let content = f.read()
    print(content)
```

`open(path)` defaults to mode `"r"`. The `with` block closes the file (flushing buffered writes) when the block ends.

## Modes

Common modes: `"r"`, `"w"`, `"a"`, `"r+"`, `"w+"`, `"a+"`, plus exclusive create `"x"` / `"x+"`. A `b` or `t` suffix is accepted for familiarity; Hyper still returns **text**, decoding invalid UTF-8 with replacement characters.

## Methods

| Method | Role |
|--------|------|
| `read()` / `read(n)` | Rest of file, or at most `n` bytes |
| `readline()` | Next line **without** trailing newline (empty at EOF) |
| `readlines()` | Remaining lines, newlines stripped |
| `write` / `writelines` | Write text |
| `seek` / `tell` / `size` | Position and size |
| `flush` / `close` / `closed` | Buffer and lifetime |
| `path` / `mode` | Open path and mode strings |

Unlike Python, `readline` and `readlines` strip trailing newlines. Standalone handles need an explicit `close()` if you do not use `with`.

## Example

Runnable sample: [`examples/file_handling/standard.hyp`](../../examples/file_handling/standard.hyp)
