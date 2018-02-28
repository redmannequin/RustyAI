use std::hash::Hash;
use std::collections::HashMap;

use core::node::Node;
use core::state::State;
use core::state::Production;
use core::state::StateType;

use search::get_path;

pub fn dfs<T>(start:T) -> Vec<T> where T:Hash+State+Production<Item=T>  {

    let mut queue: Vec<Node<T>> = Vec::new();
    let mut visited: HashMap<u64,Node<T>> = HashMap::new();
    
    let mut goal_node: Option<Node<T>> = None;
    queue.push(Node::new(start));

    while let Some(mut node) = queue.pop() {

        //if visited.len()%100 == 0 {
        //    println!("queue: {} | visited: {} ", queue.len(), visited.len());
        //}
        
        let node_id = node.get_id();
        if visited.contains_key(&node_id) { continue; }

        match node.get_state() {
            StateType::Live => {
                for mut neighbor_node in node.production() {
                    let neighbor_id = neighbor_node.get_id();
                    if !visited.contains_key(&neighbor_id) {
                        neighbor_node.add_parent(node_id);
                        queue.push(neighbor_node);
                    }
                }
                visited.insert(node_id, node);
            },
            StateType::Goal => {
                goal_node = Some(node);
                break;
            },
            StateType::Dead => continue,
        }
    }

    return get_path(goal_node, &mut visited);
}

