# par_slice

[![downloads](https://img.shields.io/crates/d/par_slice)](https://crates.io/crates/par_slice)
[![dependents](https://img.shields.io/librariesio/dependents/cargo/par_slice)](https://crates.io/crates/par_slice/reverse_dependencies)
![license](https://img.shields.io/crates/l/par_slice)
[![lines of code](https://tokei.rs/b1/github/MatteoH2O1999/par-slice-rs)](https://github.com/MatteoH2O1999/par-slice-rs)
[![Latest version](https://img.shields.io/crates/v/par_slice.svg)](https://crates.io/crates/par_slice)
[![Documentation](https://docs.rs/par_slice/badge.svg)](https://docs.rs/par_slice)

ParSlice is a utility crate to allow easier access to data in parallel when data races
are avoided at compile time or through other means but the compiler has no way to know.

## Basic usage

On a basic level, using this crate is easy:

```rust
use par_slice::*;
use std::thread::scope;

// Let's create a slice accessible in parallel with 6 values initialized to 0.
let slice = ParSlice::with_value(0, 6);

// Let's update even indexes to 42 in one thread and odd indexes
// to 69 in another thread.
scope(|s|{
    s.spawn(|| {
        for i in 0..6 {
            if i % 2 == 0 {
                let mut_ref = unsafe { slice.get_mut(i) };
                *mut_ref = 42;
            }
        }
    });
    s.spawn(|| {
        for i in 0..6 {
            if i % 2 != 0 {
                let mut_ref = unsafe { slice.get_mut(i) };
                *mut_ref = 69;
            }
        }
    });
});

// Let's convert the parallel slice into a boxed slice.
let boxed_slice = slice.into();

assert_eq!(boxed_slice.as_ref(), &[42, 69, 42, 69, 42, 69]);
```

At the same time, though, it is extremely easy to geneate undefined behavior:

```rust
use par_slice::*;
use std::thread::scope;

// Let's create a slice accessible in parallel with 6 values initialized to 0.
let slice = ParSlice::with_value(0, 6);

// Let's update even indexes to 42 in one thread and odd indexes
// to 69 in another thread.
// This is UB as the two threads may hold a mutable reference to the same element,
// thus violating Rust's aliasing rules.
scope(|s|{
    s.spawn(|| {
        for i in 0..6 {
            let mut_ref = unsafe { slice.get_mut(i) };
            if i % 2 == 0 {
                *mut_ref = 42;
            }
        }
    });
    s.spawn(|| {
        for i in 0..6 {
            let mut_ref = unsafe { slice.get_mut(i) };
            if i % 2 != 0 {
                *mut_ref = 69;
            }
        }
    });
});

// Let's convert the parallel slice into a boxed slice.
let boxed_slice = slice.into();

assert_eq!(boxed_slice.as_ref(), &[42, 69, 42, 69, 42, 69]);
```

## Access Paradigms

In order to reduce the risk of producing UB, this crate offers 3 levels of access,
each with their invariants:

* [`PointerIndex`](https://docs.rs/par_slice/latest/par_slice/trait.PointerIndex.html) and [`PointerChunkIndex`](https://docs.rs/par_slice/latest/par_slice/trait.PointerChunkIndex.html) allow access through pointers, allowing the maximum safety at the cost of ergonomics: creating the pointers is always safe, but dereferencing them while avoiding data races and abiding by Rust's aliasing rules is up to the user.
* [`UnsafeNoRefIndex`](https://docs.rs/par_slice/latest/par_slice/trait.UnsafeNoRefIndex.html) and [`UnsafeNoRefChunkIndex`](https://docs.rs/par_slice/latest/par_slice/trait.UnsafeNoRefChunkIndex.html) allow access through setters and getters. This allows the user to not think about reference aliasing and lifetimes (as no references are ever created) and to only handle the possibility of data races.
* [`UnsafeIndex`](https://docs.rs/par_slice/latest/par_slice/trait.UnsafeIndex.html) and [`UnsafeChunkIndex`](https://docs.rs/par_slice/latest/par_slice/trait.UnsafeChunkIndex.html) allow access through references, allowing the maximum ergonomics at the cost of safety: using the references is always safe, but the user must guarantee that Rust's aliasing rules are always respected (under penalty of [undefined behavior]).

## Real-World Use Case

But why should I want this?

This is particularily useful in `Breadth-First visits` situations, especially on data structures like graphs, when we want to be able to access in parallel arbitrary data but with the BFS guarantee of not visiting the same node twice.
