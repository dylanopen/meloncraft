use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PaleOakWood {
    pub r#axis: Axis,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl BlockState for PaleOakWood {
    fn to_id(self) -> i32 {
        if block_state.r#axis == Axis::X { return 22; }
        if block_state.r#axis == Axis::Y { return 23; }
        if block_state.r#axis == Axis::Z { return 24; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 22 {
            return Some(PaleOakWood {
                r#axis: Axis::X,
            });
        }
        if state_id == 23 {
            return Some(PaleOakWood {
                r#axis: Axis::Y,
            });
        }
        if state_id == 24 {
            return Some(PaleOakWood {
                r#axis: Axis::Z,
            });
        }
        return None;
    }
}

