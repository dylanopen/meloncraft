use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedStainedGlassPane {
    pub east: bool,
    pub waterlogged: bool,
    pub west: bool,
    pub south: bool,
    pub north: bool,
}

impl BlockState for RedStainedGlassPane {
    fn to_id(&self) -> i32 {
        if self.r#west == true
            && self.r#north == false
            && self.r#east == true
            && self.r#waterlogged == false
            && self.r#south == false
        {
            return 11720;
        }
        if self.r#waterlogged == true
            && self.r#west == true
            && self.r#south == false
            && self.r#east == false
            && self.r#north == false
        {
            return 11734;
        }
        if self.r#south == false
            && self.r#west == false
            && self.r#east == false
            && self.r#waterlogged == true
            && self.r#north == false
        {
            return 11735;
        }
        if self.r#south == true
            && self.r#east == true
            && self.r#north == false
            && self.r#waterlogged == true
            && self.r#west == false
        {
            return 11715;
        }
        if self.r#north == true
            && self.r#waterlogged == false
            && self.r#west == true
            && self.r#south == true
            && self.r#east == true
        {
            return 11708;
        }
        if self.r#north == true
            && self.r#waterlogged == true
            && self.r#west == true
            && self.r#south == false
            && self.r#east == false
        {
            return 11726;
        }
        if self.r#north == false
            && self.r#south == false
            && self.r#east == false
            && self.r#waterlogged == false
            && self.r#west == false
        {
            return 11737;
        }
        if self.r#waterlogged == false
            && self.r#south == false
            && self.r#west == true
            && self.r#east == false
            && self.r#north == true
        {
            return 11728;
        }
        if self.r#east == true
            && self.r#waterlogged == true
            && self.r#west == true
            && self.r#north == true
            && self.r#south == false
        {
            return 11710;
        }
        if self.r#waterlogged == true
            && self.r#south == true
            && self.r#east == true
            && self.r#north == true
            && self.r#west == false
        {
            return 11707;
        }
        if self.r#west == true
            && self.r#waterlogged == false
            && self.r#north == true
            && self.r#east == true
            && self.r#south == false
        {
            return 11712;
        }
        if self.r#north == false
            && self.r#south == true
            && self.r#waterlogged == true
            && self.r#east == false
            && self.r#west == true
        {
            return 11730;
        }
        if self.r#north == false
            && self.r#south == true
            && self.r#west == true
            && self.r#east == true
            && self.r#waterlogged == true
        {
            return 11714;
        }
        if self.r#north == true
            && self.r#south == false
            && self.r#west == false
            && self.r#east == false
            && self.r#waterlogged == true
        {
            return 11727;
        }
        if self.r#east == false
            && self.r#north == true
            && self.r#south == true
            && self.r#waterlogged == true
            && self.r#west == true
        {
            return 11722;
        }
        if self.r#east == true
            && self.r#north == false
            && self.r#waterlogged == false
            && self.r#west == false
            && self.r#south == false
        {
            return 11721;
        }
        if self.r#west == true
            && self.r#south == false
            && self.r#east == false
            && self.r#waterlogged == false
            && self.r#north == false
        {
            return 11736;
        }
        if self.r#north == false
            && self.r#east == true
            && self.r#south == false
            && self.r#waterlogged == true
            && self.r#west == true
        {
            return 11718;
        }
        if self.r#waterlogged == true
            && self.r#south == false
            && self.r#west == false
            && self.r#east == true
            && self.r#north == true
        {
            return 11711;
        }
        if self.r#waterlogged == false
            && self.r#east == true
            && self.r#north == false
            && self.r#south == true
            && self.r#west == true
        {
            return 11716;
        }
        if self.r#east == false
            && self.r#waterlogged == true
            && self.r#west == false
            && self.r#south == true
            && self.r#north == false
        {
            return 11731;
        }
        if self.r#east == false
            && self.r#waterlogged == false
            && self.r#north == false
            && self.r#south == true
            && self.r#west == true
        {
            return 11732;
        }
        if self.r#waterlogged == false
            && self.r#north == false
            && self.r#south == true
            && self.r#east == false
            && self.r#west == false
        {
            return 11733;
        }
        if self.r#south == true
            && self.r#north == true
            && self.r#waterlogged == false
            && self.r#east == true
            && self.r#west == false
        {
            return 11709;
        }
        if self.r#south == false
            && self.r#north == true
            && self.r#west == false
            && self.r#waterlogged == false
            && self.r#east == false
        {
            return 11729;
        }
        if self.r#west == true
            && self.r#north == true
            && self.r#east == true
            && self.r#south == true
            && self.r#waterlogged == true
        {
            return 11706;
        }
        if self.r#north == false
            && self.r#east == true
            && self.r#south == true
            && self.r#waterlogged == false
            && self.r#west == false
        {
            return 11717;
        }
        if self.r#east == false
            && self.r#west == false
            && self.r#south == true
            && self.r#north == true
            && self.r#waterlogged == false
        {
            return 11725;
        }
        if self.r#south == false
            && self.r#waterlogged == false
            && self.r#north == true
            && self.r#west == false
            && self.r#east == true
        {
            return 11713;
        }
        if self.r#east == true
            && self.r#west == false
            && self.r#south == false
            && self.r#north == false
            && self.r#waterlogged == true
        {
            return 11719;
        }
        if self.r#west == true
            && self.r#waterlogged == false
            && self.r#north == true
            && self.r#south == true
            && self.r#east == false
        {
            return 11724;
        }
        if self.r#north == true
            && self.r#waterlogged == true
            && self.r#east == false
            && self.r#south == true
            && self.r#west == false
        {
            return 11723;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 11720 {
            return Some(RedStainedGlassPane {
                r#west: true,
                r#north: false,
                r#east: true,
                r#waterlogged: false,
                r#south: false,
            });
        }
        if state_id == 11734 {
            return Some(RedStainedGlassPane {
                r#waterlogged: true,
                r#west: true,
                r#south: false,
                r#east: false,
                r#north: false,
            });
        }
        if state_id == 11735 {
            return Some(RedStainedGlassPane {
                r#south: false,
                r#west: false,
                r#east: false,
                r#waterlogged: true,
                r#north: false,
            });
        }
        if state_id == 11715 {
            return Some(RedStainedGlassPane {
                r#south: true,
                r#east: true,
                r#north: false,
                r#waterlogged: true,
                r#west: false,
            });
        }
        if state_id == 11708 {
            return Some(RedStainedGlassPane {
                r#north: true,
                r#waterlogged: false,
                r#west: true,
                r#south: true,
                r#east: true,
            });
        }
        if state_id == 11726 {
            return Some(RedStainedGlassPane {
                r#north: true,
                r#waterlogged: true,
                r#west: true,
                r#south: false,
                r#east: false,
            });
        }
        if state_id == 11737 {
            return Some(RedStainedGlassPane {
                r#north: false,
                r#south: false,
                r#east: false,
                r#waterlogged: false,
                r#west: false,
            });
        }
        if state_id == 11728 {
            return Some(RedStainedGlassPane {
                r#waterlogged: false,
                r#south: false,
                r#west: true,
                r#east: false,
                r#north: true,
            });
        }
        if state_id == 11710 {
            return Some(RedStainedGlassPane {
                r#east: true,
                r#waterlogged: true,
                r#west: true,
                r#north: true,
                r#south: false,
            });
        }
        if state_id == 11707 {
            return Some(RedStainedGlassPane {
                r#waterlogged: true,
                r#south: true,
                r#east: true,
                r#north: true,
                r#west: false,
            });
        }
        if state_id == 11712 {
            return Some(RedStainedGlassPane {
                r#west: true,
                r#waterlogged: false,
                r#north: true,
                r#east: true,
                r#south: false,
            });
        }
        if state_id == 11730 {
            return Some(RedStainedGlassPane {
                r#north: false,
                r#south: true,
                r#waterlogged: true,
                r#east: false,
                r#west: true,
            });
        }
        if state_id == 11714 {
            return Some(RedStainedGlassPane {
                r#north: false,
                r#south: true,
                r#west: true,
                r#east: true,
                r#waterlogged: true,
            });
        }
        if state_id == 11727 {
            return Some(RedStainedGlassPane {
                r#north: true,
                r#south: false,
                r#west: false,
                r#east: false,
                r#waterlogged: true,
            });
        }
        if state_id == 11722 {
            return Some(RedStainedGlassPane {
                r#east: false,
                r#north: true,
                r#south: true,
                r#waterlogged: true,
                r#west: true,
            });
        }
        if state_id == 11721 {
            return Some(RedStainedGlassPane {
                r#east: true,
                r#north: false,
                r#waterlogged: false,
                r#west: false,
                r#south: false,
            });
        }
        if state_id == 11736 {
            return Some(RedStainedGlassPane {
                r#west: true,
                r#south: false,
                r#east: false,
                r#waterlogged: false,
                r#north: false,
            });
        }
        if state_id == 11718 {
            return Some(RedStainedGlassPane {
                r#north: false,
                r#east: true,
                r#south: false,
                r#waterlogged: true,
                r#west: true,
            });
        }
        if state_id == 11711 {
            return Some(RedStainedGlassPane {
                r#waterlogged: true,
                r#south: false,
                r#west: false,
                r#east: true,
                r#north: true,
            });
        }
        if state_id == 11716 {
            return Some(RedStainedGlassPane {
                r#waterlogged: false,
                r#east: true,
                r#north: false,
                r#south: true,
                r#west: true,
            });
        }
        if state_id == 11731 {
            return Some(RedStainedGlassPane {
                r#east: false,
                r#waterlogged: true,
                r#west: false,
                r#south: true,
                r#north: false,
            });
        }
        if state_id == 11732 {
            return Some(RedStainedGlassPane {
                r#east: false,
                r#waterlogged: false,
                r#north: false,
                r#south: true,
                r#west: true,
            });
        }
        if state_id == 11733 {
            return Some(RedStainedGlassPane {
                r#waterlogged: false,
                r#north: false,
                r#south: true,
                r#east: false,
                r#west: false,
            });
        }
        if state_id == 11709 {
            return Some(RedStainedGlassPane {
                r#south: true,
                r#north: true,
                r#waterlogged: false,
                r#east: true,
                r#west: false,
            });
        }
        if state_id == 11729 {
            return Some(RedStainedGlassPane {
                r#south: false,
                r#north: true,
                r#west: false,
                r#waterlogged: false,
                r#east: false,
            });
        }
        if state_id == 11706 {
            return Some(RedStainedGlassPane {
                r#west: true,
                r#north: true,
                r#east: true,
                r#south: true,
                r#waterlogged: true,
            });
        }
        if state_id == 11717 {
            return Some(RedStainedGlassPane {
                r#north: false,
                r#east: true,
                r#south: true,
                r#waterlogged: false,
                r#west: false,
            });
        }
        if state_id == 11725 {
            return Some(RedStainedGlassPane {
                r#east: false,
                r#west: false,
                r#south: true,
                r#north: true,
                r#waterlogged: false,
            });
        }
        if state_id == 11713 {
            return Some(RedStainedGlassPane {
                r#south: false,
                r#waterlogged: false,
                r#north: true,
                r#west: false,
                r#east: true,
            });
        }
        if state_id == 11719 {
            return Some(RedStainedGlassPane {
                r#east: true,
                r#west: false,
                r#south: false,
                r#north: false,
                r#waterlogged: true,
            });
        }
        if state_id == 11724 {
            return Some(RedStainedGlassPane {
                r#west: true,
                r#waterlogged: false,
                r#north: true,
                r#south: true,
                r#east: false,
            });
        }
        if state_id == 11723 {
            return Some(RedStainedGlassPane {
                r#north: true,
                r#waterlogged: true,
                r#east: false,
                r#south: true,
                r#west: false,
            });
        }
        return None;
    }
}
