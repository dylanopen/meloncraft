use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OxidizedCopperChain {
    pub r#axis: Axis,
    pub waterlogged: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl BlockState for OxidizedCopperChain {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == false && self.r#axis == Axis::X {
            return 8070;
        }
        if self.r#axis == Axis::Y && self.r#waterlogged == true {
            return 8071;
        }
        if self.r#axis == Axis::Y && self.r#waterlogged == false {
            return 8072;
        }
        if self.r#axis == Axis::X && self.r#waterlogged == true {
            return 8069;
        }
        if self.r#waterlogged == true && self.r#axis == Axis::Z {
            return 8073;
        }
        if self.r#waterlogged == false && self.r#axis == Axis::Z {
            return 8074;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 8070 {
            return Some(OxidizedCopperChain {
                r#waterlogged: false,
                r#axis: Axis::X,
            });
        }
        if state_id == 8071 {
            return Some(OxidizedCopperChain {
                r#axis: Axis::Y,
                r#waterlogged: true,
            });
        }
        if state_id == 8072 {
            return Some(OxidizedCopperChain {
                r#axis: Axis::Y,
                r#waterlogged: false,
            });
        }
        if state_id == 8069 {
            return Some(OxidizedCopperChain {
                r#axis: Axis::X,
                r#waterlogged: true,
            });
        }
        if state_id == 8073 {
            return Some(OxidizedCopperChain {
                r#waterlogged: true,
                r#axis: Axis::Z,
            });
        }
        if state_id == 8074 {
            return Some(OxidizedCopperChain {
                r#waterlogged: false,
                r#axis: Axis::Z,
            });
        }
        return None;
    }
}
