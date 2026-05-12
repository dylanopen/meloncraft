use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SmallDripleaf {
    pub r#facing: Facing,
    pub waterlogged: bool,
    pub r#half: Half,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Half {
    Upper,
    Lower,
}

impl BlockState for SmallDripleaf {
    fn to_id(self) -> i32 {
        if block_state.r#facing == Facing::North && block_state.r#half == Half::Lower && block_state.r#waterlogged == true { return 27703; }
        if block_state.r#facing == Facing::East && block_state.r#half == Half::Upper && block_state.r#waterlogged == true { return 27713; }
        if block_state.r#half == Half::Upper && block_state.r#facing == Facing::East && block_state.r#waterlogged == false { return 27714; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::North && block_state.r#half == Half::Upper { return 27702; }
        if block_state.r#facing == Facing::North && block_state.r#half == Half::Lower && block_state.r#waterlogged == false { return 27704; }
        if block_state.r#waterlogged == true && block_state.r#half == Half::Upper && block_state.r#facing == Facing::West { return 27709; }
        if block_state.r#facing == Facing::East && block_state.r#half == Half::Lower && block_state.r#waterlogged == false { return 27716; }
        if block_state.r#facing == Facing::West && block_state.r#half == Half::Lower && block_state.r#waterlogged == false { return 27712; }
        if block_state.r#facing == Facing::West && block_state.r#half == Half::Upper && block_state.r#waterlogged == false { return 27710; }
        if block_state.r#facing == Facing::South && block_state.r#half == Half::Upper && block_state.r#waterlogged == false { return 27706; }
        if block_state.r#half == Half::Upper && block_state.r#waterlogged == true && block_state.r#facing == Facing::North { return 27701; }
        if block_state.r#facing == Facing::East && block_state.r#half == Half::Lower && block_state.r#waterlogged == true { return 27715; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::South && block_state.r#half == Half::Lower { return 27708; }
        if block_state.r#half == Half::Upper && block_state.r#facing == Facing::South && block_state.r#waterlogged == true { return 27705; }
        if block_state.r#facing == Facing::West && block_state.r#half == Half::Lower && block_state.r#waterlogged == true { return 27711; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::South && block_state.r#half == Half::Lower { return 27707; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 27703 {
            return Some(SmallDripleaf {
                r#facing: Facing::North,
                r#half: Half::Lower,
                r#waterlogged: true,
            });
        }
        if state_id == 27713 {
            return Some(SmallDripleaf {
                r#facing: Facing::East,
                r#half: Half::Upper,
                r#waterlogged: true,
            });
        }
        if state_id == 27714 {
            return Some(SmallDripleaf {
                r#half: Half::Upper,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 27702 {
            return Some(SmallDripleaf {
                r#waterlogged: false,
                r#facing: Facing::North,
                r#half: Half::Upper,
            });
        }
        if state_id == 27704 {
            return Some(SmallDripleaf {
                r#facing: Facing::North,
                r#half: Half::Lower,
                r#waterlogged: false,
            });
        }
        if state_id == 27709 {
            return Some(SmallDripleaf {
                r#waterlogged: true,
                r#half: Half::Upper,
                r#facing: Facing::West,
            });
        }
        if state_id == 27716 {
            return Some(SmallDripleaf {
                r#facing: Facing::East,
                r#half: Half::Lower,
                r#waterlogged: false,
            });
        }
        if state_id == 27712 {
            return Some(SmallDripleaf {
                r#facing: Facing::West,
                r#half: Half::Lower,
                r#waterlogged: false,
            });
        }
        if state_id == 27710 {
            return Some(SmallDripleaf {
                r#facing: Facing::West,
                r#half: Half::Upper,
                r#waterlogged: false,
            });
        }
        if state_id == 27706 {
            return Some(SmallDripleaf {
                r#facing: Facing::South,
                r#half: Half::Upper,
                r#waterlogged: false,
            });
        }
        if state_id == 27701 {
            return Some(SmallDripleaf {
                r#half: Half::Upper,
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 27715 {
            return Some(SmallDripleaf {
                r#facing: Facing::East,
                r#half: Half::Lower,
                r#waterlogged: true,
            });
        }
        if state_id == 27708 {
            return Some(SmallDripleaf {
                r#waterlogged: false,
                r#facing: Facing::South,
                r#half: Half::Lower,
            });
        }
        if state_id == 27705 {
            return Some(SmallDripleaf {
                r#half: Half::Upper,
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 27711 {
            return Some(SmallDripleaf {
                r#facing: Facing::West,
                r#half: Half::Lower,
                r#waterlogged: true,
            });
        }
        if state_id == 27707 {
            return Some(SmallDripleaf {
                r#waterlogged: true,
                r#facing: Facing::South,
                r#half: Half::Lower,
            });
        }
        return None;
    }
}

