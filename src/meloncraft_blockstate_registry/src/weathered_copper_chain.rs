use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WeatheredCopperChain {
    pub r#axis: Axis,
    pub waterlogged: bool,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl BlockState for WeatheredCopperChain {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == false && self.r#axis == Axis::X { return 8064; }
        if self.r#waterlogged == false && self.r#axis == Axis::Y { return 8066; }
        if self.r#axis == Axis::Z && self.r#waterlogged == true { return 8067; }
        if self.r#axis == Axis::Z && self.r#waterlogged == false { return 8068; }
        if self.r#waterlogged == true && self.r#axis == Axis::X { return 8063; }
        if self.r#axis == Axis::Y && self.r#waterlogged == true { return 8065; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 8064 {
            return Some(WeatheredCopperChain {
                r#waterlogged: false,
                r#axis: Axis::X,
            });
        }
        if state_id == 8066 {
            return Some(WeatheredCopperChain {
                r#waterlogged: false,
                r#axis: Axis::Y,
            });
        }
        if state_id == 8067 {
            return Some(WeatheredCopperChain {
                r#axis: Axis::Z,
                r#waterlogged: true,
            });
        }
        if state_id == 8068 {
            return Some(WeatheredCopperChain {
                r#axis: Axis::Z,
                r#waterlogged: false,
            });
        }
        if state_id == 8063 {
            return Some(WeatheredCopperChain {
                r#waterlogged: true,
                r#axis: Axis::X,
            });
        }
        if state_id == 8065 {
            return Some(WeatheredCopperChain {
                r#axis: Axis::Y,
                r#waterlogged: true,
            });
        }
        return None;
    }
}

