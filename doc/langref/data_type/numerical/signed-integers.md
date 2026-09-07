# Signed integers

Signed integer types store both positive and negative values. Hyper exposes fixed widths that map cleanly to machine integers:

| Type | Typical range (two’s complement) |
|------|----------------------------------|
| `i8` | −128 … 127 |
| `i16` | −32768 … 32767 |
| `i32` | −2³¹ … 2³¹−1 |
| `i64` | −2⁶³ … 2⁶³−1 |

Aliases such as `int32` appear in examples and type annotations alongside the `iN` forms; use the name your code already prefers, consistently.

```hyper
let a: i8 = -128
let b: i16 = -32768
let c: i32 = -2147483648
let mut d: i64 = 9223372036854775807
```

## Arithmetic

`+`, `-`, `*`, `/`, `//`, `%`, and `**` apply to integers. Integer `/`, `%`, and `//` guard division by zero at runtime and raise a `RuntimeError` (exit code 70) when the divisor is zero.

Unannotated integer literals typically widen to a default integer representation suitable for everyday arithmetic (`i64`-scale on the compile path). Annotate when you need a specific width.

## Example

Runnable sample: [`examples/data_type/numerical/signed-integers.hyp`](../../../examples/data_type/numerical/signed-integers.hyp)
