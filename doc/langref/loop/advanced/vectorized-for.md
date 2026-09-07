# Vectorized for (`@vectorize`)

The `@vectorize` decorator marks a `for` loop as a **SIMD-oriented hint**: the intent is to run independent per-index work using CPU vector registers (and, longer term, related accelerators).

```hyper
@vectorize
for i in range(1000):
    arr[i] = a[i] + b[i]
```

## Current compile behavior

Today the compile path still executes **every index sequentially**. Results match a normal `for`; you do not get production SIMD or GPU codegen yet. The decorator is accepted and lowers so programs keep working while backends catch up.

Treat `@vectorize` as a forward-looking annotation: write loop bodies that would be safe to vectorize (independent iterations, no conflicting writes), knowing that runtime is still one-index-at-a-time for now.

## Rules

- Place `@vectorize` immediately above the `for`.
- Prefer elementwise updates without cross-iteration dependencies.
- See also [`@parallel`](parallel-for.md) and the combined form.

## Example

Runnable sample: [`examples/loop/advanced/vectorized-for.hyp`](../../../examples/loop/advanced/vectorized-for.hyp)
