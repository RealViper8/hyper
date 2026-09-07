# Assignment operators

Plain assignment uses `=`. Compound forms update a **mutable** binding in place by combining an arithmetic operator with assignment.

```hyper
let mut x = 10
x += 5
x -= 3
x *= 2
x /= 4
x %= 3
x **= 2
```

| Form | Meaning |
|------|---------|
| `x = e` | Replace the binding’s value |
| `x += e` | `x = x + e` |
| `x -= e` | `x = x - e` |
| `x *= e` | `x = x * e` |
| `x /= e` | `x = x / e` |
| `x %= e` | `x = x % e` |
| `x **= e` | `x = x ** e` |

## Rules

- The left-hand side must be mutable (`let mut`, or a `mut` field / `ref` parameter that allows writes).
- Compound assignment is an update of one binding, not a new `let`.
- Division compounds inherit the same zero-divisor runtime checks as `/` and `%`.

## Example

Runnable sample: [`examples/operator/assignment.hyp`](../../examples/operator/assignment.hyp)
