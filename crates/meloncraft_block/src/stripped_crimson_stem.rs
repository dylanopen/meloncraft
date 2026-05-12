use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StrippedCrimsonStem {
    pub r#axis: Axis,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl BlockState for StrippedCrimsonStem {
    fn to_id(self) -> i32 {
        if block_state.r#axis == Axis::Y { return 20764; }
        if block_state.r#axis == Axis::Z { return 20765; }
        if block_state.r#axis == Axis::X { return 20763; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 20764 {
            return Some(StrippedCrimsonStem {
                r#axis: Axis::Y,
            });
        }
        if state_id == 20765 {
            return Some(StrippedCrimsonStem {
                r#axis: Axis::Z,
            });
        }
        if state_id == 20763 {
            return Some(StrippedCrimsonStem {
                r#axis: Axis::X,
            });
        }
        return None;
    }
}

