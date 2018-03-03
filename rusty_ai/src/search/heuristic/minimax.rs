use std::hash::Hash;
use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::cmp;
use std::f32;

use core::node::Node;
use core::state::State;
use core::state::StateCost;
use core::state::Production;
use core::state::StateType;

use search::get_path;
use search::heuristic::Score;

pub fn minimax<T>(start: T, depth: i8) where T:Clone+Hash+State+StateCost+Production<Item=T> {

    let mut maximizingPlayer = true;
    let node = Node::new(start);

    let mut n = depth;
    
    while n != 0 {
        
        n = n - 1;
        let mut bestValue;
        let cond: Box<Fn(Score,Score)->Score>;

        if maximizingPlayer {
            bestValue = Score {id: 0, score: f32::NEG_INFINITY};
            let min = |x:Score, y:Score| -> Score { cmp::min(x,y) };
            cond = Box::new(min);
        } else {
            bestValue = Score {id: 0, score: f32::INFINITY};
            let max = |x:Score, y:Score| -> Score { cmp::max(x,y) };
            cond =  Box::new(max);
        }        

        for mut neighbor_node in node.production() {
            let score = Score::new(&node);
            bestValue = cond(bestValue, score);
        }

        maximizingPlayer = !maximizingPlayer;
    }
}