use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlueStainedGlassPane {
    pub waterlogged: bool,
    pub east: bool,
    pub north: bool,
    pub west: bool,
    pub south: bool,
}


impl BlockState for BlueStainedGlassPane {
    fn to_id(&self) -> i32 {
        if self.r#east == true && self.r#south == false && self.r#waterlogged == false && self.r#west == true && self.r#north == false { return 11624; }
        if self.r#south == true && self.r#north == false && self.r#east == true && self.r#waterlogged == true && self.r#west == false { return 11619; }
        if self.r#west == false && self.r#waterlogged == false && self.r#south == true && self.r#north == true && self.r#east == false { return 11629; }
        if self.r#east == true && self.r#north == true && self.r#south == true && self.r#waterlogged == false && self.r#west == true { return 11612; }
        if self.r#waterlogged == true && self.r#west == true && self.r#east == true && self.r#south == false && self.r#north == false { return 11622; }
        if self.r#north == true && self.r#waterlogged == false && self.r#west == true && self.r#east == true && self.r#south == false { return 11616; }
        if self.r#west == false && self.r#east == false && self.r#south == false && self.r#waterlogged == false && self.r#north == false { return 11641; }
        if self.r#east == true && self.r#west == true && self.r#south == true && self.r#north == false && self.r#waterlogged == false { return 11620; }
        if self.r#waterlogged == false && self.r#west == false && self.r#east == true && self.r#north == false && self.r#south == false { return 11625; }
        if self.r#south == false && self.r#north == true && self.r#waterlogged == true && self.r#east == true && self.r#west == false { return 11615; }
        if self.r#south == true && self.r#east == false && self.r#north == false && self.r#waterlogged == false && self.r#west == true { return 11636; }
        if self.r#north == true && self.r#west == false && self.r#east == true && self.r#south == true && self.r#waterlogged == false { return 11613; }
        if self.r#east == false && self.r#waterlogged == false && self.r#north == true && self.r#south == false && self.r#west == true { return 11632; }
        if self.r#waterlogged == true && self.r#west == false && self.r#north == true && self.r#south == false && self.r#east == false { return 11631; }
        if self.r#north == true && self.r#south == false && self.r#east == true && self.r#waterlogged == false && self.r#west == false { return 11617; }
        if self.r#north == true && self.r#south == true && self.r#waterlogged == true && self.r#west == false && self.r#east == true { return 11611; }
        if self.r#north == true && self.r#waterlogged == true && self.r#west == true && self.r#east == true && self.r#south == true { return 11610; }
        if self.r#waterlogged == false && self.r#west == false && self.r#south == true && self.r#north == false && self.r#east == false { return 11637; }
        if self.r#west == true && self.r#south == true && self.r#north == false && self.r#east == true && self.r#waterlogged == true { return 11618; }
        if self.r#waterlogged == true && self.r#east == false && self.r#north == true && self.r#south == true && self.r#west == true { return 11626; }
        if self.r#east == true && self.r#waterlogged == true && self.r#north == true && self.r#south == false && self.r#west == true { return 11614; }
        if self.r#north == false && self.r#waterlogged == true && self.r#east == false && self.r#west == true && self.r#south == true { return 11634; }
        if self.r#waterlogged == true && self.r#north == false && self.r#west == false && self.r#south == true && self.r#east == false { return 11635; }
        if self.r#north == false && self.r#waterlogged == true && self.r#west == true && self.r#south == false && self.r#east == false { return 11638; }
        if self.r#east == false && self.r#north == true && self.r#waterlogged == true && self.r#west == false && self.r#south == true { return 11627; }
        if self.r#north == false && self.r#west == false && self.r#south == false && self.r#east == false && self.r#waterlogged == true { return 11639; }
        if self.r#east == false && self.r#north == true && self.r#west == true && self.r#south == false && self.r#waterlogged == true { return 11630; }
        if self.r#south == true && self.r#west == false && self.r#east == true && self.r#waterlogged == false && self.r#north == false { return 11621; }
        if self.r#north == false && self.r#waterlogged == true && self.r#south == false && self.r#west == false && self.r#east == true { return 11623; }
        if self.r#south == true && self.r#north == true && self.r#west == true && self.r#waterlogged == false && self.r#east == false { return 11628; }
        if self.r#waterlogged == false && self.r#west == true && self.r#east == false && self.r#north == false && self.r#south == false { return 11640; }
        if self.r#west == false && self.r#south == false && self.r#waterlogged == false && self.r#east == false && self.r#north == true { return 11633; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 11624 {
            return Some(BlueStainedGlassPane {
                r#east: true,
                r#south: false,
                r#waterlogged: false,
                r#west: true,
                r#north: false,
            });
        }
        if state_id == 11619 {
            return Some(BlueStainedGlassPane {
                r#south: true,
                r#north: false,
                r#east: true,
                r#waterlogged: true,
                r#west: false,
            });
        }
        if state_id == 11629 {
            return Some(BlueStainedGlassPane {
                r#west: false,
                r#waterlogged: false,
                r#south: true,
                r#north: true,
                r#east: false,
            });
        }
        if state_id == 11612 {
            return Some(BlueStainedGlassPane {
                r#east: true,
                r#north: true,
                r#south: true,
                r#waterlogged: false,
                r#west: true,
            });
        }
        if state_id == 11622 {
            return Some(BlueStainedGlassPane {
                r#waterlogged: true,
                r#west: true,
                r#east: true,
                r#south: false,
                r#north: false,
            });
        }
        if state_id == 11616 {
            return Some(BlueStainedGlassPane {
                r#north: true,
                r#waterlogged: false,
                r#west: true,
                r#east: true,
                r#south: false,
            });
        }
        if state_id == 11641 {
            return Some(BlueStainedGlassPane {
                r#west: false,
                r#east: false,
                r#south: false,
                r#waterlogged: false,
                r#north: false,
            });
        }
        if state_id == 11620 {
            return Some(BlueStainedGlassPane {
                r#east: true,
                r#west: true,
                r#south: true,
                r#north: false,
                r#waterlogged: false,
            });
        }
        if state_id == 11625 {
            return Some(BlueStainedGlassPane {
                r#waterlogged: false,
                r#west: false,
                r#east: true,
                r#north: false,
                r#south: false,
            });
        }
        if state_id == 11615 {
            return Some(BlueStainedGlassPane {
                r#south: false,
                r#north: true,
                r#waterlogged: true,
                r#east: true,
                r#west: false,
            });
        }
        if state_id == 11636 {
            return Some(BlueStainedGlassPane {
                r#south: true,
                r#east: false,
                r#north: false,
                r#waterlogged: false,
                r#west: true,
            });
        }
        if state_id == 11613 {
            return Some(BlueStainedGlassPane {
                r#north: true,
                r#west: false,
                r#east: true,
                r#south: true,
                r#waterlogged: false,
            });
        }
        if state_id == 11632 {
            return Some(BlueStainedGlassPane {
                r#east: false,
                r#waterlogged: false,
                r#north: true,
                r#south: false,
                r#west: true,
            });
        }
        if state_id == 11631 {
            return Some(BlueStainedGlassPane {
                r#waterlogged: true,
                r#west: false,
                r#north: true,
                r#south: false,
                r#east: false,
            });
        }
        if state_id == 11617 {
            return Some(BlueStainedGlassPane {
                r#north: true,
                r#south: false,
                r#east: true,
                r#waterlogged: false,
                r#west: false,
            });
        }
        if state_id == 11611 {
            return Some(BlueStainedGlassPane {
                r#north: true,
                r#south: true,
                r#waterlogged: true,
                r#west: false,
                r#east: true,
            });
        }
        if state_id == 11610 {
            return Some(BlueStainedGlassPane {
                r#north: true,
                r#waterlogged: true,
                r#west: true,
                r#east: true,
                r#south: true,
            });
        }
        if state_id == 11637 {
            return Some(BlueStainedGlassPane {
                r#waterlogged: false,
                r#west: false,
                r#south: true,
                r#north: false,
                r#east: false,
            });
        }
        if state_id == 11618 {
            return Some(BlueStainedGlassPane {
                r#west: true,
                r#south: true,
                r#north: false,
                r#east: true,
                r#waterlogged: true,
            });
        }
        if state_id == 11626 {
            return Some(BlueStainedGlassPane {
                r#waterlogged: true,
                r#east: false,
                r#north: true,
                r#south: true,
                r#west: true,
            });
        }
        if state_id == 11614 {
            return Some(BlueStainedGlassPane {
                r#east: true,
                r#waterlogged: true,
                r#north: true,
                r#south: false,
                r#west: true,
            });
        }
        if state_id == 11634 {
            return Some(BlueStainedGlassPane {
                r#north: false,
                r#waterlogged: true,
                r#east: false,
                r#west: true,
                r#south: true,
            });
        }
        if state_id == 11635 {
            return Some(BlueStainedGlassPane {
                r#waterlogged: true,
                r#north: false,
                r#west: false,
                r#south: true,
                r#east: false,
            });
        }
        if state_id == 11638 {
            return Some(BlueStainedGlassPane {
                r#north: false,
                r#waterlogged: true,
                r#west: true,
                r#south: false,
                r#east: false,
            });
        }
        if state_id == 11627 {
            return Some(BlueStainedGlassPane {
                r#east: false,
                r#north: true,
                r#waterlogged: true,
                r#west: false,
                r#south: true,
            });
        }
        if state_id == 11639 {
            return Some(BlueStainedGlassPane {
                r#north: false,
                r#west: false,
                r#south: false,
                r#east: false,
                r#waterlogged: true,
            });
        }
        if state_id == 11630 {
            return Some(BlueStainedGlassPane {
                r#east: false,
                r#north: true,
                r#west: true,
                r#south: false,
                r#waterlogged: true,
            });
        }
        if state_id == 11621 {
            return Some(BlueStainedGlassPane {
                r#south: true,
                r#west: false,
                r#east: true,
                r#waterlogged: false,
                r#north: false,
            });
        }
        if state_id == 11623 {
            return Some(BlueStainedGlassPane {
                r#north: false,
                r#waterlogged: true,
                r#south: false,
                r#west: false,
                r#east: true,
            });
        }
        if state_id == 11628 {
            return Some(BlueStainedGlassPane {
                r#south: true,
                r#north: true,
                r#west: true,
                r#waterlogged: false,
                r#east: false,
            });
        }
        if state_id == 11640 {
            return Some(BlueStainedGlassPane {
                r#waterlogged: false,
                r#west: true,
                r#east: false,
                r#north: false,
                r#south: false,
            });
        }
        if state_id == 11633 {
            return Some(BlueStainedGlassPane {
                r#west: false,
                r#south: false,
                r#waterlogged: false,
                r#east: false,
                r#north: true,
            });
        }
        return None;
    }
}

