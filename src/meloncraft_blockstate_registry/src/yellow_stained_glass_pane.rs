use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct YellowStainedGlassPane {
    pub south: bool,
    pub north: bool,
    pub west: bool,
    pub east: bool,
    pub waterlogged: bool,
}

impl BlockState for YellowStainedGlassPane {
    fn to_id(&self) -> i32 {
        if self.r#west == false
            && self.r#east == true
            && self.r#south == false
            && self.r#north == false
            && self.r#waterlogged == true
        {
            return 11399;
        }
        if self.r#waterlogged == false
            && self.r#west == true
            && self.r#north == false
            && self.r#east == true
            && self.r#south == false
        {
            return 11400;
        }
        if self.r#waterlogged == true
            && self.r#north == false
            && self.r#south == false
            && self.r#east == false
            && self.r#west == true
        {
            return 11414;
        }
        if self.r#north == true
            && self.r#west == true
            && self.r#south == false
            && self.r#waterlogged == false
            && self.r#east == false
        {
            return 11408;
        }
        if self.r#south == false
            && self.r#north == false
            && self.r#waterlogged == false
            && self.r#east == false
            && self.r#west == false
        {
            return 11417;
        }
        if self.r#waterlogged == false
            && self.r#west == false
            && self.r#east == true
            && self.r#south == true
            && self.r#north == true
        {
            return 11389;
        }
        if self.r#east == true
            && self.r#north == false
            && self.r#west == false
            && self.r#waterlogged == false
            && self.r#south == true
        {
            return 11397;
        }
        if self.r#waterlogged == true
            && self.r#west == false
            && self.r#north == false
            && self.r#east == false
            && self.r#south == false
        {
            return 11415;
        }
        if self.r#south == false
            && self.r#waterlogged == false
            && self.r#east == true
            && self.r#west == false
            && self.r#north == true
        {
            return 11393;
        }
        if self.r#east == false
            && self.r#waterlogged == true
            && self.r#west == false
            && self.r#south == true
            && self.r#north == true
        {
            return 11403;
        }
        if self.r#south == false
            && self.r#west == true
            && self.r#north == true
            && self.r#waterlogged == true
            && self.r#east == true
        {
            return 11390;
        }
        if self.r#waterlogged == false
            && self.r#east == true
            && self.r#south == false
            && self.r#west == true
            && self.r#north == true
        {
            return 11392;
        }
        if self.r#west == false
            && self.r#east == true
            && self.r#north == true
            && self.r#waterlogged == true
            && self.r#south == true
        {
            return 11387;
        }
        if self.r#east == false
            && self.r#west == false
            && self.r#north == true
            && self.r#waterlogged == false
            && self.r#south == false
        {
            return 11409;
        }
        if self.r#south == true
            && self.r#west == true
            && self.r#waterlogged == false
            && self.r#north == false
            && self.r#east == false
        {
            return 11412;
        }
        if self.r#east == true
            && self.r#north == false
            && self.r#south == false
            && self.r#waterlogged == false
            && self.r#west == false
        {
            return 11401;
        }
        if self.r#east == true
            && self.r#waterlogged == true
            && self.r#west == true
            && self.r#north == false
            && self.r#south == true
        {
            return 11394;
        }
        if self.r#north == false
            && self.r#south == true
            && self.r#waterlogged == true
            && self.r#east == false
            && self.r#west == true
        {
            return 11410;
        }
        if self.r#east == false
            && self.r#waterlogged == false
            && self.r#north == false
            && self.r#south == true
            && self.r#west == false
        {
            return 11413;
        }
        if self.r#north == true
            && self.r#east == false
            && self.r#waterlogged == true
            && self.r#west == true
            && self.r#south == false
        {
            return 11406;
        }
        if self.r#south == true
            && self.r#waterlogged == false
            && self.r#east == false
            && self.r#west == false
            && self.r#north == true
        {
            return 11405;
        }
        if self.r#north == true
            && self.r#south == false
            && self.r#west == false
            && self.r#east == false
            && self.r#waterlogged == true
        {
            return 11407;
        }
        if self.r#north == true
            && self.r#south == false
            && self.r#west == false
            && self.r#waterlogged == true
            && self.r#east == true
        {
            return 11391;
        }
        if self.r#east == false
            && self.r#south == true
            && self.r#north == true
            && self.r#west == true
            && self.r#waterlogged == true
        {
            return 11402;
        }
        if self.r#east == true
            && self.r#north == true
            && self.r#south == true
            && self.r#waterlogged == false
            && self.r#west == true
        {
            return 11388;
        }
        if self.r#west == true
            && self.r#waterlogged == false
            && self.r#north == false
            && self.r#east == true
            && self.r#south == true
        {
            return 11396;
        }
        if self.r#west == true
            && self.r#waterlogged == true
            && self.r#north == true
            && self.r#east == true
            && self.r#south == true
        {
            return 11386;
        }
        if self.r#north == false
            && self.r#south == true
            && self.r#east == true
            && self.r#waterlogged == true
            && self.r#west == false
        {
            return 11395;
        }
        if self.r#south == false
            && self.r#west == true
            && self.r#north == false
            && self.r#waterlogged == true
            && self.r#east == true
        {
            return 11398;
        }
        if self.r#south == false
            && self.r#east == false
            && self.r#waterlogged == false
            && self.r#north == false
            && self.r#west == true
        {
            return 11416;
        }
        if self.r#south == true
            && self.r#waterlogged == false
            && self.r#west == true
            && self.r#east == false
            && self.r#north == true
        {
            return 11404;
        }
        if self.r#west == false
            && self.r#north == false
            && self.r#south == true
            && self.r#east == false
            && self.r#waterlogged == true
        {
            return 11411;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 11399 {
            return Some(YellowStainedGlassPane {
                r#west: false,
                r#east: true,
                r#south: false,
                r#north: false,
                r#waterlogged: true,
            });
        }
        if state_id == 11400 {
            return Some(YellowStainedGlassPane {
                r#waterlogged: false,
                r#west: true,
                r#north: false,
                r#east: true,
                r#south: false,
            });
        }
        if state_id == 11414 {
            return Some(YellowStainedGlassPane {
                r#waterlogged: true,
                r#north: false,
                r#south: false,
                r#east: false,
                r#west: true,
            });
        }
        if state_id == 11408 {
            return Some(YellowStainedGlassPane {
                r#north: true,
                r#west: true,
                r#south: false,
                r#waterlogged: false,
                r#east: false,
            });
        }
        if state_id == 11417 {
            return Some(YellowStainedGlassPane {
                r#south: false,
                r#north: false,
                r#waterlogged: false,
                r#east: false,
                r#west: false,
            });
        }
        if state_id == 11389 {
            return Some(YellowStainedGlassPane {
                r#waterlogged: false,
                r#west: false,
                r#east: true,
                r#south: true,
                r#north: true,
            });
        }
        if state_id == 11397 {
            return Some(YellowStainedGlassPane {
                r#east: true,
                r#north: false,
                r#west: false,
                r#waterlogged: false,
                r#south: true,
            });
        }
        if state_id == 11415 {
            return Some(YellowStainedGlassPane {
                r#waterlogged: true,
                r#west: false,
                r#north: false,
                r#east: false,
                r#south: false,
            });
        }
        if state_id == 11393 {
            return Some(YellowStainedGlassPane {
                r#south: false,
                r#waterlogged: false,
                r#east: true,
                r#west: false,
                r#north: true,
            });
        }
        if state_id == 11403 {
            return Some(YellowStainedGlassPane {
                r#east: false,
                r#waterlogged: true,
                r#west: false,
                r#south: true,
                r#north: true,
            });
        }
        if state_id == 11390 {
            return Some(YellowStainedGlassPane {
                r#south: false,
                r#west: true,
                r#north: true,
                r#waterlogged: true,
                r#east: true,
            });
        }
        if state_id == 11392 {
            return Some(YellowStainedGlassPane {
                r#waterlogged: false,
                r#east: true,
                r#south: false,
                r#west: true,
                r#north: true,
            });
        }
        if state_id == 11387 {
            return Some(YellowStainedGlassPane {
                r#west: false,
                r#east: true,
                r#north: true,
                r#waterlogged: true,
                r#south: true,
            });
        }
        if state_id == 11409 {
            return Some(YellowStainedGlassPane {
                r#east: false,
                r#west: false,
                r#north: true,
                r#waterlogged: false,
                r#south: false,
            });
        }
        if state_id == 11412 {
            return Some(YellowStainedGlassPane {
                r#south: true,
                r#west: true,
                r#waterlogged: false,
                r#north: false,
                r#east: false,
            });
        }
        if state_id == 11401 {
            return Some(YellowStainedGlassPane {
                r#east: true,
                r#north: false,
                r#south: false,
                r#waterlogged: false,
                r#west: false,
            });
        }
        if state_id == 11394 {
            return Some(YellowStainedGlassPane {
                r#east: true,
                r#waterlogged: true,
                r#west: true,
                r#north: false,
                r#south: true,
            });
        }
        if state_id == 11410 {
            return Some(YellowStainedGlassPane {
                r#north: false,
                r#south: true,
                r#waterlogged: true,
                r#east: false,
                r#west: true,
            });
        }
        if state_id == 11413 {
            return Some(YellowStainedGlassPane {
                r#east: false,
                r#waterlogged: false,
                r#north: false,
                r#south: true,
                r#west: false,
            });
        }
        if state_id == 11406 {
            return Some(YellowStainedGlassPane {
                r#north: true,
                r#east: false,
                r#waterlogged: true,
                r#west: true,
                r#south: false,
            });
        }
        if state_id == 11405 {
            return Some(YellowStainedGlassPane {
                r#south: true,
                r#waterlogged: false,
                r#east: false,
                r#west: false,
                r#north: true,
            });
        }
        if state_id == 11407 {
            return Some(YellowStainedGlassPane {
                r#north: true,
                r#south: false,
                r#west: false,
                r#east: false,
                r#waterlogged: true,
            });
        }
        if state_id == 11391 {
            return Some(YellowStainedGlassPane {
                r#north: true,
                r#south: false,
                r#west: false,
                r#waterlogged: true,
                r#east: true,
            });
        }
        if state_id == 11402 {
            return Some(YellowStainedGlassPane {
                r#east: false,
                r#south: true,
                r#north: true,
                r#west: true,
                r#waterlogged: true,
            });
        }
        if state_id == 11388 {
            return Some(YellowStainedGlassPane {
                r#east: true,
                r#north: true,
                r#south: true,
                r#waterlogged: false,
                r#west: true,
            });
        }
        if state_id == 11396 {
            return Some(YellowStainedGlassPane {
                r#west: true,
                r#waterlogged: false,
                r#north: false,
                r#east: true,
                r#south: true,
            });
        }
        if state_id == 11386 {
            return Some(YellowStainedGlassPane {
                r#west: true,
                r#waterlogged: true,
                r#north: true,
                r#east: true,
                r#south: true,
            });
        }
        if state_id == 11395 {
            return Some(YellowStainedGlassPane {
                r#north: false,
                r#south: true,
                r#east: true,
                r#waterlogged: true,
                r#west: false,
            });
        }
        if state_id == 11398 {
            return Some(YellowStainedGlassPane {
                r#south: false,
                r#west: true,
                r#north: false,
                r#waterlogged: true,
                r#east: true,
            });
        }
        if state_id == 11416 {
            return Some(YellowStainedGlassPane {
                r#south: false,
                r#east: false,
                r#waterlogged: false,
                r#north: false,
                r#west: true,
            });
        }
        if state_id == 11404 {
            return Some(YellowStainedGlassPane {
                r#south: true,
                r#waterlogged: false,
                r#west: true,
                r#east: false,
                r#north: true,
            });
        }
        if state_id == 11411 {
            return Some(YellowStainedGlassPane {
                r#west: false,
                r#north: false,
                r#south: true,
                r#east: false,
                r#waterlogged: true,
            });
        }
        return None;
    }
}
