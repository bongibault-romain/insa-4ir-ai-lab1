use crate::board::*;
use crate::heuristics::*;
use crate::min_heap::*;
use std::collections::*;
use std::time::Duration;

/// Statistics of the search, used to evaluate the performance of the search algorithms.
/// Feel free to add more fields to this struct if you need them.
pub struct Stats {
    /// Numbers of states expanded during search
    pub expanded: usize,
    /// Total runtime spend in the search.
    ///
    /// ```rust
    /// let start_time: Instant = std::time::Instant::now();
    /// // do something
    /// let runtime: Duration = start_time.elapsed();
    /// ```
    pub runtime: Duration,
}

impl Stats {
    /// Creates a new `Stats` instance with the given expanded states count and runtime.
    pub fn new(expanded: usize, runtime: Duration) -> Stats {
        Stats { expanded, runtime }
    }
}

pub fn search(init_state: Board) -> (Option<Vec<Direction>>, Stats) {
    let start = std::time::Instant::now();
    // MinHeap provide allows to store the states to explore, with associated priority
    let mut heap: MinHeap<Board> = MinHeap::new();
    // the standard library provides a HashMap, that can be used to store the cost or other things
    let mut costs: HashMap<Board, u32> = HashMap::new();
    // ...

    // here is an example to measure the runtime and returns the statistics
    let runtime = start.elapsed();
    // example to construct a Stats instance
    let stats = Stats::new(0, runtime);
    // return the results and associated stats
    (todo!(), stats)
}

#[cfg(test)]
mod test {

    #[test]
    fn test_search() {
        use super::*;

        // validates that search oes return the optimal plan on the first 20 isntances

        for (expected_cost, init) in &INSTANCES[0..20] {
            let (path, stats) = search(*init);
            let path = path.expect("no plan");
            assert!(init.is_valid_plan(&path));
            assert_eq!(path.len(), *expected_cost as usize);
        }
    }
}
