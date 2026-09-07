# Boolean operators

Logical combination uses the keywords `and`, `or`, and `not` — not `&&` / `||` / `!`.

```hyper
let x = true
let y = false
let a = x and y
let b = x or y
let c = not x
```

| Operator | Role |
|----------|------|
| `and` | True only if both sides are true |
| `or` | True if at least one side is true |
| `not` | Negates a boolean |

## Notes

- Prefer these keywords in Hyper source; they match the Python-like surface.
- Combine with comparisons for conditions: `if x > 0 and ready:`.
- Parenthesize mixed `and` / `or` chains when readability suffers — clarity beats relying on precedence alone.

## Example

Runnable sample: [`examples/operator/boolean.hyp`](../../examples/operator/boolean.hyp)
