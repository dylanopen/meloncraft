use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StrippedPaleOakWood {
    pub r#axis: Axis,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl BlockState for StrippedPaleOakWood {
    fn to_id(self) -> i32 {
        if block_state.r#axis == Axis::Y { return 247; }
        if block_state.r#axis == Axis::X { return 246; }
        if block_state.r#axis == Axis::Z { return 248; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 247 {
            return Some(StrippedPaleOakWood {
                r#axis: Axis::Y,
            });
        }
        if state_id == 246 {
            return Some(StrippedPaleOakWood {
                r#axis: Axis::X,
            });
        }
        if state_id == 248 {
            return Some(StrippedPaleOakWood {
                r#axis: Axis::Z,
            });
        }
        return None;
    }
}

