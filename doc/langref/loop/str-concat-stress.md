# String concatenation stress

Tight loops of the form `s = s + "x"` allocate a new string on every iteration. Correctness is easy to check (`s.len()` grows by one each time); the hard part is **ownership**: without reclaiming the previous value of `s`, every prefix stays live and RSS grows roughly quadratically.

```hyper
let mut s = ""
for i in range(80000):
    s = s + "x"
print(s.len())
```

## Compile-path ownership

On the compile path, store-back concatenation (`s = s + …`) passes consume flags into the runtime concat helper so the previous owned `s` can be freed when it is safe. Interned string literals are never freed. Forms like `t = a + b` do **not** consume `a` or `b` — those names must remain valid.

This page’s example is the stress program used to verify that behavior (issue #46). After the fix, peak RSS stays bounded while the printed length is still `80000`.

## Notes

- Prefer this pattern when you must grow a string in a loop; the compiler recognizes the store-back shape.
- Overwriting a local with a fresh non-concat string can still leave the previous value unreclaimed (linear leftovers, not the quadratic prefix leak).
- Deeper notes live under `doc/compiler/str-concat-ownership.md` and the stress showcase.

## Example

Runnable sample: [`examples/loop/str-concat-stress.hyp`](../../examples/loop/str-concat-stress.hyp)
