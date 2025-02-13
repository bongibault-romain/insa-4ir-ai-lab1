#![allow(unused)] // suppress warnings for unused code (there is plenty when you start)

// declare other modules that are in other files and must be compiled
mod board;
mod heuristics;
mod min_heap;
mod search;

// import the content of the modules
use board::*;
use heuristics::*;
use search::*;

fn main() {
    let (_, mut board) = INSTANCES[31];
    let plan = [Direction::Right, Direction::Up, Direction::Right, Direction::Down];

    let (result, stats) = search(board);

    println!("Path: {:?}", result);
    println!("Time: {:?}, Nodes: {:?}", stats.runtime, stats.expanded)

    // println!("Is the plan valid: {}", board.is_valid_plan(&plan));
    // board.play(&plan);

}
