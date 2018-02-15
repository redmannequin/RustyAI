use std::hash::Hash;
use std::cmp::Ordering;

use core::node::Node;

// State
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
    fn get_reward(&self) -> i8;
    fn get_heuristic(&self) -> i8;
}

impl<T> StateCost for Node<T> where T:Hash+StateCost {
    fn get_reward(&self) -> i8 {
        return self.get_data().get_reward();
    }

    fn get_heuristic(&self) -> i8 {
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

//
impl<T> PartialOrd for Node<T> where T:Hash+StateCost {
    fn partial_cmp(&self, other: &Node<T>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for Node<T> where T:Hash+StateCost {
    fn cmp(&self, other: &Node<T>) -> Ordering {
        let x = self.get_reward() + self.get_heuristic();
        let y = other.get_reward() + other.get_heuristic();
        return x.cmp(&y);
    }
}

impl<T> PartialEq for Node<T> where T:Hash+StateCost {
    fn eq(&self, other: &Node<T>) -> bool {
        let x = self.get_reward() + self.get_heuristic();
        let y = other.get_reward() + other.get_heuristic();
        return x == y;
    }
}

impl<T> Eq for Node<T> where T:Hash+StateCost {}