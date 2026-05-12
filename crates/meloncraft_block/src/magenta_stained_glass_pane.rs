use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MagentaStainedGlassPane {
    pub east: bool,
    pub west: bool,
    pub waterlogged: bool,
    pub south: bool,
    pub north: bool,
}


impl BlockState for MagentaStainedGlassPane {
    fn to_id(self) -> i32 {
        if block_state.r#east == false && block_state.r#north == false && block_state.r#waterlogged == false && block_state.r#south == false && block_state.r#west == false { return 11353; }
        if block_state.r#east == true && block_state.r#waterlogged == true && block_state.r#north == true && block_state.r#west == false && block_state.r#south == true { return 11323; }
        if block_state.r#waterlogged == true && block_state.r#north == true && block_state.r#west == true && block_state.r#south == true && block_state.r#east == false { return 11338; }
        if block_state.r#south == true && block_state.r#west == true && block_state.r#waterlogged == false && block_state.r#east == false && block_state.r#north == true { return 11340; }
        if block_state.r#east == false && block_state.r#north == false && block_state.r#waterlogged == true && block_state.r#west == true && block_state.r#south == true { return 11346; }
        if block_state.r#south == false && block_state.r#east == false && block_state.r#north == false && block_state.r#waterlogged == true && block_state.r#west == false { return 11351; }
        if block_state.r#west == false && block_state.r#east == true && block_state.r#north == true && block_state.r#south == true && block_state.r#waterlogged == false { return 11325; }
        if block_state.r#east == false && block_state.r#waterlogged == true && block_state.r#south == true && block_state.r#north == false && block_state.r#west == false { return 11347; }
        if block_state.r#west == false && block_state.r#east == true && block_state.r#north == false && block_state.r#south == false && block_state.r#waterlogged == true { return 11335; }
        if block_state.r#waterlogged == false && block_state.r#north == true && block_state.r#south == false && block_state.r#east == false && block_state.r#west == true { return 11344; }
        if block_state.r#east == true && block_state.r#waterlogged == true && block_state.r#south == false && block_state.r#west == true && block_state.r#north == true { return 11326; }
        if block_state.r#north == false && block_state.r#south == true && block_state.r#east == true && block_state.r#west == false && block_state.r#waterlogged == false { return 11333; }
        if block_state.r#south == false && block_state.r#east == true && block_state.r#waterlogged == true && block_state.r#north == false && block_state.r#west == true { return 11334; }
        if block_state.r#east == false && block_state.r#west == true && block_state.r#waterlogged == false && block_state.r#south == false && block_state.r#north == false { return 11352; }
        if block_state.r#waterlogged == false && block_state.r#west == false && block_state.r#north == true && block_state.r#east == false && block_state.r#south == true { return 11341; }
        if block_state.r#waterlogged == true && block_state.r#east == true && block_state.r#north == false && block_state.r#south == true && block_state.r#west == false { return 11331; }
        if block_state.r#west == true && block_state.r#south == true && block_state.r#waterlogged == false && block_state.r#east == true && block_state.r#north == true { return 11324; }
        if block_state.r#waterlogged == true && block_state.r#north == false && block_state.r#west == true && block_state.r#east == true && block_state.r#south == true { return 11330; }
        if block_state.r#east == true && block_state.r#north == false && block_state.r#south == false && block_state.r#waterlogged == false && block_state.r#west == false { return 11337; }
        if block_state.r#east == false && block_state.r#west == true && block_state.r#north == false && block_state.r#waterlogged == false && block_state.r#south == true { return 11348; }
        if block_state.r#east == false && block_state.r#north == false && block_state.r#south == false && block_state.r#waterlogged == true && block_state.r#west == true { return 11350; }
        if block_state.r#north == true && block_state.r#west == false && block_state.r#east == true && block_state.r#south == false && block_state.r#waterlogged == true { return 11327; }
        if block_state.r#north == false && block_state.r#east == true && block_state.r#west == true && block_state.r#south == false && block_state.r#waterlogged == false { return 11336; }
        if block_state.r#waterlogged == true && block_state.r#east == false && block_state.r#south == false && block_state.r#west == false && block_state.r#north == true { return 11343; }
        if block_state.r#north == true && block_state.r#east == true && block_state.r#west == true && block_state.r#south == false && block_state.r#waterlogged == false { return 11328; }
        if block_state.r#north == true && block_state.r#east == false && block_state.r#waterlogged == false && block_state.r#west == false && block_state.r#south == false { return 11345; }
        if block_state.r#north == true && block_state.r#east == false && block_state.r#waterlogged == true && block_state.r#west == false && block_state.r#south == true { return 11339; }
        if block_state.r#north == true && block_state.r#south == true && block_state.r#waterlogged == true && block_state.r#east == true && block_state.r#west == true { return 11322; }
        if block_state.r#waterlogged == false && block_state.r#east == true && block_state.r#west == false && block_state.r#south == false && block_state.r#north == true { return 11329; }
        if block_state.r#waterlogged == false && block_state.r#south == true && block_state.r#west == true && block_state.r#east == true && block_state.r#north == false { return 11332; }
        if block_state.r#east == false && block_state.r#south == false && block_state.r#waterlogged == true && block_state.r#west == true && block_state.r#north == true { return 11342; }
        if block_state.r#south == true && block_state.r#east == false && block_state.r#north == false && block_state.r#waterlogged == false && block_state.r#west == false { return 11349; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 11353 {
            return Some(MagentaStainedGlassPane {
                r#east: false,
                r#north: false,
                r#waterlogged: false,
                r#south: false,
                r#west: false,
            });
        }
        if state_id == 11323 {
            return Some(MagentaStainedGlassPane {
                r#east: true,
                r#waterlogged: true,
                r#north: true,
                r#west: false,
                r#south: true,
            });
        }
        if state_id == 11338 {
            return Some(MagentaStainedGlassPane {
                r#waterlogged: true,
                r#north: true,
                r#west: true,
                r#south: true,
                r#east: false,
            });
        }
        if state_id == 11340 {
            return Some(MagentaStainedGlassPane {
                r#south: true,
                r#west: true,
                r#waterlogged: false,
                r#east: false,
                r#north: true,
            });
        }
        if state_id == 11346 {
            return Some(MagentaStainedGlassPane {
                r#east: false,
                r#north: false,
                r#waterlogged: true,
                r#west: true,
                r#south: true,
            });
        }
        if state_id == 11351 {
            return Some(MagentaStainedGlassPane {
                r#south: false,
                r#east: false,
                r#north: false,
                r#waterlogged: true,
                r#west: false,
            });
        }
        if state_id == 11325 {
            return Some(MagentaStainedGlassPane {
                r#west: false,
                r#east: true,
                r#north: true,
                r#south: true,
                r#waterlogged: false,
            });
        }
        if state_id == 11347 {
            return Some(MagentaStainedGlassPane {
                r#east: false,
                r#waterlogged: true,
                r#south: true,
                r#north: false,
                r#west: false,
            });
        }
        if state_id == 11335 {
            return Some(MagentaStainedGlassPane {
                r#west: false,
                r#east: true,
                r#north: false,
                r#south: false,
                r#waterlogged: true,
            });
        }
        if state_id == 11344 {
            return Some(MagentaStainedGlassPane {
                r#waterlogged: false,
                r#north: true,
                r#south: false,
                r#east: false,
                r#west: true,
            });
        }
        if state_id == 11326 {
            return Some(MagentaStainedGlassPane {
                r#east: true,
                r#waterlogged: true,
                r#south: false,
                r#west: true,
                r#north: true,
            });
        }
        if state_id == 11333 {
            return Some(MagentaStainedGlassPane {
                r#north: false,
                r#south: true,
                r#east: true,
                r#west: false,
                r#waterlogged: false,
            });
        }
        if state_id == 11334 {
            return Some(MagentaStainedGlassPane {
                r#south: false,
                r#east: true,
                r#waterlogged: true,
                r#north: false,
                r#west: true,
            });
        }
        if state_id == 11352 {
            return Some(MagentaStainedGlassPane {
                r#east: false,
                r#west: true,
                r#waterlogged: false,
                r#south: false,
                r#north: false,
            });
        }
        if state_id == 11341 {
            return Some(MagentaStainedGlassPane {
                r#waterlogged: false,
                r#west: false,
                r#north: true,
                r#east: false,
                r#south: true,
            });
        }
        if state_id == 11331 {
            return Some(MagentaStainedGlassPane {
                r#waterlogged: true,
                r#east: true,
                r#north: false,
                r#south: true,
                r#west: false,
            });
        }
        if state_id == 11324 {
            return Some(MagentaStainedGlassPane {
                r#west: true,
                r#south: true,
                r#waterlogged: false,
                r#east: true,
                r#north: true,
            });
        }
        if state_id == 11330 {
            return Some(MagentaStainedGlassPane {
                r#waterlogged: true,
                r#north: false,
                r#west: true,
                r#east: true,
                r#south: true,
            });
        }
        if state_id == 11337 {
            return Some(MagentaStainedGlassPane {
                r#east: true,
                r#north: false,
                r#south: false,
                r#waterlogged: false,
                r#west: false,
            });
        }
        if state_id == 11348 {
            return Some(MagentaStainedGlassPane {
                r#east: false,
                r#west: true,
                r#north: false,
                r#waterlogged: false,
                r#south: true,
            });
        }
        if state_id == 11350 {
            return Some(MagentaStainedGlassPane {
                r#east: false,
                r#north: false,
                r#south: false,
                r#waterlogged: true,
                r#west: true,
            });
        }
        if state_id == 11327 {
            return Some(MagentaStainedGlassPane {
                r#north: true,
                r#west: false,
                r#east: true,
                r#south: false,
                r#waterlogged: true,
            });
        }
        if state_id == 11336 {
            return Some(MagentaStainedGlassPane {
                r#north: false,
                r#east: true,
                r#west: true,
                r#south: false,
                r#waterlogged: false,
            });
        }
        if state_id == 11343 {
            return Some(MagentaStainedGlassPane {
                r#waterlogged: true,
                r#east: false,
                r#south: false,
                r#west: false,
                r#north: true,
            });
        }
        if state_id == 11328 {
            return Some(MagentaStainedGlassPane {
                r#north: true,
                r#east: true,
                r#west: true,
                r#south: false,
                r#waterlogged: false,
            });
        }
        if state_id == 11345 {
            return Some(MagentaStainedGlassPane {
                r#north: true,
                r#east: false,
                r#waterlogged: false,
                r#west: false,
                r#south: false,
            });
        }
        if state_id == 11339 {
            return Some(MagentaStainedGlassPane {
                r#north: true,
                r#east: false,
                r#waterlogged: true,
                r#west: false,
                r#south: true,
            });
        }
        if state_id == 11322 {
            return Some(MagentaStainedGlassPane {
                r#north: true,
                r#south: true,
                r#waterlogged: true,
                r#east: true,
                r#west: true,
            });
        }
        if state_id == 11329 {
            return Some(MagentaStainedGlassPane {
                r#waterlogged: false,
                r#east: true,
                r#west: false,
                r#south: false,
                r#north: true,
            });
        }
        if state_id == 11332 {
            return Some(MagentaStainedGlassPane {
                r#waterlogged: false,
                r#south: true,
                r#west: true,
                r#east: true,
                r#north: false,
            });
        }
        if state_id == 11342 {
            return Some(MagentaStainedGlassPane {
                r#east: false,
                r#south: false,
                r#waterlogged: true,
                r#west: true,
                r#north: true,
            });
        }
        if state_id == 11349 {
            return Some(MagentaStainedGlassPane {
                r#south: true,
                r#east: false,
                r#north: false,
                r#waterlogged: false,
                r#west: false,
            });
        }
        return None;
    }
}

