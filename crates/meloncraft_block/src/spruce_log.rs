use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpruceLog {
    pub r#axis: Axis,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl BlockState for SpruceLog {
    fn to_id(self) -> i32 {
        if block_state.r#axis == Axis::X { return 139; }
        if block_state.r#axis == Axis::Z { return 141; }
        if block_state.r#axis == Axis::Y { return 140; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 139 {
            return Some(SpruceLog {
                r#axis: Axis::X,
            });
        }
        if state_id == 141 {
            return Some(SpruceLog {
                r#axis: Axis::Z,
            });
        }
        if state_id == 140 {
            return Some(SpruceLog {
                r#axis: Axis::Y,
            });
        }
        return None;
    }
}

