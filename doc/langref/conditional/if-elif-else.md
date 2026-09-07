# If / elif / else

Branching uses indented blocks under `if`, optional `elif` arms, and an optional final `else`.

```hyper
let x = 10

if x > 15:
    print("Greater than 15")
elif x == 10:
    print("Equal to 10")
else:
    print("Less than 10")
```

## How it works

Conditions are tested in order. The first true branch runs; remaining arms are skipped. If no condition matches and there is an `else`, that block runs.

You may use a lone `if`, `if`/`else`, or any number of `elif` arms. Nested `if` statements are allowed; keep indentation consistent (spaces only — mixed tabs/spaces in indent are `IndentationError`).

Combine comparisons with `and` / `or` / `not` inside conditions. For a single expression choosing between two values, see [Ternary expressions](ternary.md).

## Example

Runnable sample: [`examples/conditional/if-elif-else.hyp`](../../examples/conditional/if-elif-else.hyp)
