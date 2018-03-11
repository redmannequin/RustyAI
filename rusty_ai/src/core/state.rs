use std::hash::Hash;
use core::node::Node;

// State
#[derive(Clone)]
pub enum StateType {
    Live,
    Dead,
    Goal
}

pub trait State {
    fn get_id(&self) -> u64;
    fn get_state(&self) -> StateType;
}

impl<T> State for Node<T> where T:Hash+State {
    fn get_state(&self) -> StateType {
        return self.get_data().get_state();
    }

    fn get_id(&self) -> u64 {
        return self.get_id();
    }
}

// Cost
pub trait StateCost {
    fn get_score(&self) -> f32;
    fn get_heuristic(&self) -> f32;
}

impl<T> StateCost for Node<T> where T:Hash+StateCost {
    fn get_score(&self) -> f32 {
        return self.get_data().get_score();
    }

    fn get_heuristic(&self) -> f32 {
        return self.get_data().get_heuristic();
    }
}

// Production
pub trait Production {
    type Item;
    fn production(&self) -> Vec<Self::Item>;
}

impl<T> Production for Node<T> where T:Hash+Production<Item=T> {
    type Item = Self;
    fn production(&self) -> Vec<Self::Item> {
        let mut nodes: Vec<Self> = Vec::new();
        for d in self.get_data().production() {
            let node = Self::new(d);
            nodes.push(node);
        }
        return nodes;
    }
}