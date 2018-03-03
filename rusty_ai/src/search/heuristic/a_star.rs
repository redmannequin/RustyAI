use std::hash::Hash;
use std::collections::HashMap;
use std::collections::BinaryHeap;


use core::node::Node;
use core::state::State;
use core::state::StateCost;
use core::state::Production;
use core::state::StateType;

use search::get_path;
use search::heuristic::Score;

pub fn a_star<T>(start: T) -> Vec<T> where T:Clone+Hash+State+StateCost+Production<Item=T> {
    
    let mut queue: BinaryHeap<Score> = BinaryHeap::new();
    let mut scores: HashMap<u64,f32> = HashMap::new();
    let mut open_set: HashMap<u64,Node<T>> = HashMap::new();
    let mut closed_set: HashMap<u64,Node<T>> = HashMap::new();

    let mut goal_node: Option<Node<T>> = None;

    let start_node = Node::new(start);
    let start_node_id = start_node.get_id();
    let start_score = Score::new(&start_node);

    queue.push(start_score.clone());
    scores.insert(start_node_id, start_score.score);
    open_set.insert(start_node_id, start_node);
    
    while let Some(score) = queue.pop() {
        let node_id = score.id;

        if scores.len()%100 == 0 {
            println!("openSet: {} | Q: {} | closeset: {} | visited: {} | D: {}", open_set.len(), queue.len(), closed_set.len(), scores.len(), score.score);
        }
        
        if let Some(node) = open_set.remove(&node_id) {
            match node.get_state() {
                StateType::Live => {
                    closed_set.insert(node_id, node.clone());
                    add_neighbors(&mut queue, &mut scores, &mut open_set, &closed_set, node);  
                },
                StateType::Goal => {
                    goal_node = Some(node);
                    break; 
                },
                StateType::Dead => continue,
            }
        }
    }
    return get_path(goal_node, &mut closed_set);
}

fn add_neighbors<T> (
    queue: &mut BinaryHeap<Score>, 
    scores: &mut HashMap<u64,f32>, 
    open_set: &mut HashMap<u64,Node<T>>, 
    closed_set: &HashMap<u64,Node<T>>, 
    node: Node<T>
) where T:Clone+Hash+State+StateCost+Production<Item=T> {
    
    let node_id = node.get_id();
    let node_score = scores.get(&node_id).unwrap().clone();
    for mut neighbor_node in node.production() {
        let neighbor_id = neighbor_node.get_id();
        neighbor_node.add_parent(node_id);

        if closed_set.contains_key(&neighbor_id) { continue; }

        let mut score = Score::new(&neighbor_node);

        if open_set.contains_key(&neighbor_id) {
            let old_score = scores.get(&neighbor_id).unwrap().clone();
            if score.score + node_score > old_score {
                continue;
            }
        }

        queue.push(score.clone());
        scores.insert(neighbor_id, score.score + node_score);
        open_set.insert(neighbor_id, neighbor_node);
    }
}