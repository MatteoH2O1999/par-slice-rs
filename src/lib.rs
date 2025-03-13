//! ParSlice is a utility crate to allow easier access to data in parallel
//! when data races are avoided at compile time or through other means but the
//! compiler has no way to know.
//!
//! # Basic usage
//!
//! On a basic level, using this crate is easy:
//!
//! ```
//! use par_slice::*;
//! use std::thread::scope;
//!
//! // Let's create a slice accessible in parallel with 6 values initialized to 0.
//! let slice = ParSlice::with_value(0, 6);
//!
//! // Let's update even indexes to 42 in one thread and odd indexes
//! // to 69 in another thread.
//! scope(|s|{
//!     s.spawn(|| {
//!         for i in 0..6 {
//!             if i % 2 == 0 {
//!                 let mut_ref = unsafe { slice.get_mut(i) };
//!                 *mut_ref = 42;
//!             }
//!         }
//!     });
//!     s.spawn(|| {
//!         for i in 0..6 {
//!             if i % 2 != 0 {
//!                 let mut_ref = unsafe { slice.get_mut(i) };
//!                 *mut_ref = 69;
//!             }
//!         }
//!     });
//! });
//!
//! // Let's convert the parallel slice into a boxed slice.
//! let boxed_slice = slice.into();
//!
//! assert_eq!(boxed_slice.as_ref(), &[42, 69, 42, 69, 42, 69]);
//! ```
//!
//! At the same time, though, it is extremely easy to geneate undefined behavior:
//!
//! ```no_run
//! use par_slice::*;
//! use std::thread::scope;
//!
//! // Let's create a slice accessible in parallel with 6 values initialized to 0.
//! let slice = ParSlice::with_value(0, 6);
//!
//! // Let's update even indexes to 42 in one thread and odd indexes
//! // to 69 in another thread.
//! // This is UB as the two threads may hold a mutable reference to the same element,
//! // thus violating Rust's aliasing rules.
//! scope(|s|{
//!     s.spawn(|| {
//!         for i in 0..6 {
//!             let mut_ref = unsafe { slice.get_mut(i) };
//!             if i % 2 == 0 {
//!                 *mut_ref = 42;
//!             }
//!         }
//!     });
//!     s.spawn(|| {
//!         for i in 0..6 {
//!             let mut_ref = unsafe { slice.get_mut(i) };
//!             if i % 2 != 0 {
//!                 *mut_ref = 69;
//!             }
//!         }
//!     });
//! });
//!
//! // Let's convert the parallel slice into a boxed slice.
//! let boxed_slice = slice.into();
//!
//! assert_eq!(boxed_slice.as_ref(), &[42, 69, 42, 69, 42, 69]);
//! ```
//!
//! # Access Paradigms
//!
//! In order to reduce the risk of producing UB, this crate offers 3 levels of access,
//! each with their invariants:
//! * [`PointerIndex`] and [`PointerChunkIndex`] allow access through pointers, allowing the
//!   maximum safety at the cost of ergonomics: creating the pointers is always safe,
//!   but dereferencing them while avoiding data races and abiding by Rust's alising
//!   rules is up to the user.
//! * [`UnsafeNoRefIndex`] and [`UnsafeNoRefChunkIndex`] allow access through setters and
//!   getters. This allows the user to not think about reference aliasing and lifetimes
//!   (as no references are ever created) and to only handle the possibility of data races.
//! * [`UnsafeIndex`] and [`UnsafeChunkIndex`] allow access through references, allowing the
//!   maximum ergonomics at the cost of safety: using the references is always safe,
//!   but the user must guarantee that Rust's aliasing rules are always respected
//!   (under penalty of [undefined behavior]).
//!
//! # Real-World Use Case
//!
//! [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
mod impls;
pub use impls::*;

mod traits;
pub use traits::*;
