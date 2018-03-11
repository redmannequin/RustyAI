
use core::state::State;
use core::state::Production;

// Evn
pub trait Evn<T> where T:State+Production<Item=T>  {
    type Action;
    fn step(&mut self, action: Self::Action) -> T;
    fn get_action_space(&self) -> u8;
    fn get_observation_space(&self) -> u8;
    fn get_reward_range(&self) -> i8;
    fn reset(&self);
}