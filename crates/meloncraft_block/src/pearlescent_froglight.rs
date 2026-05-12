use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PearlescentFroglight {
    pub r#axis: Axis,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl BlockState for PearlescentFroglight {
    fn to_id(self) -> i32 {
        if block_state.r#axis == Axis::Y { return 29387; }
        if block_state.r#axis == Axis::X { return 29386; }
        if block_state.r#axis == Axis::Z { return 29388; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 29387 {
            return Some(PearlescentFroglight {
                r#axis: Axis::Y,
            });
        }
        if state_id == 29386 {
            return Some(PearlescentFroglight {
                r#axis: Axis::X,
            });
        }
        if state_id == 29388 {
            return Some(PearlescentFroglight {
                r#axis: Axis::Z,
            });
        }
        return None;
    }
}

