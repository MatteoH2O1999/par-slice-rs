use par_slice::*;
use rayon::prelude::*;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Mutex,
};

struct Tree {
    successors: Vec<Vec<usize>>,
}

impl Tree {
    fn new(levels: u32) -> Self {
        let mut successors = Vec::new();
        let levels = levels - 1;

        let mut current_level = 0;
        let mut current_successor = 1;

        while current_level != levels {
            let nodes_at_level = 2_usize.pow(current_level);
            for _ in 0..nodes_at_level {
                successors.push(vec![current_successor, current_successor + 1]);
                current_successor += 2;
            }
            current_level += 1;
        }
        for _ in 0..2_usize.pow(levels) {
            successors.push(Vec::new());
        }

        Self { successors }
    }

    fn successors(&self, node: usize) -> &[usize] {
        self.successors[node].as_slice()
    }
}

#[test]
fn test_pointer_bfs() {
    let graph = Tree::new(4);

    let visited = (0..15)
        .into_iter()
        .map(|i| AtomicBool::new(i == 0))
        .collect::<Vec<_>>();
    let mut dists = vec![-1; 15];

    let mut current_frontier = vec![0];
    let next_frontier = Mutex::new(Vec::new());
    let mut current_dist = 0;

    {
        let dists = dists.as_pointer_par_slice();
        while !current_frontier.is_empty() {
            current_frontier.par_iter().for_each(|&node| {
                // Update dist value
                let dist_ptr = dists.get_mut_ptr_unchecked(node);
                unsafe {
                    // Safety: each node is accessed exactly once because of
                    // AtomicBool::swap so no data races can happen
                    // and node is always < dists.len() by construction
                    *dist_ptr = current_dist;
                }

                // Enumerate successors for next frontier
                for &succ in graph.successors(node) {
                    if !visited[succ].swap(true, Ordering::Relaxed) {
                        next_frontier.lock().unwrap().push(succ);
                    }
                }
            });
            current_dist += 1;
            current_frontier.clear();
            std::mem::swap(&mut current_frontier, &mut next_frontier.lock().unwrap());
        }
    }

    assert_eq!(dists, vec![0, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 3]);
}

#[test]
fn test_data_race_bfs() {
    let graph = Tree::new(4);

    let visited = (0..15)
        .into_iter()
        .map(|i| AtomicBool::new(i == 0))
        .collect::<Vec<_>>();
    let mut dists = vec![-1; 15];

    let mut current_frontier = vec![0];
    let next_frontier = Mutex::new(Vec::new());
    let mut current_dist = 0;

    {
        let dists = dists.as_data_race_par_slice();
        while !current_frontier.is_empty() {
            current_frontier.par_iter().for_each(|&node| {
                // Update dist value
                unsafe {
                    // Safety: each node is accessed exactly once because of
                    // AtomicBool::swap so no data races can happen
                    // and node is always < dists.len() by construction
                    dists.set_unchecked(node, current_dist);
                }

                // Enumerate successors for next frontier
                for &succ in graph.successors(node) {
                    if !visited[succ].swap(true, Ordering::Relaxed) {
                        next_frontier.lock().unwrap().push(succ);
                    }
                }
            });
            current_dist += 1;
            current_frontier.clear();
            std::mem::swap(&mut current_frontier, &mut next_frontier.lock().unwrap());
        }
    }

    assert_eq!(dists, vec![0, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 3]);
}

#[test]
fn test_unsafe_bfs() {
    let graph = Tree::new(4);

    let visited = (0..15)
        .into_iter()
        .map(|i| AtomicBool::new(i == 0))
        .collect::<Vec<_>>();
    let mut dists = vec![-1; 15];

    let mut current_frontier = vec![0];
    let next_frontier = Mutex::new(Vec::new());
    let mut current_dist = 0;

    {
        let dists = dists.as_unsafe_par_slice();
        while !current_frontier.is_empty() {
            current_frontier.par_iter().for_each(|&node| {
                // Update dist value
                let dist = unsafe {
                    // Safety: each node is accessed exactly once because of
                    // AtomicBool::swap, so no two &mut to the same memory can
                    // exist and node is always < dists.len() by construction
                    dists.get_mut_unchecked(node)
                };
                *dist = current_dist;

                // Enumerate successors for next frontier
                for &succ in graph.successors(node) {
                    if !visited[succ].swap(true, Ordering::Relaxed) {
                        next_frontier.lock().unwrap().push(succ);
                    }
                }
            });
            current_dist += 1;
            current_frontier.clear();
            std::mem::swap(&mut current_frontier, &mut next_frontier.lock().unwrap());
        }
    }

    assert_eq!(dists, vec![0, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 3]);
}
