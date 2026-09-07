# Imports

Modules load other `.hyp` files. Three forms are supported:

```hyper
import math
from math import add
import math as m
```

| Form | Effect |
|------|--------|
| `import math` | Bind the module; use `math.add`, `math.VERSION`, … |
| `from math import add` | Bring `add` into the current namespace |
| `import math as m` | Same as import, with a shorter alias |

The sample `import.hyp` loads the sibling module [`math.hyp`](math.md) from the same folder. Module names map to files: `math` → `math.hyp` beside the importer (or on the module search path used by the toolchain).

## Builtin modules

Some modules are provided by the runtime rather than a `.hyp` file — notably `json`. Syntax is the same (`import json`); there is nothing to place on disk.

## Notes

- Imported functions and values behave like local definitions once bound.
- Prefer explicit `import module` when you want a namespace; use `from` for a few names you call often.

## Example

Runnable sample: [`examples/module/import.hyp`](../../examples/module/import.hyp)
