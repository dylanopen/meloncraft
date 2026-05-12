use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HayBlock {
    pub r#axis: Axis,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl BlockState for HayBlock {
    fn to_id(self) -> i32 {
        if block_state.r#axis == Axis::Y { return 12692; }
        if block_state.r#axis == Axis::Z { return 12693; }
        if block_state.r#axis == Axis::X { return 12691; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 12692 {
            return Some(HayBlock {
                r#axis: Axis::Y,
            });
        }
        if state_id == 12693 {
            return Some(HayBlock {
                r#axis: Axis::Z,
            });
        }
        if state_id == 12691 {
            return Some(HayBlock {
                r#axis: Axis::X,
            });
        }
        return None;
    }
}

