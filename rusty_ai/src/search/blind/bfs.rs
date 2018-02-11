use std::hash::Hash;
use std::collections::HashMap;
use std::collections::VecDeque;

use core::node::Node;

use core::state::State;
use core::state::Production;

pub fn bfs<T>(start:T) -> Vec<T> where T:Hash+State+Production<Item=T>  {
    
    let mut visited: HashMap<u64,Node<T>> = HashMap::new();
    let mut queue: VecDeque<Node<T>> = VecDeque::new();

    let mut node: Node<T> = Node::new(start);
    queue.push_back(node);
    
    let mut count = 0;
    let mut node_id:u64;

    let mut depth:usize = 0;
    
    while !queue.is_empty() {
        count += 1;

        if count%100000 == 0 {
            println!("count: {} | visited {} | queue: {} | depth {} ", count, visited.len(), queue.len(), depth);
        }
        
        node = queue.pop_front().unwrap();
        node_id = node.get_id();
        depth = node.get_data().get_score();
        
        if node.get_data().is_goal() { break; }
        if node.get_data().is_end_state() { continue; }
        if visited.contains_key(&node_id) { continue; }
        
        for mut neighbor_node in node.production() {
            let neighbor_id = neighbor_node.get_id();
            if !visited.contains_key(&neighbor_id) {
                neighbor_node.add_parent(node_id);
                queue.push_back(neighbor_node);
            }
        }
        visited.insert(node_id, node);
    }
    let mut final_path:Vec<T> = Vec::new();
    return final_path;
}