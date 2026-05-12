use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AcaciaWood {
    pub r#axis: Axis,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl BlockState for AcaciaWood {
    fn to_id(self) -> i32 {
        if block_state.r#axis == Axis::Y { return 214; }
        if block_state.r#axis == Axis::Z { return 215; }
        if block_state.r#axis == Axis::X { return 213; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 214 {
            return Some(AcaciaWood {
                r#axis: Axis::Y,
            });
        }
        if state_id == 215 {
            return Some(AcaciaWood {
                r#axis: Axis::Z,
            });
        }
        if state_id == 213 {
            return Some(AcaciaWood {
                r#axis: Axis::X,
            });
        }
        return None;
    }
}

