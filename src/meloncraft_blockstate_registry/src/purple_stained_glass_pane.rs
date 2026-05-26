use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PurpleStainedGlassPane {
    pub north: bool,
    pub south: bool,
    pub east: bool,
    pub waterlogged: bool,
    pub west: bool,
}

impl BlockState for PurpleStainedGlassPane {
    fn to_id(&self) -> i32 {
        if self.r#north == false
            && self.r#waterlogged == true
            && self.r#south == false
            && self.r#east == false
            && self.r#west == true
        {
            return 11606;
        }
        if self.r#west == false
            && self.r#east == false
            && self.r#south == true
            && self.r#north == true
            && self.r#waterlogged == true
        {
            return 11595;
        }
        if self.r#west == false
            && self.r#waterlogged == true
            && self.r#east == true
            && self.r#north == true
            && self.r#south == true
        {
            return 11579;
        }
        if self.r#waterlogged == true
            && self.r#west == true
            && self.r#north == false
            && self.r#east == true
            && self.r#south == true
        {
            return 11586;
        }
        if self.r#north == false
            && self.r#south == true
            && self.r#waterlogged == false
            && self.r#east == true
            && self.r#west == true
        {
            return 11588;
        }
        if self.r#south == true
            && self.r#east == true
            && self.r#north == true
            && self.r#waterlogged == true
            && self.r#west == true
        {
            return 11578;
        }
        if self.r#east == true
            && self.r#south == false
            && self.r#west == true
            && self.r#north == true
            && self.r#waterlogged == true
        {
            return 11582;
        }
        if self.r#north == true
            && self.r#waterlogged == true
            && self.r#south == false
            && self.r#west == false
            && self.r#east == true
        {
            return 11583;
        }
        if self.r#north == true
            && self.r#waterlogged == false
            && self.r#west == false
            && self.r#east == true
            && self.r#south == false
        {
            return 11585;
        }
        if self.r#west == false
            && self.r#east == true
            && self.r#south == true
            && self.r#north == true
            && self.r#waterlogged == false
        {
            return 11581;
        }
        if self.r#north == false
            && self.r#east == true
            && self.r#south == true
            && self.r#waterlogged == false
            && self.r#west == false
        {
            return 11589;
        }
        if self.r#west == false
            && self.r#north == true
            && self.r#south == true
            && self.r#east == false
            && self.r#waterlogged == false
        {
            return 11597;
        }
        if self.r#east == false
            && self.r#north == true
            && self.r#west == true
            && self.r#waterlogged == true
            && self.r#south == false
        {
            return 11598;
        }
        if self.r#waterlogged == true
            && self.r#west == false
            && self.r#south == false
            && self.r#east == true
            && self.r#north == false
        {
            return 11591;
        }
        if self.r#waterlogged == true
            && self.r#north == false
            && self.r#west == false
            && self.r#south == true
            && self.r#east == true
        {
            return 11587;
        }
        if self.r#south == true
            && self.r#waterlogged == false
            && self.r#north == true
            && self.r#east == true
            && self.r#west == true
        {
            return 11580;
        }
        if self.r#east == false
            && self.r#west == false
            && self.r#south == false
            && self.r#waterlogged == true
            && self.r#north == true
        {
            return 11599;
        }
        if self.r#west == false
            && self.r#east == false
            && self.r#waterlogged == false
            && self.r#south == true
            && self.r#north == false
        {
            return 11605;
        }
        if self.r#waterlogged == true
            && self.r#west == true
            && self.r#east == true
            && self.r#north == false
            && self.r#south == false
        {
            return 11590;
        }
        if self.r#east == false
            && self.r#waterlogged == false
            && self.r#north == true
            && self.r#west == true
            && self.r#south == false
        {
            return 11600;
        }
        if self.r#east == false
            && self.r#north == false
            && self.r#west == false
            && self.r#south == true
            && self.r#waterlogged == true
        {
            return 11603;
        }
        if self.r#west == true
            && self.r#south == true
            && self.r#north == false
            && self.r#east == false
            && self.r#waterlogged == true
        {
            return 11602;
        }
        if self.r#waterlogged == true
            && self.r#east == false
            && self.r#south == true
            && self.r#north == true
            && self.r#west == true
        {
            return 11594;
        }
        if self.r#west == false
            && self.r#north == true
            && self.r#waterlogged == false
            && self.r#south == false
            && self.r#east == false
        {
            return 11601;
        }
        if self.r#south == true
            && self.r#north == false
            && self.r#west == true
            && self.r#east == false
            && self.r#waterlogged == false
        {
            return 11604;
        }
        if self.r#north == true
            && self.r#east == true
            && self.r#waterlogged == false
            && self.r#south == false
            && self.r#west == true
        {
            return 11584;
        }
        if self.r#south == false
            && self.r#west == true
            && self.r#east == true
            && self.r#north == false
            && self.r#waterlogged == false
        {
            return 11592;
        }
        if self.r#north == true
            && self.r#west == true
            && self.r#east == false
            && self.r#south == true
            && self.r#waterlogged == false
        {
            return 11596;
        }
        if self.r#waterlogged == false
            && self.r#south == false
            && self.r#north == false
            && self.r#west == true
            && self.r#east == false
        {
            return 11608;
        }
        if self.r#west == false
            && self.r#waterlogged == false
            && self.r#north == false
            && self.r#east == false
            && self.r#south == false
        {
            return 11609;
        }
        if self.r#east == true
            && self.r#north == false
            && self.r#south == false
            && self.r#west == false
            && self.r#waterlogged == false
        {
            return 11593;
        }
        if self.r#east == false
            && self.r#south == false
            && self.r#waterlogged == true
            && self.r#west == false
            && self.r#north == false
        {
            return 11607;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 11606 {
            return Some(PurpleStainedGlassPane {
                r#north: false,
                r#waterlogged: true,
                r#south: false,
                r#east: false,
                r#west: true,
            });
        }
        if state_id == 11595 {
            return Some(PurpleStainedGlassPane {
                r#west: false,
                r#east: false,
                r#south: true,
                r#north: true,
                r#waterlogged: true,
            });
        }
        if state_id == 11579 {
            return Some(PurpleStainedGlassPane {
                r#west: false,
                r#waterlogged: true,
                r#east: true,
                r#north: true,
                r#south: true,
            });
        }
        if state_id == 11586 {
            return Some(PurpleStainedGlassPane {
                r#waterlogged: true,
                r#west: true,
                r#north: false,
                r#east: true,
                r#south: true,
            });
        }
        if state_id == 11588 {
            return Some(PurpleStainedGlassPane {
                r#north: false,
                r#south: true,
                r#waterlogged: false,
                r#east: true,
                r#west: true,
            });
        }
        if state_id == 11578 {
            return Some(PurpleStainedGlassPane {
                r#south: true,
                r#east: true,
                r#north: true,
                r#waterlogged: true,
                r#west: true,
            });
        }
        if state_id == 11582 {
            return Some(PurpleStainedGlassPane {
                r#east: true,
                r#south: false,
                r#west: true,
                r#north: true,
                r#waterlogged: true,
            });
        }
        if state_id == 11583 {
            return Some(PurpleStainedGlassPane {
                r#north: true,
                r#waterlogged: true,
                r#south: false,
                r#west: false,
                r#east: true,
            });
        }
        if state_id == 11585 {
            return Some(PurpleStainedGlassPane {
                r#north: true,
                r#waterlogged: false,
                r#west: false,
                r#east: true,
                r#south: false,
            });
        }
        if state_id == 11581 {
            return Some(PurpleStainedGlassPane {
                r#west: false,
                r#east: true,
                r#south: true,
                r#north: true,
                r#waterlogged: false,
            });
        }
        if state_id == 11589 {
            return Some(PurpleStainedGlassPane {
                r#north: false,
                r#east: true,
                r#south: true,
                r#waterlogged: false,
                r#west: false,
            });
        }
        if state_id == 11597 {
            return Some(PurpleStainedGlassPane {
                r#west: false,
                r#north: true,
                r#south: true,
                r#east: false,
                r#waterlogged: false,
            });
        }
        if state_id == 11598 {
            return Some(PurpleStainedGlassPane {
                r#east: false,
                r#north: true,
                r#west: true,
                r#waterlogged: true,
                r#south: false,
            });
        }
        if state_id == 11591 {
            return Some(PurpleStainedGlassPane {
                r#waterlogged: true,
                r#west: false,
                r#south: false,
                r#east: true,
                r#north: false,
            });
        }
        if state_id == 11587 {
            return Some(PurpleStainedGlassPane {
                r#waterlogged: true,
                r#north: false,
                r#west: false,
                r#south: true,
                r#east: true,
            });
        }
        if state_id == 11580 {
            return Some(PurpleStainedGlassPane {
                r#south: true,
                r#waterlogged: false,
                r#north: true,
                r#east: true,
                r#west: true,
            });
        }
        if state_id == 11599 {
            return Some(PurpleStainedGlassPane {
                r#east: false,
                r#west: false,
                r#south: false,
                r#waterlogged: true,
                r#north: true,
            });
        }
        if state_id == 11605 {
            return Some(PurpleStainedGlassPane {
                r#west: false,
                r#east: false,
                r#waterlogged: false,
                r#south: true,
                r#north: false,
            });
        }
        if state_id == 11590 {
            return Some(PurpleStainedGlassPane {
                r#waterlogged: true,
                r#west: true,
                r#east: true,
                r#north: false,
                r#south: false,
            });
        }
        if state_id == 11600 {
            return Some(PurpleStainedGlassPane {
                r#east: false,
                r#waterlogged: false,
                r#north: true,
                r#west: true,
                r#south: false,
            });
        }
        if state_id == 11603 {
            return Some(PurpleStainedGlassPane {
                r#east: false,
                r#north: false,
                r#west: false,
                r#south: true,
                r#waterlogged: true,
            });
        }
        if state_id == 11602 {
            return Some(PurpleStainedGlassPane {
                r#west: true,
                r#south: true,
                r#north: false,
                r#east: false,
                r#waterlogged: true,
            });
        }
        if state_id == 11594 {
            return Some(PurpleStainedGlassPane {
                r#waterlogged: true,
                r#east: false,
                r#south: true,
                r#north: true,
                r#west: true,
            });
        }
        if state_id == 11601 {
            return Some(PurpleStainedGlassPane {
                r#west: false,
                r#north: true,
                r#waterlogged: false,
                r#south: false,
                r#east: false,
            });
        }
        if state_id == 11604 {
            return Some(PurpleStainedGlassPane {
                r#south: true,
                r#north: false,
                r#west: true,
                r#east: false,
                r#waterlogged: false,
            });
        }
        if state_id == 11584 {
            return Some(PurpleStainedGlassPane {
                r#north: true,
                r#east: true,
                r#waterlogged: false,
                r#south: false,
                r#west: true,
            });
        }
        if state_id == 11592 {
            return Some(PurpleStainedGlassPane {
                r#south: false,
                r#west: true,
                r#east: true,
                r#north: false,
                r#waterlogged: false,
            });
        }
        if state_id == 11596 {
            return Some(PurpleStainedGlassPane {
                r#north: true,
                r#west: true,
                r#east: false,
                r#south: true,
                r#waterlogged: false,
            });
        }
        if state_id == 11608 {
            return Some(PurpleStainedGlassPane {
                r#waterlogged: false,
                r#south: false,
                r#north: false,
                r#west: true,
                r#east: false,
            });
        }
        if state_id == 11609 {
            return Some(PurpleStainedGlassPane {
                r#west: false,
                r#waterlogged: false,
                r#north: false,
                r#east: false,
                r#south: false,
            });
        }
        if state_id == 11593 {
            return Some(PurpleStainedGlassPane {
                r#east: true,
                r#north: false,
                r#south: false,
                r#west: false,
                r#waterlogged: false,
            });
        }
        if state_id == 11607 {
            return Some(PurpleStainedGlassPane {
                r#east: false,
                r#south: false,
                r#waterlogged: true,
                r#west: false,
                r#north: false,
            });
        }
        return None;
    }
}
