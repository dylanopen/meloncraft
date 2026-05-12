use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BirchFence {
    pub waterlogged: bool,
    pub west: bool,
    pub south: bool,
    pub east: bool,
    pub north: bool,
}


impl BlockState for BirchFence {
    fn to_id(&self) -> i32 {
        if self.r#west == true && self.r#north == false && self.r#waterlogged == true && self.r#south == false && self.r#east == false { return 13630; }
        if self.r#east == true && self.r#north == false && self.r#west == false && self.r#waterlogged == false && self.r#south == false { return 13617; }
        if self.r#waterlogged == false && self.r#north == true && self.r#east == false && self.r#west == true && self.r#south == false { return 13624; }
        if self.r#south == false && self.r#east == false && self.r#north == false && self.r#waterlogged == true && self.r#west == false { return 13631; }
        if self.r#waterlogged == false && self.r#east == false && self.r#north == false && self.r#south == true && self.r#west == true { return 13628; }
        if self.r#west == false && self.r#east == true && self.r#south == false && self.r#north == true && self.r#waterlogged == false { return 13609; }
        if self.r#waterlogged == true && self.r#east == false && self.r#south == true && self.r#west == true && self.r#north == false { return 13626; }
        if self.r#north == true && self.r#west == false && self.r#waterlogged == true && self.r#east == true && self.r#south == true { return 13603; }
        if self.r#east == true && self.r#north == false && self.r#waterlogged == false && self.r#west == false && self.r#south == true { return 13613; }
        if self.r#south == false && self.r#east == true && self.r#west == true && self.r#north == false && self.r#waterlogged == false { return 13616; }
        if self.r#east == false && self.r#north == true && self.r#waterlogged == false && self.r#west == false && self.r#south == true { return 13621; }
        if self.r#north == false && self.r#east == true && self.r#west == false && self.r#south == false && self.r#waterlogged == true { return 13615; }
        if self.r#north == true && self.r#south == true && self.r#waterlogged == false && self.r#west == true && self.r#east == false { return 13620; }
        if self.r#west == false && self.r#south == false && self.r#waterlogged == true && self.r#north == true && self.r#east == true { return 13607; }
        if self.r#west == true && self.r#south == false && self.r#east == false && self.r#north == true && self.r#waterlogged == true { return 13622; }
        if self.r#waterlogged == true && self.r#south == true && self.r#west == false && self.r#north == true && self.r#east == false { return 13619; }
        if self.r#waterlogged == false && self.r#north == false && self.r#east == false && self.r#west == false && self.r#south == false { return 13633; }
        if self.r#waterlogged == false && self.r#east == true && self.r#west == false && self.r#south == true && self.r#north == true { return 13605; }
        if self.r#waterlogged == true && self.r#south == true && self.r#north == false && self.r#east == true && self.r#west == false { return 13611; }
        if self.r#east == false && self.r#north == true && self.r#west == true && self.r#waterlogged == true && self.r#south == true { return 13618; }
        if self.r#waterlogged == true && self.r#west == true && self.r#east == true && self.r#north == false && self.r#south == false { return 13614; }
        if self.r#south == true && self.r#east == false && self.r#waterlogged == true && self.r#west == false && self.r#north == false { return 13627; }
        if self.r#east == true && self.r#waterlogged == true && self.r#north == true && self.r#south == false && self.r#west == true { return 13606; }
        if self.r#east == true && self.r#waterlogged == true && self.r#west == true && self.r#north == true && self.r#south == true { return 13602; }
        if self.r#south == true && self.r#waterlogged == false && self.r#east == false && self.r#north == false && self.r#west == false { return 13629; }
        if self.r#west == true && self.r#south == true && self.r#north == false && self.r#east == true && self.r#waterlogged == true { return 13610; }
        if self.r#east == true && self.r#north == true && self.r#south == true && self.r#waterlogged == false && self.r#west == true { return 13604; }
        if self.r#north == false && self.r#south == true && self.r#east == true && self.r#west == true && self.r#waterlogged == false { return 13612; }
        if self.r#east == false && self.r#waterlogged == false && self.r#west == true && self.r#north == false && self.r#south == false { return 13632; }
        if self.r#east == false && self.r#west == false && self.r#north == true && self.r#south == false && self.r#waterlogged == true { return 13623; }
        if self.r#east == false && self.r#waterlogged == false && self.r#south == false && self.r#west == false && self.r#north == true { return 13625; }
        if self.r#south == false && self.r#north == true && self.r#waterlogged == false && self.r#east == true && self.r#west == true { return 13608; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 13630 {
            return Some(BirchFence {
                r#west: true,
                r#north: false,
                r#waterlogged: true,
                r#south: false,
                r#east: false,
            });
        }
        if state_id == 13617 {
            return Some(BirchFence {
                r#east: true,
                r#north: false,
                r#west: false,
                r#waterlogged: false,
                r#south: false,
            });
        }
        if state_id == 13624 {
            return Some(BirchFence {
                r#waterlogged: false,
                r#north: true,
                r#east: false,
                r#west: true,
                r#south: false,
            });
        }
        if state_id == 13631 {
            return Some(BirchFence {
                r#south: false,
                r#east: false,
                r#north: false,
                r#waterlogged: true,
                r#west: false,
            });
        }
        if state_id == 13628 {
            return Some(BirchFence {
                r#waterlogged: false,
                r#east: false,
                r#north: false,
                r#south: true,
                r#west: true,
            });
        }
        if state_id == 13609 {
            return Some(BirchFence {
                r#west: false,
                r#east: true,
                r#south: false,
                r#north: true,
                r#waterlogged: false,
            });
        }
        if state_id == 13626 {
            return Some(BirchFence {
                r#waterlogged: true,
                r#east: false,
                r#south: true,
                r#west: true,
                r#north: false,
            });
        }
        if state_id == 13603 {
            return Some(BirchFence {
                r#north: true,
                r#west: false,
                r#waterlogged: true,
                r#east: true,
                r#south: true,
            });
        }
        if state_id == 13613 {
            return Some(BirchFence {
                r#east: true,
                r#north: false,
                r#waterlogged: false,
                r#west: false,
                r#south: true,
            });
        }
        if state_id == 13616 {
            return Some(BirchFence {
                r#south: false,
                r#east: true,
                r#west: true,
                r#north: false,
                r#waterlogged: false,
            });
        }
        if state_id == 13621 {
            return Some(BirchFence {
                r#east: false,
                r#north: true,
                r#waterlogged: false,
                r#west: false,
                r#south: true,
            });
        }
        if state_id == 13615 {
            return Some(BirchFence {
                r#north: false,
                r#east: true,
                r#west: false,
                r#south: false,
                r#waterlogged: true,
            });
        }
        if state_id == 13620 {
            return Some(BirchFence {
                r#north: true,
                r#south: true,
                r#waterlogged: false,
                r#west: true,
                r#east: false,
            });
        }
        if state_id == 13607 {
            return Some(BirchFence {
                r#west: false,
                r#south: false,
                r#waterlogged: true,
                r#north: true,
                r#east: true,
            });
        }
        if state_id == 13622 {
            return Some(BirchFence {
                r#west: true,
                r#south: false,
                r#east: false,
                r#north: true,
                r#waterlogged: true,
            });
        }
        if state_id == 13619 {
            return Some(BirchFence {
                r#waterlogged: true,
                r#south: true,
                r#west: false,
                r#north: true,
                r#east: false,
            });
        }
        if state_id == 13633 {
            return Some(BirchFence {
                r#waterlogged: false,
                r#north: false,
                r#east: false,
                r#west: false,
                r#south: false,
            });
        }
        if state_id == 13605 {
            return Some(BirchFence {
                r#waterlogged: false,
                r#east: true,
                r#west: false,
                r#south: true,
                r#north: true,
            });
        }
        if state_id == 13611 {
            return Some(BirchFence {
                r#waterlogged: true,
                r#south: true,
                r#north: false,
                r#east: true,
                r#west: false,
            });
        }
        if state_id == 13618 {
            return Some(BirchFence {
                r#east: false,
                r#north: true,
                r#west: true,
                r#waterlogged: true,
                r#south: true,
            });
        }
        if state_id == 13614 {
            return Some(BirchFence {
                r#waterlogged: true,
                r#west: true,
                r#east: true,
                r#north: false,
                r#south: false,
            });
        }
        if state_id == 13627 {
            return Some(BirchFence {
                r#south: true,
                r#east: false,
                r#waterlogged: true,
                r#west: false,
                r#north: false,
            });
        }
        if state_id == 13606 {
            return Some(BirchFence {
                r#east: true,
                r#waterlogged: true,
                r#north: true,
                r#south: false,
                r#west: true,
            });
        }
        if state_id == 13602 {
            return Some(BirchFence {
                r#east: true,
                r#waterlogged: true,
                r#west: true,
                r#north: true,
                r#south: true,
            });
        }
        if state_id == 13629 {
            return Some(BirchFence {
                r#south: true,
                r#waterlogged: false,
                r#east: false,
                r#north: false,
                r#west: false,
            });
        }
        if state_id == 13610 {
            return Some(BirchFence {
                r#west: true,
                r#south: true,
                r#north: false,
                r#east: true,
                r#waterlogged: true,
            });
        }
        if state_id == 13604 {
            return Some(BirchFence {
                r#east: true,
                r#north: true,
                r#south: true,
                r#waterlogged: false,
                r#west: true,
            });
        }
        if state_id == 13612 {
            return Some(BirchFence {
                r#north: false,
                r#south: true,
                r#east: true,
                r#west: true,
                r#waterlogged: false,
            });
        }
        if state_id == 13632 {
            return Some(BirchFence {
                r#east: false,
                r#waterlogged: false,
                r#west: true,
                r#north: false,
                r#south: false,
            });
        }
        if state_id == 13623 {
            return Some(BirchFence {
                r#east: false,
                r#west: false,
                r#north: true,
                r#south: false,
                r#waterlogged: true,
            });
        }
        if state_id == 13625 {
            return Some(BirchFence {
                r#east: false,
                r#waterlogged: false,
                r#south: false,
                r#west: false,
                r#north: true,
            });
        }
        if state_id == 13608 {
            return Some(BirchFence {
                r#south: false,
                r#north: true,
                r#waterlogged: false,
                r#east: true,
                r#west: true,
            });
        }
        return None;
    }
}

