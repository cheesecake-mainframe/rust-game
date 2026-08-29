# Attribution

rust-game is an original project. Some exercises are inspired by or adapted from
the following open-source resources:

## Rust by Example
- **Source:** https://github.com/rust-lang/rust-by-example
- **License:** MIT / Apache-2.0
- **Usage:** Exercise patterns and concept ordering are informed by Rust by Example.
  All exercises are rewritten to fit rust-game's gamified format, hints, and progression.

## Rustlings
- **Source:** https://github.com/rust-lang/rustlings
- **License:** MIT
- **Usage:** Exercise structure (broken code with TODO markers, test-based verification)
  is inspired by Rustlings. No exercises are copied directly; all content is original
  or substantially rewritten.

## The Rust Programming Language (The Book)
- **Source:** https://doc.rust-lang.org/book/
- **License:** MIT / Apache-2.0
- **Usage:** Module ordering and concept progression follow The Book's chapter structure.

### Lesson chapter citations

Each module lesson in `exercises/lessons/` is written from a specific Book chapter,
recorded as `book_url` in `exercises/info.toml` and as a `Source:` line at the foot
of the lesson. The lessons are original prose written for this project's audience
and format — they are not excerpts of the Book.

| Module | Rust Book chapter |
|---|---|
| `01_getting_started` | https://doc.rust-lang.org/book/ch01-02-hello-world.html |
| `02_variables_types` | https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html |
| `03_functions_control_flow` | https://doc.rust-lang.org/book/ch03-03-how-functions-work.html |
| `04_ownership_moves` | https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html |
| `05_borrowing_references` | https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html |
| `06_structs_methods` | https://doc.rust-lang.org/book/ch05-01-defining-structs.html |
| `07_enums_pattern_matching` | https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html |
| `08_error_handling` | https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html |
| `09_collections` | https://doc.rust-lang.org/book/ch08-00-common-collections.html |
| `10_modules_packages` | https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html |
| `11_generics_traits` | https://doc.rust-lang.org/book/ch10-02-traits.html |
| `12_lifetimes` | https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html |
| `13_closures_iterators` | https://doc.rust-lang.org/book/ch13-01-closures.html |
| `14_smart_pointers` | https://doc.rust-lang.org/book/ch15-05-interior-mutability.html |
| `15_concurrency` | https://doc.rust-lang.org/book/ch16-03-shared-state.html |
| `16_testing_docs` | https://doc.rust-lang.org/book/ch11-01-writing-tests.html |
| `17_macros` | https://doc.rust-lang.org/book/ch20-05-macros.html |
| `18_async_rust` | https://doc.rust-lang.org/book/ch17-05-traits-for-async.html |

---

When adapting exercises, contributors must:
1. Rewrite the exercise to fit rust-game's format (hints, XP, exercise type)
2. Record the source in this file
3. Verify that the adaptation complies with the source's license terms
