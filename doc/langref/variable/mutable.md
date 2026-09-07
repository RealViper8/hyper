# Mutable variables

Use `let mut` when a binding must change after its initial assignment. Mutation is opt-in: Hyper does not make every local writable by default.

```hyper
let mut x = 0
x = 1
x += 1
```

Mutable bindings work with compound assignment (`+=`, `-=`, and the rest) and with loops that update a counter or accumulator. Struct fields that should change are marked `mut` separately on the field declaration; see the struct pages.

## Optional types

As with immutable bindings, you can annotate the type:

```hyper
let mut total: i64 = 0
let mut name: string = ""
```

The annotation fixes the type for the lifetime of the binding. Later assignments must be compatible with that type.

## Rules

- Declare `let mut` at the binding site; you cannot “upgrade” an immutable `let` later.
- Prefer immutability when reassignment is not required — it keeps intent obvious.
- Collections that grow in place (`append`) also need a mutable binding to the collection itself.

## Example

Runnable sample: [`examples/variable/mutable.hyp`](../../examples/variable/mutable.hyp)
