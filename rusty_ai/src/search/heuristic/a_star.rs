use std::hash::Hash;
use std::collections::HashSet;
use std::collections::BinaryHeap;

use core::node::Node;
use core::state::State;
use core::state::StateCost;
use core::state::Production;

pub fn a_start<T>(start: T) -> Vec<T> where T:Hash+State+StateCost+Production<Item=T> {
    let mut queue: BinaryHeap<Node<T>> = BinaryHeap::new();
    let mut visited: HashSet<u64> = HashSet::new();
    
    let mut node_id:u64;
    let mut node: Node<T> = Node::new(start);

    queue.push(node);
    while !queue.is_empty() {

        node = queue.pop().unwrap();
        node_id = node.get_id();

        if visited.contains(&node_id) { continue; }
        visited.insert(node_id.clone());
        
        if node.get_data().is_goal() { break; }
        if node.get_data().is_end_state() { continue; }

        for mut neighbor_node in node.production() {
            let neighbor_id = neighbor_node.get_id();
            if !visited.contains(&neighbor_id) {
                neighbor_node.add_parent(node_id);
                queue.push(neighbor_node);
            }
        }
    }

    let mut final_path:Vec<T> = Vec::new();
    return final_path;
}