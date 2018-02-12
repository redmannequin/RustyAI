use std::hash::Hash;
use std::collections::HashMap;
use std::collections::BinaryHeap;

use core::node::Node;
use core::state::State;
use core::state::StateCost;
use core::state::Production;

use search::get_path;

pub fn a_start<T>(start: T) -> Vec<T> where T:Hash+State+StateCost+Production<Item=T> {
    
    let mut queue: BinaryHeap<Node<T>> = BinaryHeap::new();
    let mut visited: HashMap<u64,Node<T>> = HashMap::new();

    let mut goal_node: Option<Node<T>> = None;
    queue.push(Node::new(start));
    
    while let Some(node) = queue.pop() {

        let node_id = node.get_id();
        if visited.contains_key(&node_id) { continue; }
        
        match node.get_state() {
            Dead => continue,
            Goal => {
                goal_node = Some(node);
                break; 
            },
        }
        
        for mut neighbor_node in node.production() {
            let neighbor_id = neighbor_node.get_id();
            if !visited.contains_key(&neighbor_id) {
                neighbor_node.add_parent(node_id);
                queue.push(neighbor_node);
            }
        }

        visited.insert(node_id, node);
    }
    
    return get_path(goal_node, &mut visited);
}