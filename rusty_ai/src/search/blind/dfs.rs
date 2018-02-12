use std::hash::Hash;
use std::collections::HashMap;

use core::node::Node;
use core::state::State;
use core::state::Production;

pub fn dfs<T>(start:T, depth:usize) -> Vec<T> where T:Hash+State+Production<Item=T>  {

    let mut queue: Vec<Node<T>> = Vec::new();
    let mut visited: HashMap<u64,Node<T>> = HashMap::new();
    
    let mut goal_node: Option<Node<T>> = None;
    queue.push(Node::new(start));

    while let Some(mut node) = queue.pop() {
        
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

    let mut final_path:Vec<T> = Vec::new();
    let mut curr_node: Box<Option<Node<T>>> = Box::new(goal_node);
    
    while let Some(mut node) = *curr_node {
        let parents = node.get_parents();
        let mut set_iter = parents.iter(); 
        let node_id = set_iter.next().unwrap().clone();
        final_path.push(node.move_data());
        curr_node = Box::new(visited.remove(&node_id));
    }
    
    return final_path;
}