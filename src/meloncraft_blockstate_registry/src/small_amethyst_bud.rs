use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SmallAmethystBud {
    pub waterlogged: bool,
    pub r#facing: Facing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    East,
    South,
    West,
    Up,
    Down,
}

impl BlockState for SmallAmethystBud {
    fn to_id(&self) -> i32 {
        if self.r#facing == Facing::Down && self.r#waterlogged == false {
            return 23249;
        }
        if self.r#waterlogged == false && self.r#facing == Facing::North {
            return 23239;
        }
        if self.r#facing == Facing::South && self.r#waterlogged == false {
            return 23243;
        }
        if self.r#waterlogged == false && self.r#facing == Facing::West {
            return 23245;
        }
        if self.r#waterlogged == false && self.r#facing == Facing::Up {
            return 23247;
        }
        if self.r#waterlogged == true && self.r#facing == Facing::East {
            return 23240;
        }
        if self.r#facing == Facing::Up && self.r#waterlogged == true {
            return 23246;
        }
        if self.r#facing == Facing::North && self.r#waterlogged == true {
            return 23238;
        }
        if self.r#facing == Facing::Down && self.r#waterlogged == true {
            return 23248;
        }
        if self.r#waterlogged == true && self.r#facing == Facing::South {
            return 23242;
        }
        if self.r#waterlogged == true && self.r#facing == Facing::West {
            return 23244;
        }
        if self.r#waterlogged == false && self.r#facing == Facing::East {
            return 23241;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 23249 {
            return Some(SmallAmethystBud {
                r#facing: Facing::Down,
                r#waterlogged: false,
            });
        }
        if state_id == 23239 {
            return Some(SmallAmethystBud {
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 23243 {
            return Some(SmallAmethystBud {
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 23245 {
            return Some(SmallAmethystBud {
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 23247 {
            return Some(SmallAmethystBud {
                r#waterlogged: false,
                r#facing: Facing::Up,
            });
        }
        if state_id == 23240 {
            return Some(SmallAmethystBud {
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 23246 {
            return Some(SmallAmethystBud {
                r#facing: Facing::Up,
                r#waterlogged: true,
            });
        }
        if state_id == 23238 {
            return Some(SmallAmethystBud {
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 23248 {
            return Some(SmallAmethystBud {
                r#facing: Facing::Down,
                r#waterlogged: true,
            });
        }
        if state_id == 23242 {
            return Some(SmallAmethystBud {
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 23244 {
            return Some(SmallAmethystBud {
                r#waterlogged: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 23241 {
            return Some(SmallAmethystBud {
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        return None;
    }
}
