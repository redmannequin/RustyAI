use std::hash::Hash;
use std::collections::HashSet;

use core::node::Node;
use core::state::State;
use core::state::Production;

pub fn dfs<T>(start:T, depth:usize) -> Vec<T> where T:Hash+State+Production<Item=T>  {
    
    let mut path: Vec<Node<T>> = Vec::with_capacity(depth);
    let mut visited: HashSet<u64> = HashSet::new();
    let mut queue: Vec<Node<T>> = Vec::new();
    
    let mut node_id:u64;
    let mut node: Node<T> = Node::new(start);
    
    queue.push(node);
    while !queue.is_empty() {
        count += 1;
        
        if count%100000 == 0 {
            println!("count: {} | visited {} | queue: {} | path: {}", count, visited.len(), queue.len(), path.len());
        }
        
        node = queue.pop().unwrap();
        node_id = node.get_id();
        
        
        if visited.contains(&node_id) { continue; }
        visited.insert(node_id.clone());
        if node.get_data().is_end_state() { continue; }

        if node.get_data().is_goal() {
            path.push(node);
            break;
        }
       
        while let Some(parent_node) = path.pop() {
            if parent_node.is_parent(node_id) {
                path.push(parent_node);
                break;
            }
        }
        
        for neighbor_node in node.production() {
            let neighbor_id = neighbor_node.get_id();
            if !visited.contains(&neighbor_id) {
                node.add_child(neighbor_node.get_id());
                queue.push(neighbor_node);
            }
        }

        path.push(node);
    }

    let mut final_path:Vec<T> = Vec::new();
    for mut node in path {
        final_path.push(node.move_data());
    }
    return final_path;
}