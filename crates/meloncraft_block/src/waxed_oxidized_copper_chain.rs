use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaxedOxidizedCopperChain {
    pub r#axis: Axis,
    pub waterlogged: bool,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl BlockState for WaxedOxidizedCopperChain {
    fn to_id(&self) -> i32 {
        if self.r#axis == Axis::Z && self.r#waterlogged == false { return 8098; }
        if self.r#axis == Axis::Y && self.r#waterlogged == true { return 8095; }
        if self.r#waterlogged == false && self.r#axis == Axis::X { return 8094; }
        if self.r#axis == Axis::Y && self.r#waterlogged == false { return 8096; }
        if self.r#axis == Axis::X && self.r#waterlogged == true { return 8093; }
        if self.r#axis == Axis::Z && self.r#waterlogged == true { return 8097; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 8098 {
            return Some(WaxedOxidizedCopperChain {
                r#axis: Axis::Z,
                r#waterlogged: false,
            });
        }
        if state_id == 8095 {
            return Some(WaxedOxidizedCopperChain {
                r#axis: Axis::Y,
                r#waterlogged: true,
            });
        }
        if state_id == 8094 {
            return Some(WaxedOxidizedCopperChain {
                r#waterlogged: false,
                r#axis: Axis::X,
            });
        }
        if state_id == 8096 {
            return Some(WaxedOxidizedCopperChain {
                r#axis: Axis::Y,
                r#waterlogged: false,
            });
        }
        if state_id == 8093 {
            return Some(WaxedOxidizedCopperChain {
                r#axis: Axis::X,
                r#waterlogged: true,
            });
        }
        if state_id == 8097 {
            return Some(WaxedOxidizedCopperChain {
                r#axis: Axis::Z,
                r#waterlogged: true,
            });
        }
        return None;
    }
}

