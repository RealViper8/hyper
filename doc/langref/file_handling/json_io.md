# JSON I/O

`json` is a **builtin module** implemented in the runtime — there is no `json.hyp` to install. Import it like any other module:

```hyper
import json

let config = {"name": "Hyper", "version": 1, "fast": true}
let text = json.dumps(config)
let parsed = json.loads(text)
```

## Functions

| Function | Role |
|----------|------|
| `json.loads(text)` | Parse a JSON string |
| `json.dumps(value)` | Compact JSON string |
| `json.dumps(value, indent)` | Pretty-print with `indent` spaces |
| `json.load(file)` | Parse from an open file handle |
| `json.dump(value, file)` | Write compact JSON |
| `json.dump(value, file, indent)` | Write pretty JSON |

## Type mapping

JSON objects become dicts; arrays become lists; strings, booleans, and `null` (`None`) map directly. Integer JSON numbers become `i64`; fractional numbers become `f64`. Struct instances serialize as objects by field name. On dump, dict keys are sorted for stable output.

File round-trips typically combine `with open(...)` and `json.dump` / `json.load`. All of these lower on the compile path.

## Example

Runnable sample: [`examples/file_handling/json_io.hyp`](../../examples/file_handling/json_io.hyp)
