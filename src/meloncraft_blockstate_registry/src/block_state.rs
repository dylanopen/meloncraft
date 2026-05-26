pub trait BlockState {
    fn to_id(&self) -> i32
    where
        Self: Sized;
    fn from_id(state_id: i32) -> Option<Self>
    where
        Self: Sized;
}
