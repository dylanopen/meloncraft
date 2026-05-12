use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BirchFence {
    pub east: bool,
    pub north: bool,
    pub south: bool,
    pub waterlogged: bool,
    pub west: bool,
}


impl BlockState for BirchFence {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == false && block_state.r#north == true && block_state.r#west == false && block_state.r#east == true && block_state.r#south == false { return 13609; }
        if block_state.r#north == false && block_state.r#south == false && block_state.r#waterlogged == true && block_state.r#west == false && block_state.r#east == true { return 13615; }
        if block_state.r#south == false && block_state.r#waterlogged == false && block_state.r#east == true && block_state.r#north == false && block_state.r#west == false { return 13617; }
        if block_state.r#north == true && block_state.r#waterlogged == true && block_state.r#east == false && block_state.r#south == false && block_state.r#west == false { return 13623; }
        if block_state.r#east == true && block_state.r#waterlogged == true && block_state.r#north == true && block_state.r#south == true && block_state.r#west == true { return 13602; }
        if block_state.r#east == false && block_state.r#north == false && block_state.r#south == true && block_state.r#waterlogged == true && block_state.r#west == true { return 13626; }
        if block_state.r#east == false && block_state.r#north == false && block_state.r#south == false && block_state.r#west == false && block_state.r#waterlogged == false { return 13633; }
        if block_state.r#west == true && block_state.r#north == false && block_state.r#waterlogged == false && block_state.r#south == false && block_state.r#east == false { return 13632; }
        if block_state.r#east == true && block_state.r#waterlogged == false && block_state.r#west == true && block_state.r#south == true && block_state.r#north == true { return 13604; }
        if block_state.r#waterlogged == true && block_state.r#south == false && block_state.r#west == true && block_state.r#north == true && block_state.r#east == false { return 13622; }
        if block_state.r#north == false && block_state.r#south == false && block_state.r#east == false && block_state.r#waterlogged == true && block_state.r#west == false { return 13631; }
        if block_state.r#east == true && block_state.r#west == false && block_state.r#north == false && block_state.r#south == true && block_state.r#waterlogged == false { return 13613; }
        if block_state.r#waterlogged == true && block_state.r#west == false && block_state.r#east == true && block_state.r#north == true && block_state.r#south == true { return 13603; }
        if block_state.r#north == true && block_state.r#south == false && block_state.r#waterlogged == false && block_state.r#east == false && block_state.r#west == false { return 13625; }
        if block_state.r#waterlogged == false && block_state.r#north == true && block_state.r#west == false && block_state.r#south == true && block_state.r#east == false { return 13621; }
        if block_state.r#waterlogged == true && block_state.r#south == false && block_state.r#north == true && block_state.r#east == true && block_state.r#west == true { return 13606; }
        if block_state.r#east == true && block_state.r#waterlogged == true && block_state.r#north == true && block_state.r#west == false && block_state.r#south == false { return 13607; }
        if block_state.r#east == true && block_state.r#south == true && block_state.r#north == false && block_state.r#waterlogged == true && block_state.r#west == true { return 13610; }
        if block_state.r#north == false && block_state.r#south == false && block_state.r#east == true && block_state.r#west == true && block_state.r#waterlogged == true { return 13614; }
        if block_state.r#east == false && block_state.r#west == true && block_state.r#north == true && block_state.r#waterlogged == true && block_state.r#south == true { return 13618; }
        if block_state.r#south == true && block_state.r#west == true && block_state.r#north == true && block_state.r#waterlogged == false && block_state.r#east == false { return 13620; }
        if block_state.r#west == true && block_state.r#waterlogged == false && block_state.r#north == false && block_state.r#east == true && block_state.r#south == true { return 13612; }
        if block_state.r#waterlogged == true && block_state.r#west == false && block_state.r#south == true && block_state.r#east == true && block_state.r#north == false { return 13611; }
        if block_state.r#east == false && block_state.r#north == true && block_state.r#south == false && block_state.r#waterlogged == false && block_state.r#west == true { return 13624; }
        if block_state.r#east == false && block_state.r#waterlogged == true && block_state.r#north == true && block_state.r#west == false && block_state.r#south == true { return 13619; }
        if block_state.r#east == false && block_state.r#waterlogged == true && block_state.r#west == false && block_state.r#north == false && block_state.r#south == true { return 13627; }
        if block_state.r#north == false && block_state.r#south == false && block_state.r#waterlogged == false && block_state.r#west == true && block_state.r#east == true { return 13616; }
        if block_state.r#waterlogged == false && block_state.r#north == false && block_state.r#west == true && block_state.r#south == true && block_state.r#east == false { return 13628; }
        if block_state.r#waterlogged == false && block_state.r#north == true && block_state.r#east == true && block_state.r#south == false && block_state.r#west == true { return 13608; }
        if block_state.r#waterlogged == true && block_state.r#west == true && block_state.r#east == false && block_state.r#south == false && block_state.r#north == false { return 13630; }
        if block_state.r#east == false && block_state.r#west == false && block_state.r#waterlogged == false && block_state.r#north == false && block_state.r#south == true { return 13629; }
        if block_state.r#south == true && block_state.r#north == true && block_state.r#waterlogged == false && block_state.r#east == true && block_state.r#west == false { return 13605; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 13609 {
            return Some(BirchFence {
                r#waterlogged: false,
                r#north: true,
                r#west: false,
                r#east: true,
                r#south: false,
            });
        }
        if state_id == 13615 {
            return Some(BirchFence {
                r#north: false,
                r#south: false,
                r#waterlogged: true,
                r#west: false,
                r#east: true,
            });
        }
        if state_id == 13617 {
            return Some(BirchFence {
                r#south: false,
                r#waterlogged: false,
                r#east: true,
                r#north: false,
                r#west: false,
            });
        }
        if state_id == 13623 {
            return Some(BirchFence {
                r#north: true,
                r#waterlogged: true,
                r#east: false,
                r#south: false,
                r#west: false,
            });
        }
        if state_id == 13602 {
            return Some(BirchFence {
                r#east: true,
                r#waterlogged: true,
                r#north: true,
                r#south: true,
                r#west: true,
            });
        }
        if state_id == 13626 {
            return Some(BirchFence {
                r#east: false,
                r#north: false,
                r#south: true,
                r#waterlogged: true,
                r#west: true,
            });
        }
        if state_id == 13633 {
            return Some(BirchFence {
                r#east: false,
                r#north: false,
                r#south: false,
                r#west: false,
                r#waterlogged: false,
            });
        }
        if state_id == 13632 {
            return Some(BirchFence {
                r#west: true,
                r#north: false,
                r#waterlogged: false,
                r#south: false,
                r#east: false,
            });
        }
        if state_id == 13604 {
            return Some(BirchFence {
                r#east: true,
                r#waterlogged: false,
                r#west: true,
                r#south: true,
                r#north: true,
            });
        }
        if state_id == 13622 {
            return Some(BirchFence {
                r#waterlogged: true,
                r#south: false,
                r#west: true,
                r#north: true,
                r#east: false,
            });
        }
        if state_id == 13631 {
            return Some(BirchFence {
                r#north: false,
                r#south: false,
                r#east: false,
                r#waterlogged: true,
                r#west: false,
            });
        }
        if state_id == 13613 {
            return Some(BirchFence {
                r#east: true,
                r#west: false,
                r#north: false,
                r#south: true,
                r#waterlogged: false,
            });
        }
        if state_id == 13603 {
            return Some(BirchFence {
                r#waterlogged: true,
                r#west: false,
                r#east: true,
                r#north: true,
                r#south: true,
            });
        }
        if state_id == 13625 {
            return Some(BirchFence {
                r#north: true,
                r#south: false,
                r#waterlogged: false,
                r#east: false,
                r#west: false,
            });
        }
        if state_id == 13621 {
            return Some(BirchFence {
                r#waterlogged: false,
                r#north: true,
                r#west: false,
                r#south: true,
                r#east: false,
            });
        }
        if state_id == 13606 {
            return Some(BirchFence {
                r#waterlogged: true,
                r#south: false,
                r#north: true,
                r#east: true,
                r#west: true,
            });
        }
        if state_id == 13607 {
            return Some(BirchFence {
                r#east: true,
                r#waterlogged: true,
                r#north: true,
                r#west: false,
                r#south: false,
            });
        }
        if state_id == 13610 {
            return Some(BirchFence {
                r#east: true,
                r#south: true,
                r#north: false,
                r#waterlogged: true,
                r#west: true,
            });
        }
        if state_id == 13614 {
            return Some(BirchFence {
                r#north: false,
                r#south: false,
                r#east: true,
                r#west: true,
                r#waterlogged: true,
            });
        }
        if state_id == 13618 {
            return Some(BirchFence {
                r#east: false,
                r#west: true,
                r#north: true,
                r#waterlogged: true,
                r#south: true,
            });
        }
        if state_id == 13620 {
            return Some(BirchFence {
                r#south: true,
                r#west: true,
                r#north: true,
                r#waterlogged: false,
                r#east: false,
            });
        }
        if state_id == 13612 {
            return Some(BirchFence {
                r#west: true,
                r#waterlogged: false,
                r#north: false,
                r#east: true,
                r#south: true,
            });
        }
        if state_id == 13611 {
            return Some(BirchFence {
                r#waterlogged: true,
                r#west: false,
                r#south: true,
                r#east: true,
                r#north: false,
            });
        }
        if state_id == 13624 {
            return Some(BirchFence {
                r#east: false,
                r#north: true,
                r#south: false,
                r#waterlogged: false,
                r#west: true,
            });
        }
        if state_id == 13619 {
            return Some(BirchFence {
                r#east: false,
                r#waterlogged: true,
                r#north: true,
                r#west: false,
                r#south: true,
            });
        }
        if state_id == 13627 {
            return Some(BirchFence {
                r#east: false,
                r#waterlogged: true,
                r#west: false,
                r#north: false,
                r#south: true,
            });
        }
        if state_id == 13616 {
            return Some(BirchFence {
                r#north: false,
                r#south: false,
                r#waterlogged: false,
                r#west: true,
                r#east: true,
            });
        }
        if state_id == 13628 {
            return Some(BirchFence {
                r#waterlogged: false,
                r#north: false,
                r#west: true,
                r#south: true,
                r#east: false,
            });
        }
        if state_id == 13608 {
            return Some(BirchFence {
                r#waterlogged: false,
                r#north: true,
                r#east: true,
                r#south: false,
                r#west: true,
            });
        }
        if state_id == 13630 {
            return Some(BirchFence {
                r#waterlogged: true,
                r#west: true,
                r#east: false,
                r#south: false,
                r#north: false,
            });
        }
        if state_id == 13629 {
            return Some(BirchFence {
                r#east: false,
                r#west: false,
                r#waterlogged: false,
                r#north: false,
                r#south: true,
            });
        }
        if state_id == 13605 {
            return Some(BirchFence {
                r#south: true,
                r#north: true,
                r#waterlogged: false,
                r#east: true,
                r#west: false,
            });
        }
        return None;
    }
}

