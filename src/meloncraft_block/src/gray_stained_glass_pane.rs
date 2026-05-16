use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GrayStainedGlassPane {
    pub waterlogged: bool,
    pub south: bool,
    pub north: bool,
    pub east: bool,
    pub west: bool,
}


impl BlockState for GrayStainedGlassPane {
    fn to_id(&self) -> i32 {
        if self.r#south == false && self.r#waterlogged == false && self.r#north == false && self.r#east == true && self.r#west == true { return 11496; }
        if self.r#west == true && self.r#south == true && self.r#east == false && self.r#north == true && self.r#waterlogged == false { return 11500; }
        if self.r#west == true && self.r#east == false && self.r#north == true && self.r#waterlogged == true && self.r#south == false { return 11502; }
        if self.r#south == true && self.r#waterlogged == false && self.r#east == false && self.r#west == true && self.r#north == false { return 11508; }
        if self.r#north == false && self.r#waterlogged == false && self.r#east == true && self.r#south == false && self.r#west == false { return 11497; }
        if self.r#south == true && self.r#west == false && self.r#north == false && self.r#east == false && self.r#waterlogged == false { return 11509; }
        if self.r#south == false && self.r#waterlogged == true && self.r#north == true && self.r#west == true && self.r#east == true { return 11486; }
        if self.r#waterlogged == true && self.r#west == false && self.r#north == true && self.r#south == false && self.r#east == true { return 11487; }
        if self.r#east == false && self.r#south == false && self.r#north == false && self.r#west == true && self.r#waterlogged == true { return 11510; }
        if self.r#east == false && self.r#south == true && self.r#north == true && self.r#waterlogged == true && self.r#west == true { return 11498; }
        if self.r#north == true && self.r#waterlogged == false && self.r#east == false && self.r#south == false && self.r#west == false { return 11505; }
        if self.r#east == true && self.r#north == false && self.r#south == false && self.r#waterlogged == true && self.r#west == true { return 11494; }
        if self.r#west == true && self.r#north == false && self.r#south == false && self.r#east == false && self.r#waterlogged == false { return 11512; }
        if self.r#waterlogged == true && self.r#north == true && self.r#south == true && self.r#west == false && self.r#east == false { return 11499; }
        if self.r#waterlogged == true && self.r#north == true && self.r#east == true && self.r#south == true && self.r#west == true { return 11482; }
        if self.r#east == false && self.r#waterlogged == false && self.r#west == true && self.r#south == false && self.r#north == true { return 11504; }
        if self.r#waterlogged == false && self.r#south == false && self.r#north == true && self.r#west == true && self.r#east == true { return 11488; }
        if self.r#west == true && self.r#east == false && self.r#waterlogged == true && self.r#north == false && self.r#south == true { return 11506; }
        if self.r#south == false && self.r#west == false && self.r#north == true && self.r#east == true && self.r#waterlogged == false { return 11489; }
        if self.r#north == false && self.r#west == false && self.r#south == false && self.r#east == false && self.r#waterlogged == false { return 11513; }
        if self.r#south == true && self.r#east == true && self.r#north == true && self.r#waterlogged == false && self.r#west == true { return 11484; }
        if self.r#east == true && self.r#south == true && self.r#waterlogged == true && self.r#west == false && self.r#north == false { return 11491; }
        if self.r#west == false && self.r#waterlogged == true && self.r#east == true && self.r#north == true && self.r#south == true { return 11483; }
        if self.r#south == true && self.r#west == true && self.r#east == true && self.r#north == false && self.r#waterlogged == true { return 11490; }
        if self.r#north == false && self.r#south == false && self.r#waterlogged == true && self.r#west == false && self.r#east == true { return 11495; }
        if self.r#east == false && self.r#waterlogged == true && self.r#west == false && self.r#south == false && self.r#north == true { return 11503; }
        if self.r#north == false && self.r#south == true && self.r#west == false && self.r#east == true && self.r#waterlogged == false { return 11493; }
        if self.r#east == false && self.r#waterlogged == true && self.r#west == false && self.r#north == false && self.r#south == false { return 11511; }
        if self.r#south == true && self.r#west == true && self.r#waterlogged == false && self.r#north == false && self.r#east == true { return 11492; }
        if self.r#south == true && self.r#west == false && self.r#east == false && self.r#north == false && self.r#waterlogged == true { return 11507; }
        if self.r#waterlogged == false && self.r#north == true && self.r#east == true && self.r#west == false && self.r#south == true { return 11485; }
        if self.r#east == false && self.r#west == false && self.r#waterlogged == false && self.r#south == true && self.r#north == true { return 11501; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 11496 {
            return Some(GrayStainedGlassPane {
                r#south: false,
                r#waterlogged: false,
                r#north: false,
                r#east: true,
                r#west: true,
            });
        }
        if state_id == 11500 {
            return Some(GrayStainedGlassPane {
                r#west: true,
                r#south: true,
                r#east: false,
                r#north: true,
                r#waterlogged: false,
            });
        }
        if state_id == 11502 {
            return Some(GrayStainedGlassPane {
                r#west: true,
                r#east: false,
                r#north: true,
                r#waterlogged: true,
                r#south: false,
            });
        }
        if state_id == 11508 {
            return Some(GrayStainedGlassPane {
                r#south: true,
                r#waterlogged: false,
                r#east: false,
                r#west: true,
                r#north: false,
            });
        }
        if state_id == 11497 {
            return Some(GrayStainedGlassPane {
                r#north: false,
                r#waterlogged: false,
                r#east: true,
                r#south: false,
                r#west: false,
            });
        }
        if state_id == 11509 {
            return Some(GrayStainedGlassPane {
                r#south: true,
                r#west: false,
                r#north: false,
                r#east: false,
                r#waterlogged: false,
            });
        }
        if state_id == 11486 {
            return Some(GrayStainedGlassPane {
                r#south: false,
                r#waterlogged: true,
                r#north: true,
                r#west: true,
                r#east: true,
            });
        }
        if state_id == 11487 {
            return Some(GrayStainedGlassPane {
                r#waterlogged: true,
                r#west: false,
                r#north: true,
                r#south: false,
                r#east: true,
            });
        }
        if state_id == 11510 {
            return Some(GrayStainedGlassPane {
                r#east: false,
                r#south: false,
                r#north: false,
                r#west: true,
                r#waterlogged: true,
            });
        }
        if state_id == 11498 {
            return Some(GrayStainedGlassPane {
                r#east: false,
                r#south: true,
                r#north: true,
                r#waterlogged: true,
                r#west: true,
            });
        }
        if state_id == 11505 {
            return Some(GrayStainedGlassPane {
                r#north: true,
                r#waterlogged: false,
                r#east: false,
                r#south: false,
                r#west: false,
            });
        }
        if state_id == 11494 {
            return Some(GrayStainedGlassPane {
                r#east: true,
                r#north: false,
                r#south: false,
                r#waterlogged: true,
                r#west: true,
            });
        }
        if state_id == 11512 {
            return Some(GrayStainedGlassPane {
                r#west: true,
                r#north: false,
                r#south: false,
                r#east: false,
                r#waterlogged: false,
            });
        }
        if state_id == 11499 {
            return Some(GrayStainedGlassPane {
                r#waterlogged: true,
                r#north: true,
                r#south: true,
                r#west: false,
                r#east: false,
            });
        }
        if state_id == 11482 {
            return Some(GrayStainedGlassPane {
                r#waterlogged: true,
                r#north: true,
                r#east: true,
                r#south: true,
                r#west: true,
            });
        }
        if state_id == 11504 {
            return Some(GrayStainedGlassPane {
                r#east: false,
                r#waterlogged: false,
                r#west: true,
                r#south: false,
                r#north: true,
            });
        }
        if state_id == 11488 {
            return Some(GrayStainedGlassPane {
                r#waterlogged: false,
                r#south: false,
                r#north: true,
                r#west: true,
                r#east: true,
            });
        }
        if state_id == 11506 {
            return Some(GrayStainedGlassPane {
                r#west: true,
                r#east: false,
                r#waterlogged: true,
                r#north: false,
                r#south: true,
            });
        }
        if state_id == 11489 {
            return Some(GrayStainedGlassPane {
                r#south: false,
                r#west: false,
                r#north: true,
                r#east: true,
                r#waterlogged: false,
            });
        }
        if state_id == 11513 {
            return Some(GrayStainedGlassPane {
                r#north: false,
                r#west: false,
                r#south: false,
                r#east: false,
                r#waterlogged: false,
            });
        }
        if state_id == 11484 {
            return Some(GrayStainedGlassPane {
                r#south: true,
                r#east: true,
                r#north: true,
                r#waterlogged: false,
                r#west: true,
            });
        }
        if state_id == 11491 {
            return Some(GrayStainedGlassPane {
                r#east: true,
                r#south: true,
                r#waterlogged: true,
                r#west: false,
                r#north: false,
            });
        }
        if state_id == 11483 {
            return Some(GrayStainedGlassPane {
                r#west: false,
                r#waterlogged: true,
                r#east: true,
                r#north: true,
                r#south: true,
            });
        }
        if state_id == 11490 {
            return Some(GrayStainedGlassPane {
                r#south: true,
                r#west: true,
                r#east: true,
                r#north: false,
                r#waterlogged: true,
            });
        }
        if state_id == 11495 {
            return Some(GrayStainedGlassPane {
                r#north: false,
                r#south: false,
                r#waterlogged: true,
                r#west: false,
                r#east: true,
            });
        }
        if state_id == 11503 {
            return Some(GrayStainedGlassPane {
                r#east: false,
                r#waterlogged: true,
                r#west: false,
                r#south: false,
                r#north: true,
            });
        }
        if state_id == 11493 {
            return Some(GrayStainedGlassPane {
                r#north: false,
                r#south: true,
                r#west: false,
                r#east: true,
                r#waterlogged: false,
            });
        }
        if state_id == 11511 {
            return Some(GrayStainedGlassPane {
                r#east: false,
                r#waterlogged: true,
                r#west: false,
                r#north: false,
                r#south: false,
            });
        }
        if state_id == 11492 {
            return Some(GrayStainedGlassPane {
                r#south: true,
                r#west: true,
                r#waterlogged: false,
                r#north: false,
                r#east: true,
            });
        }
        if state_id == 11507 {
            return Some(GrayStainedGlassPane {
                r#south: true,
                r#west: false,
                r#east: false,
                r#north: false,
                r#waterlogged: true,
            });
        }
        if state_id == 11485 {
            return Some(GrayStainedGlassPane {
                r#waterlogged: false,
                r#north: true,
                r#east: true,
                r#west: false,
                r#south: true,
            });
        }
        if state_id == 11501 {
            return Some(GrayStainedGlassPane {
                r#east: false,
                r#west: false,
                r#waterlogged: false,
                r#south: true,
                r#north: true,
            });
        }
        return None;
    }
}

