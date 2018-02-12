

// Actor
pub trait Actor<T> {
    type Item;
    fn new() -> Self::Item;
    fn choose_action(&self) -> u8;
    fn learn(&mut self, curr_state: T, action: u8, next_state: T);
}