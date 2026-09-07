# Immutable variables

By default, a Hyper binding is **immutable**. You introduce it with `let` and give it a value once; later assignment to that name is rejected.

```hyper
let x = 0
```

Immutability is the everyday default: once a name is bound, readers can treat it as stable for the rest of the scope. Use this for constants, configuration values, and any binding you do not intend to overwrite.

## Optional types

You may annotate the type explicitly:

```hyper
let count: i64 = 0
let label: string = "ready"
```

When the annotation is omitted, the compiler infers a type from the initializer. Either form is valid; annotations help when the value is ambiguous or when you want a specific integer width.

## Rules

- Prefer `let` unless the binding must change.
- Reassignment requires `let mut` (see [Mutable variables](mutable.md)).
- Nested scopes can shadow an outer name with a new `let`; that does not mutate the outer binding.

## Example

Runnable sample: [`examples/variable/immutable.hyp`](../../examples/variable/immutable.hyp)
