use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StrippedAcaciaLog {
    pub r#axis: Axis,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl BlockState for StrippedAcaciaLog {
    fn to_id(self) -> i32 {
        if block_state.r#axis == Axis::Z { return 182; }
        if block_state.r#axis == Axis::X { return 180; }
        if block_state.r#axis == Axis::Y { return 181; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 182 {
            return Some(StrippedAcaciaLog {
                r#axis: Axis::Z,
            });
        }
        if state_id == 180 {
            return Some(StrippedAcaciaLog {
                r#axis: Axis::X,
            });
        }
        if state_id == 181 {
            return Some(StrippedAcaciaLog {
                r#axis: Axis::Y,
            });
        }
        return None;
    }
}

