use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JungleFence {
    pub east: bool,
    pub west: bool,
    pub waterlogged: bool,
    pub north: bool,
    pub south: bool,
}

impl BlockState for JungleFence {
    fn to_id(&self) -> i32 {
        if self.r#south == false
            && self.r#east == false
            && self.r#north == false
            && self.r#west == false
            && self.r#waterlogged == false
        {
            return 13665;
        }
        if self.r#north == true
            && self.r#south == false
            && self.r#east == false
            && self.r#waterlogged == true
            && self.r#west == false
        {
            return 13655;
        }
        if self.r#west == false
            && self.r#south == true
            && self.r#north == true
            && self.r#east == false
            && self.r#waterlogged == true
        {
            return 13651;
        }
        if self.r#south == true
            && self.r#east == false
            && self.r#waterlogged == false
            && self.r#west == true
            && self.r#north == false
        {
            return 13660;
        }
        if self.r#waterlogged == false
            && self.r#west == false
            && self.r#south == true
            && self.r#east == false
            && self.r#north == false
        {
            return 13661;
        }
        if self.r#east == true
            && self.r#waterlogged == false
            && self.r#south == true
            && self.r#west == false
            && self.r#north == false
        {
            return 13645;
        }
        if self.r#south == true
            && self.r#east == true
            && self.r#north == false
            && self.r#waterlogged == true
            && self.r#west == false
        {
            return 13643;
        }
        if self.r#south == false
            && self.r#waterlogged == true
            && self.r#east == true
            && self.r#north == true
            && self.r#west == true
        {
            return 13638;
        }
        if self.r#west == false
            && self.r#north == false
            && self.r#east == false
            && self.r#waterlogged == true
            && self.r#south == true
        {
            return 13659;
        }
        if self.r#west == false
            && self.r#south == true
            && self.r#east == false
            && self.r#waterlogged == false
            && self.r#north == true
        {
            return 13653;
        }
        if self.r#north == true
            && self.r#east == true
            && self.r#south == true
            && self.r#west == true
            && self.r#waterlogged == false
        {
            return 13636;
        }
        if self.r#east == true
            && self.r#waterlogged == true
            && self.r#west == false
            && self.r#north == true
            && self.r#south == false
        {
            return 13639;
        }
        if self.r#west == true
            && self.r#north == true
            && self.r#south == false
            && self.r#east == false
            && self.r#waterlogged == true
        {
            return 13654;
        }
        if self.r#south == true
            && self.r#waterlogged == true
            && self.r#east == false
            && self.r#west == true
            && self.r#north == false
        {
            return 13658;
        }
        if self.r#south == false
            && self.r#north == false
            && self.r#east == false
            && self.r#waterlogged == true
            && self.r#west == true
        {
            return 13662;
        }
        if self.r#north == false
            && self.r#west == false
            && self.r#south == false
            && self.r#waterlogged == true
            && self.r#east == false
        {
            return 13663;
        }
        if self.r#east == true
            && self.r#west == true
            && self.r#waterlogged == true
            && self.r#south == true
            && self.r#north == false
        {
            return 13642;
        }
        if self.r#east == true
            && self.r#north == false
            && self.r#south == false
            && self.r#waterlogged == false
            && self.r#west == true
        {
            return 13648;
        }
        if self.r#waterlogged == false
            && self.r#west == true
            && self.r#north == false
            && self.r#east == true
            && self.r#south == true
        {
            return 13644;
        }
        if self.r#waterlogged == false
            && self.r#west == false
            && self.r#south == false
            && self.r#north == false
            && self.r#east == true
        {
            return 13649;
        }
        if self.r#east == false
            && self.r#waterlogged == false
            && self.r#west == true
            && self.r#south == false
            && self.r#north == true
        {
            return 13656;
        }
        if self.r#north == false
            && self.r#waterlogged == false
            && self.r#east == false
            && self.r#west == true
            && self.r#south == false
        {
            return 13664;
        }
        if self.r#south == false
            && self.r#waterlogged == false
            && self.r#east == true
            && self.r#west == true
            && self.r#north == true
        {
            return 13640;
        }
        if self.r#west == true
            && self.r#east == false
            && self.r#north == true
            && self.r#south == true
            && self.r#waterlogged == true
        {
            return 13650;
        }
        if self.r#east == false
            && self.r#north == true
            && self.r#waterlogged == false
            && self.r#west == true
            && self.r#south == true
        {
            return 13652;
        }
        if self.r#waterlogged == true
            && self.r#north == false
            && self.r#west == false
            && self.r#south == false
            && self.r#east == true
        {
            return 13647;
        }
        if self.r#north == true
            && self.r#west == false
            && self.r#waterlogged == false
            && self.r#south == false
            && self.r#east == false
        {
            return 13657;
        }
        if self.r#south == true
            && self.r#east == true
            && self.r#waterlogged == true
            && self.r#west == false
            && self.r#north == true
        {
            return 13635;
        }
        if self.r#east == true
            && self.r#south == false
            && self.r#waterlogged == true
            && self.r#north == false
            && self.r#west == true
        {
            return 13646;
        }
        if self.r#south == true
            && self.r#east == true
            && self.r#north == true
            && self.r#waterlogged == true
            && self.r#west == true
        {
            return 13634;
        }
        if self.r#waterlogged == false
            && self.r#south == true
            && self.r#west == false
            && self.r#east == true
            && self.r#north == true
        {
            return 13637;
        }
        if self.r#north == true
            && self.r#south == false
            && self.r#waterlogged == false
            && self.r#west == false
            && self.r#east == true
        {
            return 13641;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 13665 {
            return Some(JungleFence {
                r#south: false,
                r#east: false,
                r#north: false,
                r#west: false,
                r#waterlogged: false,
            });
        }
        if state_id == 13655 {
            return Some(JungleFence {
                r#north: true,
                r#south: false,
                r#east: false,
                r#waterlogged: true,
                r#west: false,
            });
        }
        if state_id == 13651 {
            return Some(JungleFence {
                r#west: false,
                r#south: true,
                r#north: true,
                r#east: false,
                r#waterlogged: true,
            });
        }
        if state_id == 13660 {
            return Some(JungleFence {
                r#south: true,
                r#east: false,
                r#waterlogged: false,
                r#west: true,
                r#north: false,
            });
        }
        if state_id == 13661 {
            return Some(JungleFence {
                r#waterlogged: false,
                r#west: false,
                r#south: true,
                r#east: false,
                r#north: false,
            });
        }
        if state_id == 13645 {
            return Some(JungleFence {
                r#east: true,
                r#waterlogged: false,
                r#south: true,
                r#west: false,
                r#north: false,
            });
        }
        if state_id == 13643 {
            return Some(JungleFence {
                r#south: true,
                r#east: true,
                r#north: false,
                r#waterlogged: true,
                r#west: false,
            });
        }
        if state_id == 13638 {
            return Some(JungleFence {
                r#south: false,
                r#waterlogged: true,
                r#east: true,
                r#north: true,
                r#west: true,
            });
        }
        if state_id == 13659 {
            return Some(JungleFence {
                r#west: false,
                r#north: false,
                r#east: false,
                r#waterlogged: true,
                r#south: true,
            });
        }
        if state_id == 13653 {
            return Some(JungleFence {
                r#west: false,
                r#south: true,
                r#east: false,
                r#waterlogged: false,
                r#north: true,
            });
        }
        if state_id == 13636 {
            return Some(JungleFence {
                r#north: true,
                r#east: true,
                r#south: true,
                r#west: true,
                r#waterlogged: false,
            });
        }
        if state_id == 13639 {
            return Some(JungleFence {
                r#east: true,
                r#waterlogged: true,
                r#west: false,
                r#north: true,
                r#south: false,
            });
        }
        if state_id == 13654 {
            return Some(JungleFence {
                r#west: true,
                r#north: true,
                r#south: false,
                r#east: false,
                r#waterlogged: true,
            });
        }
        if state_id == 13658 {
            return Some(JungleFence {
                r#south: true,
                r#waterlogged: true,
                r#east: false,
                r#west: true,
                r#north: false,
            });
        }
        if state_id == 13662 {
            return Some(JungleFence {
                r#south: false,
                r#north: false,
                r#east: false,
                r#waterlogged: true,
                r#west: true,
            });
        }
        if state_id == 13663 {
            return Some(JungleFence {
                r#north: false,
                r#west: false,
                r#south: false,
                r#waterlogged: true,
                r#east: false,
            });
        }
        if state_id == 13642 {
            return Some(JungleFence {
                r#east: true,
                r#west: true,
                r#waterlogged: true,
                r#south: true,
                r#north: false,
            });
        }
        if state_id == 13648 {
            return Some(JungleFence {
                r#east: true,
                r#north: false,
                r#south: false,
                r#waterlogged: false,
                r#west: true,
            });
        }
        if state_id == 13644 {
            return Some(JungleFence {
                r#waterlogged: false,
                r#west: true,
                r#north: false,
                r#east: true,
                r#south: true,
            });
        }
        if state_id == 13649 {
            return Some(JungleFence {
                r#waterlogged: false,
                r#west: false,
                r#south: false,
                r#north: false,
                r#east: true,
            });
        }
        if state_id == 13656 {
            return Some(JungleFence {
                r#east: false,
                r#waterlogged: false,
                r#west: true,
                r#south: false,
                r#north: true,
            });
        }
        if state_id == 13664 {
            return Some(JungleFence {
                r#north: false,
                r#waterlogged: false,
                r#east: false,
                r#west: true,
                r#south: false,
            });
        }
        if state_id == 13640 {
            return Some(JungleFence {
                r#south: false,
                r#waterlogged: false,
                r#east: true,
                r#west: true,
                r#north: true,
            });
        }
        if state_id == 13650 {
            return Some(JungleFence {
                r#west: true,
                r#east: false,
                r#north: true,
                r#south: true,
                r#waterlogged: true,
            });
        }
        if state_id == 13652 {
            return Some(JungleFence {
                r#east: false,
                r#north: true,
                r#waterlogged: false,
                r#west: true,
                r#south: true,
            });
        }
        if state_id == 13647 {
            return Some(JungleFence {
                r#waterlogged: true,
                r#north: false,
                r#west: false,
                r#south: false,
                r#east: true,
            });
        }
        if state_id == 13657 {
            return Some(JungleFence {
                r#north: true,
                r#west: false,
                r#waterlogged: false,
                r#south: false,
                r#east: false,
            });
        }
        if state_id == 13635 {
            return Some(JungleFence {
                r#south: true,
                r#east: true,
                r#waterlogged: true,
                r#west: false,
                r#north: true,
            });
        }
        if state_id == 13646 {
            return Some(JungleFence {
                r#east: true,
                r#south: false,
                r#waterlogged: true,
                r#north: false,
                r#west: true,
            });
        }
        if state_id == 13634 {
            return Some(JungleFence {
                r#south: true,
                r#east: true,
                r#north: true,
                r#waterlogged: true,
                r#west: true,
            });
        }
        if state_id == 13637 {
            return Some(JungleFence {
                r#waterlogged: false,
                r#south: true,
                r#west: false,
                r#east: true,
                r#north: true,
            });
        }
        if state_id == 13641 {
            return Some(JungleFence {
                r#north: true,
                r#south: false,
                r#waterlogged: false,
                r#west: false,
                r#east: true,
            });
        }
        return None;
    }
}
