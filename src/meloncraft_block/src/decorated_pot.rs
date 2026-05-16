use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecoratedPot {
    pub waterlogged: bool,
    pub cracked: bool,
    pub r#facing: Facing,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for DecoratedPot {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == false && self.r#facing == Facing::East && self.r#cracked == true { return 29398; }
        if self.r#cracked == true && self.r#facing == Facing::South && self.r#waterlogged == false { return 29394; }
        if self.r#facing == Facing::West && self.r#cracked == true && self.r#waterlogged == false { return 29396; }
        if self.r#cracked == false && self.r#facing == Facing::West && self.r#waterlogged == false { return 29404; }
        if self.r#cracked == false && self.r#waterlogged == false && self.r#facing == Facing::East { return 29406; }
        if self.r#facing == Facing::South && self.r#waterlogged == true && self.r#cracked == true { return 29393; }
        if self.r#cracked == false && self.r#waterlogged == true && self.r#facing == Facing::North { return 29399; }
        if self.r#facing == Facing::West && self.r#cracked == false && self.r#waterlogged == true { return 29403; }
        if self.r#facing == Facing::South && self.r#waterlogged == true && self.r#cracked == false { return 29401; }
        if self.r#cracked == true && self.r#facing == Facing::North && self.r#waterlogged == false { return 29392; }
        if self.r#waterlogged == true && self.r#cracked == true && self.r#facing == Facing::North { return 29391; }
        if self.r#facing == Facing::West && self.r#cracked == true && self.r#waterlogged == true { return 29395; }
        if self.r#waterlogged == false && self.r#cracked == false && self.r#facing == Facing::North { return 29400; }
        if self.r#cracked == false && self.r#waterlogged == true && self.r#facing == Facing::East { return 29405; }
        if self.r#waterlogged == false && self.r#cracked == false && self.r#facing == Facing::South { return 29402; }
        if self.r#facing == Facing::East && self.r#waterlogged == true && self.r#cracked == true { return 29397; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 29398 {
            return Some(DecoratedPot {
                r#waterlogged: false,
                r#facing: Facing::East,
                r#cracked: true,
            });
        }
        if state_id == 29394 {
            return Some(DecoratedPot {
                r#cracked: true,
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 29396 {
            return Some(DecoratedPot {
                r#facing: Facing::West,
                r#cracked: true,
                r#waterlogged: false,
            });
        }
        if state_id == 29404 {
            return Some(DecoratedPot {
                r#cracked: false,
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 29406 {
            return Some(DecoratedPot {
                r#cracked: false,
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 29393 {
            return Some(DecoratedPot {
                r#facing: Facing::South,
                r#waterlogged: true,
                r#cracked: true,
            });
        }
        if state_id == 29399 {
            return Some(DecoratedPot {
                r#cracked: false,
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 29403 {
            return Some(DecoratedPot {
                r#facing: Facing::West,
                r#cracked: false,
                r#waterlogged: true,
            });
        }
        if state_id == 29401 {
            return Some(DecoratedPot {
                r#facing: Facing::South,
                r#waterlogged: true,
                r#cracked: false,
            });
        }
        if state_id == 29392 {
            return Some(DecoratedPot {
                r#cracked: true,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 29391 {
            return Some(DecoratedPot {
                r#waterlogged: true,
                r#cracked: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 29395 {
            return Some(DecoratedPot {
                r#facing: Facing::West,
                r#cracked: true,
                r#waterlogged: true,
            });
        }
        if state_id == 29400 {
            return Some(DecoratedPot {
                r#waterlogged: false,
                r#cracked: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 29405 {
            return Some(DecoratedPot {
                r#cracked: false,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 29402 {
            return Some(DecoratedPot {
                r#waterlogged: false,
                r#cracked: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 29397 {
            return Some(DecoratedPot {
                r#facing: Facing::East,
                r#waterlogged: true,
                r#cracked: true,
            });
        }
        return None;
    }
}

