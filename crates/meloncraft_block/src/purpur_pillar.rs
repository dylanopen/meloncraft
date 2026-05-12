use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PurpurPillar {
    pub r#axis: Axis,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl BlockState for PurpurPillar {
    fn to_id(&self) -> i32 {
        if self.r#axis == Axis::Z { return 14513; }
        if self.r#axis == Axis::X { return 14511; }
        if self.r#axis == Axis::Y { return 14512; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14513 {
            return Some(PurpurPillar {
                r#axis: Axis::Z,
            });
        }
        if state_id == 14511 {
            return Some(PurpurPillar {
                r#axis: Axis::X,
            });
        }
        if state_id == 14512 {
            return Some(PurpurPillar {
                r#axis: Axis::Y,
            });
        }
        return None;
    }
}

