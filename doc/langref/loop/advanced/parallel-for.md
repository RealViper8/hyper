# Parallel for (`@parallel`)

The `@parallel` decorator marks a `for` loop as intended for **multi-threaded** execution across CPU cores. The conceptual model is: each index may run concurrently, so the body must be safe without shared mutable races.

```hyper
@parallel
for i in range(1_000_000):
    process_data(i)
```

## Current compile behavior

On today’s compile path, `@parallel` is emitted as a **sequential** loop. Per-index results match ordinary `for`; threaded codegen is planned but not shipped. Programs that use the decorator still typecheck and run.

## Control-flow restriction

`break` and `continue` are **rejected** inside a `@parallel` (or `@parallel @vectorize`) body. Early exit has no single well-defined meaning when iterations are meant to be split across threads.

Write parallel bodies as independent per-index work. Keep reduction-style sharing for a future model or do it outside the parallel region.

## Example

Runnable sample: [`examples/loop/advanced/parallel-for.hyp`](../../../examples/loop/advanced/parallel-for.hyp)
