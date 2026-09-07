# Collection methods

Lists, arrays, and dictionaries expose a small set of methods that lower on the compile path. The same ideas are also available as builtins where noted.

## Lists and arrays

| Method | Meaning |
|--------|---------|
| `coll.len()` | Number of elements |
| `coll.append(x)` | Append one element (needs a mutable binding) |

```hyper
let mut items = [1, 2]
print(items.len())
items.append(3)
```

## Dictionaries

| Method | Meaning |
|--------|---------|
| `d.len()` | Number of entries |
| `d.keys()` | Keys of the map |

```hyper
let mut scores = {"math": 100, "physics": 95}
print(scores.len())
print(scores.keys())
```

## Builtin `len`

`len(x)` is the free-function form for containers and strings. Prefer whichever style matches surrounding code; both are supported.

Strings also have `s.len()` and a larger method set documented under [Strings](../data_type/string.md).

## Example

Runnable sample: [`examples/collection/methods.hyp`](../../examples/collection/methods.hyp)
