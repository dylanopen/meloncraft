use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MagentaStainedGlassPane {
    pub waterlogged: bool,
    pub south: bool,
    pub east: bool,
    pub west: bool,
    pub north: bool,
}

impl BlockState for MagentaStainedGlassPane {
    fn to_id(&self) -> i32 {
        if self.r#west == true
            && self.r#east == true
            && self.r#waterlogged == true
            && self.r#north == true
            && self.r#south == true
        {
            return 11322;
        }
        if self.r#south == true
            && self.r#east == true
            && self.r#west == false
            && self.r#waterlogged == true
            && self.r#north == true
        {
            return 11323;
        }
        if self.r#north == true
            && self.r#east == false
            && self.r#south == true
            && self.r#west == false
            && self.r#waterlogged == false
        {
            return 11341;
        }
        if self.r#north == true
            && self.r#waterlogged == true
            && self.r#east == true
            && self.r#south == false
            && self.r#west == true
        {
            return 11326;
        }
        if self.r#west == true
            && self.r#south == true
            && self.r#east == false
            && self.r#north == true
            && self.r#waterlogged == false
        {
            return 11340;
        }
        if self.r#waterlogged == true
            && self.r#east == false
            && self.r#north == true
            && self.r#south == false
            && self.r#west == true
        {
            return 11342;
        }
        if self.r#south == false
            && self.r#west == true
            && self.r#north == true
            && self.r#waterlogged == false
            && self.r#east == true
        {
            return 11328;
        }
        if self.r#east == true
            && self.r#north == false
            && self.r#south == false
            && self.r#west == false
            && self.r#waterlogged == false
        {
            return 11337;
        }
        if self.r#east == false
            && self.r#west == true
            && self.r#waterlogged == true
            && self.r#south == false
            && self.r#north == false
        {
            return 11350;
        }
        if self.r#east == false
            && self.r#north == true
            && self.r#waterlogged == true
            && self.r#west == true
            && self.r#south == true
        {
            return 11338;
        }
        if self.r#waterlogged == false
            && self.r#north == true
            && self.r#south == false
            && self.r#west == false
            && self.r#east == false
        {
            return 11345;
        }
        if self.r#waterlogged == false
            && self.r#west == false
            && self.r#south == true
            && self.r#east == false
            && self.r#north == false
        {
            return 11349;
        }
        if self.r#west == false
            && self.r#south == false
            && self.r#waterlogged == true
            && self.r#north == true
            && self.r#east == false
        {
            return 11343;
        }
        if self.r#east == false
            && self.r#south == true
            && self.r#west == false
            && self.r#waterlogged == true
            && self.r#north == false
        {
            return 11347;
        }
        if self.r#east == true
            && self.r#north == true
            && self.r#waterlogged == false
            && self.r#south == false
            && self.r#west == false
        {
            return 11329;
        }
        if self.r#east == true
            && self.r#waterlogged == false
            && self.r#south == true
            && self.r#west == false
            && self.r#north == true
        {
            return 11325;
        }
        if self.r#south == false
            && self.r#north == false
            && self.r#waterlogged == true
            && self.r#east == true
            && self.r#west == false
        {
            return 11335;
        }
        if self.r#east == true
            && self.r#south == false
            && self.r#north == true
            && self.r#waterlogged == true
            && self.r#west == false
        {
            return 11327;
        }
        if self.r#north == false
            && self.r#east == false
            && self.r#south == false
            && self.r#west == true
            && self.r#waterlogged == false
        {
            return 11352;
        }
        if self.r#waterlogged == false
            && self.r#west == false
            && self.r#east == true
            && self.r#south == true
            && self.r#north == false
        {
            return 11333;
        }
        if self.r#east == false
            && self.r#north == false
            && self.r#south == true
            && self.r#waterlogged == true
            && self.r#west == true
        {
            return 11346;
        }
        if self.r#south == true
            && self.r#north == false
            && self.r#east == true
            && self.r#west == false
            && self.r#waterlogged == true
        {
            return 11331;
        }
        if self.r#east == true
            && self.r#south == false
            && self.r#waterlogged == true
            && self.r#north == false
            && self.r#west == true
        {
            return 11334;
        }
        if self.r#north == false
            && self.r#waterlogged == false
            && self.r#east == true
            && self.r#south == true
            && self.r#west == true
        {
            return 11332;
        }
        if self.r#east == false
            && self.r#north == true
            && self.r#waterlogged == true
            && self.r#west == false
            && self.r#south == true
        {
            return 11339;
        }
        if self.r#east == false
            && self.r#north == true
            && self.r#west == true
            && self.r#south == false
            && self.r#waterlogged == false
        {
            return 11344;
        }
        if self.r#west == true
            && self.r#south == true
            && self.r#waterlogged == false
            && self.r#east == false
            && self.r#north == false
        {
            return 11348;
        }
        if self.r#east == true
            && self.r#waterlogged == false
            && self.r#south == true
            && self.r#west == true
            && self.r#north == true
        {
            return 11324;
        }
        if self.r#waterlogged == true
            && self.r#north == false
            && self.r#south == false
            && self.r#east == false
            && self.r#west == false
        {
            return 11351;
        }
        if self.r#east == false
            && self.r#north == false
            && self.r#waterlogged == false
            && self.r#west == false
            && self.r#south == false
        {
            return 11353;
        }
        if self.r#west == true
            && self.r#east == true
            && self.r#south == false
            && self.r#north == false
            && self.r#waterlogged == false
        {
            return 11336;
        }
        if self.r#east == true
            && self.r#north == false
            && self.r#waterlogged == true
            && self.r#south == true
            && self.r#west == true
        {
            return 11330;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 11322 {
            return Some(MagentaStainedGlassPane {
                r#west: true,
                r#east: true,
                r#waterlogged: true,
                r#north: true,
                r#south: true,
            });
        }
        if state_id == 11323 {
            return Some(MagentaStainedGlassPane {
                r#south: true,
                r#east: true,
                r#west: false,
                r#waterlogged: true,
                r#north: true,
            });
        }
        if state_id == 11341 {
            return Some(MagentaStainedGlassPane {
                r#north: true,
                r#east: false,
                r#south: true,
                r#west: false,
                r#waterlogged: false,
            });
        }
        if state_id == 11326 {
            return Some(MagentaStainedGlassPane {
                r#north: true,
                r#waterlogged: true,
                r#east: true,
                r#south: false,
                r#west: true,
            });
        }
        if state_id == 11340 {
            return Some(MagentaStainedGlassPane {
                r#west: true,
                r#south: true,
                r#east: false,
                r#north: true,
                r#waterlogged: false,
            });
        }
        if state_id == 11342 {
            return Some(MagentaStainedGlassPane {
                r#waterlogged: true,
                r#east: false,
                r#north: true,
                r#south: false,
                r#west: true,
            });
        }
        if state_id == 11328 {
            return Some(MagentaStainedGlassPane {
                r#south: false,
                r#west: true,
                r#north: true,
                r#waterlogged: false,
                r#east: true,
            });
        }
        if state_id == 11337 {
            return Some(MagentaStainedGlassPane {
                r#east: true,
                r#north: false,
                r#south: false,
                r#west: false,
                r#waterlogged: false,
            });
        }
        if state_id == 11350 {
            return Some(MagentaStainedGlassPane {
                r#east: false,
                r#west: true,
                r#waterlogged: true,
                r#south: false,
                r#north: false,
            });
        }
        if state_id == 11338 {
            return Some(MagentaStainedGlassPane {
                r#east: false,
                r#north: true,
                r#waterlogged: true,
                r#west: true,
                r#south: true,
            });
        }
        if state_id == 11345 {
            return Some(MagentaStainedGlassPane {
                r#waterlogged: false,
                r#north: true,
                r#south: false,
                r#west: false,
                r#east: false,
            });
        }
        if state_id == 11349 {
            return Some(MagentaStainedGlassPane {
                r#waterlogged: false,
                r#west: false,
                r#south: true,
                r#east: false,
                r#north: false,
            });
        }
        if state_id == 11343 {
            return Some(MagentaStainedGlassPane {
                r#west: false,
                r#south: false,
                r#waterlogged: true,
                r#north: true,
                r#east: false,
            });
        }
        if state_id == 11347 {
            return Some(MagentaStainedGlassPane {
                r#east: false,
                r#south: true,
                r#west: false,
                r#waterlogged: true,
                r#north: false,
            });
        }
        if state_id == 11329 {
            return Some(MagentaStainedGlassPane {
                r#east: true,
                r#north: true,
                r#waterlogged: false,
                r#south: false,
                r#west: false,
            });
        }
        if state_id == 11325 {
            return Some(MagentaStainedGlassPane {
                r#east: true,
                r#waterlogged: false,
                r#south: true,
                r#west: false,
                r#north: true,
            });
        }
        if state_id == 11335 {
            return Some(MagentaStainedGlassPane {
                r#south: false,
                r#north: false,
                r#waterlogged: true,
                r#east: true,
                r#west: false,
            });
        }
        if state_id == 11327 {
            return Some(MagentaStainedGlassPane {
                r#east: true,
                r#south: false,
                r#north: true,
                r#waterlogged: true,
                r#west: false,
            });
        }
        if state_id == 11352 {
            return Some(MagentaStainedGlassPane {
                r#north: false,
                r#east: false,
                r#south: false,
                r#west: true,
                r#waterlogged: false,
            });
        }
        if state_id == 11333 {
            return Some(MagentaStainedGlassPane {
                r#waterlogged: false,
                r#west: false,
                r#east: true,
                r#south: true,
                r#north: false,
            });
        }
        if state_id == 11346 {
            return Some(MagentaStainedGlassPane {
                r#east: false,
                r#north: false,
                r#south: true,
                r#waterlogged: true,
                r#west: true,
            });
        }
        if state_id == 11331 {
            return Some(MagentaStainedGlassPane {
                r#south: true,
                r#north: false,
                r#east: true,
                r#west: false,
                r#waterlogged: true,
            });
        }
        if state_id == 11334 {
            return Some(MagentaStainedGlassPane {
                r#east: true,
                r#south: false,
                r#waterlogged: true,
                r#north: false,
                r#west: true,
            });
        }
        if state_id == 11332 {
            return Some(MagentaStainedGlassPane {
                r#north: false,
                r#waterlogged: false,
                r#east: true,
                r#south: true,
                r#west: true,
            });
        }
        if state_id == 11339 {
            return Some(MagentaStainedGlassPane {
                r#east: false,
                r#north: true,
                r#waterlogged: true,
                r#west: false,
                r#south: true,
            });
        }
        if state_id == 11344 {
            return Some(MagentaStainedGlassPane {
                r#east: false,
                r#north: true,
                r#west: true,
                r#south: false,
                r#waterlogged: false,
            });
        }
        if state_id == 11348 {
            return Some(MagentaStainedGlassPane {
                r#west: true,
                r#south: true,
                r#waterlogged: false,
                r#east: false,
                r#north: false,
            });
        }
        if state_id == 11324 {
            return Some(MagentaStainedGlassPane {
                r#east: true,
                r#waterlogged: false,
                r#south: true,
                r#west: true,
                r#north: true,
            });
        }
        if state_id == 11351 {
            return Some(MagentaStainedGlassPane {
                r#waterlogged: true,
                r#north: false,
                r#south: false,
                r#east: false,
                r#west: false,
            });
        }
        if state_id == 11353 {
            return Some(MagentaStainedGlassPane {
                r#east: false,
                r#north: false,
                r#waterlogged: false,
                r#west: false,
                r#south: false,
            });
        }
        if state_id == 11336 {
            return Some(MagentaStainedGlassPane {
                r#west: true,
                r#east: true,
                r#south: false,
                r#north: false,
                r#waterlogged: false,
            });
        }
        if state_id == 11330 {
            return Some(MagentaStainedGlassPane {
                r#east: true,
                r#north: false,
                r#waterlogged: true,
                r#south: true,
                r#west: true,
            });
        }
        return None;
    }
}
