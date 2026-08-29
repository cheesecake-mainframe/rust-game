# The Builders' Guild

In Python, one `class` block holds both the data and the behavior. Rust splits that in
two: a `struct` declares the fields and their types, and a separate `impl` block holds
the functions that operate on them. There is no `__init__`, no `self.x = x` that invents
a field on the fly, and no way to build a value with half its fields missing.

## Coming from Python/JS

You are used to writing this:

```python
class Session:
    def __init__(self, user):
        self.user = user
        self.hits = 0

    def visit(self):
        self.hits += 1
```

The Rust equivalent:

```rust
struct Session {
    user: String,
    hits: u32,
}

impl Session {
    fn new(user: &str) -> Self {   // `Self` is an alias for `Session`
        Session { user: user.to_string(), hits: 0 }
    }

    fn visit(&mut self) {
        self.hits += 1;
    }
}
```

Three things changed. First, the fields are declared with types up front, so the compiler
knows a `Session` is exactly a `String` plus a `u32` — a fixed layout, not a per-instance
dictionary. Second, `new` is not special. It is an ordinary function that happens to
return `Self`; you could name it `open` or write three of them (`new`, `from_id`,
`restored`). It lives under the type's namespace, so you call it `Session::new("ada")`
with `::`, not on an instance. Third, `self` is an explicit parameter, and its *form*
carries the ownership rules you learned in the last two modules right into method calls.

Why split data from behavior at all? Because behavior arrives from more than one place:
your own `impl` block, plus trait implementations (module 11) written later, possibly by
someone else. Keeping the struct as pure layout lets capabilities be bolted on
independently.

## The rules

1. **Every field is declared, and every field must be supplied.** No defaults, no partial
   construction. When a local variable already has the field's name, field init shorthand
   lets you write `Session { user, hits }` instead of `user: user, hits: hits`.
2. **The first parameter picks the borrow.** `&self` reads, `&mut self` mutates (and the
   caller's binding must itself be `mut`), plain `self` consumes the value — the caller
   cannot use it afterwards. No `self` at all means an *associated function*, called as
   `Type::name()`.
3. **Everything is private by default, fields included.** Private means visible to the
   module that defined it and that module's children — not to the rest of the crate.
   `pub` on the struct does *not* make its fields public; each one needs its own `pub`.
4. **Tuple structs have positional fields**, reached with `.0`, `.1`. Wrapping a single
   value — `struct Meters(f64)` — is the newtype pattern: `Meters` and `Seconds` become
   incompatible types, so the compiler catches unit mix-ups that a bare `f64` cannot.
5. **Nothing prints by itself.** `{:?}` requires `#[derive(Debug)]`; `{}` requires a
   hand-written `Display` impl.

## Worked example

```rust
#[derive(Debug)]
struct Torch {
    fuel: f64,
    lit: bool,
}

struct Ashes {
    grams: f64,
}

impl Torch {
    // No `self` parameter: an associated function, called as Torch::new(..).
    fn new(fuel: f64) -> Torch {
        Torch { fuel: fuel.max(0.0), lit: false }
    }

    fn brightness(&self) -> f64 {          // borrows, read-only
        if self.lit { self.fuel * 0.8 } else { 0.0 }
    }

    fn burn(&mut self, minutes: f64) {     // borrows, read-write
        self.fuel = (self.fuel - minutes * 0.5).max(0.0);
        if self.fuel == 0.0 {
            self.lit = false;
        }
    }

    fn into_ashes(self) -> Ashes {         // consumes the Torch
        Ashes { grams: self.fuel * 2.0 }
    }
}

struct Meters(f64); // tuple struct: fields by position, not name

fn main() {
    let mut torch = Torch::new(10.0);
    torch.lit = true;
    torch.burn(4.0);
    println!("{torch:?} brightness {:.1}", torch.brightness());

    let ashes = torch.into_ashes(); // torch is moved here; unusable after
    println!("{:.1} g of ash", ashes.grams);

    let depth = Meters(3.5);
    println!("{} m", depth.0);
}
```

## The error you will hit

Calling a `&mut self` method — here `Counter::bump` — on a binding you declared without
`mut`:

```
error[E0596]: cannot borrow `counter` as mutable, as it is not declared as mutable
  --> src/main.rs:17:5
   |
17 |     counter.bump();
   |     ^^^^^^^ cannot borrow as mutable
   |
help: consider changing this to be mutable
   |
16 |     let mut counter = Counter::new();
   |         +++
```

Mutability lives on the *binding*, not on the method. `counter.bump()` is sugar for
`Counter::bump(&mut counter)`, and you cannot take a `&mut` to something you promised not
to change. In Python every attribute is writable and nothing announces which methods
mutate; in Rust the signature tells you, and the call site has to agree.

## What's next

`defining_structs` (privacy and construction errors), `methods` (build a full `impl`
block for `Rectangle`), and `tuple_structs` (positional fields and the newtype pattern).

Source: https://doc.rust-lang.org/book/ch05-01-defining-structs.html
