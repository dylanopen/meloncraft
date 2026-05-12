use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SmallDripleaf {
    pub r#half: Half,
    pub waterlogged: bool,
    pub r#facing: Facing,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Half {
    Upper,
    Lower,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for SmallDripleaf {
    fn to_id(&self) -> i32 {
        if self.r#facing == Facing::West && self.r#half == Half::Upper && self.r#waterlogged == false { return 27710; }
        if self.r#half == Half::Lower && self.r#waterlogged == false && self.r#facing == Facing::North { return 27704; }
        if self.r#facing == Facing::West && self.r#waterlogged == true && self.r#half == Half::Lower { return 27711; }
        if self.r#facing == Facing::East && self.r#half == Half::Upper && self.r#waterlogged == true { return 27713; }
        if self.r#facing == Facing::East && self.r#half == Half::Lower && self.r#waterlogged == false { return 27716; }
        if self.r#half == Half::Upper && self.r#facing == Facing::East && self.r#waterlogged == false { return 27714; }
        if self.r#waterlogged == false && self.r#facing == Facing::South && self.r#half == Half::Lower { return 27708; }
        if self.r#half == Half::Lower && self.r#waterlogged == false && self.r#facing == Facing::West { return 27712; }
        if self.r#facing == Facing::South && self.r#half == Half::Upper && self.r#waterlogged == true { return 27705; }
        if self.r#waterlogged == true && self.r#half == Half::Lower && self.r#facing == Facing::North { return 27703; }
        if self.r#facing == Facing::West && self.r#half == Half::Upper && self.r#waterlogged == true { return 27709; }
        if self.r#waterlogged == false && self.r#facing == Facing::South && self.r#half == Half::Upper { return 27706; }
        if self.r#half == Half::Upper && self.r#waterlogged == true && self.r#facing == Facing::North { return 27701; }
        if self.r#facing == Facing::South && self.r#half == Half::Lower && self.r#waterlogged == true { return 27707; }
        if self.r#half == Half::Upper && self.r#facing == Facing::North && self.r#waterlogged == false { return 27702; }
        if self.r#waterlogged == true && self.r#facing == Facing::East && self.r#half == Half::Lower { return 27715; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 27710 {
            return Some(SmallDripleaf {
                r#facing: Facing::West,
                r#half: Half::Upper,
                r#waterlogged: false,
            });
        }
        if state_id == 27704 {
            return Some(SmallDripleaf {
                r#half: Half::Lower,
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 27711 {
            return Some(SmallDripleaf {
                r#facing: Facing::West,
                r#waterlogged: true,
                r#half: Half::Lower,
            });
        }
        if state_id == 27713 {
            return Some(SmallDripleaf {
                r#facing: Facing::East,
                r#half: Half::Upper,
                r#waterlogged: true,
            });
        }
        if state_id == 27716 {
            return Some(SmallDripleaf {
                r#facing: Facing::East,
                r#half: Half::Lower,
                r#waterlogged: false,
            });
        }
        if state_id == 27714 {
            return Some(SmallDripleaf {
                r#half: Half::Upper,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 27708 {
            return Some(SmallDripleaf {
                r#waterlogged: false,
                r#facing: Facing::South,
                r#half: Half::Lower,
            });
        }
        if state_id == 27712 {
            return Some(SmallDripleaf {
                r#half: Half::Lower,
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 27705 {
            return Some(SmallDripleaf {
                r#facing: Facing::South,
                r#half: Half::Upper,
                r#waterlogged: true,
            });
        }
        if state_id == 27703 {
            return Some(SmallDripleaf {
                r#waterlogged: true,
                r#half: Half::Lower,
                r#facing: Facing::North,
            });
        }
        if state_id == 27709 {
            return Some(SmallDripleaf {
                r#facing: Facing::West,
                r#half: Half::Upper,
                r#waterlogged: true,
            });
        }
        if state_id == 27706 {
            return Some(SmallDripleaf {
                r#waterlogged: false,
                r#facing: Facing::South,
                r#half: Half::Upper,
            });
        }
        if state_id == 27701 {
            return Some(SmallDripleaf {
                r#half: Half::Upper,
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 27707 {
            return Some(SmallDripleaf {
                r#facing: Facing::South,
                r#half: Half::Lower,
                r#waterlogged: true,
            });
        }
        if state_id == 27702 {
            return Some(SmallDripleaf {
                r#half: Half::Upper,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 27715 {
            return Some(SmallDripleaf {
                r#waterlogged: true,
                r#facing: Facing::East,
                r#half: Half::Lower,
            });
        }
        return None;
    }
}

