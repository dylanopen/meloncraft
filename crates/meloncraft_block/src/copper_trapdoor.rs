use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CopperTrapdoor {
    pub open: bool,
    pub r#half: Half,
    pub r#facing: Facing,
    pub powered: bool,
    pub waterlogged: bool,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Half {
    Top,
    Bottom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for CopperTrapdoor {
    fn to_id(self) -> i32 {
        if block_state.r#powered == true && block_state.r#half == Half::Top && block_state.r#waterlogged == false && block_state.r#open == false && block_state.r#facing == Facing::East { return 26386; }
        if block_state.r#half == Half::Bottom && block_state.r#waterlogged == false && block_state.r#powered == false && block_state.r#facing == Facing::North && block_state.r#open == true { return 26344; }
        if block_state.r#powered == true && block_state.r#waterlogged == true && block_state.r#open == true && block_state.r#facing == Facing::North && block_state.r#half == Half::Top { return 26333; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::South && block_state.r#open == true && block_state.r#powered == true && block_state.r#half == Half::Bottom { return 26357; }
        if block_state.r#facing == Facing::West && block_state.r#half == Half::Top && block_state.r#waterlogged == false && block_state.r#open == false && block_state.r#powered == false { return 26372; }
        if block_state.r#powered == true && block_state.r#facing == Facing::South && block_state.r#waterlogged == false && block_state.r#half == Half::Top && block_state.r#open == true { return 26350; }
        if block_state.r#facing == Facing::West && block_state.r#half == Half::Top && block_state.r#powered == true && block_state.r#open == true && block_state.r#waterlogged == false { return 26366; }
        if block_state.r#facing == Facing::East && block_state.r#half == Half::Top && block_state.r#waterlogged == true && block_state.r#open == true && block_state.r#powered == true { return 26381; }
        if block_state.r#facing == Facing::West && block_state.r#powered == false && block_state.r#waterlogged == true && block_state.r#half == Half::Top && block_state.r#open == true { return 26367; }
        if block_state.r#half == Half::Bottom && block_state.r#open == false && block_state.r#powered == true && block_state.r#waterlogged == false && block_state.r#facing == Facing::North { return 26346; }
        if block_state.r#facing == Facing::South && block_state.r#powered == true && block_state.r#half == Half::Bottom && block_state.r#open == true && block_state.r#waterlogged == false { return 26358; }
        if block_state.r#facing == Facing::East && block_state.r#half == Half::Top && block_state.r#waterlogged == true && block_state.r#powered == false && block_state.r#open == true { return 26383; }
        if block_state.r#powered == false && block_state.r#waterlogged == true && block_state.r#half == Half::Top && block_state.r#open == false && block_state.r#facing == Facing::South { return 26355; }
        if block_state.r#waterlogged == true && block_state.r#open == true && block_state.r#facing == Facing::East && block_state.r#powered == false && block_state.r#half == Half::Bottom { return 26391; }
        if block_state.r#open == false && block_state.r#facing == Facing::South && block_state.r#half == Half::Top && block_state.r#powered == false && block_state.r#waterlogged == false { return 26356; }
        if block_state.r#powered == false && block_state.r#half == Half::Top && block_state.r#facing == Facing::East && block_state.r#open == false && block_state.r#waterlogged == false { return 26388; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::North && block_state.r#powered == false && block_state.r#half == Half::Bottom && block_state.r#open == false { return 26347; }
        if block_state.r#facing == Facing::East && block_state.r#half == Half::Bottom && block_state.r#open == true && block_state.r#powered == false && block_state.r#waterlogged == false { return 26392; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::South && block_state.r#open == true && block_state.r#half == Half::Top && block_state.r#powered == false { return 26351; }
        if block_state.r#powered == false && block_state.r#waterlogged == false && block_state.r#half == Half::Top && block_state.r#facing == Facing::North && block_state.r#open == true { return 26336; }
        if block_state.r#waterlogged == true && block_state.r#open == false && block_state.r#facing == Facing::East && block_state.r#powered == true && block_state.r#half == Half::Bottom { return 26393; }
        if block_state.r#waterlogged == true && block_state.r#half == Half::Top && block_state.r#open == true && block_state.r#facing == Facing::North && block_state.r#powered == false { return 26335; }
        if block_state.r#facing == Facing::North && block_state.r#half == Half::Bottom && block_state.r#waterlogged == false && block_state.r#powered == true && block_state.r#open == true { return 26342; }
        if block_state.r#open == false && block_state.r#powered == false && block_state.r#facing == Facing::North && block_state.r#half == Half::Top && block_state.r#waterlogged == true { return 26339; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::West && block_state.r#half == Half::Bottom && block_state.r#powered == false && block_state.r#open == false { return 26379; }
        if block_state.r#half == Half::Bottom && block_state.r#open == false && block_state.r#facing == Facing::South && block_state.r#powered == false && block_state.r#waterlogged == true { return 26363; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::North && block_state.r#open == false && block_state.r#half == Half::Top && block_state.r#powered == true { return 26337; }
        if block_state.r#half == Half::Bottom && block_state.r#open == true && block_state.r#powered == true && block_state.r#facing == Facing::East && block_state.r#waterlogged == false { return 26390; }
        if block_state.r#powered == true && block_state.r#facing == Facing::North && block_state.r#open == true && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true { return 26341; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::North && block_state.r#half == Half::Bottom && block_state.r#open == false && block_state.r#powered == true { return 26345; }
        if block_state.r#waterlogged == true && block_state.r#open == false && block_state.r#powered == true && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::West { return 26377; }
        if block_state.r#half == Half::Top && block_state.r#powered == true && block_state.r#waterlogged == false && block_state.r#facing == Facing::North && block_state.r#open == false { return 26338; }
        if block_state.r#half == Half::Top && block_state.r#waterlogged == true && block_state.r#facing == Facing::South && block_state.r#open == true && block_state.r#powered == true { return 26349; }
        if block_state.r#waterlogged == false && block_state.r#open == false && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::West && block_state.r#powered == true { return 26378; }
        if block_state.r#powered == true && block_state.r#open == true && block_state.r#facing == Facing::West && block_state.r#half == Half::Bottom && block_state.r#waterlogged == false { return 26374; }
        if block_state.r#facing == Facing::West && block_state.r#half == Half::Bottom && block_state.r#powered == false && block_state.r#waterlogged == true && block_state.r#open == true { return 26375; }
        if block_state.r#powered == true && block_state.r#waterlogged == false && block_state.r#facing == Facing::East && block_state.r#half == Half::Top && block_state.r#open == true { return 26382; }
        if block_state.r#half == Half::Top && block_state.r#facing == Facing::South && block_state.r#powered == true && block_state.r#open == false && block_state.r#waterlogged == false { return 26354; }
        if block_state.r#facing == Facing::South && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true && block_state.r#open == false && block_state.r#powered == true { return 26361; }
        if block_state.r#waterlogged == true && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::North && block_state.r#open == true && block_state.r#powered == false { return 26343; }
        if block_state.r#facing == Facing::West && block_state.r#powered == true && block_state.r#waterlogged == false && block_state.r#open == false && block_state.r#half == Half::Top { return 26370; }
        if block_state.r#powered == false && block_state.r#waterlogged == false && block_state.r#facing == Facing::South && block_state.r#open == true && block_state.r#half == Half::Top { return 26352; }
        if block_state.r#facing == Facing::West && block_state.r#open == true && block_state.r#waterlogged == false && block_state.r#powered == false && block_state.r#half == Half::Top { return 26368; }
        if block_state.r#facing == Facing::West && block_state.r#half == Half::Top && block_state.r#waterlogged == true && block_state.r#open == false && block_state.r#powered == false { return 26371; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::West && block_state.r#half == Half::Top && block_state.r#open == true && block_state.r#powered == true { return 26365; }
        if block_state.r#half == Half::Top && block_state.r#powered == true && block_state.r#waterlogged == true && block_state.r#facing == Facing::South && block_state.r#open == false { return 26353; }
        if block_state.r#half == Half::Top && block_state.r#open == false && block_state.r#waterlogged == true && block_state.r#facing == Facing::East && block_state.r#powered == false { return 26387; }
        if block_state.r#powered == true && block_state.r#facing == Facing::East && block_state.r#waterlogged == true && block_state.r#open == false && block_state.r#half == Half::Top { return 26385; }
        if block_state.r#facing == Facing::East && block_state.r#half == Half::Bottom && block_state.r#powered == true && block_state.r#open == true && block_state.r#waterlogged == true { return 26389; }
        if block_state.r#powered == false && block_state.r#half == Half::Bottom && block_state.r#open == false && block_state.r#waterlogged == false && block_state.r#facing == Facing::South { return 26364; }
        if block_state.r#half == Half::Top && block_state.r#powered == true && block_state.r#facing == Facing::West && block_state.r#waterlogged == true && block_state.r#open == false { return 26369; }
        if block_state.r#powered == false && block_state.r#open == true && block_state.r#waterlogged == false && block_state.r#half == Half::Top && block_state.r#facing == Facing::East { return 26384; }
        if block_state.r#half == Half::Bottom && block_state.r#facing == Facing::East && block_state.r#powered == true && block_state.r#waterlogged == false && block_state.r#open == false { return 26394; }
        if block_state.r#facing == Facing::West && block_state.r#open == false && block_state.r#powered == false && block_state.r#half == Half::Bottom && block_state.r#waterlogged == false { return 26380; }
        if block_state.r#half == Half::Bottom && block_state.r#powered == false && block_state.r#waterlogged == true && block_state.r#facing == Facing::South && block_state.r#open == true { return 26359; }
        if block_state.r#powered == false && block_state.r#facing == Facing::North && block_state.r#half == Half::Top && block_state.r#open == false && block_state.r#waterlogged == false { return 26340; }
        if block_state.r#waterlogged == true && block_state.r#half == Half::Bottom && block_state.r#powered == true && block_state.r#open == true && block_state.r#facing == Facing::West { return 26373; }
        if block_state.r#powered == false && block_state.r#waterlogged == false && block_state.r#facing == Facing::East && block_state.r#half == Half::Bottom && block_state.r#open == false { return 26396; }
        if block_state.r#half == Half::Bottom && block_state.r#open == false && block_state.r#waterlogged == false && block_state.r#facing == Facing::South && block_state.r#powered == true { return 26362; }
        if block_state.r#half == Half::Bottom && block_state.r#facing == Facing::South && block_state.r#powered == false && block_state.r#open == true && block_state.r#waterlogged == false { return 26360; }
        if block_state.r#waterlogged == true && block_state.r#half == Half::Bottom && block_state.r#open == false && block_state.r#facing == Facing::East && block_state.r#powered == false { return 26395; }
        if block_state.r#waterlogged == false && block_state.r#half == Half::Bottom && block_state.r#powered == false && block_state.r#facing == Facing::North && block_state.r#open == false { return 26348; }
        if block_state.r#facing == Facing::West && block_state.r#half == Half::Bottom && block_state.r#powered == false && block_state.r#open == true && block_state.r#waterlogged == false { return 26376; }
        if block_state.r#waterlogged == false && block_state.r#half == Half::Top && block_state.r#facing == Facing::North && block_state.r#open == true && block_state.r#powered == true { return 26334; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 26386 {
            return Some(CopperTrapdoor {
                r#powered: true,
                r#half: Half::Top,
                r#waterlogged: false,
                r#open: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 26344 {
            return Some(CopperTrapdoor {
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#powered: false,
                r#facing: Facing::North,
                r#open: true,
            });
        }
        if state_id == 26333 {
            return Some(CopperTrapdoor {
                r#powered: true,
                r#waterlogged: true,
                r#open: true,
                r#facing: Facing::North,
                r#half: Half::Top,
            });
        }
        if state_id == 26357 {
            return Some(CopperTrapdoor {
                r#waterlogged: true,
                r#facing: Facing::South,
                r#open: true,
                r#powered: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 26372 {
            return Some(CopperTrapdoor {
                r#facing: Facing::West,
                r#half: Half::Top,
                r#waterlogged: false,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 26350 {
            return Some(CopperTrapdoor {
                r#powered: true,
                r#facing: Facing::South,
                r#waterlogged: false,
                r#half: Half::Top,
                r#open: true,
            });
        }
        if state_id == 26366 {
            return Some(CopperTrapdoor {
                r#facing: Facing::West,
                r#half: Half::Top,
                r#powered: true,
                r#open: true,
                r#waterlogged: false,
            });
        }
        if state_id == 26381 {
            return Some(CopperTrapdoor {
                r#facing: Facing::East,
                r#half: Half::Top,
                r#waterlogged: true,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 26367 {
            return Some(CopperTrapdoor {
                r#facing: Facing::West,
                r#powered: false,
                r#waterlogged: true,
                r#half: Half::Top,
                r#open: true,
            });
        }
        if state_id == 26346 {
            return Some(CopperTrapdoor {
                r#half: Half::Bottom,
                r#open: false,
                r#powered: true,
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 26358 {
            return Some(CopperTrapdoor {
                r#facing: Facing::South,
                r#powered: true,
                r#half: Half::Bottom,
                r#open: true,
                r#waterlogged: false,
            });
        }
        if state_id == 26383 {
            return Some(CopperTrapdoor {
                r#facing: Facing::East,
                r#half: Half::Top,
                r#waterlogged: true,
                r#powered: false,
                r#open: true,
            });
        }
        if state_id == 26355 {
            return Some(CopperTrapdoor {
                r#powered: false,
                r#waterlogged: true,
                r#half: Half::Top,
                r#open: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 26391 {
            return Some(CopperTrapdoor {
                r#waterlogged: true,
                r#open: true,
                r#facing: Facing::East,
                r#powered: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 26356 {
            return Some(CopperTrapdoor {
                r#open: false,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#powered: false,
                r#waterlogged: false,
            });
        }
        if state_id == 26388 {
            return Some(CopperTrapdoor {
                r#powered: false,
                r#half: Half::Top,
                r#facing: Facing::East,
                r#open: false,
                r#waterlogged: false,
            });
        }
        if state_id == 26347 {
            return Some(CopperTrapdoor {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#powered: false,
                r#half: Half::Bottom,
                r#open: false,
            });
        }
        if state_id == 26392 {
            return Some(CopperTrapdoor {
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#open: true,
                r#powered: false,
                r#waterlogged: false,
            });
        }
        if state_id == 26351 {
            return Some(CopperTrapdoor {
                r#waterlogged: true,
                r#facing: Facing::South,
                r#open: true,
                r#half: Half::Top,
                r#powered: false,
            });
        }
        if state_id == 26336 {
            return Some(CopperTrapdoor {
                r#powered: false,
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::North,
                r#open: true,
            });
        }
        if state_id == 26393 {
            return Some(CopperTrapdoor {
                r#waterlogged: true,
                r#open: false,
                r#facing: Facing::East,
                r#powered: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 26335 {
            return Some(CopperTrapdoor {
                r#waterlogged: true,
                r#half: Half::Top,
                r#open: true,
                r#facing: Facing::North,
                r#powered: false,
            });
        }
        if state_id == 26342 {
            return Some(CopperTrapdoor {
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#powered: true,
                r#open: true,
            });
        }
        if state_id == 26339 {
            return Some(CopperTrapdoor {
                r#open: false,
                r#powered: false,
                r#facing: Facing::North,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 26379 {
            return Some(CopperTrapdoor {
                r#waterlogged: true,
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#powered: false,
                r#open: false,
            });
        }
        if state_id == 26363 {
            return Some(CopperTrapdoor {
                r#half: Half::Bottom,
                r#open: false,
                r#facing: Facing::South,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 26337 {
            return Some(CopperTrapdoor {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#open: false,
                r#half: Half::Top,
                r#powered: true,
            });
        }
        if state_id == 26390 {
            return Some(CopperTrapdoor {
                r#half: Half::Bottom,
                r#open: true,
                r#powered: true,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 26341 {
            return Some(CopperTrapdoor {
                r#powered: true,
                r#facing: Facing::North,
                r#open: true,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 26345 {
            return Some(CopperTrapdoor {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#open: false,
                r#powered: true,
            });
        }
        if state_id == 26377 {
            return Some(CopperTrapdoor {
                r#waterlogged: true,
                r#open: false,
                r#powered: true,
                r#half: Half::Bottom,
                r#facing: Facing::West,
            });
        }
        if state_id == 26338 {
            return Some(CopperTrapdoor {
                r#half: Half::Top,
                r#powered: true,
                r#waterlogged: false,
                r#facing: Facing::North,
                r#open: false,
            });
        }
        if state_id == 26349 {
            return Some(CopperTrapdoor {
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::South,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 26378 {
            return Some(CopperTrapdoor {
                r#waterlogged: false,
                r#open: false,
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#powered: true,
            });
        }
        if state_id == 26374 {
            return Some(CopperTrapdoor {
                r#powered: true,
                r#open: true,
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 26375 {
            return Some(CopperTrapdoor {
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#powered: false,
                r#waterlogged: true,
                r#open: true,
            });
        }
        if state_id == 26382 {
            return Some(CopperTrapdoor {
                r#powered: true,
                r#waterlogged: false,
                r#facing: Facing::East,
                r#half: Half::Top,
                r#open: true,
            });
        }
        if state_id == 26354 {
            return Some(CopperTrapdoor {
                r#half: Half::Top,
                r#facing: Facing::South,
                r#powered: true,
                r#open: false,
                r#waterlogged: false,
            });
        }
        if state_id == 26361 {
            return Some(CopperTrapdoor {
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#open: false,
                r#powered: true,
            });
        }
        if state_id == 26343 {
            return Some(CopperTrapdoor {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#open: true,
                r#powered: false,
            });
        }
        if state_id == 26370 {
            return Some(CopperTrapdoor {
                r#facing: Facing::West,
                r#powered: true,
                r#waterlogged: false,
                r#open: false,
                r#half: Half::Top,
            });
        }
        if state_id == 26352 {
            return Some(CopperTrapdoor {
                r#powered: false,
                r#waterlogged: false,
                r#facing: Facing::South,
                r#open: true,
                r#half: Half::Top,
            });
        }
        if state_id == 26368 {
            return Some(CopperTrapdoor {
                r#facing: Facing::West,
                r#open: true,
                r#waterlogged: false,
                r#powered: false,
                r#half: Half::Top,
            });
        }
        if state_id == 26371 {
            return Some(CopperTrapdoor {
                r#facing: Facing::West,
                r#half: Half::Top,
                r#waterlogged: true,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 26365 {
            return Some(CopperTrapdoor {
                r#waterlogged: true,
                r#facing: Facing::West,
                r#half: Half::Top,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 26353 {
            return Some(CopperTrapdoor {
                r#half: Half::Top,
                r#powered: true,
                r#waterlogged: true,
                r#facing: Facing::South,
                r#open: false,
            });
        }
        if state_id == 26387 {
            return Some(CopperTrapdoor {
                r#half: Half::Top,
                r#open: false,
                r#waterlogged: true,
                r#facing: Facing::East,
                r#powered: false,
            });
        }
        if state_id == 26385 {
            return Some(CopperTrapdoor {
                r#powered: true,
                r#facing: Facing::East,
                r#waterlogged: true,
                r#open: false,
                r#half: Half::Top,
            });
        }
        if state_id == 26389 {
            return Some(CopperTrapdoor {
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#powered: true,
                r#open: true,
                r#waterlogged: true,
            });
        }
        if state_id == 26364 {
            return Some(CopperTrapdoor {
                r#powered: false,
                r#half: Half::Bottom,
                r#open: false,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 26369 {
            return Some(CopperTrapdoor {
                r#half: Half::Top,
                r#powered: true,
                r#facing: Facing::West,
                r#waterlogged: true,
                r#open: false,
            });
        }
        if state_id == 26384 {
            return Some(CopperTrapdoor {
                r#powered: false,
                r#open: true,
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::East,
            });
        }
        if state_id == 26394 {
            return Some(CopperTrapdoor {
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#powered: true,
                r#waterlogged: false,
                r#open: false,
            });
        }
        if state_id == 26380 {
            return Some(CopperTrapdoor {
                r#facing: Facing::West,
                r#open: false,
                r#powered: false,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 26359 {
            return Some(CopperTrapdoor {
                r#half: Half::Bottom,
                r#powered: false,
                r#waterlogged: true,
                r#facing: Facing::South,
                r#open: true,
            });
        }
        if state_id == 26340 {
            return Some(CopperTrapdoor {
                r#powered: false,
                r#facing: Facing::North,
                r#half: Half::Top,
                r#open: false,
                r#waterlogged: false,
            });
        }
        if state_id == 26373 {
            return Some(CopperTrapdoor {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#powered: true,
                r#open: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 26396 {
            return Some(CopperTrapdoor {
                r#powered: false,
                r#waterlogged: false,
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#open: false,
            });
        }
        if state_id == 26362 {
            return Some(CopperTrapdoor {
                r#half: Half::Bottom,
                r#open: false,
                r#waterlogged: false,
                r#facing: Facing::South,
                r#powered: true,
            });
        }
        if state_id == 26360 {
            return Some(CopperTrapdoor {
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#powered: false,
                r#open: true,
                r#waterlogged: false,
            });
        }
        if state_id == 26395 {
            return Some(CopperTrapdoor {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#open: false,
                r#facing: Facing::East,
                r#powered: false,
            });
        }
        if state_id == 26348 {
            return Some(CopperTrapdoor {
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#powered: false,
                r#facing: Facing::North,
                r#open: false,
            });
        }
        if state_id == 26376 {
            return Some(CopperTrapdoor {
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#powered: false,
                r#open: true,
                r#waterlogged: false,
            });
        }
        if state_id == 26334 {
            return Some(CopperTrapdoor {
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::North,
                r#open: true,
                r#powered: true,
            });
        }
        return None;
    }
}

