use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaxedCopperChain {
    pub r#axis: Axis,
    pub waterlogged: bool,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl BlockState for WaxedCopperChain {
    fn to_id(&self) -> i32 {
        if self.r#axis == Axis::X && self.r#waterlogged == false { return 8076; }
        if self.r#axis == Axis::X && self.r#waterlogged == true { return 8075; }
        if self.r#waterlogged == true && self.r#axis == Axis::Z { return 8079; }
        if self.r#waterlogged == true && self.r#axis == Axis::Y { return 8077; }
        if self.r#axis == Axis::Y && self.r#waterlogged == false { return 8078; }
        if self.r#axis == Axis::Z && self.r#waterlogged == false { return 8080; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 8076 {
            return Some(WaxedCopperChain {
                r#axis: Axis::X,
                r#waterlogged: false,
            });
        }
        if state_id == 8075 {
            return Some(WaxedCopperChain {
                r#axis: Axis::X,
                r#waterlogged: true,
            });
        }
        if state_id == 8079 {
            return Some(WaxedCopperChain {
                r#waterlogged: true,
                r#axis: Axis::Z,
            });
        }
        if state_id == 8077 {
            return Some(WaxedCopperChain {
                r#waterlogged: true,
                r#axis: Axis::Y,
            });
        }
        if state_id == 8078 {
            return Some(WaxedCopperChain {
                r#axis: Axis::Y,
                r#waterlogged: false,
            });
        }
        if state_id == 8080 {
            return Some(WaxedCopperChain {
                r#axis: Axis::Z,
                r#waterlogged: false,
            });
        }
        return None;
    }
}

