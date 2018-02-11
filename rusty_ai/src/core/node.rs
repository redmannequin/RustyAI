use std::hash::{Hash, Hasher};
use std::collections::BTreeSet;
use std::collections::hash_map::DefaultHasher;

// Node
#[derive(Clone)]
pub struct Node<T> where T:Hash {
    id: u64,
    data: T,
    parents: BTreeSet<u64>,
    children: BTreeSet<u64>
}

impl<T> Node<T> where T:Hash {

    // create and reutrn new Node
    pub fn new(data: T) -> Self {

        let mut hasher = DefaultHasher::new();
        data.hash(&mut hasher);

        let id:u64 = hasher.finish();

        return Node {
            id: id,
            data: data,
            parents: BTreeSet::new(),
            children: BTreeSet::new(),
        };
    }

    // PARENT
    pub fn add_parent(&mut self, node_id: u64) {
        self.parents.insert(node_id);
    }
    
    pub fn remove_parent(&mut self, node_id: u64) {
        self.parents.remove(&node_id);
    }

    pub fn is_parent(&self, node_id: u64) -> bool {
        return self.children.contains(&node_id);
    }

    pub fn get_parents(&self) -> BTreeSet<u64> {
        return self.parents.clone();
    }

    pub fn parent_count(&self) -> usize {
        return self.parents.len();
    }

    // CHILDREN
    pub fn add_child(&mut self, node_id: u64) {
        self.children.insert(node_id);
    }

    pub fn remove_child(&mut self, node_id: u64) {
        self.children.remove(&node_id);
    }

    pub fn is_child(&self, node_id: u64) -> bool {
        return self.parents.contains(&node_id);
    }

    pub fn get_children(&self) -> BTreeSet<u64> {
        return self.children.clone();
    }

    pub fn child_count(&self) -> usize {
        return self.children.len();
    }

    // DATA
    pub fn get_data(&self) -> &T { 
        return &self.data;
    }

    pub fn move_data(self) -> T {
        return self.data;
    }

    pub fn get_mut_data(&mut self) -> &mut T { 
        return &mut self.data; 
    }

    pub fn get_id(&self) -> u64 {
        return self.id;
    }
}

// Hash
impl<T> Hash for Node<T> where T:Hash {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
