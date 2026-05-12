use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecoratedPot {
    pub r#facing: Facing,
    pub cracked: bool,
    pub waterlogged: bool,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for DecoratedPot {
    fn to_id(self) -> i32 {
        if block_state.r#cracked == false && block_state.r#facing == Facing::South && block_state.r#waterlogged == true { return 29401; }
        if block_state.r#cracked == true && block_state.r#waterlogged == true && block_state.r#facing == Facing::West { return 29395; }
        if block_state.r#cracked == false && block_state.r#facing == Facing::East && block_state.r#waterlogged == false { return 29406; }
        if block_state.r#cracked == true && block_state.r#facing == Facing::South && block_state.r#waterlogged == true { return 29393; }
        if block_state.r#cracked == false && block_state.r#waterlogged == true && block_state.r#facing == Facing::North { return 29399; }
        if block_state.r#waterlogged == true && block_state.r#cracked == true && block_state.r#facing == Facing::East { return 29397; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == false && block_state.r#cracked == true { return 29392; }
        if block_state.r#cracked == true && block_state.r#facing == Facing::West && block_state.r#waterlogged == false { return 29396; }
        if block_state.r#cracked == false && block_state.r#waterlogged == false && block_state.r#facing == Facing::North { return 29400; }
        if block_state.r#waterlogged == false && block_state.r#cracked == false && block_state.r#facing == Facing::West { return 29404; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::East && block_state.r#cracked == true { return 29398; }
        if block_state.r#waterlogged == true && block_state.r#cracked == false && block_state.r#facing == Facing::East { return 29405; }
        if block_state.r#cracked == true && block_state.r#waterlogged == false && block_state.r#facing == Facing::South { return 29394; }
        if block_state.r#waterlogged == true && block_state.r#cracked == false && block_state.r#facing == Facing::West { return 29403; }
        if block_state.r#waterlogged == true && block_state.r#cracked == true && block_state.r#facing == Facing::North { return 29391; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::South && block_state.r#cracked == false { return 29402; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 29401 {
            return Some(DecoratedPot {
                r#cracked: false,
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 29395 {
            return Some(DecoratedPot {
                r#cracked: true,
                r#waterlogged: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 29406 {
            return Some(DecoratedPot {
                r#cracked: false,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 29393 {
            return Some(DecoratedPot {
                r#cracked: true,
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 29399 {
            return Some(DecoratedPot {
                r#cracked: false,
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 29397 {
            return Some(DecoratedPot {
                r#waterlogged: true,
                r#cracked: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 29392 {
            return Some(DecoratedPot {
                r#facing: Facing::North,
                r#waterlogged: false,
                r#cracked: true,
            });
        }
        if state_id == 29396 {
            return Some(DecoratedPot {
                r#cracked: true,
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 29400 {
            return Some(DecoratedPot {
                r#cracked: false,
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 29404 {
            return Some(DecoratedPot {
                r#waterlogged: false,
                r#cracked: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 29398 {
            return Some(DecoratedPot {
                r#waterlogged: false,
                r#facing: Facing::East,
                r#cracked: true,
            });
        }
        if state_id == 29405 {
            return Some(DecoratedPot {
                r#waterlogged: true,
                r#cracked: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 29394 {
            return Some(DecoratedPot {
                r#cracked: true,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 29403 {
            return Some(DecoratedPot {
                r#waterlogged: true,
                r#cracked: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 29391 {
            return Some(DecoratedPot {
                r#waterlogged: true,
                r#cracked: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 29402 {
            return Some(DecoratedPot {
                r#waterlogged: false,
                r#facing: Facing::South,
                r#cracked: false,
            });
        }
        return None;
    }
}

