use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PaleOakSign {
    pub waterlogged: bool,
    pub rotation: i32,
}


impl BlockState for PaleOakSign {
    fn to_id(&self) -> i32 {
        if self.r#rotation == 9 && self.r#waterlogged == true { return 5376; }
        if self.r#waterlogged == true && self.r#rotation == 4 { return 5366; }
        if self.r#rotation == 11 && self.r#waterlogged == false { return 5381; }
        if self.r#rotation == 0 && self.r#waterlogged == false { return 5359; }
        if self.r#waterlogged == true && self.r#rotation == 7 { return 5372; }
        if self.r#waterlogged == false && self.r#rotation == 7 { return 5373; }
        if self.r#rotation == 0 && self.r#waterlogged == true { return 5358; }
        if self.r#rotation == 6 && self.r#waterlogged == true { return 5370; }
        if self.r#waterlogged == true && self.r#rotation == 12 { return 5382; }
        if self.r#waterlogged == false && self.r#rotation == 12 { return 5383; }
        if self.r#waterlogged == false && self.r#rotation == 1 { return 5361; }
        if self.r#rotation == 2 && self.r#waterlogged == false { return 5363; }
        if self.r#rotation == 10 && self.r#waterlogged == false { return 5379; }
        if self.r#rotation == 13 && self.r#waterlogged == true { return 5384; }
        if self.r#rotation == 8 && self.r#waterlogged == true { return 5374; }
        if self.r#rotation == 4 && self.r#waterlogged == false { return 5367; }
        if self.r#rotation == 5 && self.r#waterlogged == true { return 5368; }
        if self.r#waterlogged == true && self.r#rotation == 11 { return 5380; }
        if self.r#rotation == 8 && self.r#waterlogged == false { return 5375; }
        if self.r#rotation == 13 && self.r#waterlogged == false { return 5385; }
        if self.r#waterlogged == true && self.r#rotation == 3 { return 5364; }
        if self.r#waterlogged == true && self.r#rotation == 1 { return 5360; }
        if self.r#rotation == 9 && self.r#waterlogged == false { return 5377; }
        if self.r#waterlogged == false && self.r#rotation == 6 { return 5371; }
        if self.r#rotation == 15 && self.r#waterlogged == false { return 5389; }
        if self.r#rotation == 10 && self.r#waterlogged == true { return 5378; }
        if self.r#rotation == 14 && self.r#waterlogged == false { return 5387; }
        if self.r#rotation == 3 && self.r#waterlogged == false { return 5365; }
        if self.r#waterlogged == false && self.r#rotation == 5 { return 5369; }
        if self.r#waterlogged == true && self.r#rotation == 15 { return 5388; }
        if self.r#rotation == 2 && self.r#waterlogged == true { return 5362; }
        if self.r#rotation == 14 && self.r#waterlogged == true { return 5386; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 5376 {
            return Some(PaleOakSign {
                r#rotation: 9,
                r#waterlogged: true,
            });
        }
        if state_id == 5366 {
            return Some(PaleOakSign {
                r#waterlogged: true,
                r#rotation: 4,
            });
        }
        if state_id == 5381 {
            return Some(PaleOakSign {
                r#rotation: 11,
                r#waterlogged: false,
            });
        }
        if state_id == 5359 {
            return Some(PaleOakSign {
                r#rotation: 0,
                r#waterlogged: false,
            });
        }
        if state_id == 5372 {
            return Some(PaleOakSign {
                r#waterlogged: true,
                r#rotation: 7,
            });
        }
        if state_id == 5373 {
            return Some(PaleOakSign {
                r#waterlogged: false,
                r#rotation: 7,
            });
        }
        if state_id == 5358 {
            return Some(PaleOakSign {
                r#rotation: 0,
                r#waterlogged: true,
            });
        }
        if state_id == 5370 {
            return Some(PaleOakSign {
                r#rotation: 6,
                r#waterlogged: true,
            });
        }
        if state_id == 5382 {
            return Some(PaleOakSign {
                r#waterlogged: true,
                r#rotation: 12,
            });
        }
        if state_id == 5383 {
            return Some(PaleOakSign {
                r#waterlogged: false,
                r#rotation: 12,
            });
        }
        if state_id == 5361 {
            return Some(PaleOakSign {
                r#waterlogged: false,
                r#rotation: 1,
            });
        }
        if state_id == 5363 {
            return Some(PaleOakSign {
                r#rotation: 2,
                r#waterlogged: false,
            });
        }
        if state_id == 5379 {
            return Some(PaleOakSign {
                r#rotation: 10,
                r#waterlogged: false,
            });
        }
        if state_id == 5384 {
            return Some(PaleOakSign {
                r#rotation: 13,
                r#waterlogged: true,
            });
        }
        if state_id == 5374 {
            return Some(PaleOakSign {
                r#rotation: 8,
                r#waterlogged: true,
            });
        }
        if state_id == 5367 {
            return Some(PaleOakSign {
                r#rotation: 4,
                r#waterlogged: false,
            });
        }
        if state_id == 5368 {
            return Some(PaleOakSign {
                r#rotation: 5,
                r#waterlogged: true,
            });
        }
        if state_id == 5380 {
            return Some(PaleOakSign {
                r#waterlogged: true,
                r#rotation: 11,
            });
        }
        if state_id == 5375 {
            return Some(PaleOakSign {
                r#rotation: 8,
                r#waterlogged: false,
            });
        }
        if state_id == 5385 {
            return Some(PaleOakSign {
                r#rotation: 13,
                r#waterlogged: false,
            });
        }
        if state_id == 5364 {
            return Some(PaleOakSign {
                r#waterlogged: true,
                r#rotation: 3,
            });
        }
        if state_id == 5360 {
            return Some(PaleOakSign {
                r#waterlogged: true,
                r#rotation: 1,
            });
        }
        if state_id == 5377 {
            return Some(PaleOakSign {
                r#rotation: 9,
                r#waterlogged: false,
            });
        }
        if state_id == 5371 {
            return Some(PaleOakSign {
                r#waterlogged: false,
                r#rotation: 6,
            });
        }
        if state_id == 5389 {
            return Some(PaleOakSign {
                r#rotation: 15,
                r#waterlogged: false,
            });
        }
        if state_id == 5378 {
            return Some(PaleOakSign {
                r#rotation: 10,
                r#waterlogged: true,
            });
        }
        if state_id == 5387 {
            return Some(PaleOakSign {
                r#rotation: 14,
                r#waterlogged: false,
            });
        }
        if state_id == 5365 {
            return Some(PaleOakSign {
                r#rotation: 3,
                r#waterlogged: false,
            });
        }
        if state_id == 5369 {
            return Some(PaleOakSign {
                r#waterlogged: false,
                r#rotation: 5,
            });
        }
        if state_id == 5388 {
            return Some(PaleOakSign {
                r#waterlogged: true,
                r#rotation: 15,
            });
        }
        if state_id == 5362 {
            return Some(PaleOakSign {
                r#rotation: 2,
                r#waterlogged: true,
            });
        }
        if state_id == 5386 {
            return Some(PaleOakSign {
                r#rotation: 14,
                r#waterlogged: true,
            });
        }
        return None;
    }
}

