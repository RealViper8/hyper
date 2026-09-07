# Floating-point numbers

Hyper provides IEEE-style floating types for approximate real numbers:

- `float32` — single precision
- `float64` — double precision

Related names (for example `f32` / `f64` style aliases where the language accepts them) follow the same idea: pick a precision and stick to it in annotations.

```hyper
let pi: float32 = 3.14
let mut gravity: float64 = 9.80665
```

## Behavior

Literals with a decimal point are floats. Arithmetic operators work as expected; results stay in floating representation unless you convert with builtins such as `int(...)` or `float(...)`.

JSON numbers with a fractional part map to `f64` on load. Prefer `float64` when interoperating with JSON or when you need more precision.

Floating comparison uses exact binary equality — for tolerances, compare differences yourself rather than relying on `==` alone.

## Example

Runnable sample: [`examples/data_type/numerical/floats.hyp`](../../../examples/data_type/numerical/floats.hyp)
