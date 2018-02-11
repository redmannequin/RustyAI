use std::hash::Hash;

use core::node::Node;

// State
pub trait State {
    fn is_goal(&self) -> bool;
    fn is_end_state(&self) -> bool;
    fn get_score(&self) -> usize;
}

// Production
pub trait Production {
    type Item;
    fn production(&self) -> Vec<Self::Item>;
}

impl<T> Production for Node<T> where T:Hash+State+Production<Item=T> {
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