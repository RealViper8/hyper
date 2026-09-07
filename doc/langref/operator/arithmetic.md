# Arithmetic operators

Hyper’s arithmetic operators cover the usual numeric toolbox:

| Operator | Meaning |
|----------|---------|
| `+` | Addition (also string concatenation) |
| `-` | Subtraction |
| `*` | Multiplication |
| `/` | Division |
| `//` | Floor division |
| `%` | Remainder |
| `**` | Exponentiation |

```hyper
let a = 10
let b = 3
let sum = a + b
let floor_div = a // b
let exp = a ** b
```

## Notes

- Integer `/`, `%`, and `//` check for a zero divisor at runtime and fail with `RuntimeError` (exit **70**).
- `+` on strings concatenates; see [String concat stress](../loop/str-concat-stress.md) for ownership of `s = s + …` loops.
- Operand types should agree in the usual way; annotate or convert when mixing widths or float/int intentionally.

Precedence follows familiar arithmetic rules: `**` binds tightly, then unary minus, then `*`, `/`, `//`, `%`, then `+` and `-`. Use parentheses when an expression would otherwise be hard to read.

## Example

Runnable sample: [`examples/operator/arithmetic.hyp`](../../examples/operator/arithmetic.hyp)
