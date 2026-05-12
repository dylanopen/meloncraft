use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetherPortal {
    pub r#axis: Axis,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    X,
    Z,
}

impl BlockState for NetherPortal {
    fn to_id(self) -> i32 {
        if block_state.r#axis == Axis::X { return 6816; }
        if block_state.r#axis == Axis::Z { return 6817; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 6816 {
            return Some(NetherPortal {
                r#axis: Axis::X,
            });
        }
        if state_id == 6817 {
            return Some(NetherPortal {
                r#axis: Axis::Z,
            });
        }
        return None;
    }
}

