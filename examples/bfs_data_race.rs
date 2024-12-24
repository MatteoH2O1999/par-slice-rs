use par_slice::*;
use std::{
    sync::{atomic::*, Mutex},
    thread::scope,
};

const NUM_THRREADS: usize = 4;
const TREE_DEPTHS: &[u32] = &[2, 3, 4, 5, 6, 7, 8];

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

    fn num_nodes(&self) -> usize {
        self.successors.len()
    }
}

fn expected_dists(depth: u32) -> Vec<isize> {
    let mut v = Vec::new();
    for i in 0..depth {
        for _ in 0..2_usize.pow(i) {
            v.push(i.try_into().unwrap());
        }
    }
    v
}

#[test]
fn test_pointer_bfs() {
    main();
}

pub fn main() {
    for &depth in TREE_DEPTHS {
        let tree = Tree::new(depth);
        let mut dists = vec![-1; tree.num_nodes()];

        let visited = (0..tree.num_nodes())
            .into_iter()
            .map(|i| AtomicBool::new(i == 0))
            .collect::<Vec<_>>();
        let mut current_frontier = vec![0];
        let next_frontier = Mutex::new(Vec::new());
        let mut current_dist = 0;
        let cursor = AtomicUsize::new(0);

        // Let's compute distances from node 0
        {
            let dists = dists.as_data_race_par_slice();

            while !current_frontier.is_empty() {
                cursor.store(0, Ordering::Relaxed);
                scope(|s| {
                    for _ in 0..NUM_THRREADS {
                        s.spawn(|| {
                            while let Some(&node) =
                                current_frontier.get(cursor.fetch_add(1, Ordering::Relaxed))
                            {
                                unsafe {
                                    // Safety: each node is accessed exactly once because of
                                    // AtomicBool::swap so no data races can happen
                                    // and node is always < dists.len() by construction
                                    dists.set_unchecked(node, current_dist)
                                }

                                for &succ in tree.successors(node) {
                                    if !visited[succ].swap(true, Ordering::Relaxed) {
                                        next_frontier.lock().unwrap().push(succ);
                                    }
                                }
                            }
                        });
                    }
                });
                current_dist += 1;
                current_frontier.clear();
                std::mem::swap(&mut current_frontier, &mut next_frontier.lock().unwrap());
            }
        }

        assert_eq!(dists, expected_dists(depth));
    }
}
