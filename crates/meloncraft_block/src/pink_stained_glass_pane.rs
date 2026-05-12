use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PinkStainedGlassPane {
    pub north: bool,
    pub east: bool,
    pub waterlogged: bool,
    pub west: bool,
    pub south: bool,
}


impl BlockState for PinkStainedGlassPane {
    fn to_id(self) -> i32 {
        if block_state.r#west == true && block_state.r#north == true && block_state.r#east == false && block_state.r#waterlogged == false && block_state.r#south == false { return 11472; }
        if block_state.r#north == true && block_state.r#east == true && block_state.r#south == false && block_state.r#waterlogged == true && block_state.r#west == true { return 11454; }
        if block_state.r#east == false && block_state.r#south == true && block_state.r#west == false && block_state.r#waterlogged == false && block_state.r#north == true { return 11469; }
        if block_state.r#waterlogged == true && block_state.r#west == false && block_state.r#east == true && block_state.r#north == false && block_state.r#south == false { return 11463; }
        if block_state.r#north == true && block_state.r#west == true && block_state.r#south == true && block_state.r#waterlogged == false && block_state.r#east == false { return 11468; }
        if block_state.r#east == true && block_state.r#waterlogged == false && block_state.r#south == true && block_state.r#west == true && block_state.r#north == true { return 11452; }
        if block_state.r#east == false && block_state.r#waterlogged == true && block_state.r#west == true && block_state.r#south == false && block_state.r#north == false { return 11478; }
        if block_state.r#waterlogged == true && block_state.r#east == false && block_state.r#north == false && block_state.r#west == false && block_state.r#south == true { return 11475; }
        if block_state.r#north == true && block_state.r#east == true && block_state.r#south == false && block_state.r#west == false && block_state.r#waterlogged == false { return 11457; }
        if block_state.r#west == false && block_state.r#south == false && block_state.r#waterlogged == true && block_state.r#north == false && block_state.r#east == false { return 11479; }
        if block_state.r#waterlogged == true && block_state.r#east == false && block_state.r#north == true && block_state.r#south == false && block_state.r#west == false { return 11471; }
        if block_state.r#east == true && block_state.r#north == false && block_state.r#south == true && block_state.r#waterlogged == true && block_state.r#west == true { return 11458; }
        if block_state.r#south == false && block_state.r#east == true && block_state.r#waterlogged == true && block_state.r#west == false && block_state.r#north == true { return 11455; }
        if block_state.r#east == false && block_state.r#waterlogged == true && block_state.r#north == true && block_state.r#west == true && block_state.r#south == true { return 11466; }
        if block_state.r#east == true && block_state.r#north == false && block_state.r#waterlogged == false && block_state.r#south == false && block_state.r#west == true { return 11464; }
        if block_state.r#north == false && block_state.r#south == false && block_state.r#west == false && block_state.r#east == false && block_state.r#waterlogged == false { return 11481; }
        if block_state.r#north == true && block_state.r#waterlogged == true && block_state.r#west == true && block_state.r#east == true && block_state.r#south == true { return 11450; }
        if block_state.r#east == true && block_state.r#waterlogged == true && block_state.r#south == false && block_state.r#west == true && block_state.r#north == false { return 11462; }
        if block_state.r#north == true && block_state.r#south == false && block_state.r#east == true && block_state.r#waterlogged == false && block_state.r#west == true { return 11456; }
        if block_state.r#east == true && block_state.r#south == true && block_state.r#north == true && block_state.r#west == false && block_state.r#waterlogged == false { return 11453; }
        if block_state.r#waterlogged == false && block_state.r#north == false && block_state.r#east == true && block_state.r#south == true && block_state.r#west == true { return 11460; }
        if block_state.r#east == true && block_state.r#waterlogged == false && block_state.r#west == false && block_state.r#north == false && block_state.r#south == false { return 11465; }
        if block_state.r#east == false && block_state.r#waterlogged == true && block_state.r#north == true && block_state.r#west == false && block_state.r#south == true { return 11467; }
        if block_state.r#south == true && block_state.r#waterlogged == false && block_state.r#north == false && block_state.r#west == true && block_state.r#east == false { return 11476; }
        if block_state.r#waterlogged == true && block_state.r#west == false && block_state.r#south == true && block_state.r#north == false && block_state.r#east == true { return 11459; }
        if block_state.r#north == false && block_state.r#waterlogged == false && block_state.r#south == true && block_state.r#east == true && block_state.r#west == false { return 11461; }
        if block_state.r#east == false && block_state.r#waterlogged == false && block_state.r#west == false && block_state.r#south == true && block_state.r#north == false { return 11477; }
        if block_state.r#east == false && block_state.r#waterlogged == true && block_state.r#west == true && block_state.r#south == false && block_state.r#north == true { return 11470; }
        if block_state.r#south == true && block_state.r#west == true && block_state.r#east == false && block_state.r#waterlogged == true && block_state.r#north == false { return 11474; }
        if block_state.r#waterlogged == false && block_state.r#south == false && block_state.r#north == false && block_state.r#east == false && block_state.r#west == true { return 11480; }
        if block_state.r#west == false && block_state.r#south == false && block_state.r#east == false && block_state.r#north == true && block_state.r#waterlogged == false { return 11473; }
        if block_state.r#south == true && block_state.r#west == false && block_state.r#north == true && block_state.r#waterlogged == true && block_state.r#east == true { return 11451; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 11472 {
            return Some(PinkStainedGlassPane {
                r#west: true,
                r#north: true,
                r#east: false,
                r#waterlogged: false,
                r#south: false,
            });
        }
        if state_id == 11454 {
            return Some(PinkStainedGlassPane {
                r#north: true,
                r#east: true,
                r#south: false,
                r#waterlogged: true,
                r#west: true,
            });
        }
        if state_id == 11469 {
            return Some(PinkStainedGlassPane {
                r#east: false,
                r#south: true,
                r#west: false,
                r#waterlogged: false,
                r#north: true,
            });
        }
        if state_id == 11463 {
            return Some(PinkStainedGlassPane {
                r#waterlogged: true,
                r#west: false,
                r#east: true,
                r#north: false,
                r#south: false,
            });
        }
        if state_id == 11468 {
            return Some(PinkStainedGlassPane {
                r#north: true,
                r#west: true,
                r#south: true,
                r#waterlogged: false,
                r#east: false,
            });
        }
        if state_id == 11452 {
            return Some(PinkStainedGlassPane {
                r#east: true,
                r#waterlogged: false,
                r#south: true,
                r#west: true,
                r#north: true,
            });
        }
        if state_id == 11478 {
            return Some(PinkStainedGlassPane {
                r#east: false,
                r#waterlogged: true,
                r#west: true,
                r#south: false,
                r#north: false,
            });
        }
        if state_id == 11475 {
            return Some(PinkStainedGlassPane {
                r#waterlogged: true,
                r#east: false,
                r#north: false,
                r#west: false,
                r#south: true,
            });
        }
        if state_id == 11457 {
            return Some(PinkStainedGlassPane {
                r#north: true,
                r#east: true,
                r#south: false,
                r#west: false,
                r#waterlogged: false,
            });
        }
        if state_id == 11479 {
            return Some(PinkStainedGlassPane {
                r#west: false,
                r#south: false,
                r#waterlogged: true,
                r#north: false,
                r#east: false,
            });
        }
        if state_id == 11471 {
            return Some(PinkStainedGlassPane {
                r#waterlogged: true,
                r#east: false,
                r#north: true,
                r#south: false,
                r#west: false,
            });
        }
        if state_id == 11458 {
            return Some(PinkStainedGlassPane {
                r#east: true,
                r#north: false,
                r#south: true,
                r#waterlogged: true,
                r#west: true,
            });
        }
        if state_id == 11455 {
            return Some(PinkStainedGlassPane {
                r#south: false,
                r#east: true,
                r#waterlogged: true,
                r#west: false,
                r#north: true,
            });
        }
        if state_id == 11466 {
            return Some(PinkStainedGlassPane {
                r#east: false,
                r#waterlogged: true,
                r#north: true,
                r#west: true,
                r#south: true,
            });
        }
        if state_id == 11464 {
            return Some(PinkStainedGlassPane {
                r#east: true,
                r#north: false,
                r#waterlogged: false,
                r#south: false,
                r#west: true,
            });
        }
        if state_id == 11481 {
            return Some(PinkStainedGlassPane {
                r#north: false,
                r#south: false,
                r#west: false,
                r#east: false,
                r#waterlogged: false,
            });
        }
        if state_id == 11450 {
            return Some(PinkStainedGlassPane {
                r#north: true,
                r#waterlogged: true,
                r#west: true,
                r#east: true,
                r#south: true,
            });
        }
        if state_id == 11462 {
            return Some(PinkStainedGlassPane {
                r#east: true,
                r#waterlogged: true,
                r#south: false,
                r#west: true,
                r#north: false,
            });
        }
        if state_id == 11456 {
            return Some(PinkStainedGlassPane {
                r#north: true,
                r#south: false,
                r#east: true,
                r#waterlogged: false,
                r#west: true,
            });
        }
        if state_id == 11453 {
            return Some(PinkStainedGlassPane {
                r#east: true,
                r#south: true,
                r#north: true,
                r#west: false,
                r#waterlogged: false,
            });
        }
        if state_id == 11460 {
            return Some(PinkStainedGlassPane {
                r#waterlogged: false,
                r#north: false,
                r#east: true,
                r#south: true,
                r#west: true,
            });
        }
        if state_id == 11465 {
            return Some(PinkStainedGlassPane {
                r#east: true,
                r#waterlogged: false,
                r#west: false,
                r#north: false,
                r#south: false,
            });
        }
        if state_id == 11467 {
            return Some(PinkStainedGlassPane {
                r#east: false,
                r#waterlogged: true,
                r#north: true,
                r#west: false,
                r#south: true,
            });
        }
        if state_id == 11476 {
            return Some(PinkStainedGlassPane {
                r#south: true,
                r#waterlogged: false,
                r#north: false,
                r#west: true,
                r#east: false,
            });
        }
        if state_id == 11459 {
            return Some(PinkStainedGlassPane {
                r#waterlogged: true,
                r#west: false,
                r#south: true,
                r#north: false,
                r#east: true,
            });
        }
        if state_id == 11461 {
            return Some(PinkStainedGlassPane {
                r#north: false,
                r#waterlogged: false,
                r#south: true,
                r#east: true,
                r#west: false,
            });
        }
        if state_id == 11477 {
            return Some(PinkStainedGlassPane {
                r#east: false,
                r#waterlogged: false,
                r#west: false,
                r#south: true,
                r#north: false,
            });
        }
        if state_id == 11470 {
            return Some(PinkStainedGlassPane {
                r#east: false,
                r#waterlogged: true,
                r#west: true,
                r#south: false,
                r#north: true,
            });
        }
        if state_id == 11474 {
            return Some(PinkStainedGlassPane {
                r#south: true,
                r#west: true,
                r#east: false,
                r#waterlogged: true,
                r#north: false,
            });
        }
        if state_id == 11480 {
            return Some(PinkStainedGlassPane {
                r#waterlogged: false,
                r#south: false,
                r#north: false,
                r#east: false,
                r#west: true,
            });
        }
        if state_id == 11473 {
            return Some(PinkStainedGlassPane {
                r#west: false,
                r#south: false,
                r#east: false,
                r#north: true,
                r#waterlogged: false,
            });
        }
        if state_id == 11451 {
            return Some(PinkStainedGlassPane {
                r#south: true,
                r#west: false,
                r#north: true,
                r#waterlogged: true,
                r#east: true,
            });
        }
        return None;
    }
}

