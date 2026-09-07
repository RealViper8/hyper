# Simple functions

Functions are declared with `fn` (or `def`). Parameters may be untyped for small scripts; the body returns with `return`.

```hyper
fn add(a, b):
    return a + b

print(add(2, 3))
```

Call a function with parentheses and arguments in declaration order. Nested functions are allowed; they do not inherit loop control from an outer `for` / `while` (a `break` inside a nested `fn` is an error).

## `fn` and `def`

Both keywords introduce a function on the compile path. Pick one style and stay consistent in a file. For annotated parameters and return types, see [Strict typing](strict-type.md). For mutable aliasing into struct fields, see [Reference parameters](reference.md).

## Notes

- A function that may `raise` must be marked `raises` — see [Raise and handle](../errors/raise_handle.md).
- Type errors at call sites are fatal under `run` and `compile` before codegen.

## Example

Runnable sample: [`examples/function/simple.hyp`](../../examples/function/simple.hyp)
