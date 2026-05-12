use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpruceFence {
    pub west: bool,
    pub east: bool,
    pub south: bool,
    pub north: bool,
    pub waterlogged: bool,
}


impl BlockState for SpruceFence {
    fn to_id(&self) -> i32 {
        if self.r#east == false && self.r#north == false && self.r#south == true && self.r#west == true && self.r#waterlogged == true { return 13594; }
        if self.r#north == false && self.r#west == false && self.r#waterlogged == true && self.r#east == true && self.r#south == false { return 13583; }
        if self.r#west == true && self.r#east == false && self.r#waterlogged == false && self.r#north == false && self.r#south == false { return 13600; }
        if self.r#south == false && self.r#north == true && self.r#east == true && self.r#west == false && self.r#waterlogged == false { return 13577; }
        if self.r#south == true && self.r#waterlogged == false && self.r#west == true && self.r#north == true && self.r#east == true { return 13572; }
        if self.r#east == true && self.r#west == true && self.r#waterlogged == true && self.r#north == true && self.r#south == true { return 13570; }
        if self.r#east == true && self.r#south == true && self.r#waterlogged == false && self.r#north == true && self.r#west == false { return 13573; }
        if self.r#north == true && self.r#south == false && self.r#east == false && self.r#waterlogged == true && self.r#west == true { return 13590; }
        if self.r#east == true && self.r#north == false && self.r#waterlogged == false && self.r#south == false && self.r#west == false { return 13585; }
        if self.r#north == false && self.r#west == false && self.r#south == false && self.r#east == false && self.r#waterlogged == false { return 13601; }
        if self.r#east == true && self.r#north == false && self.r#south == false && self.r#waterlogged == true && self.r#west == true { return 13582; }
        if self.r#waterlogged == true && self.r#east == true && self.r#south == false && self.r#north == true && self.r#west == true { return 13574; }
        if self.r#north == false && self.r#east == true && self.r#waterlogged == true && self.r#south == true && self.r#west == false { return 13579; }
        if self.r#east == false && self.r#north == false && self.r#west == false && self.r#south == false && self.r#waterlogged == true { return 13599; }
        if self.r#waterlogged == false && self.r#south == false && self.r#east == true && self.r#north == true && self.r#west == true { return 13576; }
        if self.r#north == false && self.r#waterlogged == true && self.r#west == true && self.r#south == true && self.r#east == true { return 13578; }
        if self.r#south == false && self.r#waterlogged == false && self.r#east == true && self.r#west == true && self.r#north == false { return 13584; }
        if self.r#waterlogged == false && self.r#west == true && self.r#east == false && self.r#south == false && self.r#north == true { return 13592; }
        if self.r#south == true && self.r#waterlogged == false && self.r#west == true && self.r#east == false && self.r#north == false { return 13596; }
        if self.r#west == false && self.r#waterlogged == true && self.r#east == true && self.r#north == true && self.r#south == false { return 13575; }
        if self.r#waterlogged == false && self.r#west == true && self.r#east == false && self.r#north == true && self.r#south == true { return 13588; }
        if self.r#east == false && self.r#west == false && self.r#north == true && self.r#waterlogged == false && self.r#south == false { return 13593; }
        if self.r#waterlogged == true && self.r#west == false && self.r#south == true && self.r#east == false && self.r#north == false { return 13595; }
        if self.r#west == true && self.r#east == false && self.r#north == false && self.r#waterlogged == true && self.r#south == false { return 13598; }
        if self.r#west == false && self.r#east == true && self.r#north == false && self.r#south == true && self.r#waterlogged == false { return 13581; }
        if self.r#north == true && self.r#waterlogged == true && self.r#south == false && self.r#west == false && self.r#east == false { return 13591; }
        if self.r#north == false && self.r#east == false && self.r#west == false && self.r#south == true && self.r#waterlogged == false { return 13597; }
        if self.r#west == true && self.r#waterlogged == false && self.r#east == true && self.r#north == false && self.r#south == true { return 13580; }
        if self.r#waterlogged == true && self.r#east == true && self.r#west == false && self.r#north == true && self.r#south == true { return 13571; }
        if self.r#east == false && self.r#north == true && self.r#south == true && self.r#west == true && self.r#waterlogged == true { return 13586; }
        if self.r#west == false && self.r#south == true && self.r#waterlogged == true && self.r#north == true && self.r#east == false { return 13587; }
        if self.r#west == false && self.r#north == true && self.r#waterlogged == false && self.r#south == true && self.r#east == false { return 13589; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 13594 {
            return Some(SpruceFence {
                r#east: false,
                r#north: false,
                r#south: true,
                r#west: true,
                r#waterlogged: true,
            });
        }
        if state_id == 13583 {
            return Some(SpruceFence {
                r#north: false,
                r#west: false,
                r#waterlogged: true,
                r#east: true,
                r#south: false,
            });
        }
        if state_id == 13600 {
            return Some(SpruceFence {
                r#west: true,
                r#east: false,
                r#waterlogged: false,
                r#north: false,
                r#south: false,
            });
        }
        if state_id == 13577 {
            return Some(SpruceFence {
                r#south: false,
                r#north: true,
                r#east: true,
                r#west: false,
                r#waterlogged: false,
            });
        }
        if state_id == 13572 {
            return Some(SpruceFence {
                r#south: true,
                r#waterlogged: false,
                r#west: true,
                r#north: true,
                r#east: true,
            });
        }
        if state_id == 13570 {
            return Some(SpruceFence {
                r#east: true,
                r#west: true,
                r#waterlogged: true,
                r#north: true,
                r#south: true,
            });
        }
        if state_id == 13573 {
            return Some(SpruceFence {
                r#east: true,
                r#south: true,
                r#waterlogged: false,
                r#north: true,
                r#west: false,
            });
        }
        if state_id == 13590 {
            return Some(SpruceFence {
                r#north: true,
                r#south: false,
                r#east: false,
                r#waterlogged: true,
                r#west: true,
            });
        }
        if state_id == 13585 {
            return Some(SpruceFence {
                r#east: true,
                r#north: false,
                r#waterlogged: false,
                r#south: false,
                r#west: false,
            });
        }
        if state_id == 13601 {
            return Some(SpruceFence {
                r#north: false,
                r#west: false,
                r#south: false,
                r#east: false,
                r#waterlogged: false,
            });
        }
        if state_id == 13582 {
            return Some(SpruceFence {
                r#east: true,
                r#north: false,
                r#south: false,
                r#waterlogged: true,
                r#west: true,
            });
        }
        if state_id == 13574 {
            return Some(SpruceFence {
                r#waterlogged: true,
                r#east: true,
                r#south: false,
                r#north: true,
                r#west: true,
            });
        }
        if state_id == 13579 {
            return Some(SpruceFence {
                r#north: false,
                r#east: true,
                r#waterlogged: true,
                r#south: true,
                r#west: false,
            });
        }
        if state_id == 13599 {
            return Some(SpruceFence {
                r#east: false,
                r#north: false,
                r#west: false,
                r#south: false,
                r#waterlogged: true,
            });
        }
        if state_id == 13576 {
            return Some(SpruceFence {
                r#waterlogged: false,
                r#south: false,
                r#east: true,
                r#north: true,
                r#west: true,
            });
        }
        if state_id == 13578 {
            return Some(SpruceFence {
                r#north: false,
                r#waterlogged: true,
                r#west: true,
                r#south: true,
                r#east: true,
            });
        }
        if state_id == 13584 {
            return Some(SpruceFence {
                r#south: false,
                r#waterlogged: false,
                r#east: true,
                r#west: true,
                r#north: false,
            });
        }
        if state_id == 13592 {
            return Some(SpruceFence {
                r#waterlogged: false,
                r#west: true,
                r#east: false,
                r#south: false,
                r#north: true,
            });
        }
        if state_id == 13596 {
            return Some(SpruceFence {
                r#south: true,
                r#waterlogged: false,
                r#west: true,
                r#east: false,
                r#north: false,
            });
        }
        if state_id == 13575 {
            return Some(SpruceFence {
                r#west: false,
                r#waterlogged: true,
                r#east: true,
                r#north: true,
                r#south: false,
            });
        }
        if state_id == 13588 {
            return Some(SpruceFence {
                r#waterlogged: false,
                r#west: true,
                r#east: false,
                r#north: true,
                r#south: true,
            });
        }
        if state_id == 13593 {
            return Some(SpruceFence {
                r#east: false,
                r#west: false,
                r#north: true,
                r#waterlogged: false,
                r#south: false,
            });
        }
        if state_id == 13595 {
            return Some(SpruceFence {
                r#waterlogged: true,
                r#west: false,
                r#south: true,
                r#east: false,
                r#north: false,
            });
        }
        if state_id == 13598 {
            return Some(SpruceFence {
                r#west: true,
                r#east: false,
                r#north: false,
                r#waterlogged: true,
                r#south: false,
            });
        }
        if state_id == 13581 {
            return Some(SpruceFence {
                r#west: false,
                r#east: true,
                r#north: false,
                r#south: true,
                r#waterlogged: false,
            });
        }
        if state_id == 13591 {
            return Some(SpruceFence {
                r#north: true,
                r#waterlogged: true,
                r#south: false,
                r#west: false,
                r#east: false,
            });
        }
        if state_id == 13597 {
            return Some(SpruceFence {
                r#north: false,
                r#east: false,
                r#west: false,
                r#south: true,
                r#waterlogged: false,
            });
        }
        if state_id == 13580 {
            return Some(SpruceFence {
                r#west: true,
                r#waterlogged: false,
                r#east: true,
                r#north: false,
                r#south: true,
            });
        }
        if state_id == 13571 {
            return Some(SpruceFence {
                r#waterlogged: true,
                r#east: true,
                r#west: false,
                r#north: true,
                r#south: true,
            });
        }
        if state_id == 13586 {
            return Some(SpruceFence {
                r#east: false,
                r#north: true,
                r#south: true,
                r#west: true,
                r#waterlogged: true,
            });
        }
        if state_id == 13587 {
            return Some(SpruceFence {
                r#west: false,
                r#south: true,
                r#waterlogged: true,
                r#north: true,
                r#east: false,
            });
        }
        if state_id == 13589 {
            return Some(SpruceFence {
                r#west: false,
                r#north: true,
                r#waterlogged: false,
                r#south: true,
                r#east: false,
            });
        }
        return None;
    }
}

