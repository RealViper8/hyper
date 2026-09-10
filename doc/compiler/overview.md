# Compiler overview

The Hyper compiler lowers parsed, type-checked AST into **Hyper IR**, then Cranelift generates a machine **object**. A host linker (preferring LLVM `clang`) links that object with the Hyper C runtime into an executable. The same pipeline powers `run` / default `compile` (temp AOT exe), `--emit-obj`, and `--emit-exe`.

## Pipeline

```text
source → scanner → parser → typecheck → lower → IR → Cranelift object → clang/cc link → AOT executable
```

Module imports are resolved at lower time: `.hyp` files under the entry directory are parsed once and their top-level bindings become mangled IR symbols.

## CLI

```bash
# AOT run (default): temp exe + execute
hyper compile program.hyp
hyper run program.hyp

# Inspect IR
hyper compile program.hyp --emit-ir

# Cranelift object file
hyper compile program.hyp --emit-obj out.o

# Linked executable (requires clang/cc/gcc/MSVC)
hyper compile program.hyp --emit-exe my_app
```

## Struct-aware lowering

The compiler tracks which struct type each local holds (constructors, field access, annotated parameters, return types). Method calls like `p.move(1)` lower to direct calls on mangled IR functions instead of dynamic dispatch.

When inference is ambiguous, add a type annotation:

```hyper
fn shift(p: Point):
    p.move(1)
```

See [Supported features](supported-features.md) and [Known limitations](known-limitations.md).

## Compile-path dictionaries

`hyper_rt_dict_get` / `hyper_rt_dict_set` / `hyper_rt_dict_push` keep the same ABI. Lookup is a hash map, not a linear scan:

| Backend | Implementation |
|---------|----------------|
| AOT (`hyper_rt.c`) | insertion-order array + open-addressing slot table |

Print, `keys()`, JSON object load, and programs like `ci/smoke/smoke.hyp` that display dicts still use **insertion order**. Duplicate keys overwrite the existing value and keep the first key’s position. AOT JSON dump (`hyper_rt_json.c`) walks that same entry array; it does not reimplement hashing.

Unit test `dict_get_set_on_medium_map` exercises 256 keys and about a million lookups, with a wall-time bound so a regression to O(n) scan fails CI.
