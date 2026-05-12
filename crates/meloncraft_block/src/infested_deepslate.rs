use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InfestedDeepslate {
    pub r#axis: Axis,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl BlockState for InfestedDeepslate {
    fn to_id(self) -> i32 {
        if block_state.r#axis == Axis::Y { return 29372; }
        if block_state.r#axis == Axis::Z { return 29373; }
        if block_state.r#axis == Axis::X { return 29371; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 29372 {
            return Some(InfestedDeepslate {
                r#axis: Axis::Y,
            });
        }
        if state_id == 29373 {
            return Some(InfestedDeepslate {
                r#axis: Axis::Z,
            });
        }
        if state_id == 29371 {
            return Some(InfestedDeepslate {
                r#axis: Axis::X,
            });
        }
        return None;
    }
}

