//! # Task Management System
//!
//! ## Basics
//!
//! A task is defined as a function that returns a [`Result`].
//!
//! In some cases, it may be convenient to return the [`Result`] from the `Builder` of a struct (represent the task with a struct).
//!
//! ## Tasks-as-types
//!
//! Rust has strong static types. These types can be seen as "tasks" that need to be completed. The tasks can be "completed" by returning a value of a specific type. If a type represents a task, then a value represents a result of the completion of that task.
//!
//! In other words:
//!
//! * A task is defined as a Rust type.
//! * A task with multiple dependencies is defined as `struct` (each dependency becomes a struct field).
//! * A task with multiple ways to complete is defined as `enum` (each way to complete becomes an enum variant).
//! * A task result is defined as a value of a Rust type.
//! * A task result is produced by a [`Builder`](https://crates.io/crates/derive_builder).
//! * If the builder returns an error, this error can be displayed to a human that should complete the task.

mod types;
pub use types::*;
