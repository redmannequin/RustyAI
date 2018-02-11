use std::hash::Hash;
use std::collections::BinaryHeap;

use core::node::Node;

use core::state::State;
use core::state::Production;

pub fn a_start<T>(start: T) -> Vec<T> where T:Hash+State+Production<Item=T> {
    //let mut heap = BinaryHeap::new();
    let mut final_path:Vec<T> = Vec::new();
    return final_path;
}