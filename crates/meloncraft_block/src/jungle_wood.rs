use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JungleWood {
    pub r#axis: Axis,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl BlockState for JungleWood {
    fn to_id(self) -> i32 {
        if block_state.r#axis == Axis::X { return 210; }
        if block_state.r#axis == Axis::Y { return 211; }
        if block_state.r#axis == Axis::Z { return 212; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 210 {
            return Some(JungleWood {
                r#axis: Axis::X,
            });
        }
        if state_id == 211 {
            return Some(JungleWood {
                r#axis: Axis::Y,
            });
        }
        if state_id == 212 {
            return Some(JungleWood {
                r#axis: Axis::Z,
            });
        }
        return None;
    }
}

