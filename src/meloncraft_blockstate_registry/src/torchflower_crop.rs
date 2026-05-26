use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TorchflowerCrop {
    pub age: i32,
}

impl BlockState for TorchflowerCrop {
    fn to_id(&self) -> i32 {
        if self.r#age == 0 {
            return 14595;
        }
        if self.r#age == 1 {
            return 14596;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14595 {
            return Some(TorchflowerCrop { r#age: 0 });
        }
        if state_id == 14596 {
            return Some(TorchflowerCrop { r#age: 1 });
        }
        return None;
    }
}
