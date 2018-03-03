use std::hash::Hash;
use std::cmp::Ordering;

use core::node::Node;
use core::state::State;
use core::state::StateCost;

pub mod a_star;
pub mod minimax;


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