use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrimsonHyphae {
    pub r#axis: Axis,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl BlockState for CrimsonHyphae {
    fn to_id(self) -> i32 {
        if block_state.r#axis == Axis::Y { return 20767; }
        if block_state.r#axis == Axis::Z { return 20768; }
        if block_state.r#axis == Axis::X { return 20766; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 20767 {
            return Some(CrimsonHyphae {
                r#axis: Axis::Y,
            });
        }
        if state_id == 20768 {
            return Some(CrimsonHyphae {
                r#axis: Axis::Z,
            });
        }
        if state_id == 20766 {
            return Some(CrimsonHyphae {
                r#axis: Axis::X,
            });
        }
        return None;
    }
}

