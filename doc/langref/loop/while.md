# While loops

A `while` loop repeats a body as long as its condition is true. The condition is checked before each iteration.

```hyper
let mut i = 0
while i < 5:
    print(i)
    i += 1
```

Counters and accumulators used in the condition almost always need `let mut` (or another mutable location) so the loop can make progress.

## Control flow

- `break` leaves the loop immediately.
- `continue` jumps back to the condition header for `while` (then re-evaluates the test).

As with `for`, `break` / `continue` outside any loop are `SyntaxError`s. Nested functions do not inherit an enclosing loop, so `break` inside a nested `fn` is an error.

## When to use `while` vs `for`

Prefer `for` / `range` for counted iteration. Prefer `while` when the stop condition is not a simple integer span — waiting on input, scanning until a sentinel, or updating state that does not map cleanly to a range.

## Example

Runnable sample: [`examples/loop/while.hyp`](../../examples/loop/while.hyp)
