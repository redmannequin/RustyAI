use std::hash::Hash;
use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::BinaryHeap;
use std::cmp::Ordering;

use core::node::Node;
use core::state::State;
use core::state::StateCost;
use core::state::Production;
use core::state::StateType;

use search::get_path;

pub fn a_star<T>(start: T) -> Vec<T> where T:Clone+Hash+State+StateCost+Production<Item=T> {
    
    let mut queue: BinaryHeap<Score> = BinaryHeap::new();
    let mut scores: HashMap<u64,f32> = HashMap::new();
    let mut openSet: HashMap<u64,Node<T>> = HashMap::new();
    let mut closedSet: HashMap<u64,Node<T>> = HashMap::new();

    let mut goal_node: Option<Node<T>> = None;

    let start_node = Node::new(start);
    let start_node_id = start_node.get_id();
    let start_score = Score::new(&start_node);

    queue.push(start_score.clone());
    scores.insert(start_node_id, start_score.score);
    openSet.insert(start_node_id, start_node);
    
    while let Some(score) = queue.pop() {
        let node_id = score.id;

        if scores.len()%100 == 0 {
            println!("openset: {} | Q: {} | closeset: {} | visited: {} | D: {}", openSet.len(), queue.len(), closedSet.len(), scores.len(), score.score);
        }
        
        if let Some(node) = openSet.remove(&node_id) {
            match node.get_state() {
                StateType::Live => {
                    closedSet.insert(node_id, node.clone());
                    add_neighbors(&mut queue, &mut scores, &mut openSet, &closedSet, node);  
                },
                StateType::Goal => {
                    goal_node = Some(node);
                    break; 
                },
                StateType::Dead => continue,
            }
        }
    }
    return get_path(goal_node, &mut closedSet);
}

fn add_neighbors<T> (
    queue: &mut BinaryHeap<Score>, 
    scores: &mut HashMap<u64,f32>, 
    openSet: &mut HashMap<u64,Node<T>>, 
    closedSet: &HashMap<u64,Node<T>>, 
    node: Node<T>
) where T:Clone+Hash+State+StateCost+Production<Item=T> {
    
    let node_id = node.get_id();
    let node_score = scores.get(&node_id).unwrap().clone();
    for mut neighbor_node in node.production() {
        let neighbor_id = neighbor_node.get_id();
        neighbor_node.add_parent(node_id);

        if closedSet.contains_key(&neighbor_id) { continue; }

        let mut score = Score::new(&neighbor_node);

        if openSet.contains_key(&neighbor_id) {
            let old_score = scores.get(&neighbor_id).unwrap().clone();
            if score.score + node_score > old_score {
                continue;
            }
        }

        queue.push(score.clone());
        scores.insert(neighbor_id, score.score + node_score);
        openSet.insert(neighbor_id, neighbor_node);
    }
}


// SCORE
#[derive(Clone)]
struct Score {
    id: u64,
    score: f32,
}

impl Score {

    fn new<T>(node: &Node<T>) -> Self where T:Hash+State+StateCost {
        let id = node.get_id();
        let score = node.get_score() + node.get_heuristic();
        return Score {
            id: id,
            score: score,
        };
    }
}

//
impl PartialOrd for Score where {
    fn partial_cmp(&self, other: &Score) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Score where {
    fn cmp(&self, other: &Score) -> Ordering {
        if self.score > other.score { return Ordering::Greater; } 
        if self.score < other.score { return Ordering::Less; }
        return Ordering::Equal;
    }
}

impl PartialEq for Score where {
    fn eq(&self, other: &Score) -> bool {
        return self.score == other.score;
    }
}

impl Eq for Score where {}