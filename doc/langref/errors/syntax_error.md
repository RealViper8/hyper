# SyntaxError

A `SyntaxError` means the program could not be accepted as valid Hyper: bad tokens, incomplete expressions, unsupported constructs on the compile path, or typecheck failures under `compile` (and fatal type errors under `run` as well).

```text
SyntaxError: line N: …
```

Exit code: **65**.

The sample leaves an expression unfinished so the parser reports an error:

```hyper
print(1 +
```

## When you see it

- Scanner/parser problems (missing operands, bad punctuation).
- Constructs the compiler cannot lower yet (reported before codegen).
- Semantic/type failures that block compilation.
- Structural mistakes such as `break` outside a loop, or `raise` inside a function not marked `raises`.

Errors are written to **stderr**. Fix the reported line (and related notes) before expecting codegen or execution to proceed.

## Example

Runnable sample: [`examples/errors/syntax_error.hyp`](../../examples/errors/syntax_error.hyp)
