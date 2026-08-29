# The Crate Workshop

In Python a module *is* a file, and `import` runs that file. In Rust, modules are declared
in code, they form one tree the compiler assembles at build time, and `use` runs nothing —
it only creates a shorter name. On top of that, everything is private by default, and
"private" is enforced by the compiler rather than suggested by a leading underscore.

A note on layout: this module's exercises each compile as a single file, so everything here
uses inline `mod name { ... }` blocks. In a real project you write `mod name;` and put the
body in `src/name.rs`. Paths and privacy behave identically either way — files are just a
delivery mechanism for the same tree.

## Coming from Python/JS

Your mental model is probably "import path = file path". `import app.config` finds
`app/config.py` at runtime, executes it, and hands back a namespace; `_secret` is private
only by politeness; a circular import explodes when the program runs. TypeScript is closer
— `export` marks the public surface — but the path is still the file.

Rust separates the two ideas. A *crate* is one compilation unit (a `main.rs` binary or a
`lib.rs` library) whose root module is named `crate`; a *package* is what Cargo.toml
describes, one or more crates. Inside a crate, `mod` declares a node in the module tree,
and paths address that tree: `crate::app::config` from the root, `self::` here, `super::`
for the parent.

Why this way? The whole tree is known at compile time: no runtime resolution to fail, no
import side effects to sequence, and modules referring to each other in a cycle is a
non-issue. Privacy becomes a checked API boundary rather than a convention — if a field is
private, no caller anywhere can touch it, and you keep the right to change it.

## The rules

1. **`mod name { ... }` declares a module**, nested as deeply as you like. The crate root
   is `crate`.
2. **Private by default — and private means "this module and its descendants".** A child
   module can call `super::helper()` even when `helper` is private; a parent cannot reach
   into a child's private items. Visibility flows downward, never up.
3. **Every segment of a path must be reachable.** A `pub fn` inside a private `mod` is
   still unreachable from outside.
4. **`pub` on a struct does not publish its fields** — each needs its own `pub`. A private
   field is how you force callers through your constructor. `pub` on an enum *does*
   publish all of its variants.
5. **Narrower visibility exists**: `pub(crate)` is public within this crate only,
   `pub(super)` only to the parent module.

```rust
mod app {
    pub mod config {
        pub(super) fn secret_key() -> u32 { 7 } // only the parent module `app`
        pub fn version() -> u32 { 3 }           // anyone
    }

    pub(crate) fn boot() -> u32 {               // anywhere in this crate
        config::secret_key() + config::version()
    }
}

fn main() {
    println!("{} {}", app::boot(), app::config::version());
}
```

6. **`use` is an alias**, in scope only where it is written. `pub use` re-exports a name,
   letting callers use a shorter path than your internal one.

## Worked example

```rust
mod inventory {
    pub mod items {
        pub struct Potion {
            pub name: String,
            strength: u32, // private: only `items` and its children see this
        }

        impl Potion {
            pub fn brew(name: &str, strength: u32) -> Potion {
                Potion { name: name.to_string(), strength: strength.min(100) }
            }

            pub fn strength(&self) -> u32 {
                self.strength
            }
        }
    }

    fn house_discount() -> u32 {
        10
    }

    pub mod pricing {
        use super::items::Potion;

        pub fn price(p: &Potion) -> u32 {
            // A child module can reach its ancestors' private items.
            p.strength() * 3 - super::house_discount()
        }
    }
}

use inventory::items::Potion;
use inventory::pricing::price;

fn main() {
    let p = Potion::brew("Elixir", 12);
    println!("{} costs {} gold", p.name, price(&p));
}
```

Note what privacy buys here: `brew` clamps `strength` to 100, and nobody outside `items`
can set it to 5000 afterwards.

## The error you will hit

Marking the function `pub` but forgetting the module that contains it:

```rust
mod engine {
    mod fuel {
        pub fn burn(litres: u32) -> u32 {
            litres * 9
        }
    }
}

fn main() {
    println!("{}", engine::fuel::burn(3));
}
```

```
error[E0603]: module `fuel` is private
  --> src/main.rs:10:28
   |
10 |     println!("{}", engine::fuel::burn(3));
   |                            ^^^^  ---- function `burn` is not publicly re-exported
   |                            |
   |                            private module
   |
note: the module `fuel` is defined here
  --> src/main.rs:2:5
   |
 2 |     mod fuel {
   |     ^^^^^^^^
```

The caret sits on `fuel`, not on `burn` — the function is public, the door to it is not.
Make the module `pub`, or, if `fuel` is an implementation detail, add
`pub use fuel::burn;` inside `engine` so callers say `engine::burn(3)`. That is what the
re-export note is pointing at, and it is how most crates present a tidy public API over a
messier internal tree.

## What's next

`organizing_code` (a `math` module with a `stats` sub-module and a `Validator` with private
state), `use_paths` (fix broken `use` statements), and `visibility` (open a restaurant, one
`pub` at a time).

Source: https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html
