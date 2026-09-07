# Structs and object creation

A `struct` groups named fields and optional methods. Fields declare visibility and mutability; `pub` makes a field accessible outside the struct’s methods, and `mut` allows assignment after initialization.

```hyper
struct Point:
    let pub mut x: i32
    let pub mut y: i32

    pub fn __init__(ref self, x_val: i32, y_val: i32):
        self.x = x_val
        self.y = y_val

    pub fn move(ref self, dx: i32, dy: i32):
        self.x = self.x + dx
        self.y = self.y + dy

let mut p = Point(x: 10, y: 20)
p.move(5, 5)
```

## Construction

Call the struct name like a constructor. Keyword arguments matching field or `__init__` parameters are the usual style in examples. After construction, call methods with `instance.method(...)`.

The compiler resolves methods when it knows the struct type — from constructor assignments, annotations, field chains, or tracked return types. If resolution fails, add an annotation or clarify the type earlier.

## Example

Runnable sample: [`examples/struct/object_creation.hyp`](../../examples/struct/object_creation.hyp)
