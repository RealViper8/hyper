# None

`None` is Hyper’s unit / absent value. Annotate a binding as `None` when the only meaningful value is absence itself, or when an API returns “no result.”

```hyper
let result: None = None
```

In JSON round-trips, JSON `null` maps to Hyper `None`. Treat `None` as a distinct type from integers, strings, and booleans — do not use it as a numeric zero or an empty string substitute unless your own code defines that convention.

## Notes

- The literal is written `None` (capital N).
- Optional-style APIs often combine a real type with explicit checks or `handle` rather than overloading `None` everywhere.
- Type errors involving `None` are fatal under `run` and `compile` before codegen.

## Example

Runnable sample: [`examples/data_type/none.hyp`](../../examples/data_type/none.hyp)
