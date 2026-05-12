use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MuddyMangroveRoots {
    pub r#axis: Axis,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl BlockState for MuddyMangroveRoots {
    fn to_id(&self) -> i32 {
        if self.r#axis == Axis::Z { return 167; }
        if self.r#axis == Axis::X { return 165; }
        if self.r#axis == Axis::Y { return 166; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 167 {
            return Some(MuddyMangroveRoots {
                r#axis: Axis::Z,
            });
        }
        if state_id == 165 {
            return Some(MuddyMangroveRoots {
                r#axis: Axis::X,
            });
        }
        if state_id == 166 {
            return Some(MuddyMangroveRoots {
                r#axis: Axis::Y,
            });
        }
        return None;
    }
}

