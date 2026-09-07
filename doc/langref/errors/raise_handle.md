# Raise and handle

Hyper has **no** `try` / `except`. Explicit error flow uses `raise`, the `raises` marker on functions, and `handle … else …`.

```hyper
fn parse_score(n: i64) raises -> i64:
    if n < 0:
        raise "score must be non-negative"
    return n

print(handle parse_score(10) else 0)
print(handle parse_score(-3) else 0)
```

## Forms

| Form | Role |
|------|------|
| `raise <expr>` | Signal failure (string, number, …). Uncaught → `RuntimeError`, exit **70**. |
| `fn f(...) raises` / `… raises -> T` / `… -> T raises` | Declares that the function may `raise`. A bare `raise` without `raises` is a `SyntaxError`. |
| `handle <attempt> else <fallback>` | Evaluate `attempt`; on raise, yield `fallback` instead. |

Module-level `raise` is allowed and exits the process. Prefer `handle` when calling `raises` functions from ordinary code.

## Design

This keeps failure paths visible at signatures and call sites. Both `run` and `compile` lower `raise` / `handle`. For the three error *kinds* and exit codes, see the sibling pages and `doc/errors/overview.md`.

## Example

Runnable sample: [`examples/errors/raise_handle.hyp`](../../examples/errors/raise_handle.hyp)
