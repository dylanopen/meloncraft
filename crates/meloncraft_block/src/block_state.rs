pub trait BlockState {
fn from_id(state_id: i32) -> Option<Self> where Self: Sized;

    fn to_id(&self) -> i32 where Self: Sized;

}
