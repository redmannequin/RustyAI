use std::hash::Hash;
use std::collections::HashMap;
use std::collections::VecDeque;

use core::node::Node;
use core::state::State;
use core::state::Production;

pub fn bfs<T>(start:T) -> Vec<T> where T:Hash+State+Production<Item=T>  {
    
    let mut visited: HashMap<u64,Node<T>> = HashMap::new();
    let q1: Vec<Node<T>> = Vec::new();
    let q2: Vec<Node<T>> = Vec::new();

    let mut parent_queue = Box::new(q1);
    let mut child_queue = Box::new(q2);
    
    let mut node_id:u64;
    let mut depth:usize = 0;
    let mut final_path:Vec<T> = Vec::new();
    let mut node: Node<T> = Node::new(start);
    
    (*parent_queue).push(node);
    while !(*parent_queue).is_empty() {
        while !(*parent_queue).is_empty() {

            node = (*parent_queue).pop().unwrap();
            node_id = node.get_id();

            if visited.contains_key(&node_id) { continue; }
            if node.get_data().is_end_state() { continue; }

            if node.get_data().is_goal() { 
                for i in 0..depth {
                    let set = node.get_parents();
                    let mut set_iter = set.iter();
                    node_id = set_iter.next().unwrap().clone();
                    final_path.push(node.move_data());
                    node = visited.remove(&node_id).unwrap();
                }
                final_path.reverse();
                break; 
            }

            for mut neighbor_node in node.production() {
                let neighbor_id = neighbor_node.get_id();
                if !visited.contains_key(&neighbor_id) {
                    neighbor_node.add_parent(node_id);
                    (*child_queue).push(neighbor_node);
                }
            }

            visited.insert(node_id, node);
        }
        
        depth = depth + 1;
        let temp = child_queue;
        child_queue = parent_queue;
        parent_queue = temp;
        
    }

    return final_path;
}