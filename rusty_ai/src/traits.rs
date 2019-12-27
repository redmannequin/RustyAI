//! A suite of traits  
//! 
use std::hash::Hash;
use std::fmt::Debug;


/// A common trait used for search
pub trait Search<T> where T: Eq + State + Neighbors  {

    // Return the path from start state to goal state if found
    fn search(start: T, goal: T) -> Option<Vec<T>>;

}


/// A trait to define a State
/// A state must have a and Id
pub trait State {
    /// The resulting Id type to uniquely identify a State 
    type Id: Eq + Hash + Clone + Debug;
    
    /// Retrives State Id from self
    fn get_id(&self) -> Self::Id;

}


/// A trait to get the distance between states
pub trait Distance where Self: State {

    /// Returns the distance between self and other
    fn dist(&self, other: &Self) -> f64;

}


/// A trait to get a States cost 
pub trait Cost where Self: State {

    /// Returns the cost 
    fn get_cost(&self) -> f64;

}


/// A trait to get a States neighbors
pub trait Neighbors where Self: State + Sized {
    
    /// Returns a list a neighbor nodes  
    fn get_neighbors(&self) -> Vec<Self>;

}