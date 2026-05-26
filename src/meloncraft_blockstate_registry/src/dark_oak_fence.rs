use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DarkOakFence {
    pub east: bool,
    pub north: bool,
    pub south: bool,
    pub waterlogged: bool,
    pub west: bool,
}

impl BlockState for DarkOakFence {
    fn to_id(&self) -> i32 {
        if self.r#north == true
            && self.r#waterlogged == false
            && self.r#east == false
            && self.r#south == true
            && self.r#west == false
        {
            return 13749;
        }
        if self.r#south == true
            && self.r#west == false
            && self.r#north == true
            && self.r#waterlogged == true
            && self.r#east == true
        {
            return 13731;
        }
        if self.r#east == true
            && self.r#south == true
            && self.r#north == true
            && self.r#waterlogged == false
            && self.r#west == true
        {
            return 13732;
        }
        if self.r#waterlogged == true
            && self.r#west == true
            && self.r#south == false
            && self.r#east == true
            && self.r#north == false
        {
            return 13742;
        }
        if self.r#waterlogged == true
            && self.r#east == true
            && self.r#south == false
            && self.r#west == false
            && self.r#north == false
        {
            return 13743;
        }
        if self.r#north == true
            && self.r#south == false
            && self.r#waterlogged == true
            && self.r#west == true
            && self.r#east == true
        {
            return 13734;
        }
        if self.r#south == true
            && self.r#waterlogged == false
            && self.r#east == true
            && self.r#north == false
            && self.r#west == false
        {
            return 13741;
        }
        if self.r#west == false
            && self.r#east == true
            && self.r#north == false
            && self.r#south == false
            && self.r#waterlogged == false
        {
            return 13745;
        }
        if self.r#east == false
            && self.r#south == true
            && self.r#waterlogged == false
            && self.r#west == true
            && self.r#north == false
        {
            return 13756;
        }
        if self.r#east == false
            && self.r#south == true
            && self.r#waterlogged == true
            && self.r#north == false
            && self.r#west == true
        {
            return 13754;
        }
        if self.r#east == false
            && self.r#north == true
            && self.r#waterlogged == false
            && self.r#west == true
            && self.r#south == false
        {
            return 13752;
        }
        if self.r#north == true
            && self.r#east == true
            && self.r#waterlogged == false
            && self.r#south == false
            && self.r#west == true
        {
            return 13736;
        }
        if self.r#east == true
            && self.r#north == false
            && self.r#waterlogged == false
            && self.r#south == true
            && self.r#west == true
        {
            return 13740;
        }
        if self.r#north == false
            && self.r#waterlogged == false
            && self.r#west == true
            && self.r#east == false
            && self.r#south == false
        {
            return 13760;
        }
        if self.r#south == true
            && self.r#waterlogged == true
            && self.r#west == true
            && self.r#east == true
            && self.r#north == false
        {
            return 13738;
        }
        if self.r#south == false
            && self.r#west == false
            && self.r#north == false
            && self.r#waterlogged == true
            && self.r#east == false
        {
            return 13759;
        }
        if self.r#north == true
            && self.r#west == false
            && self.r#east == true
            && self.r#south == false
            && self.r#waterlogged == true
        {
            return 13735;
        }
        if self.r#south == false
            && self.r#waterlogged == false
            && self.r#west == false
            && self.r#east == false
            && self.r#north == false
        {
            return 13761;
        }
        if self.r#north == true
            && self.r#south == true
            && self.r#east == false
            && self.r#waterlogged == true
            && self.r#west == false
        {
            return 13747;
        }
        if self.r#south == true
            && self.r#waterlogged == false
            && self.r#west == false
            && self.r#east == true
            && self.r#north == true
        {
            return 13733;
        }
        if self.r#west == false
            && self.r#east == true
            && self.r#north == false
            && self.r#waterlogged == true
            && self.r#south == true
        {
            return 13739;
        }
        if self.r#north == true
            && self.r#south == false
            && self.r#east == false
            && self.r#west == true
            && self.r#waterlogged == true
        {
            return 13750;
        }
        if self.r#west == false
            && self.r#south == false
            && self.r#east == false
            && self.r#waterlogged == true
            && self.r#north == true
        {
            return 13751;
        }
        if self.r#east == false
            && self.r#waterlogged == true
            && self.r#west == false
            && self.r#south == true
            && self.r#north == false
        {
            return 13755;
        }
        if self.r#east == false
            && self.r#west == false
            && self.r#north == false
            && self.r#south == true
            && self.r#waterlogged == false
        {
            return 13757;
        }
        if self.r#north == false
            && self.r#south == false
            && self.r#east == true
            && self.r#waterlogged == false
            && self.r#west == true
        {
            return 13744;
        }
        if self.r#waterlogged == true
            && self.r#east == true
            && self.r#west == true
            && self.r#south == true
            && self.r#north == true
        {
            return 13730;
        }
        if self.r#south == false
            && self.r#north == true
            && self.r#waterlogged == false
            && self.r#west == false
            && self.r#east == true
        {
            return 13737;
        }
        if self.r#east == false
            && self.r#waterlogged == true
            && self.r#west == true
            && self.r#north == false
            && self.r#south == false
        {
            return 13758;
        }
        if self.r#waterlogged == false
            && self.r#north == true
            && self.r#south == false
            && self.r#east == false
            && self.r#west == false
        {
            return 13753;
        }
        if self.r#waterlogged == false
            && self.r#north == true
            && self.r#east == false
            && self.r#south == true
            && self.r#west == true
        {
            return 13748;
        }
        if self.r#north == true
            && self.r#west == true
            && self.r#east == false
            && self.r#waterlogged == true
            && self.r#south == true
        {
            return 13746;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 13749 {
            return Some(DarkOakFence {
                r#north: true,
                r#waterlogged: false,
                r#east: false,
                r#south: true,
                r#west: false,
            });
        }
        if state_id == 13731 {
            return Some(DarkOakFence {
                r#south: true,
                r#west: false,
                r#north: true,
                r#waterlogged: true,
                r#east: true,
            });
        }
        if state_id == 13732 {
            return Some(DarkOakFence {
                r#east: true,
                r#south: true,
                r#north: true,
                r#waterlogged: false,
                r#west: true,
            });
        }
        if state_id == 13742 {
            return Some(DarkOakFence {
                r#waterlogged: true,
                r#west: true,
                r#south: false,
                r#east: true,
                r#north: false,
            });
        }
        if state_id == 13743 {
            return Some(DarkOakFence {
                r#waterlogged: true,
                r#east: true,
                r#south: false,
                r#west: false,
                r#north: false,
            });
        }
        if state_id == 13734 {
            return Some(DarkOakFence {
                r#north: true,
                r#south: false,
                r#waterlogged: true,
                r#west: true,
                r#east: true,
            });
        }
        if state_id == 13741 {
            return Some(DarkOakFence {
                r#south: true,
                r#waterlogged: false,
                r#east: true,
                r#north: false,
                r#west: false,
            });
        }
        if state_id == 13745 {
            return Some(DarkOakFence {
                r#west: false,
                r#east: true,
                r#north: false,
                r#south: false,
                r#waterlogged: false,
            });
        }
        if state_id == 13756 {
            return Some(DarkOakFence {
                r#east: false,
                r#south: true,
                r#waterlogged: false,
                r#west: true,
                r#north: false,
            });
        }
        if state_id == 13754 {
            return Some(DarkOakFence {
                r#east: false,
                r#south: true,
                r#waterlogged: true,
                r#north: false,
                r#west: true,
            });
        }
        if state_id == 13752 {
            return Some(DarkOakFence {
                r#east: false,
                r#north: true,
                r#waterlogged: false,
                r#west: true,
                r#south: false,
            });
        }
        if state_id == 13736 {
            return Some(DarkOakFence {
                r#north: true,
                r#east: true,
                r#waterlogged: false,
                r#south: false,
                r#west: true,
            });
        }
        if state_id == 13740 {
            return Some(DarkOakFence {
                r#east: true,
                r#north: false,
                r#waterlogged: false,
                r#south: true,
                r#west: true,
            });
        }
        if state_id == 13760 {
            return Some(DarkOakFence {
                r#north: false,
                r#waterlogged: false,
                r#west: true,
                r#east: false,
                r#south: false,
            });
        }
        if state_id == 13738 {
            return Some(DarkOakFence {
                r#south: true,
                r#waterlogged: true,
                r#west: true,
                r#east: true,
                r#north: false,
            });
        }
        if state_id == 13759 {
            return Some(DarkOakFence {
                r#south: false,
                r#west: false,
                r#north: false,
                r#waterlogged: true,
                r#east: false,
            });
        }
        if state_id == 13735 {
            return Some(DarkOakFence {
                r#north: true,
                r#west: false,
                r#east: true,
                r#south: false,
                r#waterlogged: true,
            });
        }
        if state_id == 13761 {
            return Some(DarkOakFence {
                r#south: false,
                r#waterlogged: false,
                r#west: false,
                r#east: false,
                r#north: false,
            });
        }
        if state_id == 13747 {
            return Some(DarkOakFence {
                r#north: true,
                r#south: true,
                r#east: false,
                r#waterlogged: true,
                r#west: false,
            });
        }
        if state_id == 13733 {
            return Some(DarkOakFence {
                r#south: true,
                r#waterlogged: false,
                r#west: false,
                r#east: true,
                r#north: true,
            });
        }
        if state_id == 13739 {
            return Some(DarkOakFence {
                r#west: false,
                r#east: true,
                r#north: false,
                r#waterlogged: true,
                r#south: true,
            });
        }
        if state_id == 13750 {
            return Some(DarkOakFence {
                r#north: true,
                r#south: false,
                r#east: false,
                r#west: true,
                r#waterlogged: true,
            });
        }
        if state_id == 13751 {
            return Some(DarkOakFence {
                r#west: false,
                r#south: false,
                r#east: false,
                r#waterlogged: true,
                r#north: true,
            });
        }
        if state_id == 13755 {
            return Some(DarkOakFence {
                r#east: false,
                r#waterlogged: true,
                r#west: false,
                r#south: true,
                r#north: false,
            });
        }
        if state_id == 13757 {
            return Some(DarkOakFence {
                r#east: false,
                r#west: false,
                r#north: false,
                r#south: true,
                r#waterlogged: false,
            });
        }
        if state_id == 13744 {
            return Some(DarkOakFence {
                r#north: false,
                r#south: false,
                r#east: true,
                r#waterlogged: false,
                r#west: true,
            });
        }
        if state_id == 13730 {
            return Some(DarkOakFence {
                r#waterlogged: true,
                r#east: true,
                r#west: true,
                r#south: true,
                r#north: true,
            });
        }
        if state_id == 13737 {
            return Some(DarkOakFence {
                r#south: false,
                r#north: true,
                r#waterlogged: false,
                r#west: false,
                r#east: true,
            });
        }
        if state_id == 13758 {
            return Some(DarkOakFence {
                r#east: false,
                r#waterlogged: true,
                r#west: true,
                r#north: false,
                r#south: false,
            });
        }
        if state_id == 13753 {
            return Some(DarkOakFence {
                r#waterlogged: false,
                r#north: true,
                r#south: false,
                r#east: false,
                r#west: false,
            });
        }
        if state_id == 13748 {
            return Some(DarkOakFence {
                r#waterlogged: false,
                r#north: true,
                r#east: false,
                r#south: true,
                r#west: true,
            });
        }
        if state_id == 13746 {
            return Some(DarkOakFence {
                r#north: true,
                r#west: true,
                r#east: false,
                r#waterlogged: true,
                r#south: true,
            });
        }
        return None;
    }
}
