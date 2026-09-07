# For loops

A `for` loop walks a sequence or a counted range. The most common form uses `range`:

```hyper
for i in range(5):
    print(i)
```

## `range` special case

On the compile path, `range` as a free expression builds an eager list. Inside `for i in range(n)` or `for i in range(a, b)`, the compiler **special-cases** the header into a counted loop instead of materializing every index first. That keeps tight numeric loops efficient.

- `range(n)` — indices `0 .. n-1`
- `range(a, b)` — indices `a .. b-1`

## Control flow

- `break` exits the innermost loop.
- `continue` skips to the next iteration; for `for` / `for-in`, the induction variable still advances through a dedicated increment path.

Both are rejected outside a loop. Inside a `@parallel` `for` body they are also rejected — early exit has no single meaning across split iterations.

## Example

Runnable sample: [`examples/loop/for.hyp`](../../examples/loop/for.hyp)
