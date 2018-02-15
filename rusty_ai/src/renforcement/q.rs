extern crate rand;

use std::hash::Hash;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::BinaryHeap;

use core::actor::Actor;
use core::state::State;
use core::state::StateType;
use core::state::StateCost;

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
        let mut state_actions = self.q_table.get_mut(&state_id).unwrap();
        let mut q_values = BinaryHeap::new();
        for mut q_value in state_actions.drain() {
            if q_value.action == action {
                q_value.expected_reward += self.alpha * correction;
            }
            q_values.push(q_value);
        }
        
        state_actions.append(&mut q_values);
    }

    fn get_state_action_QValue(&self, state_id: u64, action: u8) -> Option<f32> {
        let state_actions = self.q_table.get(&state_id).unwrap();
        let mut result: Option<f32> = None;
        for q_value in state_actions.iter() {
            if q_value.action == action {
                result = Some(q_value.expected_reward);
                break;
            }
        }
        return result;
    }

    fn check_state(&mut self, state_id: u64) {
        if self.q_table.contains_key(&state_id) {
            return;
        }
        let mut heap =  BinaryHeap::new();
        for i in 0..self.action_space {
            heap.push(QValue {
                action: i,
                expected_reward: 0.0,
            });
        }
        self.q_table.insert(state_id, heap);
    }

}

impl<T> Actor<T> for QActor where T:State+StateCost {
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

    fn choose_action(&mut self, state_id: u64) -> u8 {
        self.check_state(state_id);
        let action: u8;
        if rand::random::<f32>() < self.epsilon {
            let q_value = self.get_max_QValue(state_id);
            action = q_value.action;
        } else {
            action = (((self.action_space-1) as f32)*rand::random::<f32>()) as u8;
        }
        return action;
    }
    
    fn learn(&mut self, curr_state: T, action: u8, next_state: T) {
        self.check_state(next_state.get_id());
        let curr_state_id = curr_state.get_id();
        let mut q_target = curr_state.get_reward() as f32;

        match next_state.get_state() {
            Dead => q_target += self.gamma * self.get_max_QValue(curr_state_id).expected_reward,
            Goal => q_target += self.gamma * self.get_max_QValue(curr_state_id).expected_reward,
        }

        let q_predict;
        if let Some(q_value) = self.get_state_action_QValue(curr_state_id, action) {
            q_predict = q_value;
        } else {
            panic!("QVlaue not found for state: {} and action: {}", curr_state_id, action);                        
        }

        let error = q_target-q_predict;
        let correction =  self.alpha*(error);
        self.update_QValue(curr_state_id, action, correction);
    }
}