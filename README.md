<!-- DO NOT EDIT -->
<!-- This file is automatically generated by README.ts. -->
<!-- Edit README.ts if you want to make changes. -->

# Task management system

[![Build](https://github.com/DenisGorbachev/tms/actions/workflows/ci.yml/badge.svg)](https://github.com/DenisGorbachev/tms)
[![Documentation](https://docs.rs/tms/badge.svg)](https://docs.rs/tms)

## Task Management System

### Basics

A task is defined as a function that returns a [`::std::result::Result`][__link0].

In some cases, it may be convenient to return the [`::std::result::Result`][__link1] from the `Builder` of a struct (represent the task with a struct).

### Tasks-as-types

Rust has strong static types. These types can be seen as “tasks” that need to be completed. The tasks can be “completed” by returning a value of a specific type. If a type represents a task, then a value represents a result of the completion of that task.

In other words:

* A task is defined as a Rust type.
* A task with multiple dependencies is defined as `struct` (each dependency becomes a struct field).
* A task with multiple ways to complete is defined as `enum` (each way to complete becomes an enum variant).
* A task result is defined as a value of a Rust type.
* A task result is produced by a [`Builder`][__link2].
* If the builder returns an error, this error can be displayed to a human that should complete the task.

   [__link0]: https://doc.rust-lang.org/stable/std/?search=result::Result
 [__link1]: https://doc.rust-lang.org/stable/std/?search=result::Result
 [__link2]: https://crates.io/crates/derive_builder


## Installation

```shell
cargo add tms url
```

**Important:** add the `url` crate too.

## Gratitude

Like the project? [⭐ Star this repo](https://github.com/DenisGorbachev/tms) on GitHub!

## License

[Apache License 2.0](LICENSE-APACHE) or [MIT License](LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this crate by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
