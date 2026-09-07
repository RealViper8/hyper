# Dictionaries

`Dict[K, V]` is a typed key–value map. Literal syntax uses braces and `"key": value` pairs, familiar from Python.

```hyper
scores: Dict[string, int32] = {
    "math": 100,
    "physics": 95
}
```

You can also write an unannotated dict literal when inference is enough; the `Dict[K, V]` form documents the contract at the binding.

## Access and methods

- Lookup: `scores["math"]`.
- Length: `scores.len()` or `len(scores)`.
- Keys: `scores.keys()` returns the key collection used by the runtime (order follows insertion on JSON load; see the JSON module docs for dump sorting).

Dictionaries lower on the compile path. As with lists, deep shared `ref` into dict payloads is limited today — prefer copying values or restructuring when aliases would be required.

## Example

Runnable sample: [`examples/collection/dictionary.hyp`](../../examples/collection/dictionary.hyp)
