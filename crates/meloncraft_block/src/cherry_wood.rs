use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CherryWood {
    pub r#axis: Axis,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl BlockState for CherryWood {
    fn to_id(self) -> i32 {
        if block_state.r#axis == Axis::Y { return 217; }
        if block_state.r#axis == Axis::Z { return 218; }
        if block_state.r#axis == Axis::X { return 216; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 217 {
            return Some(CherryWood {
                r#axis: Axis::Y,
            });
        }
        if state_id == 218 {
            return Some(CherryWood {
                r#axis: Axis::Z,
            });
        }
        if state_id == 216 {
            return Some(CherryWood {
                r#axis: Axis::X,
            });
        }
        return None;
    }
}

