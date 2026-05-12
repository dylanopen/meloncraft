use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct YellowStainedGlassPane {
    pub east: bool,
    pub south: bool,
    pub west: bool,
    pub waterlogged: bool,
    pub north: bool,
}


impl BlockState for YellowStainedGlassPane {
    fn to_id(self) -> i32 {
        if block_state.r#north == true && block_state.r#waterlogged == false && block_state.r#west == true && block_state.r#east == false && block_state.r#south == false { return 11408; }
        if block_state.r#east == false && block_state.r#south == true && block_state.r#waterlogged == false && block_state.r#west == true && block_state.r#north == false { return 11412; }
        if block_state.r#waterlogged == true && block_state.r#east == false && block_state.r#south == true && block_state.r#north == false && block_state.r#west == false { return 11411; }
        if block_state.r#east == true && block_state.r#south == true && block_state.r#north == true && block_state.r#west == false && block_state.r#waterlogged == true { return 11387; }
        if block_state.r#south == true && block_state.r#west == true && block_state.r#waterlogged == true && block_state.r#east == true && block_state.r#north == false { return 11394; }
        if block_state.r#east == true && block_state.r#waterlogged == false && block_state.r#west == false && block_state.r#north == true && block_state.r#south == true { return 11389; }
        if block_state.r#north == true && block_state.r#south == true && block_state.r#west == false && block_state.r#east == false && block_state.r#waterlogged == true { return 11403; }
        if block_state.r#west == true && block_state.r#north == false && block_state.r#waterlogged == true && block_state.r#east == false && block_state.r#south == false { return 11414; }
        if block_state.r#west == true && block_state.r#east == true && block_state.r#north == true && block_state.r#south == false && block_state.r#waterlogged == true { return 11390; }
        if block_state.r#north == true && block_state.r#west == true && block_state.r#waterlogged == false && block_state.r#east == true && block_state.r#south == false { return 11392; }
        if block_state.r#north == false && block_state.r#east == false && block_state.r#south == false && block_state.r#west == false && block_state.r#waterlogged == true { return 11415; }
        if block_state.r#south == true && block_state.r#north == true && block_state.r#east == true && block_state.r#west == true && block_state.r#waterlogged == false { return 11388; }
        if block_state.r#north == false && block_state.r#east == true && block_state.r#south == true && block_state.r#west == false && block_state.r#waterlogged == true { return 11395; }
        if block_state.r#waterlogged == true && block_state.r#east == true && block_state.r#north == false && block_state.r#west == false && block_state.r#south == false { return 11399; }
        if block_state.r#waterlogged == false && block_state.r#west == false && block_state.r#east == true && block_state.r#south == false && block_state.r#north == true { return 11393; }
        if block_state.r#west == false && block_state.r#east == false && block_state.r#north == true && block_state.r#waterlogged == false && block_state.r#south == false { return 11409; }
        if block_state.r#east == true && block_state.r#waterlogged == true && block_state.r#north == true && block_state.r#south == false && block_state.r#west == false { return 11391; }
        if block_state.r#east == false && block_state.r#waterlogged == true && block_state.r#west == true && block_state.r#south == true && block_state.r#north == true { return 11402; }
        if block_state.r#north == false && block_state.r#east == false && block_state.r#west == true && block_state.r#waterlogged == true && block_state.r#south == true { return 11410; }
        if block_state.r#south == false && block_state.r#west == true && block_state.r#east == false && block_state.r#north == false && block_state.r#waterlogged == false { return 11416; }
        if block_state.r#south == false && block_state.r#east == false && block_state.r#north == false && block_state.r#waterlogged == false && block_state.r#west == false { return 11417; }
        if block_state.r#south == false && block_state.r#waterlogged == true && block_state.r#east == false && block_state.r#north == true && block_state.r#west == false { return 11407; }
        if block_state.r#east == true && block_state.r#south == false && block_state.r#north == false && block_state.r#waterlogged == false && block_state.r#west == true { return 11400; }
        if block_state.r#north == false && block_state.r#south == true && block_state.r#waterlogged == false && block_state.r#east == true && block_state.r#west == true { return 11396; }
        if block_state.r#south == true && block_state.r#east == true && block_state.r#waterlogged == false && block_state.r#north == false && block_state.r#west == false { return 11397; }
        if block_state.r#east == false && block_state.r#north == true && block_state.r#waterlogged == true && block_state.r#west == true && block_state.r#south == false { return 11406; }
        if block_state.r#waterlogged == false && block_state.r#north == false && block_state.r#east == true && block_state.r#west == false && block_state.r#south == false { return 11401; }
        if block_state.r#waterlogged == false && block_state.r#south == true && block_state.r#west == false && block_state.r#north == true && block_state.r#east == false { return 11405; }
        if block_state.r#west == true && block_state.r#south == false && block_state.r#north == false && block_state.r#waterlogged == true && block_state.r#east == true { return 11398; }
        if block_state.r#waterlogged == false && block_state.r#west == false && block_state.r#north == false && block_state.r#south == true && block_state.r#east == false { return 11413; }
        if block_state.r#north == true && block_state.r#south == true && block_state.r#east == true && block_state.r#waterlogged == true && block_state.r#west == true { return 11386; }
        if block_state.r#north == true && block_state.r#waterlogged == false && block_state.r#south == true && block_state.r#west == true && block_state.r#east == false { return 11404; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 11408 {
            return Some(YellowStainedGlassPane {
                r#north: true,
                r#waterlogged: false,
                r#west: true,
                r#east: false,
                r#south: false,
            });
        }
        if state_id == 11412 {
            return Some(YellowStainedGlassPane {
                r#east: false,
                r#south: true,
                r#waterlogged: false,
                r#west: true,
                r#north: false,
            });
        }
        if state_id == 11411 {
            return Some(YellowStainedGlassPane {
                r#waterlogged: true,
                r#east: false,
                r#south: true,
                r#north: false,
                r#west: false,
            });
        }
        if state_id == 11387 {
            return Some(YellowStainedGlassPane {
                r#east: true,
                r#south: true,
                r#north: true,
                r#west: false,
                r#waterlogged: true,
            });
        }
        if state_id == 11394 {
            return Some(YellowStainedGlassPane {
                r#south: true,
                r#west: true,
                r#waterlogged: true,
                r#east: true,
                r#north: false,
            });
        }
        if state_id == 11389 {
            return Some(YellowStainedGlassPane {
                r#east: true,
                r#waterlogged: false,
                r#west: false,
                r#north: true,
                r#south: true,
            });
        }
        if state_id == 11403 {
            return Some(YellowStainedGlassPane {
                r#north: true,
                r#south: true,
                r#west: false,
                r#east: false,
                r#waterlogged: true,
            });
        }
        if state_id == 11414 {
            return Some(YellowStainedGlassPane {
                r#west: true,
                r#north: false,
                r#waterlogged: true,
                r#east: false,
                r#south: false,
            });
        }
        if state_id == 11390 {
            return Some(YellowStainedGlassPane {
                r#west: true,
                r#east: true,
                r#north: true,
                r#south: false,
                r#waterlogged: true,
            });
        }
        if state_id == 11392 {
            return Some(YellowStainedGlassPane {
                r#north: true,
                r#west: true,
                r#waterlogged: false,
                r#east: true,
                r#south: false,
            });
        }
        if state_id == 11415 {
            return Some(YellowStainedGlassPane {
                r#north: false,
                r#east: false,
                r#south: false,
                r#west: false,
                r#waterlogged: true,
            });
        }
        if state_id == 11388 {
            return Some(YellowStainedGlassPane {
                r#south: true,
                r#north: true,
                r#east: true,
                r#west: true,
                r#waterlogged: false,
            });
        }
        if state_id == 11395 {
            return Some(YellowStainedGlassPane {
                r#north: false,
                r#east: true,
                r#south: true,
                r#west: false,
                r#waterlogged: true,
            });
        }
        if state_id == 11399 {
            return Some(YellowStainedGlassPane {
                r#waterlogged: true,
                r#east: true,
                r#north: false,
                r#west: false,
                r#south: false,
            });
        }
        if state_id == 11393 {
            return Some(YellowStainedGlassPane {
                r#waterlogged: false,
                r#west: false,
                r#east: true,
                r#south: false,
                r#north: true,
            });
        }
        if state_id == 11409 {
            return Some(YellowStainedGlassPane {
                r#west: false,
                r#east: false,
                r#north: true,
                r#waterlogged: false,
                r#south: false,
            });
        }
        if state_id == 11391 {
            return Some(YellowStainedGlassPane {
                r#east: true,
                r#waterlogged: true,
                r#north: true,
                r#south: false,
                r#west: false,
            });
        }
        if state_id == 11402 {
            return Some(YellowStainedGlassPane {
                r#east: false,
                r#waterlogged: true,
                r#west: true,
                r#south: true,
                r#north: true,
            });
        }
        if state_id == 11410 {
            return Some(YellowStainedGlassPane {
                r#north: false,
                r#east: false,
                r#west: true,
                r#waterlogged: true,
                r#south: true,
            });
        }
        if state_id == 11416 {
            return Some(YellowStainedGlassPane {
                r#south: false,
                r#west: true,
                r#east: false,
                r#north: false,
                r#waterlogged: false,
            });
        }
        if state_id == 11417 {
            return Some(YellowStainedGlassPane {
                r#south: false,
                r#east: false,
                r#north: false,
                r#waterlogged: false,
                r#west: false,
            });
        }
        if state_id == 11407 {
            return Some(YellowStainedGlassPane {
                r#south: false,
                r#waterlogged: true,
                r#east: false,
                r#north: true,
                r#west: false,
            });
        }
        if state_id == 11400 {
            return Some(YellowStainedGlassPane {
                r#east: true,
                r#south: false,
                r#north: false,
                r#waterlogged: false,
                r#west: true,
            });
        }
        if state_id == 11396 {
            return Some(YellowStainedGlassPane {
                r#north: false,
                r#south: true,
                r#waterlogged: false,
                r#east: true,
                r#west: true,
            });
        }
        if state_id == 11397 {
            return Some(YellowStainedGlassPane {
                r#south: true,
                r#east: true,
                r#waterlogged: false,
                r#north: false,
                r#west: false,
            });
        }
        if state_id == 11406 {
            return Some(YellowStainedGlassPane {
                r#east: false,
                r#north: true,
                r#waterlogged: true,
                r#west: true,
                r#south: false,
            });
        }
        if state_id == 11401 {
            return Some(YellowStainedGlassPane {
                r#waterlogged: false,
                r#north: false,
                r#east: true,
                r#west: false,
                r#south: false,
            });
        }
        if state_id == 11405 {
            return Some(YellowStainedGlassPane {
                r#waterlogged: false,
                r#south: true,
                r#west: false,
                r#north: true,
                r#east: false,
            });
        }
        if state_id == 11398 {
            return Some(YellowStainedGlassPane {
                r#west: true,
                r#south: false,
                r#north: false,
                r#waterlogged: true,
                r#east: true,
            });
        }
        if state_id == 11413 {
            return Some(YellowStainedGlassPane {
                r#waterlogged: false,
                r#west: false,
                r#north: false,
                r#south: true,
                r#east: false,
            });
        }
        if state_id == 11386 {
            return Some(YellowStainedGlassPane {
                r#north: true,
                r#south: true,
                r#east: true,
                r#waterlogged: true,
                r#west: true,
            });
        }
        if state_id == 11404 {
            return Some(YellowStainedGlassPane {
                r#north: true,
                r#waterlogged: false,
                r#south: true,
                r#west: true,
                r#east: false,
            });
        }
        return None;
    }
}

