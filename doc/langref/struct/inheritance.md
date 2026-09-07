# Composition (“inheritance” example)

The sample named *inheritance* demonstrates **composition with nested structs**, not classical class inheritance with a subtype hierarchy.

```hyper
struct Animal:
    let pub name: string

struct Dog:
    let pub animal: Animal
    let pub breed: string

let base_animal = Animal(name: "Rex")
let my_dog = Dog(animal: base_animal, breed: "Bulldog")
print(my_dog.breed)
```

`Dog` **contains** an `Animal` value in the field `animal`. There is no automatic field promotion or method inheritance from `Animal` into `Dog`. Access nested data explicitly (for example `my_dog.animal.name`) when you need the inner fields.

## Why this shape

Hyper’s struct system today focuses on concrete types, fields, and methods. Nesting structs is the supported way to build layered data. If you need a shared method interface across types, see [Traits](traits.md) (conformance by method name and arity).

## Example

Runnable sample: [`examples/struct/inheritance.hyp`](../../examples/struct/inheritance.hyp)
