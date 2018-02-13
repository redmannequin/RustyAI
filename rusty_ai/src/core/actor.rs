

// Actor
pub trait Actor<T> {
    type Item;
    fn new(alpha:f32, gamma:f32, epsilon:f32, action_space:u8) -> Self::Item;
    fn choose_action(&self, state_id: u64) -> u8;
    fn learn(&mut self, curr_state: T, action: u8, next_state_id: T);
}