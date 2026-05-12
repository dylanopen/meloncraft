use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpruceWood {
    pub r#axis: Axis,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl BlockState for SpruceWood {
    fn to_id(self) -> i32 {
        if block_state.r#axis == Axis::X { return 204; }
        if block_state.r#axis == Axis::Y { return 205; }
        if block_state.r#axis == Axis::Z { return 206; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 204 {
            return Some(SpruceWood {
                r#axis: Axis::X,
            });
        }
        if state_id == 205 {
            return Some(SpruceWood {
                r#axis: Axis::Y,
            });
        }
        if state_id == 206 {
            return Some(SpruceWood {
                r#axis: Axis::Z,
            });
        }
        return None;
    }
}

