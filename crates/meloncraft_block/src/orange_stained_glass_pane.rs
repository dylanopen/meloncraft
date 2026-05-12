use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrangeStainedGlassPane {
    pub east: bool,
    pub waterlogged: bool,
    pub west: bool,
    pub south: bool,
    pub north: bool,
}


impl BlockState for OrangeStainedGlassPane {
    fn to_id(self) -> i32 {
        if block_state.r#north == false && block_state.r#east == true && block_state.r#south == false && block_state.r#waterlogged == false && block_state.r#west == true { return 11304; }
        if block_state.r#north == true && block_state.r#east == true && block_state.r#waterlogged == true && block_state.r#west == false && block_state.r#south == true { return 11291; }
        if block_state.r#waterlogged == true && block_state.r#east == false && block_state.r#south == false && block_state.r#north == true && block_state.r#west == false { return 11311; }
        if block_state.r#waterlogged == true && block_state.r#north == false && block_state.r#east == false && block_state.r#west == false && block_state.r#south == false { return 11319; }
        if block_state.r#waterlogged == true && block_state.r#south == false && block_state.r#west == false && block_state.r#east == true && block_state.r#north == true { return 11295; }
        if block_state.r#east == false && block_state.r#north == false && block_state.r#south == false && block_state.r#waterlogged == false && block_state.r#west == false { return 11321; }
        if block_state.r#west == true && block_state.r#north == true && block_state.r#waterlogged == true && block_state.r#east == false && block_state.r#south == true { return 11306; }
        if block_state.r#south == false && block_state.r#east == true && block_state.r#north == true && block_state.r#waterlogged == false && block_state.r#west == false { return 11297; }
        if block_state.r#west == true && block_state.r#waterlogged == true && block_state.r#east == true && block_state.r#north == true && block_state.r#south == false { return 11294; }
        if block_state.r#west == true && block_state.r#east == true && block_state.r#south == true && block_state.r#waterlogged == true && block_state.r#north == true { return 11290; }
        if block_state.r#waterlogged == false && block_state.r#east == false && block_state.r#west == true && block_state.r#south == true && block_state.r#north == false { return 11316; }
        if block_state.r#waterlogged == false && block_state.r#north == true && block_state.r#west == false && block_state.r#east == false && block_state.r#south == true { return 11309; }
        if block_state.r#waterlogged == true && block_state.r#south == false && block_state.r#east == false && block_state.r#north == true && block_state.r#west == true { return 11310; }
        if block_state.r#east == false && block_state.r#south == true && block_state.r#waterlogged == true && block_state.r#west == false && block_state.r#north == false { return 11315; }
        if block_state.r#south == true && block_state.r#north == true && block_state.r#east == true && block_state.r#waterlogged == false && block_state.r#west == false { return 11293; }
        if block_state.r#south == true && block_state.r#waterlogged == true && block_state.r#east == false && block_state.r#west == true && block_state.r#north == false { return 11314; }
        if block_state.r#east == false && block_state.r#north == false && block_state.r#waterlogged == false && block_state.r#west == false && block_state.r#south == true { return 11317; }
        if block_state.r#south == true && block_state.r#west == false && block_state.r#east == false && block_state.r#waterlogged == true && block_state.r#north == true { return 11307; }
        if block_state.r#north == false && block_state.r#south == true && block_state.r#waterlogged == true && block_state.r#east == true && block_state.r#west == false { return 11299; }
        if block_state.r#south == true && block_state.r#east == false && block_state.r#north == true && block_state.r#waterlogged == false && block_state.r#west == true { return 11308; }
        if block_state.r#west == true && block_state.r#east == true && block_state.r#waterlogged == false && block_state.r#south == true && block_state.r#north == false { return 11300; }
        if block_state.r#west == true && block_state.r#south == true && block_state.r#north == false && block_state.r#waterlogged == true && block_state.r#east == true { return 11298; }
        if block_state.r#north == false && block_state.r#east == true && block_state.r#south == false && block_state.r#waterlogged == true && block_state.r#west == true { return 11302; }
        if block_state.r#south == false && block_state.r#west == false && block_state.r#waterlogged == true && block_state.r#east == true && block_state.r#north == false { return 11303; }
        if block_state.r#waterlogged == false && block_state.r#west == false && block_state.r#east == true && block_state.r#north == false && block_state.r#south == true { return 11301; }
        if block_state.r#west == false && block_state.r#south == false && block_state.r#north == true && block_state.r#waterlogged == false && block_state.r#east == false { return 11313; }
        if block_state.r#south == false && block_state.r#east == false && block_state.r#waterlogged == true && block_state.r#west == true && block_state.r#north == false { return 11318; }
        if block_state.r#waterlogged == false && block_state.r#west == true && block_state.r#east == true && block_state.r#north == true && block_state.r#south == true { return 11292; }
        if block_state.r#east == false && block_state.r#waterlogged == false && block_state.r#west == true && block_state.r#south == false && block_state.r#north == true { return 11312; }
        if block_state.r#west == false && block_state.r#north == false && block_state.r#waterlogged == false && block_state.r#east == true && block_state.r#south == false { return 11305; }
        if block_state.r#south == false && block_state.r#north == true && block_state.r#east == true && block_state.r#west == true && block_state.r#waterlogged == false { return 11296; }
        if block_state.r#east == false && block_state.r#north == false && block_state.r#waterlogged == false && block_state.r#south == false && block_state.r#west == true { return 11320; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 11304 {
            return Some(OrangeStainedGlassPane {
                r#north: false,
                r#east: true,
                r#south: false,
                r#waterlogged: false,
                r#west: true,
            });
        }
        if state_id == 11291 {
            return Some(OrangeStainedGlassPane {
                r#north: true,
                r#east: true,
                r#waterlogged: true,
                r#west: false,
                r#south: true,
            });
        }
        if state_id == 11311 {
            return Some(OrangeStainedGlassPane {
                r#waterlogged: true,
                r#east: false,
                r#south: false,
                r#north: true,
                r#west: false,
            });
        }
        if state_id == 11319 {
            return Some(OrangeStainedGlassPane {
                r#waterlogged: true,
                r#north: false,
                r#east: false,
                r#west: false,
                r#south: false,
            });
        }
        if state_id == 11295 {
            return Some(OrangeStainedGlassPane {
                r#waterlogged: true,
                r#south: false,
                r#west: false,
                r#east: true,
                r#north: true,
            });
        }
        if state_id == 11321 {
            return Some(OrangeStainedGlassPane {
                r#east: false,
                r#north: false,
                r#south: false,
                r#waterlogged: false,
                r#west: false,
            });
        }
        if state_id == 11306 {
            return Some(OrangeStainedGlassPane {
                r#west: true,
                r#north: true,
                r#waterlogged: true,
                r#east: false,
                r#south: true,
            });
        }
        if state_id == 11297 {
            return Some(OrangeStainedGlassPane {
                r#south: false,
                r#east: true,
                r#north: true,
                r#waterlogged: false,
                r#west: false,
            });
        }
        if state_id == 11294 {
            return Some(OrangeStainedGlassPane {
                r#west: true,
                r#waterlogged: true,
                r#east: true,
                r#north: true,
                r#south: false,
            });
        }
        if state_id == 11290 {
            return Some(OrangeStainedGlassPane {
                r#west: true,
                r#east: true,
                r#south: true,
                r#waterlogged: true,
                r#north: true,
            });
        }
        if state_id == 11316 {
            return Some(OrangeStainedGlassPane {
                r#waterlogged: false,
                r#east: false,
                r#west: true,
                r#south: true,
                r#north: false,
            });
        }
        if state_id == 11309 {
            return Some(OrangeStainedGlassPane {
                r#waterlogged: false,
                r#north: true,
                r#west: false,
                r#east: false,
                r#south: true,
            });
        }
        if state_id == 11310 {
            return Some(OrangeStainedGlassPane {
                r#waterlogged: true,
                r#south: false,
                r#east: false,
                r#north: true,
                r#west: true,
            });
        }
        if state_id == 11315 {
            return Some(OrangeStainedGlassPane {
                r#east: false,
                r#south: true,
                r#waterlogged: true,
                r#west: false,
                r#north: false,
            });
        }
        if state_id == 11293 {
            return Some(OrangeStainedGlassPane {
                r#south: true,
                r#north: true,
                r#east: true,
                r#waterlogged: false,
                r#west: false,
            });
        }
        if state_id == 11314 {
            return Some(OrangeStainedGlassPane {
                r#south: true,
                r#waterlogged: true,
                r#east: false,
                r#west: true,
                r#north: false,
            });
        }
        if state_id == 11317 {
            return Some(OrangeStainedGlassPane {
                r#east: false,
                r#north: false,
                r#waterlogged: false,
                r#west: false,
                r#south: true,
            });
        }
        if state_id == 11307 {
            return Some(OrangeStainedGlassPane {
                r#south: true,
                r#west: false,
                r#east: false,
                r#waterlogged: true,
                r#north: true,
            });
        }
        if state_id == 11299 {
            return Some(OrangeStainedGlassPane {
                r#north: false,
                r#south: true,
                r#waterlogged: true,
                r#east: true,
                r#west: false,
            });
        }
        if state_id == 11308 {
            return Some(OrangeStainedGlassPane {
                r#south: true,
                r#east: false,
                r#north: true,
                r#waterlogged: false,
                r#west: true,
            });
        }
        if state_id == 11300 {
            return Some(OrangeStainedGlassPane {
                r#west: true,
                r#east: true,
                r#waterlogged: false,
                r#south: true,
                r#north: false,
            });
        }
        if state_id == 11298 {
            return Some(OrangeStainedGlassPane {
                r#west: true,
                r#south: true,
                r#north: false,
                r#waterlogged: true,
                r#east: true,
            });
        }
        if state_id == 11302 {
            return Some(OrangeStainedGlassPane {
                r#north: false,
                r#east: true,
                r#south: false,
                r#waterlogged: true,
                r#west: true,
            });
        }
        if state_id == 11303 {
            return Some(OrangeStainedGlassPane {
                r#south: false,
                r#west: false,
                r#waterlogged: true,
                r#east: true,
                r#north: false,
            });
        }
        if state_id == 11301 {
            return Some(OrangeStainedGlassPane {
                r#waterlogged: false,
                r#west: false,
                r#east: true,
                r#north: false,
                r#south: true,
            });
        }
        if state_id == 11313 {
            return Some(OrangeStainedGlassPane {
                r#west: false,
                r#south: false,
                r#north: true,
                r#waterlogged: false,
                r#east: false,
            });
        }
        if state_id == 11318 {
            return Some(OrangeStainedGlassPane {
                r#south: false,
                r#east: false,
                r#waterlogged: true,
                r#west: true,
                r#north: false,
            });
        }
        if state_id == 11292 {
            return Some(OrangeStainedGlassPane {
                r#waterlogged: false,
                r#west: true,
                r#east: true,
                r#north: true,
                r#south: true,
            });
        }
        if state_id == 11312 {
            return Some(OrangeStainedGlassPane {
                r#east: false,
                r#waterlogged: false,
                r#west: true,
                r#south: false,
                r#north: true,
            });
        }
        if state_id == 11305 {
            return Some(OrangeStainedGlassPane {
                r#west: false,
                r#north: false,
                r#waterlogged: false,
                r#east: true,
                r#south: false,
            });
        }
        if state_id == 11296 {
            return Some(OrangeStainedGlassPane {
                r#south: false,
                r#north: true,
                r#east: true,
                r#west: true,
                r#waterlogged: false,
            });
        }
        if state_id == 11320 {
            return Some(OrangeStainedGlassPane {
                r#east: false,
                r#north: false,
                r#waterlogged: false,
                r#south: false,
                r#west: true,
            });
        }
        return None;
    }
}

