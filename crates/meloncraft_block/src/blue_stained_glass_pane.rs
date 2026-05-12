use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlueStainedGlassPane {
    pub east: bool,
    pub north: bool,
    pub south: bool,
    pub waterlogged: bool,
    pub west: bool,
}


impl BlockState for BlueStainedGlassPane {
    fn to_id(self) -> i32 {
        if block_state.r#north == false && block_state.r#south == true && block_state.r#east == false && block_state.r#west == true && block_state.r#waterlogged == true { return 11634; }
        if block_state.r#west == true && block_state.r#waterlogged == false && block_state.r#east == true && block_state.r#north == false && block_state.r#south == false { return 11624; }
        if block_state.r#north == true && block_state.r#west == true && block_state.r#waterlogged == false && block_state.r#east == false && block_state.r#south == false { return 11632; }
        if block_state.r#north == true && block_state.r#west == false && block_state.r#east == false && block_state.r#south == false && block_state.r#waterlogged == true { return 11631; }
        if block_state.r#east == false && block_state.r#south == true && block_state.r#north == true && block_state.r#west == false && block_state.r#waterlogged == true { return 11627; }
        if block_state.r#west == false && block_state.r#south == true && block_state.r#east == false && block_state.r#north == true && block_state.r#waterlogged == false { return 11629; }
        if block_state.r#waterlogged == true && block_state.r#west == true && block_state.r#east == false && block_state.r#north == false && block_state.r#south == false { return 11638; }
        if block_state.r#east == true && block_state.r#waterlogged == true && block_state.r#west == false && block_state.r#north == false && block_state.r#south == true { return 11619; }
        if block_state.r#south == false && block_state.r#east == false && block_state.r#north == false && block_state.r#west == true && block_state.r#waterlogged == false { return 11640; }
        if block_state.r#west == false && block_state.r#waterlogged == false && block_state.r#east == true && block_state.r#north == true && block_state.r#south == false { return 11617; }
        if block_state.r#south == true && block_state.r#east == true && block_state.r#north == false && block_state.r#waterlogged == false && block_state.r#west == true { return 11620; }
        if block_state.r#waterlogged == true && block_state.r#south == false && block_state.r#east == false && block_state.r#north == false && block_state.r#west == false { return 11639; }
        if block_state.r#waterlogged == true && block_state.r#west == true && block_state.r#east == true && block_state.r#south == true && block_state.r#north == false { return 11618; }
        if block_state.r#north == true && block_state.r#east == true && block_state.r#waterlogged == true && block_state.r#west == false && block_state.r#south == false { return 11615; }
        if block_state.r#north == true && block_state.r#east == true && block_state.r#west == true && block_state.r#waterlogged == false && block_state.r#south == false { return 11616; }
        if block_state.r#east == false && block_state.r#north == true && block_state.r#west == true && block_state.r#waterlogged == true && block_state.r#south == false { return 11630; }
        if block_state.r#south == false && block_state.r#east == false && block_state.r#waterlogged == false && block_state.r#north == false && block_state.r#west == false { return 11641; }
        if block_state.r#north == true && block_state.r#west == false && block_state.r#waterlogged == false && block_state.r#east == true && block_state.r#south == true { return 11613; }
        if block_state.r#waterlogged == true && block_state.r#south == true && block_state.r#north == true && block_state.r#west == true && block_state.r#east == true { return 11610; }
        if block_state.r#waterlogged == true && block_state.r#east == true && block_state.r#west == true && block_state.r#north == false && block_state.r#south == false { return 11622; }
        if block_state.r#west == false && block_state.r#north == true && block_state.r#east == true && block_state.r#waterlogged == true && block_state.r#south == true { return 11611; }
        if block_state.r#east == true && block_state.r#waterlogged == false && block_state.r#south == false && block_state.r#west == false && block_state.r#north == false { return 11625; }
        if block_state.r#east == false && block_state.r#north == true && block_state.r#south == false && block_state.r#west == false && block_state.r#waterlogged == false { return 11633; }
        if block_state.r#north == false && block_state.r#waterlogged == false && block_state.r#east == true && block_state.r#west == false && block_state.r#south == true { return 11621; }
        if block_state.r#east == true && block_state.r#waterlogged == true && block_state.r#west == false && block_state.r#north == false && block_state.r#south == false { return 11623; }
        if block_state.r#north == true && block_state.r#west == true && block_state.r#east == true && block_state.r#waterlogged == false && block_state.r#south == true { return 11612; }
        if block_state.r#waterlogged == true && block_state.r#south == false && block_state.r#north == true && block_state.r#east == true && block_state.r#west == true { return 11614; }
        if block_state.r#north == true && block_state.r#west == true && block_state.r#south == true && block_state.r#waterlogged == true && block_state.r#east == false { return 11626; }
        if block_state.r#south == true && block_state.r#waterlogged == false && block_state.r#east == false && block_state.r#west == false && block_state.r#north == false { return 11637; }
        if block_state.r#east == false && block_state.r#waterlogged == false && block_state.r#west == true && block_state.r#south == true && block_state.r#north == true { return 11628; }
        if block_state.r#waterlogged == false && block_state.r#north == false && block_state.r#east == false && block_state.r#west == true && block_state.r#south == true { return 11636; }
        if block_state.r#south == true && block_state.r#north == false && block_state.r#west == false && block_state.r#east == false && block_state.r#waterlogged == true { return 11635; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 11634 {
            return Some(BlueStainedGlassPane {
                r#north: false,
                r#south: true,
                r#east: false,
                r#west: true,
                r#waterlogged: true,
            });
        }
        if state_id == 11624 {
            return Some(BlueStainedGlassPane {
                r#west: true,
                r#waterlogged: false,
                r#east: true,
                r#north: false,
                r#south: false,
            });
        }
        if state_id == 11632 {
            return Some(BlueStainedGlassPane {
                r#north: true,
                r#west: true,
                r#waterlogged: false,
                r#east: false,
                r#south: false,
            });
        }
        if state_id == 11631 {
            return Some(BlueStainedGlassPane {
                r#north: true,
                r#west: false,
                r#east: false,
                r#south: false,
                r#waterlogged: true,
            });
        }
        if state_id == 11627 {
            return Some(BlueStainedGlassPane {
                r#east: false,
                r#south: true,
                r#north: true,
                r#west: false,
                r#waterlogged: true,
            });
        }
        if state_id == 11629 {
            return Some(BlueStainedGlassPane {
                r#west: false,
                r#south: true,
                r#east: false,
                r#north: true,
                r#waterlogged: false,
            });
        }
        if state_id == 11638 {
            return Some(BlueStainedGlassPane {
                r#waterlogged: true,
                r#west: true,
                r#east: false,
                r#north: false,
                r#south: false,
            });
        }
        if state_id == 11619 {
            return Some(BlueStainedGlassPane {
                r#east: true,
                r#waterlogged: true,
                r#west: false,
                r#north: false,
                r#south: true,
            });
        }
        if state_id == 11640 {
            return Some(BlueStainedGlassPane {
                r#south: false,
                r#east: false,
                r#north: false,
                r#west: true,
                r#waterlogged: false,
            });
        }
        if state_id == 11617 {
            return Some(BlueStainedGlassPane {
                r#west: false,
                r#waterlogged: false,
                r#east: true,
                r#north: true,
                r#south: false,
            });
        }
        if state_id == 11620 {
            return Some(BlueStainedGlassPane {
                r#south: true,
                r#east: true,
                r#north: false,
                r#waterlogged: false,
                r#west: true,
            });
        }
        if state_id == 11639 {
            return Some(BlueStainedGlassPane {
                r#waterlogged: true,
                r#south: false,
                r#east: false,
                r#north: false,
                r#west: false,
            });
        }
        if state_id == 11618 {
            return Some(BlueStainedGlassPane {
                r#waterlogged: true,
                r#west: true,
                r#east: true,
                r#south: true,
                r#north: false,
            });
        }
        if state_id == 11615 {
            return Some(BlueStainedGlassPane {
                r#north: true,
                r#east: true,
                r#waterlogged: true,
                r#west: false,
                r#south: false,
            });
        }
        if state_id == 11616 {
            return Some(BlueStainedGlassPane {
                r#north: true,
                r#east: true,
                r#west: true,
                r#waterlogged: false,
                r#south: false,
            });
        }
        if state_id == 11630 {
            return Some(BlueStainedGlassPane {
                r#east: false,
                r#north: true,
                r#west: true,
                r#waterlogged: true,
                r#south: false,
            });
        }
        if state_id == 11641 {
            return Some(BlueStainedGlassPane {
                r#south: false,
                r#east: false,
                r#waterlogged: false,
                r#north: false,
                r#west: false,
            });
        }
        if state_id == 11613 {
            return Some(BlueStainedGlassPane {
                r#north: true,
                r#west: false,
                r#waterlogged: false,
                r#east: true,
                r#south: true,
            });
        }
        if state_id == 11610 {
            return Some(BlueStainedGlassPane {
                r#waterlogged: true,
                r#south: true,
                r#north: true,
                r#west: true,
                r#east: true,
            });
        }
        if state_id == 11622 {
            return Some(BlueStainedGlassPane {
                r#waterlogged: true,
                r#east: true,
                r#west: true,
                r#north: false,
                r#south: false,
            });
        }
        if state_id == 11611 {
            return Some(BlueStainedGlassPane {
                r#west: false,
                r#north: true,
                r#east: true,
                r#waterlogged: true,
                r#south: true,
            });
        }
        if state_id == 11625 {
            return Some(BlueStainedGlassPane {
                r#east: true,
                r#waterlogged: false,
                r#south: false,
                r#west: false,
                r#north: false,
            });
        }
        if state_id == 11633 {
            return Some(BlueStainedGlassPane {
                r#east: false,
                r#north: true,
                r#south: false,
                r#west: false,
                r#waterlogged: false,
            });
        }
        if state_id == 11621 {
            return Some(BlueStainedGlassPane {
                r#north: false,
                r#waterlogged: false,
                r#east: true,
                r#west: false,
                r#south: true,
            });
        }
        if state_id == 11623 {
            return Some(BlueStainedGlassPane {
                r#east: true,
                r#waterlogged: true,
                r#west: false,
                r#north: false,
                r#south: false,
            });
        }
        if state_id == 11612 {
            return Some(BlueStainedGlassPane {
                r#north: true,
                r#west: true,
                r#east: true,
                r#waterlogged: false,
                r#south: true,
            });
        }
        if state_id == 11614 {
            return Some(BlueStainedGlassPane {
                r#waterlogged: true,
                r#south: false,
                r#north: true,
                r#east: true,
                r#west: true,
            });
        }
        if state_id == 11626 {
            return Some(BlueStainedGlassPane {
                r#north: true,
                r#west: true,
                r#south: true,
                r#waterlogged: true,
                r#east: false,
            });
        }
        if state_id == 11637 {
            return Some(BlueStainedGlassPane {
                r#south: true,
                r#waterlogged: false,
                r#east: false,
                r#west: false,
                r#north: false,
            });
        }
        if state_id == 11628 {
            return Some(BlueStainedGlassPane {
                r#east: false,
                r#waterlogged: false,
                r#west: true,
                r#south: true,
                r#north: true,
            });
        }
        if state_id == 11636 {
            return Some(BlueStainedGlassPane {
                r#waterlogged: false,
                r#north: false,
                r#east: false,
                r#west: true,
                r#south: true,
            });
        }
        if state_id == 11635 {
            return Some(BlueStainedGlassPane {
                r#south: true,
                r#north: false,
                r#west: false,
                r#east: false,
                r#waterlogged: true,
            });
        }
        return None;
    }
}

