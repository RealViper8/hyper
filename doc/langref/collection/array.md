# Arrays

`Array[T]` is a **homogeneously typed** sequence. Every element must match the element type `T`.

```hyper
arr: Array[int32] = [10, 20, 30, 40]
```

Use arrays when the type system should enforce uniformity — numeric buffers, fixed-schema columns, or APIs that expect one element type. Lists remain available for mixed values.

## Behavior

- Construction looks like a list literal; the annotation (or inferred `Array[T]`) is what makes it an array.
- Methods mirror lists for the common cases: `len()` / `append(x)` on a mutable binding.
- Element type names follow the usual integer and float names (`int32`, `i64`, `float64`, …).

Generics beyond collection type parameters (`Array[T]`, `Dict[K, V]`) are not a general language feature yet — user-defined generic functions remain aspirational.

## Example

Runnable sample: [`examples/collection/array.hyp`](../../examples/collection/array.hyp)
