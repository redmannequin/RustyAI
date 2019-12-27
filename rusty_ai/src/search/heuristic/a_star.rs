use std::collections::HashMap;
use std::collections::BinaryHeap;
use std::cmp::Ordering;

use std::rc::Rc;

use crate::traits::Search;
use crate::traits::State;
use crate::traits::Neighbors;
use crate::traits::Cost;
use crate::traits::Distance;

/// A* Struct
pub struct AStar;
 
/// A* Search
impl<T> Search<T> for AStar
where T: Eq + State + Neighbors + Distance + Cost + Clone {

    fn search(start: T, goal: T) -> Option<Vec<T>> {
        let goal_id = goal.get_id();
        let mut queue: BinaryHeap<Rc<Node<T>>> = BinaryHeap::new();
        let mut scores: HashMap<T::Id,Rc<Node<T>>> = HashMap::new();

        let node = Rc::new(Node::new(start, None, 0.0));
        scores.insert(node.get_id(), node.clone());
        queue.push(node);

        let mut found = false;
        
        // get next node
        while let Some(node) = queue.pop() {
            let node_id = node.get_id();
            if node_id == goal_id { 
                found = true;
                break;
            }

            for neighbor in node.get_neighbors() {
                let neighbor = Rc::new(neighbor);
                if let Some(old_node) = scores.get(&neighbor.get_id()) {
                    if old_node.score <= neighbor.score { continue; }
                } 
                scores.insert(neighbor.get_id(), neighbor.clone());
                queue.push(neighbor);
            }
        }

        if found {
            let mut path = Vec::new();
            let mut curr = goal_id;
            while let Some(node) = scores.remove(&curr) {
                path.push(node.data.clone());
                curr = match &node.parent {
                    Some(id) => id.clone(),
                    None => break
                };
            }
            Some(path)
        } else {
            None
        }
    }
}


/// A* Node
/// 
/// 
#[derive(Debug,Clone)]
struct Node<T> where T: State + Clone {
    data: T,
    parent: Option<T::Id>,
    dist: f64,
    score: f64,
}

impl<T> Node<T> where T: State + Cost + Clone {
    fn new(data: T, parent: Option<T::Id>, dist: f64) -> Self {
        let score = data.get_cost() + dist;
        Self { data, parent, dist, score }
    }
}

impl<T> State for Node<T> where T: State + Clone {
    type Id = T::Id;

    fn get_id(&self) -> T::Id {
        self.data.get_id()
    }

}

impl<T> Neighbors for Node<T> where T: State + Neighbors + Cost + Distance + Clone {
    fn get_neighbors(&self) -> Vec<Self> {
        let mut neighbors = Vec::new();
        for neighbor in self.data.get_neighbors() {
            let dist = self.dist + self.data.dist(&neighbor);
            neighbors.push(Node::new(neighbor, Some(self.get_id()), dist));
        }
        neighbors
    }
}

impl<T> Ord for Node<T> where T: State + Cost + Clone {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.score == other.score {
            Ordering::Equal
        } else if self.score < other.score {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    }
}

impl<T> PartialOrd for Node<T> where T: State + Cost + Clone {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> PartialEq for Node<T> where T: State + Cost + Clone {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

impl<T> Eq for Node<T> where T: Cost + Clone {}

#[test]
fn astar_test_00() {

    #[derive(Debug,PartialEq,Eq,Clone)]
    struct Test(u16);

    impl State for Test {
        type Id = u16;

        fn get_id(&self) -> Self::Id {
            self.0.clone()
        }
    }

    impl Distance for Test {
        fn dist(&self, other: &Self) -> f64 {
            let x: f64 = self.0.into();
            let y: f64 = other.0.into();
            y - x
        }
    }

    impl Cost for Test {
        fn get_cost(&self) -> f64 {
            let x: f64 = self.0.into();
            20.0 - x
        }
    }

    impl Neighbors for Test {
        fn get_neighbors(&self) -> Vec<Self> {
            let mut a = Vec::with_capacity(4);
            a.push(Test(self.0+1));
            a.push(Test(self.0+2));
            a.push(Test(self.0+3));
            a.push(Test(self.0+4));
            a
        }
    }

    println!("{:?}", AStar::search(Test(1), Test(20)));

    assert!(true);
}