# Parallel and vectorized for

Stacking `@parallel` and `@vectorize` states the strongest performance intent: distribute iterations across threads **and** use SIMD-style acceleration within workers.

```hyper
@parallel
@vectorize
for i in range(10_000_000):
    matrix[i] *= 2.0
```

Decorator order in samples places `@parallel` above `@vectorize`; both attach to the following `for`.

## Current compile behavior

Both hints are recognized, but the loop still runs **sequentially** on the compile path — same per-index semantics as a plain `for`. Production threaded codegen and SIMD/GPU lowering are not available yet; this is a known limitation, not a silent change of meaning for results.

## Practical guidance

- Keep iterations independent (no `break` / `continue` in the body).
- Prefer simple element updates that would vectorize cleanly later.
- Use the decorators in real code only when you accept today’s sequential execution and want the annotation for future backends.

## Example

Runnable sample: [`examples/loop/advanced/parallel-vectorized-for.hyp`](../../../examples/loop/advanced/parallel-vectorized-for.hyp)
