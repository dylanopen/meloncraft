use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OakWood {
    pub r#axis: Axis,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl BlockState for OakWood {
    fn to_id(self) -> i32 {
        if block_state.r#axis == Axis::X { return 201; }
        if block_state.r#axis == Axis::Y { return 202; }
        if block_state.r#axis == Axis::Z { return 203; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 201 {
            return Some(OakWood {
                r#axis: Axis::X,
            });
        }
        if state_id == 202 {
            return Some(OakWood {
                r#axis: Axis::Y,
            });
        }
        if state_id == 203 {
            return Some(OakWood {
                r#axis: Axis::Z,
            });
        }
        return None;
    }
}

