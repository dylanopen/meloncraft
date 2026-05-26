use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BrownStainedGlassPane {
    pub south: bool,
    pub waterlogged: bool,
    pub west: bool,
    pub east: bool,
    pub north: bool,
}

impl BlockState for BrownStainedGlassPane {
    fn to_id(&self) -> i32 {
        if self.r#south == false
            && self.r#west == false
            && self.r#north == false
            && self.r#waterlogged == false
            && self.r#east == true
        {
            return 11657;
        }
        if self.r#east == false
            && self.r#south == false
            && self.r#west == true
            && self.r#waterlogged == true
            && self.r#north == true
        {
            return 11662;
        }
        if self.r#west == false
            && self.r#south == true
            && self.r#east == true
            && self.r#north == false
            && self.r#waterlogged == true
        {
            return 11651;
        }
        if self.r#west == true
            && self.r#north == false
            && self.r#south == true
            && self.r#waterlogged == false
            && self.r#east == true
        {
            return 11652;
        }
        if self.r#north == true
            && self.r#west == true
            && self.r#east == false
            && self.r#south == false
            && self.r#waterlogged == false
        {
            return 11664;
        }
        if self.r#east == true
            && self.r#north == false
            && self.r#south == false
            && self.r#waterlogged == false
            && self.r#west == true
        {
            return 11656;
        }
        if self.r#waterlogged == false
            && self.r#east == false
            && self.r#west == false
            && self.r#south == true
            && self.r#north == false
        {
            return 11669;
        }
        if self.r#south == false
            && self.r#waterlogged == false
            && self.r#west == true
            && self.r#east == true
            && self.r#north == true
        {
            return 11648;
        }
        if self.r#west == true
            && self.r#north == false
            && self.r#waterlogged == true
            && self.r#south == false
            && self.r#east == false
        {
            return 11670;
        }
        if self.r#east == true
            && self.r#waterlogged == true
            && self.r#west == true
            && self.r#north == true
            && self.r#south == false
        {
            return 11646;
        }
        if self.r#west == false
            && self.r#south == false
            && self.r#east == false
            && self.r#waterlogged == false
            && self.r#north == false
        {
            return 11673;
        }
        if self.r#waterlogged == false
            && self.r#north == false
            && self.r#west == true
            && self.r#south == true
            && self.r#east == false
        {
            return 11668;
        }
        if self.r#east == true
            && self.r#south == true
            && self.r#north == true
            && self.r#west == false
            && self.r#waterlogged == true
        {
            return 11643;
        }
        if self.r#north == true
            && self.r#south == true
            && self.r#east == false
            && self.r#waterlogged == false
            && self.r#west == true
        {
            return 11660;
        }
        if self.r#waterlogged == true
            && self.r#south == false
            && self.r#north == true
            && self.r#east == true
            && self.r#west == false
        {
            return 11647;
        }
        if self.r#south == false
            && self.r#waterlogged == true
            && self.r#east == true
            && self.r#west == false
            && self.r#north == false
        {
            return 11655;
        }
        if self.r#north == false
            && self.r#south == true
            && self.r#west == false
            && self.r#east == false
            && self.r#waterlogged == true
        {
            return 11667;
        }
        if self.r#north == true
            && self.r#east == true
            && self.r#south == true
            && self.r#west == true
            && self.r#waterlogged == false
        {
            return 11644;
        }
        if self.r#east == true
            && self.r#waterlogged == true
            && self.r#west == true
            && self.r#south == true
            && self.r#north == false
        {
            return 11650;
        }
        if self.r#west == false
            && self.r#south == false
            && self.r#north == true
            && self.r#east == false
            && self.r#waterlogged == true
        {
            return 11663;
        }
        if self.r#west == true
            && self.r#north == true
            && self.r#waterlogged == true
            && self.r#east == true
            && self.r#south == true
        {
            return 11642;
        }
        if self.r#north == false
            && self.r#south == true
            && self.r#east == true
            && self.r#waterlogged == false
            && self.r#west == false
        {
            return 11653;
        }
        if self.r#south == true
            && self.r#north == false
            && self.r#waterlogged == true
            && self.r#east == false
            && self.r#west == true
        {
            return 11666;
        }
        if self.r#east == true
            && self.r#waterlogged == false
            && self.r#north == true
            && self.r#south == true
            && self.r#west == false
        {
            return 11645;
        }
        if self.r#south == false
            && self.r#west == true
            && self.r#east == false
            && self.r#north == false
            && self.r#waterlogged == false
        {
            return 11672;
        }
        if self.r#north == false
            && self.r#west == false
            && self.r#east == false
            && self.r#south == false
            && self.r#waterlogged == true
        {
            return 11671;
        }
        if self.r#north == false
            && self.r#south == false
            && self.r#east == true
            && self.r#west == true
            && self.r#waterlogged == true
        {
            return 11654;
        }
        if self.r#north == true
            && self.r#east == false
            && self.r#waterlogged == false
            && self.r#west == false
            && self.r#south == true
        {
            return 11661;
        }
        if self.r#west == false
            && self.r#north == true
            && self.r#south == false
            && self.r#waterlogged == false
            && self.r#east == false
        {
            return 11665;
        }
        if self.r#east == false
            && self.r#north == true
            && self.r#south == true
            && self.r#west == true
            && self.r#waterlogged == true
        {
            return 11658;
        }
        if self.r#south == false
            && self.r#west == false
            && self.r#east == true
            && self.r#north == true
            && self.r#waterlogged == false
        {
            return 11649;
        }
        if self.r#south == true
            && self.r#north == true
            && self.r#east == false
            && self.r#waterlogged == true
            && self.r#west == false
        {
            return 11659;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 11657 {
            return Some(BrownStainedGlassPane {
                r#south: false,
                r#west: false,
                r#north: false,
                r#waterlogged: false,
                r#east: true,
            });
        }
        if state_id == 11662 {
            return Some(BrownStainedGlassPane {
                r#east: false,
                r#south: false,
                r#west: true,
                r#waterlogged: true,
                r#north: true,
            });
        }
        if state_id == 11651 {
            return Some(BrownStainedGlassPane {
                r#west: false,
                r#south: true,
                r#east: true,
                r#north: false,
                r#waterlogged: true,
            });
        }
        if state_id == 11652 {
            return Some(BrownStainedGlassPane {
                r#west: true,
                r#north: false,
                r#south: true,
                r#waterlogged: false,
                r#east: true,
            });
        }
        if state_id == 11664 {
            return Some(BrownStainedGlassPane {
                r#north: true,
                r#west: true,
                r#east: false,
                r#south: false,
                r#waterlogged: false,
            });
        }
        if state_id == 11656 {
            return Some(BrownStainedGlassPane {
                r#east: true,
                r#north: false,
                r#south: false,
                r#waterlogged: false,
                r#west: true,
            });
        }
        if state_id == 11669 {
            return Some(BrownStainedGlassPane {
                r#waterlogged: false,
                r#east: false,
                r#west: false,
                r#south: true,
                r#north: false,
            });
        }
        if state_id == 11648 {
            return Some(BrownStainedGlassPane {
                r#south: false,
                r#waterlogged: false,
                r#west: true,
                r#east: true,
                r#north: true,
            });
        }
        if state_id == 11670 {
            return Some(BrownStainedGlassPane {
                r#west: true,
                r#north: false,
                r#waterlogged: true,
                r#south: false,
                r#east: false,
            });
        }
        if state_id == 11646 {
            return Some(BrownStainedGlassPane {
                r#east: true,
                r#waterlogged: true,
                r#west: true,
                r#north: true,
                r#south: false,
            });
        }
        if state_id == 11673 {
            return Some(BrownStainedGlassPane {
                r#west: false,
                r#south: false,
                r#east: false,
                r#waterlogged: false,
                r#north: false,
            });
        }
        if state_id == 11668 {
            return Some(BrownStainedGlassPane {
                r#waterlogged: false,
                r#north: false,
                r#west: true,
                r#south: true,
                r#east: false,
            });
        }
        if state_id == 11643 {
            return Some(BrownStainedGlassPane {
                r#east: true,
                r#south: true,
                r#north: true,
                r#west: false,
                r#waterlogged: true,
            });
        }
        if state_id == 11660 {
            return Some(BrownStainedGlassPane {
                r#north: true,
                r#south: true,
                r#east: false,
                r#waterlogged: false,
                r#west: true,
            });
        }
        if state_id == 11647 {
            return Some(BrownStainedGlassPane {
                r#waterlogged: true,
                r#south: false,
                r#north: true,
                r#east: true,
                r#west: false,
            });
        }
        if state_id == 11655 {
            return Some(BrownStainedGlassPane {
                r#south: false,
                r#waterlogged: true,
                r#east: true,
                r#west: false,
                r#north: false,
            });
        }
        if state_id == 11667 {
            return Some(BrownStainedGlassPane {
                r#north: false,
                r#south: true,
                r#west: false,
                r#east: false,
                r#waterlogged: true,
            });
        }
        if state_id == 11644 {
            return Some(BrownStainedGlassPane {
                r#north: true,
                r#east: true,
                r#south: true,
                r#west: true,
                r#waterlogged: false,
            });
        }
        if state_id == 11650 {
            return Some(BrownStainedGlassPane {
                r#east: true,
                r#waterlogged: true,
                r#west: true,
                r#south: true,
                r#north: false,
            });
        }
        if state_id == 11663 {
            return Some(BrownStainedGlassPane {
                r#west: false,
                r#south: false,
                r#north: true,
                r#east: false,
                r#waterlogged: true,
            });
        }
        if state_id == 11642 {
            return Some(BrownStainedGlassPane {
                r#west: true,
                r#north: true,
                r#waterlogged: true,
                r#east: true,
                r#south: true,
            });
        }
        if state_id == 11653 {
            return Some(BrownStainedGlassPane {
                r#north: false,
                r#south: true,
                r#east: true,
                r#waterlogged: false,
                r#west: false,
            });
        }
        if state_id == 11666 {
            return Some(BrownStainedGlassPane {
                r#south: true,
                r#north: false,
                r#waterlogged: true,
                r#east: false,
                r#west: true,
            });
        }
        if state_id == 11645 {
            return Some(BrownStainedGlassPane {
                r#east: true,
                r#waterlogged: false,
                r#north: true,
                r#south: true,
                r#west: false,
            });
        }
        if state_id == 11672 {
            return Some(BrownStainedGlassPane {
                r#south: false,
                r#west: true,
                r#east: false,
                r#north: false,
                r#waterlogged: false,
            });
        }
        if state_id == 11671 {
            return Some(BrownStainedGlassPane {
                r#north: false,
                r#west: false,
                r#east: false,
                r#south: false,
                r#waterlogged: true,
            });
        }
        if state_id == 11654 {
            return Some(BrownStainedGlassPane {
                r#north: false,
                r#south: false,
                r#east: true,
                r#west: true,
                r#waterlogged: true,
            });
        }
        if state_id == 11661 {
            return Some(BrownStainedGlassPane {
                r#north: true,
                r#east: false,
                r#waterlogged: false,
                r#west: false,
                r#south: true,
            });
        }
        if state_id == 11665 {
            return Some(BrownStainedGlassPane {
                r#west: false,
                r#north: true,
                r#south: false,
                r#waterlogged: false,
                r#east: false,
            });
        }
        if state_id == 11658 {
            return Some(BrownStainedGlassPane {
                r#east: false,
                r#north: true,
                r#south: true,
                r#west: true,
                r#waterlogged: true,
            });
        }
        if state_id == 11649 {
            return Some(BrownStainedGlassPane {
                r#south: false,
                r#west: false,
                r#east: true,
                r#north: true,
                r#waterlogged: false,
            });
        }
        if state_id == 11659 {
            return Some(BrownStainedGlassPane {
                r#south: true,
                r#north: true,
                r#east: false,
                r#waterlogged: true,
                r#west: false,
            });
        }
        return None;
    }
}
