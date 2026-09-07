# Strictly typed functions

Parameters and return types can be annotated. Annotations document the contract and let the type checker reject mismatches before codegen.

```hyper
fn add(a: int32, b: int32) -> int32:
    return a + b
```

Use the same type names as elsewhere (`i32` / `int32`, `string`, `bool`, struct names, `Array[T]`, …). A function that both returns a value and may fail is written with `raises` adjacent to the signature — for example `fn f(...) raises -> i64` or `fn f(...) -> i64 raises`.

## Why annotate

- Catches width and type mistakes early (fatal under `run` and `compile`).
- Helps method and struct resolution when the compiler must know a type at the call site.
- Makes public APIs readable without running the body.

Untyped parameters remain valid for small examples; prefer annotations as programs grow.

## Example

Runnable sample: [`examples/function/strict-type.hyp`](../../examples/function/strict-type.hyp)
