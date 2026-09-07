# Lists

A list is an ordered, heterogeneous sequence written with square brackets. Elements may mix types in one list.

```hyper
let items = [1, "Hyper", true, 3.14]
```

Lists are the flexible collection for everyday scripting-style code: mixed payloads, growing with `append`, and indexing by position. When every element shares one type and you want that constraint in the type, prefer [`Array[T]`](array.md).

## Working with lists

- Bind with `let` or `let mut`. Mutation of the sequence (append) needs `let mut`.
- Length: `items.len()` or `len(items)`.
- Grow: `items.append(value)`.
- Indexing uses `items[i]` (zero-based), consistent with Python-like surface syntax.

Lists lower on the compile path together with their common methods. Shared `ref` payloads into list/dict interiors remain a known limitation — see the compiler limitations notes if you need interior aliases.

## Example

Runnable sample: [`examples/collection/list.hyp`](../../examples/collection/list.hyp)
