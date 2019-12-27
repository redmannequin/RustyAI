use std::collections::HashMap;
use std::collections::HashSet;

use crate::traits::Search;
use crate::traits::State;
use crate::traits::Neighbors;

pub struct DFS;

impl<T> Search<T> for DFS 
where T: Eq + State + Neighbors + Clone {

    fn search(start: T, goal: T) -> Option<Vec<T>> {
        let start_id = start.get_id();
        let goal_id = goal.get_id();
        let mut visited: HashSet<T::Id> = HashSet::new();
        let mut queue: Vec<T> = Vec::new();
        let mut paths = HashMap::new();

        let start_node = Node::new(start.clone(), None);

        paths.insert(start_id, start_node);
        queue.push(start);
        let mut found = false;

        // get next node
        while let Some(node) = queue.pop() {
            let node_id = node.get_id();
            if node_id == goal_id { 
                found = true;
                break
            } else if visited.contains(&node_id) { 
                continue; 
            }
            
            // add node neighbors to queue 
            for neighbor in node.get_neighbors() {
                let neighbor_id = neighbor.get_id();

                // if node already has parent 
                if !paths.contains_key(&neighbor_id) {
                    let neighbor_node = Node::new(neighbor.clone(), Some(node_id.clone()));
                    paths.insert(neighbor_id, neighbor_node);
                    queue.push(neighbor);
                }
            }

            // set node as visited 
            visited.insert(node_id);
        }

        if found {
            let mut path = Vec::new();
            let mut curr = goal_id;
            while let Some(node) = paths.remove(&curr) {
                path.push(node.data);
                curr = match node.parent {
                    Some(id) => id,
                    None => break
                };
            }
            Some(path)
        } else {
            None
        }
    }

}

/// Depth First Search
/// 
struct Node<T> where T: State {
    data: T,
    parent: Option<T::Id>
}

impl<T> Node<T> where T: State {
    pub fn new(data: T, parent: Option<T::Id>) -> Self {
        Self {
            data,
            parent
        }
    }
}

impl<T> State for Node<T> where T: State + Clone {
    type Id = T::Id;

    fn get_id(&self) -> Self::Id {
        self.data.get_id()
    }
}

impl<T> Neighbors for Node<T> where T: State + Neighbors + Clone {
    fn get_neighbors(&self) -> Vec<Self> {
        let mut neighbors = Vec::new();
        for neighbor in self.data.get_neighbors() {
            neighbors.push(Node::new(neighbor, Some(self.get_id())));
        }
        neighbors
    }
}

#[test]
fn dfs_test_00() {

    use rand::SeedableRng;
    use rand::rngs::StdRng;
    use rand::Rng;

    #[derive(Debug,PartialEq,Eq,Clone)]
    struct Test(u16);

    impl State for Test {
        type Id = u16;

        fn get_id(&self) -> Self::Id {
            self.0.clone()
        }
    }

    impl Neighbors for Test {
        fn get_neighbors(&self) -> Vec<Self> {
            let mut rng: StdRng = SeedableRng::seed_from_u64(self.0.clone() as u64);
            let mut a = Vec::with_capacity(4);
            a.push(Test(rng.gen()));
            a.push(Test(rng.gen()));
            a.push(Test(rng.gen()));
            a.push(Test(rng.gen()));
            a
        }
    }

    println!("{:?}", DFS::search(Test(0), Test(50004)));

    assert!(true);
}