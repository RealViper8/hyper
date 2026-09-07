# Ternary expressions

Hyper’s conditional expression uses the Python-style order: value-if-true, then the condition, then value-if-false.

```hyper
let age = 18
let status = "Adult" if age >= 18 else "Minor"
```

The general form is:

```text
<expr_if_true> if <condition> else <expr_if_false>
```

Exactly one of the two result expressions is evaluated conceptually as the chosen branch; use this for small choices that fit on one line.

## When to prefer blocks

If either arm needs multiple statements, side effects that are hard to read inline, or nested ternaries that obscure intent, write a normal `if` / `else` block instead. Ternaries shine for binding a single result from a clear boolean test.

## Example

Runnable sample: [`examples/conditional/ternary.hyp`](../../examples/conditional/ternary.hyp)
