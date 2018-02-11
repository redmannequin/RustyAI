
use core::state::State;
use core::state::Production;

// Evn
pub trait Evn<T> where T:State+Production<Item=T>  {
    
    fn step(&mut self, action: Action) -> T;
    
    fn get_action_space(&self) -> u8;
    fn get_observation_space(&self) -> u8;
    fn get_reward_range(&self) -> i8;
    fn reset(&self);
}