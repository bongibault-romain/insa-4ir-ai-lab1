use crate::board::*;
use crate::heuristics::*;
use crate::min_heap::*;
use std::collections::*;
use std::time::Duration;
use std::u32::MAX;

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

pub fn search(init_state: Board, heuristic: Heuristic) -> (Option<Vec<Direction>>, Stats) {
    // record the start time when starting the search (so we can later the time that elapsed since)
    let start = std::time::Instant::now();

    // frontier: MinHeap provide allows to store the states to explore, with associated priority
    let mut heap: MinHeap<Board> = MinHeap::new();

    // the standard library provides a HashMap, that can be used to store the cost and predecessors of each state
    // assocaciates each state on the frontier to the best cost to reach it
    let mut path_costs: HashMap<Board, u32> = HashMap::new();
    // assocaciates each state on the frontier to the its best parent state and the action to it (parent, action)
    let mut predecessors: HashMap<Board, (Board, Direction)> = HashMap::new();

    // keeps track all states that have been expanded
    let mut expanded: HashSet<Board> = HashSet::new();

    let directions = [Direction::Left, Direction::Right, Direction::Up, Direction::Down];
    
    let mut result: Option<Vec<Direction>> = None;

    heap.insert(init_state, 0);
    path_costs.insert(init_state, 0);

    while !heap.is_empty() {
        let s: Board = heap.pop().unwrap();
        expanded.insert(s);

        if (s == Board::GOAL) {
            let mut current: Board = Board::GOAL;
            let mut r: Vec<Direction> = Vec::new();
        
            while current != init_state {
                let (parent, direction) = predecessors.get(&current).unwrap();         
        
                r.insert(0, *direction);
                current = *parent;
            }

            result = Some(r);

            break;
        }

        let s_cost = *path_costs.get(&s).unwrap();

        for direction in directions {
            let child = s.apply(direction);

            if (child.is_none() || expanded.contains(&child.unwrap())) {
                continue;
            }

            let unwrapped_child = child.unwrap();
            let child_cost = match path_costs.get(&unwrapped_child) {
                Some(a) => *a,
                None => u32::MAX
            };

            let h_value = heuristic.estimate(&unwrapped_child);

            if (child_cost > s_cost + 1) {
                path_costs.insert(unwrapped_child, s_cost + 1);
                predecessors.insert(unwrapped_child, (s, direction));
                heap.insert(unwrapped_child, s_cost + 1 + h_value);
            } else {
                heap.insert(unwrapped_child, child_cost + h_value);
            }
        }
    }

    // here is an example to measure the runtime and returns the statistics
    let runtime = start.elapsed();
    // example to construct a Stats instance
    let stats = Stats::new(expanded.len(), runtime);
    // return the results and associated stats

    (result, stats)
}

#[cfg(test)]
mod test {

    #[test]
    fn test_search() {
        use super::*;

        // validates that search does return the optimal plan on the first 20 isntances

        for (expected_cost, init) in &INSTANCES[0..20] {
            let (path, stats) = search(*init, Heuristic::Blind);
            let path = path.expect("no plan");
            assert!(init.is_valid_plan(&path));
            assert_eq!(path.len(), *expected_cost as usize);
        }
    }
}
