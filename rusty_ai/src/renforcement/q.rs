extern crate rand;

use std::collections::HashMap;

pub struct QActor {
    alpha: f32,
    gamma: f32,
    epsilon: f32,
    action_space: u8,
    q_table: HashMap<u64,Vec<f32>>,
}

impl<T> Actor<T> for QActor {
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

    fn choose_action(&self) -> u8 {

    }
    
    fn learn(&mut self, curr_state: T, action: u8, next_state: T) {

    }
}