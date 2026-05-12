use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BigDripleafStem {
    pub r#facing: Facing,
    pub waterlogged: bool,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for BigDripleafStem {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::North { return 27694; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == false { return 27698; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == true { return 27693; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::East { return 27699; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == false { return 27696; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == false { return 27700; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::South { return 27695; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == true { return 27697; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 27694 {
            return Some(BigDripleafStem {
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 27698 {
            return Some(BigDripleafStem {
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 27693 {
            return Some(BigDripleafStem {
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 27699 {
            return Some(BigDripleafStem {
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 27696 {
            return Some(BigDripleafStem {
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 27700 {
            return Some(BigDripleafStem {
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 27695 {
            return Some(BigDripleafStem {
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 27697 {
            return Some(BigDripleafStem {
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        return None;
    }
}

