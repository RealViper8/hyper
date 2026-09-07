# Comparison operators

Comparisons produce boolean results and drive `if`, loops, and ternary expressions.

| Operator | Meaning |
|----------|---------|
| `==` | Equal |
| `!=` | Not equal |
| `>` | Greater than |
| `<` | Less than |
| `>=` | Greater or equal |
| `<=` | Less or equal |

```hyper
let x = 5
let y = 10
let a = x == y
let b = x != y
let c = x < y
```

## Notes

- Chain logic with `and` / `or` rather than C-style chained comparisons unless your expression is deliberately simple.
- Floating equality is exact binary equality; use an epsilon check when comparing computed floats.
- Type mismatches in comparisons are reported as type errors (fatal under `run` and `compile` before codegen).

## Example

Runnable sample: [`examples/operator/comparison.hyp`](../../examples/operator/comparison.hyp)
