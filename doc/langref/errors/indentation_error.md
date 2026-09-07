# IndentationError

Hyper uses indentation to define blocks, in the Python-like style. An `IndentationError` means the indent structure is inconsistent: unexpected indent, bad dedent, or mixed tabs and spaces in indentation.

```text
IndentationError: line N: …
```

Exit code: **65** (same family as `SyntaxError` for process status).

The sample places a statement at an illegal indent level:

```hyper
let x = 1
    let y = 2
```

## How to avoid it

- Indent with spaces consistently within a file.
- Align `elif` / `else` with their `if`, and dedent cleanly when a block ends.
- Do not mix tabs and spaces for indentation.

Like other diagnostics, messages go to stderr with a line number. The program does not run past a fatal indent failure in the frontend.

## Example

Runnable sample: [`examples/errors/indentation_error.hyp`](../../examples/errors/indentation_error.hyp)
