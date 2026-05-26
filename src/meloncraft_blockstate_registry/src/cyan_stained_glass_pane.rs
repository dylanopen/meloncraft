use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CyanStainedGlassPane {
    pub west: bool,
    pub waterlogged: bool,
    pub north: bool,
    pub east: bool,
    pub south: bool,
}

impl BlockState for CyanStainedGlassPane {
    fn to_id(&self) -> i32 {
        if self.r#north == true
            && self.r#waterlogged == true
            && self.r#west == true
            && self.r#east == false
            && self.r#south == true
        {
            return 11562;
        }
        if self.r#west == false
            && self.r#south == false
            && self.r#north == false
            && self.r#east == false
            && self.r#waterlogged == true
        {
            return 11575;
        }
        if self.r#east == true
            && self.r#north == false
            && self.r#south == false
            && self.r#waterlogged == true
            && self.r#west == false
        {
            return 11559;
        }
        if self.r#north == true
            && self.r#south == false
            && self.r#waterlogged == false
            && self.r#west == false
            && self.r#east == false
        {
            return 11569;
        }
        if self.r#south == true
            && self.r#north == false
            && self.r#east == false
            && self.r#waterlogged == false
            && self.r#west == true
        {
            return 11572;
        }
        if self.r#north == true
            && self.r#waterlogged == false
            && self.r#east == false
            && self.r#south == false
            && self.r#west == true
        {
            return 11568;
        }
        if self.r#east == true
            && self.r#waterlogged == true
            && self.r#west == false
            && self.r#south == true
            && self.r#north == false
        {
            return 11555;
        }
        if self.r#east == true
            && self.r#south == true
            && self.r#west == false
            && self.r#north == false
            && self.r#waterlogged == false
        {
            return 11557;
        }
        if self.r#east == false
            && self.r#south == false
            && self.r#west == true
            && self.r#north == false
            && self.r#waterlogged == false
        {
            return 11576;
        }
        if self.r#west == true
            && self.r#north == false
            && self.r#east == true
            && self.r#south == false
            && self.r#waterlogged == true
        {
            return 11558;
        }
        if self.r#south == false
            && self.r#waterlogged == true
            && self.r#east == false
            && self.r#north == false
            && self.r#west == true
        {
            return 11574;
        }
        if self.r#waterlogged == true
            && self.r#east == false
            && self.r#south == false
            && self.r#west == true
            && self.r#north == true
        {
            return 11566;
        }
        if self.r#west == false
            && self.r#north == false
            && self.r#waterlogged == false
            && self.r#east == false
            && self.r#south == false
        {
            return 11577;
        }
        if self.r#south == true
            && self.r#east == false
            && self.r#waterlogged == true
            && self.r#west == true
            && self.r#north == false
        {
            return 11570;
        }
        if self.r#north == true
            && self.r#waterlogged == false
            && self.r#south == true
            && self.r#east == true
            && self.r#west == true
        {
            return 11548;
        }
        if self.r#north == true
            && self.r#south == true
            && self.r#east == false
            && self.r#waterlogged == false
            && self.r#west == false
        {
            return 11565;
        }
        if self.r#east == true
            && self.r#north == true
            && self.r#waterlogged == true
            && self.r#west == true
            && self.r#south == false
        {
            return 11550;
        }
        if self.r#east == true
            && self.r#north == true
            && self.r#waterlogged == false
            && self.r#west == false
            && self.r#south == true
        {
            return 11549;
        }
        if self.r#east == false
            && self.r#south == true
            && self.r#waterlogged == false
            && self.r#north == false
            && self.r#west == false
        {
            return 11573;
        }
        if self.r#west == true
            && self.r#north == false
            && self.r#east == true
            && self.r#south == true
            && self.r#waterlogged == true
        {
            return 11554;
        }
        if self.r#west == false
            && self.r#south == false
            && self.r#north == true
            && self.r#waterlogged == true
            && self.r#east == true
        {
            return 11551;
        }
        if self.r#waterlogged == true
            && self.r#south == true
            && self.r#north == true
            && self.r#west == true
            && self.r#east == true
        {
            return 11546;
        }
        if self.r#south == true
            && self.r#waterlogged == true
            && self.r#east == true
            && self.r#west == false
            && self.r#north == true
        {
            return 11547;
        }
        if self.r#waterlogged == false
            && self.r#west == false
            && self.r#south == false
            && self.r#east == true
            && self.r#north == true
        {
            return 11553;
        }
        if self.r#north == true
            && self.r#south == true
            && self.r#east == false
            && self.r#west == false
            && self.r#waterlogged == true
        {
            return 11563;
        }
        if self.r#north == true
            && self.r#south == false
            && self.r#east == true
            && self.r#waterlogged == false
            && self.r#west == true
        {
            return 11552;
        }
        if self.r#waterlogged == false
            && self.r#west == true
            && self.r#east == true
            && self.r#south == false
            && self.r#north == false
        {
            return 11560;
        }
        if self.r#south == true
            && self.r#north == false
            && self.r#waterlogged == false
            && self.r#west == true
            && self.r#east == true
        {
            return 11556;
        }
        if self.r#west == false
            && self.r#north == false
            && self.r#east == true
            && self.r#south == false
            && self.r#waterlogged == false
        {
            return 11561;
        }
        if self.r#east == false
            && self.r#south == true
            && self.r#waterlogged == false
            && self.r#west == true
            && self.r#north == true
        {
            return 11564;
        }
        if self.r#west == false
            && self.r#north == false
            && self.r#south == true
            && self.r#east == false
            && self.r#waterlogged == true
        {
            return 11571;
        }
        if self.r#waterlogged == true
            && self.r#north == true
            && self.r#south == false
            && self.r#west == false
            && self.r#east == false
        {
            return 11567;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 11562 {
            return Some(CyanStainedGlassPane {
                r#north: true,
                r#waterlogged: true,
                r#west: true,
                r#east: false,
                r#south: true,
            });
        }
        if state_id == 11575 {
            return Some(CyanStainedGlassPane {
                r#west: false,
                r#south: false,
                r#north: false,
                r#east: false,
                r#waterlogged: true,
            });
        }
        if state_id == 11559 {
            return Some(CyanStainedGlassPane {
                r#east: true,
                r#north: false,
                r#south: false,
                r#waterlogged: true,
                r#west: false,
            });
        }
        if state_id == 11569 {
            return Some(CyanStainedGlassPane {
                r#north: true,
                r#south: false,
                r#waterlogged: false,
                r#west: false,
                r#east: false,
            });
        }
        if state_id == 11572 {
            return Some(CyanStainedGlassPane {
                r#south: true,
                r#north: false,
                r#east: false,
                r#waterlogged: false,
                r#west: true,
            });
        }
        if state_id == 11568 {
            return Some(CyanStainedGlassPane {
                r#north: true,
                r#waterlogged: false,
                r#east: false,
                r#south: false,
                r#west: true,
            });
        }
        if state_id == 11555 {
            return Some(CyanStainedGlassPane {
                r#east: true,
                r#waterlogged: true,
                r#west: false,
                r#south: true,
                r#north: false,
            });
        }
        if state_id == 11557 {
            return Some(CyanStainedGlassPane {
                r#east: true,
                r#south: true,
                r#west: false,
                r#north: false,
                r#waterlogged: false,
            });
        }
        if state_id == 11576 {
            return Some(CyanStainedGlassPane {
                r#east: false,
                r#south: false,
                r#west: true,
                r#north: false,
                r#waterlogged: false,
            });
        }
        if state_id == 11558 {
            return Some(CyanStainedGlassPane {
                r#west: true,
                r#north: false,
                r#east: true,
                r#south: false,
                r#waterlogged: true,
            });
        }
        if state_id == 11574 {
            return Some(CyanStainedGlassPane {
                r#south: false,
                r#waterlogged: true,
                r#east: false,
                r#north: false,
                r#west: true,
            });
        }
        if state_id == 11566 {
            return Some(CyanStainedGlassPane {
                r#waterlogged: true,
                r#east: false,
                r#south: false,
                r#west: true,
                r#north: true,
            });
        }
        if state_id == 11577 {
            return Some(CyanStainedGlassPane {
                r#west: false,
                r#north: false,
                r#waterlogged: false,
                r#east: false,
                r#south: false,
            });
        }
        if state_id == 11570 {
            return Some(CyanStainedGlassPane {
                r#south: true,
                r#east: false,
                r#waterlogged: true,
                r#west: true,
                r#north: false,
            });
        }
        if state_id == 11548 {
            return Some(CyanStainedGlassPane {
                r#north: true,
                r#waterlogged: false,
                r#south: true,
                r#east: true,
                r#west: true,
            });
        }
        if state_id == 11565 {
            return Some(CyanStainedGlassPane {
                r#north: true,
                r#south: true,
                r#east: false,
                r#waterlogged: false,
                r#west: false,
            });
        }
        if state_id == 11550 {
            return Some(CyanStainedGlassPane {
                r#east: true,
                r#north: true,
                r#waterlogged: true,
                r#west: true,
                r#south: false,
            });
        }
        if state_id == 11549 {
            return Some(CyanStainedGlassPane {
                r#east: true,
                r#north: true,
                r#waterlogged: false,
                r#west: false,
                r#south: true,
            });
        }
        if state_id == 11573 {
            return Some(CyanStainedGlassPane {
                r#east: false,
                r#south: true,
                r#waterlogged: false,
                r#north: false,
                r#west: false,
            });
        }
        if state_id == 11554 {
            return Some(CyanStainedGlassPane {
                r#west: true,
                r#north: false,
                r#east: true,
                r#south: true,
                r#waterlogged: true,
            });
        }
        if state_id == 11551 {
            return Some(CyanStainedGlassPane {
                r#west: false,
                r#south: false,
                r#north: true,
                r#waterlogged: true,
                r#east: true,
            });
        }
        if state_id == 11546 {
            return Some(CyanStainedGlassPane {
                r#waterlogged: true,
                r#south: true,
                r#north: true,
                r#west: true,
                r#east: true,
            });
        }
        if state_id == 11547 {
            return Some(CyanStainedGlassPane {
                r#south: true,
                r#waterlogged: true,
                r#east: true,
                r#west: false,
                r#north: true,
            });
        }
        if state_id == 11553 {
            return Some(CyanStainedGlassPane {
                r#waterlogged: false,
                r#west: false,
                r#south: false,
                r#east: true,
                r#north: true,
            });
        }
        if state_id == 11563 {
            return Some(CyanStainedGlassPane {
                r#north: true,
                r#south: true,
                r#east: false,
                r#west: false,
                r#waterlogged: true,
            });
        }
        if state_id == 11552 {
            return Some(CyanStainedGlassPane {
                r#north: true,
                r#south: false,
                r#east: true,
                r#waterlogged: false,
                r#west: true,
            });
        }
        if state_id == 11560 {
            return Some(CyanStainedGlassPane {
                r#waterlogged: false,
                r#west: true,
                r#east: true,
                r#south: false,
                r#north: false,
            });
        }
        if state_id == 11556 {
            return Some(CyanStainedGlassPane {
                r#south: true,
                r#north: false,
                r#waterlogged: false,
                r#west: true,
                r#east: true,
            });
        }
        if state_id == 11561 {
            return Some(CyanStainedGlassPane {
                r#west: false,
                r#north: false,
                r#east: true,
                r#south: false,
                r#waterlogged: false,
            });
        }
        if state_id == 11564 {
            return Some(CyanStainedGlassPane {
                r#east: false,
                r#south: true,
                r#waterlogged: false,
                r#west: true,
                r#north: true,
            });
        }
        if state_id == 11571 {
            return Some(CyanStainedGlassPane {
                r#west: false,
                r#north: false,
                r#south: true,
                r#east: false,
                r#waterlogged: true,
            });
        }
        if state_id == 11567 {
            return Some(CyanStainedGlassPane {
                r#waterlogged: true,
                r#north: true,
                r#south: false,
                r#west: false,
                r#east: false,
            });
        }
        return None;
    }
}
