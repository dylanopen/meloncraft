use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExposedCopperTrapdoor {
    pub powered: bool,
    pub waterlogged: bool,
    pub open: bool,
    pub r#facing: Facing,
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
    Top,
    Bottom,
}

impl BlockState for ExposedCopperTrapdoor {
    fn to_id(&self) -> i32 {
        if self.r#powered == false && self.r#waterlogged == true && self.r#open == true && self.r#facing == Facing::North && self.r#half == Half::Top { return 26399; }
        if self.r#facing == Facing::East && self.r#powered == false && self.r#open == false && self.r#half == Half::Bottom && self.r#waterlogged == false { return 26460; }
        if self.r#waterlogged == true && self.r#facing == Facing::South && self.r#powered == false && self.r#half == Half::Bottom && self.r#open == true { return 26423; }
        if self.r#open == false && self.r#waterlogged == true && self.r#half == Half::Top && self.r#powered == true && self.r#facing == Facing::North { return 26401; }
        if self.r#waterlogged == false && self.r#open == true && self.r#powered == true && self.r#half == Half::Bottom && self.r#facing == Facing::East { return 26454; }
        if self.r#open == true && self.r#half == Half::Top && self.r#waterlogged == true && self.r#facing == Facing::North && self.r#powered == true { return 26397; }
        if self.r#half == Half::Top && self.r#powered == true && self.r#open == false && self.r#facing == Facing::West && self.r#waterlogged == false { return 26434; }
        if self.r#half == Half::Top && self.r#facing == Facing::West && self.r#powered == false && self.r#waterlogged == true && self.r#open == true { return 26431; }
        if self.r#facing == Facing::East && self.r#open == true && self.r#half == Half::Top && self.r#powered == false && self.r#waterlogged == false { return 26448; }
        if self.r#waterlogged == false && self.r#half == Half::Bottom && self.r#powered == false && self.r#facing == Facing::West && self.r#open == true { return 26440; }
        if self.r#facing == Facing::East && self.r#half == Half::Bottom && self.r#powered == false && self.r#waterlogged == true && self.r#open == true { return 26455; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::South && self.r#open == false && self.r#powered == true && self.r#waterlogged == true { return 26425; }
        if self.r#waterlogged == false && self.r#open == false && self.r#facing == Facing::South && self.r#half == Half::Bottom && self.r#powered == false { return 26428; }
        if self.r#facing == Facing::South && self.r#open == true && self.r#half == Half::Bottom && self.r#powered == false && self.r#waterlogged == false { return 26424; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::West && self.r#waterlogged == true && self.r#open == true && self.r#powered == true { return 26437; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::West && self.r#open == false && self.r#powered == true && self.r#waterlogged == false { return 26442; }
        if self.r#half == Half::Bottom && self.r#open == false && self.r#facing == Facing::North && self.r#waterlogged == true && self.r#powered == false { return 26411; }
        if self.r#waterlogged == true && self.r#half == Half::Top && self.r#facing == Facing::East && self.r#open == true && self.r#powered == true { return 26445; }
        if self.r#half == Half::Top && self.r#facing == Facing::East && self.r#powered == true && self.r#waterlogged == false && self.r#open == true { return 26446; }
        if self.r#open == true && self.r#powered == false && self.r#waterlogged == false && self.r#half == Half::Bottom && self.r#facing == Facing::East { return 26456; }
        if self.r#open == true && self.r#facing == Facing::West && self.r#powered == false && self.r#half == Half::Top && self.r#waterlogged == false { return 26432; }
        if self.r#open == false && self.r#powered == true && self.r#waterlogged == true && self.r#facing == Facing::East && self.r#half == Half::Top { return 26449; }
        if self.r#facing == Facing::South && self.r#open == false && self.r#waterlogged == false && self.r#half == Half::Bottom && self.r#powered == true { return 26426; }
        if self.r#open == true && self.r#facing == Facing::North && self.r#half == Half::Bottom && self.r#powered == false && self.r#waterlogged == true { return 26407; }
        if self.r#facing == Facing::East && self.r#open == false && self.r#half == Half::Bottom && self.r#powered == true && self.r#waterlogged == false { return 26458; }
        if self.r#waterlogged == false && self.r#half == Half::Bottom && self.r#open == true && self.r#powered == true && self.r#facing == Facing::West { return 26438; }
        if self.r#powered == false && self.r#half == Half::Top && self.r#facing == Facing::East && self.r#waterlogged == true && self.r#open == true { return 26447; }
        if self.r#half == Half::Top && self.r#powered == true && self.r#open == true && self.r#facing == Facing::South && self.r#waterlogged == true { return 26413; }
        if self.r#powered == false && self.r#facing == Facing::North && self.r#open == false && self.r#waterlogged == false && self.r#half == Half::Top { return 26404; }
        if self.r#powered == false && self.r#waterlogged == true && self.r#half == Half::Top && self.r#open == true && self.r#facing == Facing::South { return 26415; }
        if self.r#facing == Facing::West && self.r#waterlogged == true && self.r#powered == false && self.r#open == false && self.r#half == Half::Bottom { return 26443; }
        if self.r#half == Half::Bottom && self.r#open == true && self.r#waterlogged == true && self.r#facing == Facing::North && self.r#powered == true { return 26405; }
        if self.r#facing == Facing::South && self.r#powered == true && self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#open == true { return 26422; }
        if self.r#facing == Facing::West && self.r#half == Half::Bottom && self.r#open == false && self.r#powered == true && self.r#waterlogged == true { return 26441; }
        if self.r#half == Half::Top && self.r#facing == Facing::South && self.r#powered == false && self.r#waterlogged == false && self.r#open == true { return 26416; }
        if self.r#waterlogged == false && self.r#facing == Facing::South && self.r#open == false && self.r#half == Half::Top && self.r#powered == true { return 26418; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::North && self.r#powered == false && self.r#waterlogged == false && self.r#open == false { return 26412; }
        if self.r#facing == Facing::North && self.r#half == Half::Bottom && self.r#powered == true && self.r#waterlogged == true && self.r#open == false { return 26409; }
        if self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#facing == Facing::East && self.r#open == false && self.r#powered == false { return 26459; }
        if self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#facing == Facing::East && self.r#powered == true && self.r#open == true { return 26453; }
        if self.r#open == false && self.r#facing == Facing::West && self.r#half == Half::Top && self.r#powered == false && self.r#waterlogged == true { return 26435; }
        if self.r#open == false && self.r#half == Half::Top && self.r#facing == Facing::North && self.r#powered == false && self.r#waterlogged == true { return 26403; }
        if self.r#half == Half::Bottom && self.r#waterlogged == true && self.r#facing == Facing::South && self.r#open == true && self.r#powered == true { return 26421; }
        if self.r#open == true && self.r#facing == Facing::North && self.r#powered == true && self.r#waterlogged == false && self.r#half == Half::Bottom { return 26406; }
        if self.r#facing == Facing::West && self.r#waterlogged == false && self.r#open == true && self.r#half == Half::Top && self.r#powered == true { return 26430; }
        if self.r#facing == Facing::North && self.r#half == Half::Bottom && self.r#open == false && self.r#waterlogged == false && self.r#powered == true { return 26410; }
        if self.r#open == false && self.r#waterlogged == false && self.r#half == Half::Top && self.r#facing == Facing::North && self.r#powered == true { return 26402; }
        if self.r#waterlogged == true && self.r#open == false && self.r#facing == Facing::West && self.r#half == Half::Top && self.r#powered == true { return 26433; }
        if self.r#facing == Facing::West && self.r#waterlogged == false && self.r#half == Half::Bottom && self.r#open == false && self.r#powered == false { return 26444; }
        if self.r#powered == true && self.r#open == true && self.r#waterlogged == false && self.r#facing == Facing::North && self.r#half == Half::Top { return 26398; }
        if self.r#facing == Facing::North && self.r#waterlogged == false && self.r#open == true && self.r#half == Half::Top && self.r#powered == false { return 26400; }
        if self.r#waterlogged == true && self.r#half == Half::Top && self.r#facing == Facing::South && self.r#open == false && self.r#powered == false { return 26419; }
        if self.r#powered == false && self.r#open == false && self.r#facing == Facing::West && self.r#half == Half::Top && self.r#waterlogged == false { return 26436; }
        if self.r#powered == false && self.r#open == false && self.r#half == Half::Top && self.r#facing == Facing::East && self.r#waterlogged == false { return 26452; }
        if self.r#half == Half::Bottom && self.r#open == false && self.r#powered == true && self.r#facing == Facing::East && self.r#waterlogged == true { return 26457; }
        if self.r#waterlogged == false && self.r#half == Half::Top && self.r#facing == Facing::East && self.r#powered == true && self.r#open == false { return 26450; }
        if self.r#waterlogged == true && self.r#half == Half::Top && self.r#facing == Facing::East && self.r#powered == false && self.r#open == false { return 26451; }
        if self.r#waterlogged == false && self.r#facing == Facing::North && self.r#half == Half::Bottom && self.r#open == true && self.r#powered == false { return 26408; }
        if self.r#waterlogged == false && self.r#powered == false && self.r#facing == Facing::South && self.r#half == Half::Top && self.r#open == false { return 26420; }
        if self.r#half == Half::Top && self.r#powered == true && self.r#waterlogged == true && self.r#facing == Facing::South && self.r#open == false { return 26417; }
        if self.r#facing == Facing::South && self.r#open == false && self.r#powered == false && self.r#waterlogged == true && self.r#half == Half::Bottom { return 26427; }
        if self.r#facing == Facing::West && self.r#half == Half::Bottom && self.r#open == true && self.r#powered == false && self.r#waterlogged == true { return 26439; }
        if self.r#facing == Facing::South && self.r#half == Half::Top && self.r#powered == true && self.r#waterlogged == false && self.r#open == true { return 26414; }
        if self.r#half == Half::Top && self.r#waterlogged == true && self.r#facing == Facing::West && self.r#powered == true && self.r#open == true { return 26429; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 26399 {
            return Some(ExposedCopperTrapdoor {
                r#powered: false,
                r#waterlogged: true,
                r#open: true,
                r#facing: Facing::North,
                r#half: Half::Top,
            });
        }
        if state_id == 26460 {
            return Some(ExposedCopperTrapdoor {
                r#facing: Facing::East,
                r#powered: false,
                r#open: false,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 26423 {
            return Some(ExposedCopperTrapdoor {
                r#waterlogged: true,
                r#facing: Facing::South,
                r#powered: false,
                r#half: Half::Bottom,
                r#open: true,
            });
        }
        if state_id == 26401 {
            return Some(ExposedCopperTrapdoor {
                r#open: false,
                r#waterlogged: true,
                r#half: Half::Top,
                r#powered: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 26454 {
            return Some(ExposedCopperTrapdoor {
                r#waterlogged: false,
                r#open: true,
                r#powered: true,
                r#half: Half::Bottom,
                r#facing: Facing::East,
            });
        }
        if state_id == 26397 {
            return Some(ExposedCopperTrapdoor {
                r#open: true,
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::North,
                r#powered: true,
            });
        }
        if state_id == 26434 {
            return Some(ExposedCopperTrapdoor {
                r#half: Half::Top,
                r#powered: true,
                r#open: false,
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 26431 {
            return Some(ExposedCopperTrapdoor {
                r#half: Half::Top,
                r#facing: Facing::West,
                r#powered: false,
                r#waterlogged: true,
                r#open: true,
            });
        }
        if state_id == 26448 {
            return Some(ExposedCopperTrapdoor {
                r#facing: Facing::East,
                r#open: true,
                r#half: Half::Top,
                r#powered: false,
                r#waterlogged: false,
            });
        }
        if state_id == 26440 {
            return Some(ExposedCopperTrapdoor {
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#powered: false,
                r#facing: Facing::West,
                r#open: true,
            });
        }
        if state_id == 26455 {
            return Some(ExposedCopperTrapdoor {
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#powered: false,
                r#waterlogged: true,
                r#open: true,
            });
        }
        if state_id == 26425 {
            return Some(ExposedCopperTrapdoor {
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#open: false,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 26428 {
            return Some(ExposedCopperTrapdoor {
                r#waterlogged: false,
                r#open: false,
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#powered: false,
            });
        }
        if state_id == 26424 {
            return Some(ExposedCopperTrapdoor {
                r#facing: Facing::South,
                r#open: true,
                r#half: Half::Bottom,
                r#powered: false,
                r#waterlogged: false,
            });
        }
        if state_id == 26437 {
            return Some(ExposedCopperTrapdoor {
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#waterlogged: true,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 26442 {
            return Some(ExposedCopperTrapdoor {
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#open: false,
                r#powered: true,
                r#waterlogged: false,
            });
        }
        if state_id == 26411 {
            return Some(ExposedCopperTrapdoor {
                r#half: Half::Bottom,
                r#open: false,
                r#facing: Facing::North,
                r#waterlogged: true,
                r#powered: false,
            });
        }
        if state_id == 26445 {
            return Some(ExposedCopperTrapdoor {
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::East,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 26446 {
            return Some(ExposedCopperTrapdoor {
                r#half: Half::Top,
                r#facing: Facing::East,
                r#powered: true,
                r#waterlogged: false,
                r#open: true,
            });
        }
        if state_id == 26456 {
            return Some(ExposedCopperTrapdoor {
                r#open: true,
                r#powered: false,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#facing: Facing::East,
            });
        }
        if state_id == 26432 {
            return Some(ExposedCopperTrapdoor {
                r#open: true,
                r#facing: Facing::West,
                r#powered: false,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 26449 {
            return Some(ExposedCopperTrapdoor {
                r#open: false,
                r#powered: true,
                r#waterlogged: true,
                r#facing: Facing::East,
                r#half: Half::Top,
            });
        }
        if state_id == 26426 {
            return Some(ExposedCopperTrapdoor {
                r#facing: Facing::South,
                r#open: false,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#powered: true,
            });
        }
        if state_id == 26407 {
            return Some(ExposedCopperTrapdoor {
                r#open: true,
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 26458 {
            return Some(ExposedCopperTrapdoor {
                r#facing: Facing::East,
                r#open: false,
                r#half: Half::Bottom,
                r#powered: true,
                r#waterlogged: false,
            });
        }
        if state_id == 26438 {
            return Some(ExposedCopperTrapdoor {
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#open: true,
                r#powered: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 26447 {
            return Some(ExposedCopperTrapdoor {
                r#powered: false,
                r#half: Half::Top,
                r#facing: Facing::East,
                r#waterlogged: true,
                r#open: true,
            });
        }
        if state_id == 26413 {
            return Some(ExposedCopperTrapdoor {
                r#half: Half::Top,
                r#powered: true,
                r#open: true,
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 26404 {
            return Some(ExposedCopperTrapdoor {
                r#powered: false,
                r#facing: Facing::North,
                r#open: false,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 26415 {
            return Some(ExposedCopperTrapdoor {
                r#powered: false,
                r#waterlogged: true,
                r#half: Half::Top,
                r#open: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 26443 {
            return Some(ExposedCopperTrapdoor {
                r#facing: Facing::West,
                r#waterlogged: true,
                r#powered: false,
                r#open: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 26405 {
            return Some(ExposedCopperTrapdoor {
                r#half: Half::Bottom,
                r#open: true,
                r#waterlogged: true,
                r#facing: Facing::North,
                r#powered: true,
            });
        }
        if state_id == 26422 {
            return Some(ExposedCopperTrapdoor {
                r#facing: Facing::South,
                r#powered: true,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#open: true,
            });
        }
        if state_id == 26441 {
            return Some(ExposedCopperTrapdoor {
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#open: false,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 26416 {
            return Some(ExposedCopperTrapdoor {
                r#half: Half::Top,
                r#facing: Facing::South,
                r#powered: false,
                r#waterlogged: false,
                r#open: true,
            });
        }
        if state_id == 26418 {
            return Some(ExposedCopperTrapdoor {
                r#waterlogged: false,
                r#facing: Facing::South,
                r#open: false,
                r#half: Half::Top,
                r#powered: true,
            });
        }
        if state_id == 26412 {
            return Some(ExposedCopperTrapdoor {
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#powered: false,
                r#waterlogged: false,
                r#open: false,
            });
        }
        if state_id == 26409 {
            return Some(ExposedCopperTrapdoor {
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#powered: true,
                r#waterlogged: true,
                r#open: false,
            });
        }
        if state_id == 26459 {
            return Some(ExposedCopperTrapdoor {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 26453 {
            return Some(ExposedCopperTrapdoor {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#powered: true,
                r#open: true,
            });
        }
        if state_id == 26435 {
            return Some(ExposedCopperTrapdoor {
                r#open: false,
                r#facing: Facing::West,
                r#half: Half::Top,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 26403 {
            return Some(ExposedCopperTrapdoor {
                r#open: false,
                r#half: Half::Top,
                r#facing: Facing::North,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 26421 {
            return Some(ExposedCopperTrapdoor {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::South,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 26406 {
            return Some(ExposedCopperTrapdoor {
                r#open: true,
                r#facing: Facing::North,
                r#powered: true,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 26430 {
            return Some(ExposedCopperTrapdoor {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#open: true,
                r#half: Half::Top,
                r#powered: true,
            });
        }
        if state_id == 26410 {
            return Some(ExposedCopperTrapdoor {
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#open: false,
                r#waterlogged: false,
                r#powered: true,
            });
        }
        if state_id == 26402 {
            return Some(ExposedCopperTrapdoor {
                r#open: false,
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::North,
                r#powered: true,
            });
        }
        if state_id == 26433 {
            return Some(ExposedCopperTrapdoor {
                r#waterlogged: true,
                r#open: false,
                r#facing: Facing::West,
                r#half: Half::Top,
                r#powered: true,
            });
        }
        if state_id == 26444 {
            return Some(ExposedCopperTrapdoor {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 26398 {
            return Some(ExposedCopperTrapdoor {
                r#powered: true,
                r#open: true,
                r#waterlogged: false,
                r#facing: Facing::North,
                r#half: Half::Top,
            });
        }
        if state_id == 26400 {
            return Some(ExposedCopperTrapdoor {
                r#facing: Facing::North,
                r#waterlogged: false,
                r#open: true,
                r#half: Half::Top,
                r#powered: false,
            });
        }
        if state_id == 26419 {
            return Some(ExposedCopperTrapdoor {
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::South,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 26436 {
            return Some(ExposedCopperTrapdoor {
                r#powered: false,
                r#open: false,
                r#facing: Facing::West,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 26452 {
            return Some(ExposedCopperTrapdoor {
                r#powered: false,
                r#open: false,
                r#half: Half::Top,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 26457 {
            return Some(ExposedCopperTrapdoor {
                r#half: Half::Bottom,
                r#open: false,
                r#powered: true,
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 26450 {
            return Some(ExposedCopperTrapdoor {
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::East,
                r#powered: true,
                r#open: false,
            });
        }
        if state_id == 26451 {
            return Some(ExposedCopperTrapdoor {
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::East,
                r#powered: false,
                r#open: false,
            });
        }
        if state_id == 26408 {
            return Some(ExposedCopperTrapdoor {
                r#waterlogged: false,
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#open: true,
                r#powered: false,
            });
        }
        if state_id == 26420 {
            return Some(ExposedCopperTrapdoor {
                r#waterlogged: false,
                r#powered: false,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#open: false,
            });
        }
        if state_id == 26417 {
            return Some(ExposedCopperTrapdoor {
                r#half: Half::Top,
                r#powered: true,
                r#waterlogged: true,
                r#facing: Facing::South,
                r#open: false,
            });
        }
        if state_id == 26427 {
            return Some(ExposedCopperTrapdoor {
                r#facing: Facing::South,
                r#open: false,
                r#powered: false,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 26439 {
            return Some(ExposedCopperTrapdoor {
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#open: true,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 26414 {
            return Some(ExposedCopperTrapdoor {
                r#facing: Facing::South,
                r#half: Half::Top,
                r#powered: true,
                r#waterlogged: false,
                r#open: true,
            });
        }
        if state_id == 26429 {
            return Some(ExposedCopperTrapdoor {
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#powered: true,
                r#open: true,
            });
        }
        return None;
    }
}

