# Traits

A `trait` names a method interface. Structs opt in by listing the trait in parentheses and implementing each required method. Conformance is checked by **method name and arity** (and the signatures you write), not by a separate vtable language feature beyond what the compiler lowers today.

```hyper
trait Speaker:
    fn speak(ref self) -> string

struct Dog(Speaker):
    let pub name: string

    pub fn speak(ref self) -> string:
        return "Woof!"

struct Cat(Speaker):
    let pub name: string

    pub fn speak(ref self) -> string:
        return "Meow!"
```

Both `Dog` and `Cat` satisfy `Speaker` because each defines `speak` with a matching shape. You call the method on the concrete struct value (`my_dog.speak()`).

## Limits

- Trait conformance lowers on the compile path (covered by CI).
- User-defined **generics** over traits (for example a generic `make_it_speak[T: Speaker]`) remain aspirational — not implemented yet.
- The compiler still needs a known struct type at the call site to resolve methods.

## Example

Runnable sample: [`examples/struct/traits.hyp`](../../examples/struct/traits.hyp)
