# The `math` sample module

`math.hyp` in `doc/examples/module/` is not the language’s standard math library. It is a **sibling module file** used by [`import.hyp`](import.md) to demonstrate how Hyper loads user modules.

```hyper
fn add(a, b):
    return a + b

fn mul(a, b):
    return a * b

let VERSION = "0.1"
```

When another file in the same directory runs `import math`, this file is loaded as the module named `math`. Callers use `math.add`, `math.mul`, and `math.VERSION`, or `from math import add`.

## Role in the docs

| File | Role |
|------|------|
| `math.hyp` | Module under test — definitions only |
| `import.hyp` | Driver that imports and prints results |

Copy this pattern for your own packages: one file exports functions and constants; another imports them. For the builtin JSON API, use `import json` instead of a local `.hyp` file.

## Example

Runnable sample: [`examples/module/math.hyp`](../../examples/module/math.hyp)
