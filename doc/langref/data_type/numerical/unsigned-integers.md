# Unsigned integers

Unsigned integer types store only non-negative values. They are useful for sizes, bit patterns, and values that must never be negative.

| Type | Maximum (inclusive) |
|------|---------------------|
| `u8` | 255 |
| `u16` | 65535 |
| `u32` | 4294967295 |
| `u64` | 18446744073709551615 |

```hyper
let a: u8 = 255
let b: u16 = 65535
let c: u32 = 4294967295
let mut d: u64 = 18446744073709551615
```

## Notes

- Prefer signed types unless you specifically need unsigned semantics or the full positive range of a given width.
- Mixing signed and unsigned in one expression can require care; annotate intermediates when the compiler asks for a clearer type.
- Division by zero is still a runtime failure for integer division operators.

## Example

Runnable sample: [`examples/data_type/numerical/unsigned-integers.hyp`](../../../examples/data_type/numerical/unsigned-integers.hyp)
