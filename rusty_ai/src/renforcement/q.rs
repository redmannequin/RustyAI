extern crate rand;

use std::hash::Hash;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::BinaryHeap;

use core::actor::Actor;

// QValue
struct QValue {
    action: u8,
    score: f32,
}

impl PartialOrd for QValue {
    fn partial_cmp(&self, other: &QValue) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for QValue {
    fn cmp(&self, other: &QValue) -> Ordering {
        if self.score > other.score {
            return Ordering::Greater;
        }

        if self.score < other.score {
            return Ordering::Less;
        }

        return Ordering::Equal;
    }
}

impl PartialEq for QValue {
    fn eq(&self, other: &QValue) -> bool {
        return self.score == other.score;
    }
}

impl Eq for QValue {}

// QActor
pub struct QActor {
    alpha: f32,
    gamma: f32,
    epsilon: f32,
    action_space: u8,
    q_table: HashMap<u64,BinaryHeap<QValue>>,
}

impl Actor for QActor {
    type Item = Self;

    fn new(alpha:f32, gamma:f32, epsilon:f32, action_space:u8) -> Self::Item {
        return QActor {
            alpha: alpha,
            gamma: gamma,
            epsilon: epsilon,
            action_space: action_space,
            q_table: HashMap::new(),
        }
    }

    fn choose_action(&self, state_id: u64) -> u8 {
        let action: u8;
        if rand::random::<f32>() < self.epsilon {
            let state_actions = self.q_table.get(&state_id).unwrap();
            let q_value = state_actions.peek().unwrap();
            action = q_value.action;
        } else {
            action = (((self.action_space-1) as f32)*rand::random::<f32>()) as u8;
        }
        return action;
    }
    
    fn learn(&mut self, curr_state_id: u64, action: u8, next_state_id: u64) {

    }
}