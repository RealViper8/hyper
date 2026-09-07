# Booleans

The `bool` type has two values: `true` and `false`. Booleans appear in conditions, comparisons, and logical operators (`and`, `or`, `not`).

```hyper
let is_fast: bool = true
let mut is_safe: bool = false
```

You can bind them with or without an annotation. Use `let mut` when the flag will flip later (for example, after validation).

## How they are used

- Conditions in `if`, `elif`, `while`, and the ternary form expect a boolean (or a value treated as a condition in the usual way for comparisons).
- Comparison operators produce `bool` results.
- The builtin `bool(...)` converts values where conversion is defined on the compile path.

There is no separate “truthy/falsy” object model beyond what the language defines for conditions; prefer explicit comparisons when clarity matters.

## Example

Runnable sample: [`examples/data_type/boolean.hyp`](../../examples/data_type/boolean.hyp)
