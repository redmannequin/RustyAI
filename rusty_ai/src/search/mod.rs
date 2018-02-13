use std::hash::Hash;
use std::collections::HashMap;

use core::node::Node;

pub mod blind;
pub mod heuristic;

pub fn get_path<T>(goal_node: Option<Node<T>>, visited: &mut HashMap<u64,Node<T>>) -> Vec<T> where T:Hash {
    let mut final_path:Vec<T> = Vec::new();
    let mut curr_node: Box<Option<Node<T>>> = Box::new(goal_node);
    
    while let Some(mut node) = *curr_node {
        let parents = node.get_parents();
        let mut set_iter = parents.iter(); 
        let node_id = set_iter.next().unwrap().clone();
        final_path.push(node.move_data());
        curr_node = Box::new(visited.remove(&node_id));
    }
    final_path.reverse();
    return final_path;
}