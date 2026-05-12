use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExposedCopperTrapdoor {
    pub r#half: Half,
    pub powered: bool,
    pub r#facing: Facing,
    pub open: bool,
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

impl BlockState for ExposedCopperTrapdoor {
    fn to_id(self) -> i32 {
        if block_state.r#facing == Facing::East && block_state.r#half == Half::Top && block_state.r#powered == true && block_state.r#open == true && block_state.r#waterlogged == false { return 26446; }
        if block_state.r#half == Half::Bottom && block_state.r#powered == false && block_state.r#waterlogged == true && block_state.r#open == false && block_state.r#facing == Facing::West { return 26443; }
        if block_state.r#half == Half::Bottom && block_state.r#open == true && block_state.r#facing == Facing::East && block_state.r#powered == true && block_state.r#waterlogged == true { return 26453; }
        if block_state.r#half == Half::Bottom && block_state.r#powered == false && block_state.r#waterlogged == false && block_state.r#open == true && block_state.r#facing == Facing::West { return 26440; }
        if block_state.r#facing == Facing::North && block_state.r#half == Half::Bottom && block_state.r#open == true && block_state.r#powered == false && block_state.r#waterlogged == false { return 26408; }
        if block_state.r#half == Half::Top && block_state.r#open == false && block_state.r#powered == false && block_state.r#waterlogged == false && block_state.r#facing == Facing::South { return 26420; }
        if block_state.r#facing == Facing::East && block_state.r#open == true && block_state.r#half == Half::Bottom && block_state.r#powered == true && block_state.r#waterlogged == false { return 26454; }
        if block_state.r#waterlogged == true && block_state.r#half == Half::Top && block_state.r#facing == Facing::West && block_state.r#powered == true && block_state.r#open == true { return 26429; }
        if block_state.r#waterlogged == true && block_state.r#half == Half::Bottom && block_state.r#powered == true && block_state.r#facing == Facing::West && block_state.r#open == true { return 26437; }
        if block_state.r#half == Half::Bottom && block_state.r#open == false && block_state.r#facing == Facing::West && block_state.r#powered == false && block_state.r#waterlogged == false { return 26444; }
        if block_state.r#facing == Facing::North && block_state.r#open == true && block_state.r#powered == true && block_state.r#waterlogged == false && block_state.r#half == Half::Top { return 26398; }
        if block_state.r#half == Half::Top && block_state.r#waterlogged == false && block_state.r#open == false && block_state.r#powered == true && block_state.r#facing == Facing::East { return 26450; }
        if block_state.r#waterlogged == true && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::West && block_state.r#open == false && block_state.r#powered == true { return 26441; }
        if block_state.r#facing == Facing::North && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true && block_state.r#open == true && block_state.r#powered == false { return 26407; }
        if block_state.r#powered == false && block_state.r#open == false && block_state.r#facing == Facing::East && block_state.r#waterlogged == false && block_state.r#half == Half::Bottom { return 26460; }
        if block_state.r#facing == Facing::South && block_state.r#half == Half::Bottom && block_state.r#powered == true && block_state.r#open == true && block_state.r#waterlogged == false { return 26422; }
        if block_state.r#half == Half::Top && block_state.r#waterlogged == true && block_state.r#open == true && block_state.r#facing == Facing::South && block_state.r#powered == false { return 26415; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::North && block_state.r#open == false && block_state.r#powered == true && block_state.r#half == Half::Bottom { return 26409; }
        if block_state.r#powered == false && block_state.r#waterlogged == true && block_state.r#facing == Facing::East && block_state.r#half == Half::Top && block_state.r#open == true { return 26447; }
        if block_state.r#open == true && block_state.r#facing == Facing::North && block_state.r#waterlogged == false && block_state.r#half == Half::Bottom && block_state.r#powered == true { return 26406; }
        if block_state.r#open == true && block_state.r#waterlogged == true && block_state.r#facing == Facing::East && block_state.r#half == Half::Bottom && block_state.r#powered == false { return 26455; }
        if block_state.r#open == false && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::North && block_state.r#powered == false && block_state.r#waterlogged == true { return 26411; }
        if block_state.r#half == Half::Bottom && block_state.r#open == false && block_state.r#powered == false && block_state.r#facing == Facing::East && block_state.r#waterlogged == true { return 26459; }
        if block_state.r#half == Half::Top && block_state.r#powered == false && block_state.r#waterlogged == true && block_state.r#facing == Facing::South && block_state.r#open == false { return 26419; }
        if block_state.r#open == true && block_state.r#facing == Facing::East && block_state.r#powered == false && block_state.r#half == Half::Top && block_state.r#waterlogged == false { return 26448; }
        if block_state.r#open == true && block_state.r#facing == Facing::West && block_state.r#powered == false && block_state.r#half == Half::Top && block_state.r#waterlogged == true { return 26431; }
        if block_state.r#powered == false && block_state.r#half == Half::Top && block_state.r#open == false && block_state.r#waterlogged == true && block_state.r#facing == Facing::North { return 26403; }
        if block_state.r#half == Half::Bottom && block_state.r#open == true && block_state.r#facing == Facing::North && block_state.r#powered == true && block_state.r#waterlogged == true { return 26405; }
        if block_state.r#half == Half::Bottom && block_state.r#powered == false && block_state.r#facing == Facing::West && block_state.r#open == true && block_state.r#waterlogged == true { return 26439; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::South && block_state.r#powered == true && block_state.r#open == false && block_state.r#half == Half::Top { return 26417; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == false && block_state.r#half == Half::Top && block_state.r#open == true && block_state.r#powered == false { return 26416; }
        if block_state.r#half == Half::Top && block_state.r#powered == false && block_state.r#open == false && block_state.r#waterlogged == true && block_state.r#facing == Facing::West { return 26435; }
        if block_state.r#facing == Facing::West && block_state.r#open == false && block_state.r#powered == true && block_state.r#waterlogged == true && block_state.r#half == Half::Top { return 26433; }
        if block_state.r#half == Half::Top && block_state.r#waterlogged == false && block_state.r#facing == Facing::West && block_state.r#open == false && block_state.r#powered == false { return 26436; }
        if block_state.r#waterlogged == false && block_state.r#half == Half::Top && block_state.r#powered == false && block_state.r#open == false && block_state.r#facing == Facing::East { return 26452; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == false && block_state.r#powered == true && block_state.r#half == Half::Bottom && block_state.r#open == false { return 26458; }
        if block_state.r#powered == true && block_state.r#waterlogged == false && block_state.r#open == false && block_state.r#facing == Facing::West && block_state.r#half == Half::Bottom { return 26442; }
        if block_state.r#powered == true && block_state.r#facing == Facing::East && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true && block_state.r#open == false { return 26457; }
        if block_state.r#half == Half::Top && block_state.r#facing == Facing::North && block_state.r#powered == false && block_state.r#open == false && block_state.r#waterlogged == false { return 26404; }
        if block_state.r#half == Half::Top && block_state.r#open == true && block_state.r#powered == false && block_state.r#facing == Facing::North && block_state.r#waterlogged == false { return 26400; }
        if block_state.r#half == Half::Bottom && block_state.r#open == false && block_state.r#facing == Facing::North && block_state.r#powered == true && block_state.r#waterlogged == false { return 26410; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == false && block_state.r#open == false && block_state.r#powered == true && block_state.r#half == Half::Top { return 26418; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == false && block_state.r#half == Half::Top && block_state.r#powered == true && block_state.r#open == true { return 26414; }
        if block_state.r#open == false && block_state.r#facing == Facing::North && block_state.r#waterlogged == true && block_state.r#powered == true && block_state.r#half == Half::Top { return 26401; }
        if block_state.r#open == false && block_state.r#powered == true && block_state.r#facing == Facing::West && block_state.r#waterlogged == false && block_state.r#half == Half::Top { return 26434; }
        if block_state.r#half == Half::Top && block_state.r#waterlogged == true && block_state.r#powered == false && block_state.r#facing == Facing::North && block_state.r#open == true { return 26399; }
        if block_state.r#powered == false && block_state.r#facing == Facing::South && block_state.r#waterlogged == true && block_state.r#half == Half::Bottom && block_state.r#open == true { return 26423; }
        if block_state.r#powered == true && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::South && block_state.r#open == false && block_state.r#waterlogged == false { return 26426; }
        if block_state.r#half == Half::Top && block_state.r#waterlogged == true && block_state.r#facing == Facing::East && block_state.r#open == false && block_state.r#powered == false { return 26451; }
        if block_state.r#powered == true && block_state.r#facing == Facing::East && block_state.r#half == Half::Top && block_state.r#waterlogged == true && block_state.r#open == true { return 26445; }
        if block_state.r#half == Half::Bottom && block_state.r#powered == false && block_state.r#waterlogged == false && block_state.r#facing == Facing::South && block_state.r#open == true { return 26424; }
        if block_state.r#open == false && block_state.r#powered == false && block_state.r#waterlogged == true && block_state.r#facing == Facing::South && block_state.r#half == Half::Bottom { return 26427; }
        if block_state.r#powered == true && block_state.r#open == true && block_state.r#facing == Facing::West && block_state.r#waterlogged == false && block_state.r#half == Half::Bottom { return 26438; }
        if block_state.r#half == Half::Bottom && block_state.r#waterlogged == false && block_state.r#powered == false && block_state.r#facing == Facing::East && block_state.r#open == true { return 26456; }
        if block_state.r#facing == Facing::South && block_state.r#half == Half::Bottom && block_state.r#open == false && block_state.r#powered == true && block_state.r#waterlogged == true { return 26425; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::South && block_state.r#powered == true && block_state.r#half == Half::Bottom && block_state.r#open == true { return 26421; }
        if block_state.r#half == Half::Top && block_state.r#waterlogged == false && block_state.r#facing == Facing::West && block_state.r#open == true && block_state.r#powered == true { return 26430; }
        if block_state.r#half == Half::Bottom && block_state.r#waterlogged == false && block_state.r#open == false && block_state.r#powered == false && block_state.r#facing == Facing::South { return 26428; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == true && block_state.r#open == true && block_state.r#half == Half::Top && block_state.r#powered == true { return 26397; }
        if block_state.r#waterlogged == false && block_state.r#half == Half::Top && block_state.r#powered == false && block_state.r#facing == Facing::West && block_state.r#open == true { return 26432; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == true && block_state.r#powered == true && block_state.r#open == true && block_state.r#half == Half::Top { return 26413; }
        if block_state.r#half == Half::Top && block_state.r#facing == Facing::North && block_state.r#open == false && block_state.r#powered == true && block_state.r#waterlogged == false { return 26402; }
        if block_state.r#waterlogged == true && block_state.r#open == false && block_state.r#facing == Facing::East && block_state.r#powered == true && block_state.r#half == Half::Top { return 26449; }
        if block_state.r#facing == Facing::North && block_state.r#open == false && block_state.r#half == Half::Bottom && block_state.r#powered == false && block_state.r#waterlogged == false { return 26412; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 26446 {
            return Some(ExposedCopperTrapdoor {
                r#facing: Facing::East,
                r#half: Half::Top,
                r#powered: true,
                r#open: true,
                r#waterlogged: false,
            });
        }
        if state_id == 26443 {
            return Some(ExposedCopperTrapdoor {
                r#half: Half::Bottom,
                r#powered: false,
                r#waterlogged: true,
                r#open: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 26453 {
            return Some(ExposedCopperTrapdoor {
                r#half: Half::Bottom,
                r#open: true,
                r#facing: Facing::East,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 26440 {
            return Some(ExposedCopperTrapdoor {
                r#half: Half::Bottom,
                r#powered: false,
                r#waterlogged: false,
                r#open: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 26408 {
            return Some(ExposedCopperTrapdoor {
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#open: true,
                r#powered: false,
                r#waterlogged: false,
            });
        }
        if state_id == 26420 {
            return Some(ExposedCopperTrapdoor {
                r#half: Half::Top,
                r#open: false,
                r#powered: false,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 26454 {
            return Some(ExposedCopperTrapdoor {
                r#facing: Facing::East,
                r#open: true,
                r#half: Half::Bottom,
                r#powered: true,
                r#waterlogged: false,
            });
        }
        if state_id == 26429 {
            return Some(ExposedCopperTrapdoor {
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::West,
                r#powered: true,
                r#open: true,
            });
        }
        if state_id == 26437 {
            return Some(ExposedCopperTrapdoor {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#powered: true,
                r#facing: Facing::West,
                r#open: true,
            });
        }
        if state_id == 26444 {
            return Some(ExposedCopperTrapdoor {
                r#half: Half::Bottom,
                r#open: false,
                r#facing: Facing::West,
                r#powered: false,
                r#waterlogged: false,
            });
        }
        if state_id == 26398 {
            return Some(ExposedCopperTrapdoor {
                r#facing: Facing::North,
                r#open: true,
                r#powered: true,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 26450 {
            return Some(ExposedCopperTrapdoor {
                r#half: Half::Top,
                r#waterlogged: false,
                r#open: false,
                r#powered: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 26441 {
            return Some(ExposedCopperTrapdoor {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#open: false,
                r#powered: true,
            });
        }
        if state_id == 26407 {
            return Some(ExposedCopperTrapdoor {
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#open: true,
                r#powered: false,
            });
        }
        if state_id == 26460 {
            return Some(ExposedCopperTrapdoor {
                r#powered: false,
                r#open: false,
                r#facing: Facing::East,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 26422 {
            return Some(ExposedCopperTrapdoor {
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#powered: true,
                r#open: true,
                r#waterlogged: false,
            });
        }
        if state_id == 26415 {
            return Some(ExposedCopperTrapdoor {
                r#half: Half::Top,
                r#waterlogged: true,
                r#open: true,
                r#facing: Facing::South,
                r#powered: false,
            });
        }
        if state_id == 26409 {
            return Some(ExposedCopperTrapdoor {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#open: false,
                r#powered: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 26447 {
            return Some(ExposedCopperTrapdoor {
                r#powered: false,
                r#waterlogged: true,
                r#facing: Facing::East,
                r#half: Half::Top,
                r#open: true,
            });
        }
        if state_id == 26406 {
            return Some(ExposedCopperTrapdoor {
                r#open: true,
                r#facing: Facing::North,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#powered: true,
            });
        }
        if state_id == 26455 {
            return Some(ExposedCopperTrapdoor {
                r#open: true,
                r#waterlogged: true,
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#powered: false,
            });
        }
        if state_id == 26411 {
            return Some(ExposedCopperTrapdoor {
                r#open: false,
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 26459 {
            return Some(ExposedCopperTrapdoor {
                r#half: Half::Bottom,
                r#open: false,
                r#powered: false,
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 26419 {
            return Some(ExposedCopperTrapdoor {
                r#half: Half::Top,
                r#powered: false,
                r#waterlogged: true,
                r#facing: Facing::South,
                r#open: false,
            });
        }
        if state_id == 26448 {
            return Some(ExposedCopperTrapdoor {
                r#open: true,
                r#facing: Facing::East,
                r#powered: false,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 26431 {
            return Some(ExposedCopperTrapdoor {
                r#open: true,
                r#facing: Facing::West,
                r#powered: false,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 26403 {
            return Some(ExposedCopperTrapdoor {
                r#powered: false,
                r#half: Half::Top,
                r#open: false,
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 26405 {
            return Some(ExposedCopperTrapdoor {
                r#half: Half::Bottom,
                r#open: true,
                r#facing: Facing::North,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 26439 {
            return Some(ExposedCopperTrapdoor {
                r#half: Half::Bottom,
                r#powered: false,
                r#facing: Facing::West,
                r#open: true,
                r#waterlogged: true,
            });
        }
        if state_id == 26417 {
            return Some(ExposedCopperTrapdoor {
                r#waterlogged: true,
                r#facing: Facing::South,
                r#powered: true,
                r#open: false,
                r#half: Half::Top,
            });
        }
        if state_id == 26416 {
            return Some(ExposedCopperTrapdoor {
                r#facing: Facing::South,
                r#waterlogged: false,
                r#half: Half::Top,
                r#open: true,
                r#powered: false,
            });
        }
        if state_id == 26435 {
            return Some(ExposedCopperTrapdoor {
                r#half: Half::Top,
                r#powered: false,
                r#open: false,
                r#waterlogged: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 26433 {
            return Some(ExposedCopperTrapdoor {
                r#facing: Facing::West,
                r#open: false,
                r#powered: true,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 26436 {
            return Some(ExposedCopperTrapdoor {
                r#half: Half::Top,
                r#waterlogged: false,
                r#facing: Facing::West,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 26452 {
            return Some(ExposedCopperTrapdoor {
                r#waterlogged: false,
                r#half: Half::Top,
                r#powered: false,
                r#open: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 26458 {
            return Some(ExposedCopperTrapdoor {
                r#facing: Facing::East,
                r#waterlogged: false,
                r#powered: true,
                r#half: Half::Bottom,
                r#open: false,
            });
        }
        if state_id == 26442 {
            return Some(ExposedCopperTrapdoor {
                r#powered: true,
                r#waterlogged: false,
                r#open: false,
                r#facing: Facing::West,
                r#half: Half::Bottom,
            });
        }
        if state_id == 26457 {
            return Some(ExposedCopperTrapdoor {
                r#powered: true,
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#open: false,
            });
        }
        if state_id == 26404 {
            return Some(ExposedCopperTrapdoor {
                r#half: Half::Top,
                r#facing: Facing::North,
                r#powered: false,
                r#open: false,
                r#waterlogged: false,
            });
        }
        if state_id == 26400 {
            return Some(ExposedCopperTrapdoor {
                r#half: Half::Top,
                r#open: true,
                r#powered: false,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 26410 {
            return Some(ExposedCopperTrapdoor {
                r#half: Half::Bottom,
                r#open: false,
                r#facing: Facing::North,
                r#powered: true,
                r#waterlogged: false,
            });
        }
        if state_id == 26418 {
            return Some(ExposedCopperTrapdoor {
                r#facing: Facing::South,
                r#waterlogged: false,
                r#open: false,
                r#powered: true,
                r#half: Half::Top,
            });
        }
        if state_id == 26414 {
            return Some(ExposedCopperTrapdoor {
                r#facing: Facing::South,
                r#waterlogged: false,
                r#half: Half::Top,
                r#powered: true,
                r#open: true,
            });
        }
        if state_id == 26401 {
            return Some(ExposedCopperTrapdoor {
                r#open: false,
                r#facing: Facing::North,
                r#waterlogged: true,
                r#powered: true,
                r#half: Half::Top,
            });
        }
        if state_id == 26434 {
            return Some(ExposedCopperTrapdoor {
                r#open: false,
                r#powered: true,
                r#facing: Facing::West,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 26399 {
            return Some(ExposedCopperTrapdoor {
                r#half: Half::Top,
                r#waterlogged: true,
                r#powered: false,
                r#facing: Facing::North,
                r#open: true,
            });
        }
        if state_id == 26423 {
            return Some(ExposedCopperTrapdoor {
                r#powered: false,
                r#facing: Facing::South,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#open: true,
            });
        }
        if state_id == 26426 {
            return Some(ExposedCopperTrapdoor {
                r#powered: true,
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#open: false,
                r#waterlogged: false,
            });
        }
        if state_id == 26451 {
            return Some(ExposedCopperTrapdoor {
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::East,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 26445 {
            return Some(ExposedCopperTrapdoor {
                r#powered: true,
                r#facing: Facing::East,
                r#half: Half::Top,
                r#waterlogged: true,
                r#open: true,
            });
        }
        if state_id == 26424 {
            return Some(ExposedCopperTrapdoor {
                r#half: Half::Bottom,
                r#powered: false,
                r#waterlogged: false,
                r#facing: Facing::South,
                r#open: true,
            });
        }
        if state_id == 26427 {
            return Some(ExposedCopperTrapdoor {
                r#open: false,
                r#powered: false,
                r#waterlogged: true,
                r#facing: Facing::South,
                r#half: Half::Bottom,
            });
        }
        if state_id == 26438 {
            return Some(ExposedCopperTrapdoor {
                r#powered: true,
                r#open: true,
                r#facing: Facing::West,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 26456 {
            return Some(ExposedCopperTrapdoor {
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#powered: false,
                r#facing: Facing::East,
                r#open: true,
            });
        }
        if state_id == 26425 {
            return Some(ExposedCopperTrapdoor {
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#open: false,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 26421 {
            return Some(ExposedCopperTrapdoor {
                r#waterlogged: true,
                r#facing: Facing::South,
                r#powered: true,
                r#half: Half::Bottom,
                r#open: true,
            });
        }
        if state_id == 26430 {
            return Some(ExposedCopperTrapdoor {
                r#half: Half::Top,
                r#waterlogged: false,
                r#facing: Facing::West,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 26428 {
            return Some(ExposedCopperTrapdoor {
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#open: false,
                r#powered: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 26397 {
            return Some(ExposedCopperTrapdoor {
                r#facing: Facing::North,
                r#waterlogged: true,
                r#open: true,
                r#half: Half::Top,
                r#powered: true,
            });
        }
        if state_id == 26432 {
            return Some(ExposedCopperTrapdoor {
                r#waterlogged: false,
                r#half: Half::Top,
                r#powered: false,
                r#facing: Facing::West,
                r#open: true,
            });
        }
        if state_id == 26413 {
            return Some(ExposedCopperTrapdoor {
                r#facing: Facing::South,
                r#waterlogged: true,
                r#powered: true,
                r#open: true,
                r#half: Half::Top,
            });
        }
        if state_id == 26402 {
            return Some(ExposedCopperTrapdoor {
                r#half: Half::Top,
                r#facing: Facing::North,
                r#open: false,
                r#powered: true,
                r#waterlogged: false,
            });
        }
        if state_id == 26449 {
            return Some(ExposedCopperTrapdoor {
                r#waterlogged: true,
                r#open: false,
                r#facing: Facing::East,
                r#powered: true,
                r#half: Half::Top,
            });
        }
        if state_id == 26412 {
            return Some(ExposedCopperTrapdoor {
                r#facing: Facing::North,
                r#open: false,
                r#half: Half::Bottom,
                r#powered: false,
                r#waterlogged: false,
            });
        }
        return None;
    }
}

