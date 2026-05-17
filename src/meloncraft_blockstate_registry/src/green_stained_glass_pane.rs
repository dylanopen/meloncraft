use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GreenStainedGlassPane {
    pub west: bool,
    pub waterlogged: bool,
    pub east: bool,
    pub north: bool,
    pub south: bool,
}


impl BlockState for GreenStainedGlassPane {
    fn to_id(&self) -> i32 {
        if self.r#east == true && self.r#south == false && self.r#west == false && self.r#waterlogged == true && self.r#north == true { return 11679; }
        if self.r#west == false && self.r#east == true && self.r#waterlogged == false && self.r#north == true && self.r#south == true { return 11677; }
        if self.r#south == false && self.r#east == false && self.r#north == false && self.r#west == true && self.r#waterlogged == true { return 11702; }
        if self.r#waterlogged == false && self.r#east == true && self.r#north == false && self.r#west == false && self.r#south == false { return 11689; }
        if self.r#east == false && self.r#north == true && self.r#waterlogged == false && self.r#west == false && self.r#south == true { return 11693; }
        if self.r#north == false && self.r#waterlogged == false && self.r#south == true && self.r#east == false && self.r#west == true { return 11700; }
        if self.r#north == false && self.r#east == false && self.r#west == false && self.r#south == true && self.r#waterlogged == false { return 11701; }
        if self.r#south == false && self.r#east == false && self.r#waterlogged == false && self.r#west == true && self.r#north == false { return 11704; }
        if self.r#south == true && self.r#east == true && self.r#north == false && self.r#west == true && self.r#waterlogged == false { return 11684; }
        if self.r#north == false && self.r#east == true && self.r#waterlogged == false && self.r#west == false && self.r#south == true { return 11685; }
        if self.r#north == false && self.r#south == false && self.r#east == true && self.r#waterlogged == true && self.r#west == false { return 11687; }
        if self.r#south == true && self.r#north == true && self.r#waterlogged == false && self.r#west == true && self.r#east == true { return 11676; }
        if self.r#east == false && self.r#west == true && self.r#north == false && self.r#waterlogged == true && self.r#south == true { return 11698; }
        if self.r#east == false && self.r#north == false && self.r#waterlogged == true && self.r#west == false && self.r#south == true { return 11699; }
        if self.r#west == true && self.r#south == false && self.r#east == true && self.r#north == true && self.r#waterlogged == true { return 11678; }
        if self.r#north == true && self.r#south == true && self.r#waterlogged == false && self.r#west == true && self.r#east == false { return 11692; }
        if self.r#waterlogged == true && self.r#south == false && self.r#west == true && self.r#north == false && self.r#east == true { return 11686; }
        if self.r#east == true && self.r#north == true && self.r#waterlogged == true && self.r#south == true && self.r#west == false { return 11675; }
        if self.r#east == true && self.r#west == true && self.r#north == false && self.r#south == false && self.r#waterlogged == false { return 11688; }
        if self.r#south == false && self.r#waterlogged == true && self.r#west == true && self.r#east == false && self.r#north == true { return 11694; }
        if self.r#east == true && self.r#west == false && self.r#north == true && self.r#waterlogged == false && self.r#south == false { return 11681; }
        if self.r#west == false && self.r#north == true && self.r#south == true && self.r#east == false && self.r#waterlogged == true { return 11691; }
        if self.r#west == true && self.r#east == true && self.r#north == true && self.r#waterlogged == true && self.r#south == true { return 11674; }
        if self.r#north == false && self.r#south == false && self.r#waterlogged == false && self.r#west == false && self.r#east == false { return 11705; }
        if self.r#south == false && self.r#west == true && self.r#east == true && self.r#north == true && self.r#waterlogged == false { return 11680; }
        if self.r#west == false && self.r#north == true && self.r#east == false && self.r#south == false && self.r#waterlogged == true { return 11695; }
        if self.r#east == true && self.r#waterlogged == true && self.r#south == true && self.r#west == false && self.r#north == false { return 11683; }
        if self.r#east == false && self.r#south == true && self.r#north == true && self.r#waterlogged == true && self.r#west == true { return 11690; }
        if self.r#north == true && self.r#west == false && self.r#east == false && self.r#south == false && self.r#waterlogged == false { return 11697; }
        if self.r#east == false && self.r#west == false && self.r#north == false && self.r#south == false && self.r#waterlogged == true { return 11703; }
        if self.r#south == true && self.r#waterlogged == true && self.r#north == false && self.r#east == true && self.r#west == true { return 11682; }
        if self.r#east == false && self.r#south == false && self.r#north == true && self.r#west == true && self.r#waterlogged == false { return 11696; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 11679 {
            return Some(GreenStainedGlassPane {
                r#east: true,
                r#south: false,
                r#west: false,
                r#waterlogged: true,
                r#north: true,
            });
        }
        if state_id == 11677 {
            return Some(GreenStainedGlassPane {
                r#west: false,
                r#east: true,
                r#waterlogged: false,
                r#north: true,
                r#south: true,
            });
        }
        if state_id == 11702 {
            return Some(GreenStainedGlassPane {
                r#south: false,
                r#east: false,
                r#north: false,
                r#west: true,
                r#waterlogged: true,
            });
        }
        if state_id == 11689 {
            return Some(GreenStainedGlassPane {
                r#waterlogged: false,
                r#east: true,
                r#north: false,
                r#west: false,
                r#south: false,
            });
        }
        if state_id == 11693 {
            return Some(GreenStainedGlassPane {
                r#east: false,
                r#north: true,
                r#waterlogged: false,
                r#west: false,
                r#south: true,
            });
        }
        if state_id == 11700 {
            return Some(GreenStainedGlassPane {
                r#north: false,
                r#waterlogged: false,
                r#south: true,
                r#east: false,
                r#west: true,
            });
        }
        if state_id == 11701 {
            return Some(GreenStainedGlassPane {
                r#north: false,
                r#east: false,
                r#west: false,
                r#south: true,
                r#waterlogged: false,
            });
        }
        if state_id == 11704 {
            return Some(GreenStainedGlassPane {
                r#south: false,
                r#east: false,
                r#waterlogged: false,
                r#west: true,
                r#north: false,
            });
        }
        if state_id == 11684 {
            return Some(GreenStainedGlassPane {
                r#south: true,
                r#east: true,
                r#north: false,
                r#west: true,
                r#waterlogged: false,
            });
        }
        if state_id == 11685 {
            return Some(GreenStainedGlassPane {
                r#north: false,
                r#east: true,
                r#waterlogged: false,
                r#west: false,
                r#south: true,
            });
        }
        if state_id == 11687 {
            return Some(GreenStainedGlassPane {
                r#north: false,
                r#south: false,
                r#east: true,
                r#waterlogged: true,
                r#west: false,
            });
        }
        if state_id == 11676 {
            return Some(GreenStainedGlassPane {
                r#south: true,
                r#north: true,
                r#waterlogged: false,
                r#west: true,
                r#east: true,
            });
        }
        if state_id == 11698 {
            return Some(GreenStainedGlassPane {
                r#east: false,
                r#west: true,
                r#north: false,
                r#waterlogged: true,
                r#south: true,
            });
        }
        if state_id == 11699 {
            return Some(GreenStainedGlassPane {
                r#east: false,
                r#north: false,
                r#waterlogged: true,
                r#west: false,
                r#south: true,
            });
        }
        if state_id == 11678 {
            return Some(GreenStainedGlassPane {
                r#west: true,
                r#south: false,
                r#east: true,
                r#north: true,
                r#waterlogged: true,
            });
        }
        if state_id == 11692 {
            return Some(GreenStainedGlassPane {
                r#north: true,
                r#south: true,
                r#waterlogged: false,
                r#west: true,
                r#east: false,
            });
        }
        if state_id == 11686 {
            return Some(GreenStainedGlassPane {
                r#waterlogged: true,
                r#south: false,
                r#west: true,
                r#north: false,
                r#east: true,
            });
        }
        if state_id == 11675 {
            return Some(GreenStainedGlassPane {
                r#east: true,
                r#north: true,
                r#waterlogged: true,
                r#south: true,
                r#west: false,
            });
        }
        if state_id == 11688 {
            return Some(GreenStainedGlassPane {
                r#east: true,
                r#west: true,
                r#north: false,
                r#south: false,
                r#waterlogged: false,
            });
        }
        if state_id == 11694 {
            return Some(GreenStainedGlassPane {
                r#south: false,
                r#waterlogged: true,
                r#west: true,
                r#east: false,
                r#north: true,
            });
        }
        if state_id == 11681 {
            return Some(GreenStainedGlassPane {
                r#east: true,
                r#west: false,
                r#north: true,
                r#waterlogged: false,
                r#south: false,
            });
        }
        if state_id == 11691 {
            return Some(GreenStainedGlassPane {
                r#west: false,
                r#north: true,
                r#south: true,
                r#east: false,
                r#waterlogged: true,
            });
        }
        if state_id == 11674 {
            return Some(GreenStainedGlassPane {
                r#west: true,
                r#east: true,
                r#north: true,
                r#waterlogged: true,
                r#south: true,
            });
        }
        if state_id == 11705 {
            return Some(GreenStainedGlassPane {
                r#north: false,
                r#south: false,
                r#waterlogged: false,
                r#west: false,
                r#east: false,
            });
        }
        if state_id == 11680 {
            return Some(GreenStainedGlassPane {
                r#south: false,
                r#west: true,
                r#east: true,
                r#north: true,
                r#waterlogged: false,
            });
        }
        if state_id == 11695 {
            return Some(GreenStainedGlassPane {
                r#west: false,
                r#north: true,
                r#east: false,
                r#south: false,
                r#waterlogged: true,
            });
        }
        if state_id == 11683 {
            return Some(GreenStainedGlassPane {
                r#east: true,
                r#waterlogged: true,
                r#south: true,
                r#west: false,
                r#north: false,
            });
        }
        if state_id == 11690 {
            return Some(GreenStainedGlassPane {
                r#east: false,
                r#south: true,
                r#north: true,
                r#waterlogged: true,
                r#west: true,
            });
        }
        if state_id == 11697 {
            return Some(GreenStainedGlassPane {
                r#north: true,
                r#west: false,
                r#east: false,
                r#south: false,
                r#waterlogged: false,
            });
        }
        if state_id == 11703 {
            return Some(GreenStainedGlassPane {
                r#east: false,
                r#west: false,
                r#north: false,
                r#south: false,
                r#waterlogged: true,
            });
        }
        if state_id == 11682 {
            return Some(GreenStainedGlassPane {
                r#south: true,
                r#waterlogged: true,
                r#north: false,
                r#east: true,
                r#west: true,
            });
        }
        if state_id == 11696 {
            return Some(GreenStainedGlassPane {
                r#east: false,
                r#south: false,
                r#north: true,
                r#west: true,
                r#waterlogged: false,
            });
        }
        return None;
    }
}

