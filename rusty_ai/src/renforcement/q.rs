extern crate rand;

use std::hash::Hash;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::BinaryHeap;

use core::actor::Actor;
use core::state::State;
use core::state::StateType;

// QValue
#[derive(Clone)]
struct QValue {
    action: u8,
    expected_reward: f32,
}

impl PartialOrd for QValue {
    fn partial_cmp(&self, other: &QValue) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for QValue {
    fn cmp(&self, other: &QValue) -> Ordering {
        if self.expected_reward > other.expected_reward {
            return Ordering::Greater;
        }

        if self.expected_reward < other.expected_reward {
            return Ordering::Less;
        }

        return Ordering::Equal;
    }
}

impl PartialEq for QValue {
    fn eq(&self, other: &QValue) -> bool {
        return self.expected_reward == other.expected_reward;
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

impl QActor {

    fn get_max_QValue(&self, state_id: u64) -> QValue {
        let state_actions = self.q_table.get(&state_id).unwrap();
        let q_value = state_actions.peek().unwrap();
        return q_value.clone();
    }

    fn update_QValue(&mut self, state_id: u64, action: u8, correction: f32) {
        let state_actions = self.q_table.get_mut(&state_id).unwrap();
    }

}

impl<T> Actor<T> for QActor where T:State  {
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
            let q_value = self.get_max_QValue(state_id);
            action = q_value.action;
        } else {
            action = (((self.action_space-1) as f32)*rand::random::<f32>()) as u8;
        }
        return action;
    }
    
    fn learn(&mut self, curr_state_id: u64, action: u8, next_state: T) {

        let mut q_target = 5.0;
        match next_state.get_state() {
            Dead => q_target += self.get_max_QValue(curr_state_id).expected_reward,
            Goal => q_target += self.get_max_QValue(curr_state_id).expected_reward,
        }

        let q_predict = q_target.clone();

        let error = q_target-q_predict;
        let correction =  self.alpha*(error);
        self.update_QValue(curr_state_id, action, correction);
    }
}