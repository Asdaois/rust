/// Used to track state of actor
#[derive(PartialEq)]
pub enum State {
    Active,
    Paused,
    Dead,
}
