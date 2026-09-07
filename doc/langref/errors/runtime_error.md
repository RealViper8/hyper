# RuntimeError

A `RuntimeError` is a failure detected while the program executes: division by zero, undefined names, some I/O failures, type problems that surface at runtime, or an uncaught `raise`.

```text
RuntimeError: line N: …
```

Exit code: **70**.

The sample triggers integer division by zero:

```hyper
let a = 10
let b = 0
print(a / b)
```

Integer `/`, `%`, and `//` guard a zero divisor and report `RuntimeError` rather than undefined CPU behavior.

## Notes

- Uncaught `raise` also becomes `RuntimeError` with exit **70**.
- Recoverable cases should use `handle` at the call site — see [Raise and handle](raise_handle.md).
- Under `run`, some type issues may appear as `warning:` while execution continues; under `compile`, type failures are `SyntaxError` and block codegen. Division by zero remains a runtime failure on both paths.

## Example

Runnable sample: [`examples/errors/runtime_error.hyp`](../../examples/errors/runtime_error.hyp)
