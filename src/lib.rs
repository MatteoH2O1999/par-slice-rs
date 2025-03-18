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
//!   but dereferencing them while avoiding data races and abiding by Rust's aliasing
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
//! But why should I want this?
//!
//! This is particularily useful in `Breadth-First visits` situations, especially on
//! data structures like graphs, when we want to be able to access in parallel
//! arbitrary data but with the BFS guarantee of not visiting the same node twice.
//!
//! Take the following trait for instance:
//!
//! ```
//! pub trait Graph {
//!     fn num_nodes(&self) -> usize;
//!     fn successors(&self, index: usize) -> impl Iterator<Item = usize>;
//! }
//! ```
//!
//! Implementing a breadth-first visit from this trait is easy:
//!
//! ```
//!# use std::sync::atomic::*;
//!# use std::sync::Mutex;
//!# use std::thread::scope;
//!# pub trait Graph {
//!#     fn num_nodes(&self) -> usize;
//!#     fn successors(&self, index: usize) -> impl Iterator<Item = usize>;
//!# }
//!#
//! const NUM_THREADS: usize = 4;
//!
//! pub fn breadth_first_visit(graph: impl Graph + Sync, start: usize) {
//!     let visited: Vec<AtomicBool> = (0..graph.num_nodes()).map(|_| AtomicBool::new(false)).collect();
//!     let mut current_frontier = vec![start];
//!     let mut next_frontier = Mutex::new(Vec::new());
//!     let cursor = AtomicUsize::new(0);
//!     visited[start].store(true, Ordering::Relaxed);
//!
//!     while !current_frontier.is_empty() {
//!         cursor.store(0, Ordering::Relaxed);
//!         scope(|s| {
//!             for _ in 0..NUM_THREADS {
//!                 s.spawn(|| {
//!                     while let Some(&node) = current_frontier.get(cursor.fetch_add(1, Ordering::Relaxed)) {
//!                         for succ in graph.successors(node) {
//!                             if !visited[succ].swap(true, Ordering::Relaxed) {
//!                                 next_frontier.lock().unwrap().push(succ);
//!                             }
//!                         }
//!                     }
//!                 });
//!             }
//!         });
//!         current_frontier.clear();
//!         std::mem::swap(&mut current_frontier, &mut next_frontier.lock().unwrap());
//!     }
//! }
//! ```
//!
//! But what if we wanted to execute arbitrary code for every node?
//!
//! ```compile_fail
//!# use std::sync::atomic::*;
//!# use std::sync::Mutex;
//!# use std::thread::scope;
//!# pub trait Graph {
//!#     fn num_nodes(&self) -> usize;
//!#     fn successors(&self, index: usize) -> impl Iterator<Item = usize>;
//!# }
//!#
//! const NUM_THREADS: usize = 4;
//!
//! // This does not compile as closure must be Sync
//! pub fn breadth_first_visit(graph: impl Graph + Sync, start: usize, mut closure: impl FnMut(usize, usize) + Sync) {
//!     let visited: Vec<AtomicBool> = (0..graph.num_nodes()).map(|_| AtomicBool::new(false)).collect();
//!     let mut current_frontier = vec![start];
//!     let mut next_frontier = Mutex::new(Vec::new());
//!     let cursor = AtomicUsize::new(0);
//!     let mut dist = 0;
//!     visited[start].store(true, Ordering::Relaxed);
//!
//!     while !current_frontier.is_empty() {
//!         cursor.store(0, Ordering::Relaxed);
//!         scope(|s| {
//!             for _ in 0..NUM_THREADS {
//!                 s.spawn(|| {
//!                     while let Some(&node) = current_frontier.get(cursor.fetch_add(1, Ordering::Relaxed)) {
//!                         closure(node, dist);
//!                         for succ in graph.successors(node) {
//!                             if !visited[succ].swap(true, Ordering::Relaxed) {
//!                                 next_frontier.lock().unwrap().push(succ);
//!                             }
//!                         }
//!                     }
//!                 });
//!             }
//!         });
//!         dist += 1;
//!         current_frontier.clear();
//!         std::mem::swap(&mut current_frontier, &mut next_frontier.lock().unwrap());
//!     }
//! }
//!
//! pub fn compute_dists(graph: impl Graph + Sync, start: usize) -> Vec<usize> {
//!     let mut dists = vec![usize::MAX; graph.num_nodes()];
//!     breadth_first_visit(graph, start, |node, dist| dists[node] = dist);
//!     dists
//! }
//! ```
//!
//! This compiles but limits heavily what we can do in the closure: this is where this
//! crate comes in as we know no node is ever visited twice.
//! Thus we can update `dists` without using atomics.
//!
//! ```
//!# use std::sync::atomic::*;
//!# use std::sync::Mutex;
//!# use std::thread::scope;
//!# pub trait Graph {
//!#     fn num_nodes(&self) -> usize;
//!#     fn successors(&self, index: usize) -> impl Iterator<Item = usize>;
//!# }
//!#
//! use par_slice::*;
//! const NUM_THREADS: usize = 4;
//!
//! pub fn breadth_first_visit(graph: impl Graph + Sync, start: usize, closure: impl Fn(usize, usize) + Sync) {
//!     let visited: Vec<AtomicBool> = (0..graph.num_nodes()).map(|_| AtomicBool::new(false)).collect();
//!     let mut current_frontier = vec![start];
//!     let mut next_frontier = Mutex::new(Vec::new());
//!     let cursor = AtomicUsize::new(0);
//!     let mut dist = 0;
//!     visited[start].store(true, Ordering::Relaxed);
//!
//!     while !current_frontier.is_empty() {
//!         cursor.store(0, Ordering::Relaxed);
//!         scope(|s| {
//!             for _ in 0..NUM_THREADS {
//!                 s.spawn(|| {
//!                     while let Some(&node) = current_frontier.get(cursor.fetch_add(1, Ordering::Relaxed)) {
//!                         closure(node, dist);
//!                         for succ in graph.successors(node) {
//!                             if !visited[succ].swap(true, Ordering::Relaxed) {
//!                                 next_frontier.lock().unwrap().push(succ);
//!                             }
//!                         }
//!                     }
//!                 });
//!             }
//!         });
//!         dist += 1;
//!         current_frontier.clear();
//!         std::mem::swap(&mut current_frontier, &mut next_frontier.lock().unwrap());
//!     }
//! }
//!
//! pub fn compute_dists(graph: impl Graph + Sync, start: usize) -> Vec<usize> {
//!     let mut dists = vec![usize::MAX; graph.num_nodes()];
//!     {
//!         let dists_shared = dists.as_par_index();
//!         breadth_first_visit(graph, start, |node, dist| {
//!             let node_ref = unsafe { dists_shared.get_mut_unchecked(node) };
//!             *node_ref = dist;
//!         });
//!     }
//!     dists
//! }
//! ```
//!
//! [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
mod impls;
pub use impls::*;

mod traits;
pub use traits::*;
