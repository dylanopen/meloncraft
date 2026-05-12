use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MangroveDoor {
    pub r#facing: Facing,
    pub r#half: Half,
    pub powered: bool,
    pub open: bool,
    pub r#hinge: Hinge,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Hinge {
    Left,
    Right,
}

impl BlockState for MangroveDoor {
    fn to_id(self) -> i32 {
        if block_state.r#open == true && block_state.r#hinge == Hinge::Right && block_state.r#half == Half::Lower && block_state.r#powered == false && block_state.r#facing == Facing::South { return 14335; }
        if block_state.r#powered == false && block_state.r#hinge == Hinge::Right && block_state.r#facing == Facing::North && block_state.r#half == Half::Upper && block_state.r#open == false { return 14313; }
        if block_state.r#hinge == Hinge::Right && block_state.r#facing == Facing::East && block_state.r#half == Half::Lower && block_state.r#powered == false && block_state.r#open == true { return 14367; }
        if block_state.r#open == true && block_state.r#powered == false && block_state.r#hinge == Hinge::Right && block_state.r#half == Half::Lower && block_state.r#facing == Facing::West { return 14351; }
        if block_state.r#hinge == Hinge::Left && block_state.r#powered == true && block_state.r#half == Half::Lower && block_state.r#facing == Facing::West && block_state.r#open == true { return 14346; }
        if block_state.r#powered == false && block_state.r#open == true && block_state.r#facing == Facing::North && block_state.r#hinge == Hinge::Right && block_state.r#half == Half::Upper { return 14311; }
        if block_state.r#powered == true && block_state.r#half == Half::Lower && block_state.r#facing == Facing::North && block_state.r#hinge == Hinge::Right && block_state.r#open == true { return 14318; }
        if block_state.r#powered == true && block_state.r#open == false && block_state.r#hinge == Hinge::Right && block_state.r#facing == Facing::North && block_state.r#half == Half::Lower { return 14320; }
        if block_state.r#hinge == Hinge::Right && block_state.r#facing == Facing::West && block_state.r#half == Half::Upper && block_state.r#open == false && block_state.r#powered == false { return 14345; }
        if block_state.r#open == false && block_state.r#half == Half::Lower && block_state.r#facing == Facing::West && block_state.r#powered == true && block_state.r#hinge == Hinge::Left { return 14348; }
        if block_state.r#powered == true && block_state.r#half == Half::Upper && block_state.r#facing == Facing::East && block_state.r#open == false && block_state.r#hinge == Hinge::Right { return 14360; }
        if block_state.r#facing == Facing::West && block_state.r#hinge == Hinge::Left && block_state.r#half == Half::Upper && block_state.r#open == true && block_state.r#powered == true { return 14338; }
        if block_state.r#facing == Facing::East && block_state.r#powered == true && block_state.r#hinge == Hinge::Left && block_state.r#open == false && block_state.r#half == Half::Lower { return 14364; }
        if block_state.r#hinge == Hinge::Right && block_state.r#facing == Facing::South && block_state.r#open == false && block_state.r#powered == false && block_state.r#half == Half::Lower { return 14337; }
        if block_state.r#facing == Facing::West && block_state.r#open == false && block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Right && block_state.r#powered == false { return 14353; }
        if block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Left && block_state.r#open == true && block_state.r#facing == Facing::East && block_state.r#powered == true { return 14354; }
        if block_state.r#facing == Facing::East && block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Right && block_state.r#open == false && block_state.r#powered == true { return 14368; }
        if block_state.r#open == true && block_state.r#powered == true && block_state.r#half == Half::Upper && block_state.r#facing == Facing::South && block_state.r#hinge == Hinge::Right { return 14326; }
        if block_state.r#facing == Facing::North && block_state.r#open == true && block_state.r#half == Half::Upper && block_state.r#powered == true && block_state.r#hinge == Hinge::Right { return 14310; }
        if block_state.r#open == false && block_state.r#half == Half::Upper && block_state.r#powered == false && block_state.r#facing == Facing::North && block_state.r#hinge == Hinge::Left { return 14309; }
        if block_state.r#hinge == Hinge::Right && block_state.r#facing == Facing::East && block_state.r#open == false && block_state.r#half == Half::Upper && block_state.r#powered == false { return 14361; }
        if block_state.r#powered == false && block_state.r#facing == Facing::South && block_state.r#open == true && block_state.r#hinge == Hinge::Left && block_state.r#half == Half::Lower { return 14331; }
        if block_state.r#half == Half::Upper && block_state.r#open == true && block_state.r#powered == false && block_state.r#facing == Facing::East && block_state.r#hinge == Hinge::Right { return 14359; }
        if block_state.r#hinge == Hinge::Left && block_state.r#powered == true && block_state.r#facing == Facing::South && block_state.r#open == true && block_state.r#half == Half::Lower { return 14330; }
        if block_state.r#facing == Facing::East && block_state.r#hinge == Hinge::Left && block_state.r#open == true && block_state.r#half == Half::Lower && block_state.r#powered == true { return 14362; }
        if block_state.r#facing == Facing::West && block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Left && block_state.r#open == false && block_state.r#powered == true { return 14340; }
        if block_state.r#hinge == Hinge::Left && block_state.r#powered == true && block_state.r#half == Half::Upper && block_state.r#open == false && block_state.r#facing == Facing::South { return 14324; }
        if block_state.r#hinge == Hinge::Left && block_state.r#facing == Facing::East && block_state.r#open == false && block_state.r#half == Half::Lower && block_state.r#powered == false { return 14365; }
        if block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Left && block_state.r#open == true && block_state.r#facing == Facing::North && block_state.r#powered == true { return 14306; }
        if block_state.r#facing == Facing::West && block_state.r#open == true && block_state.r#hinge == Hinge::Left && block_state.r#half == Half::Upper && block_state.r#powered == false { return 14339; }
        if block_state.r#hinge == Hinge::Left && block_state.r#open == true && block_state.r#half == Half::Lower && block_state.r#facing == Facing::West && block_state.r#powered == false { return 14347; }
        if block_state.r#facing == Facing::East && block_state.r#powered == false && block_state.r#hinge == Hinge::Left && block_state.r#half == Half::Lower && block_state.r#open == true { return 14363; }
        if block_state.r#powered == true && block_state.r#facing == Facing::North && block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Left && block_state.r#open == false { return 14316; }
        if block_state.r#facing == Facing::South && block_state.r#half == Half::Upper && block_state.r#powered == false && block_state.r#hinge == Hinge::Left && block_state.r#open == true { return 14323; }
        if block_state.r#open == false && block_state.r#powered == false && block_state.r#hinge == Hinge::Left && block_state.r#half == Half::Upper && block_state.r#facing == Facing::South { return 14325; }
        if block_state.r#facing == Facing::West && block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Left && block_state.r#open == false && block_state.r#powered == false { return 14341; }
        if block_state.r#facing == Facing::West && block_state.r#half == Half::Upper && block_state.r#open == true && block_state.r#powered == false && block_state.r#hinge == Hinge::Right { return 14343; }
        if block_state.r#facing == Facing::West && block_state.r#powered == true && block_state.r#hinge == Hinge::Right && block_state.r#open == true && block_state.r#half == Half::Upper { return 14342; }
        if block_state.r#hinge == Hinge::Right && block_state.r#open == false && block_state.r#facing == Facing::West && block_state.r#powered == true && block_state.r#half == Half::Lower { return 14352; }
        if block_state.r#facing == Facing::East && block_state.r#powered == true && block_state.r#hinge == Hinge::Right && block_state.r#half == Half::Upper && block_state.r#open == true { return 14358; }
        if block_state.r#powered == false && block_state.r#facing == Facing::South && block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Right && block_state.r#open == true { return 14327; }
        if block_state.r#facing == Facing::South && block_state.r#hinge == Hinge::Right && block_state.r#powered == true && block_state.r#half == Half::Lower && block_state.r#open == true { return 14334; }
        if block_state.r#open == false && block_state.r#half == Half::Lower && block_state.r#powered == false && block_state.r#facing == Facing::East && block_state.r#hinge == Hinge::Right { return 14369; }
        if block_state.r#hinge == Hinge::Right && block_state.r#facing == Facing::South && block_state.r#half == Half::Lower && block_state.r#open == false && block_state.r#powered == true { return 14336; }
        if block_state.r#half == Half::Upper && block_state.r#powered == true && block_state.r#facing == Facing::South && block_state.r#hinge == Hinge::Left && block_state.r#open == true { return 14322; }
        if block_state.r#open == true && block_state.r#powered == false && block_state.r#hinge == Hinge::Left && block_state.r#facing == Facing::North && block_state.r#half == Half::Upper { return 14307; }
        if block_state.r#open == false && block_state.r#facing == Facing::South && block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Left && block_state.r#powered == true { return 14332; }
        if block_state.r#open == true && block_state.r#half == Half::Lower && block_state.r#powered == true && block_state.r#hinge == Hinge::Right && block_state.r#facing == Facing::West { return 14350; }
        if block_state.r#powered == false && block_state.r#half == Half::Lower && block_state.r#facing == Facing::North && block_state.r#hinge == Hinge::Left && block_state.r#open == false { return 14317; }
        if block_state.r#powered == true && block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Left && block_state.r#open == true && block_state.r#facing == Facing::North { return 14314; }
        if block_state.r#facing == Facing::East && block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Right && block_state.r#open == true && block_state.r#powered == true { return 14366; }
        if block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Left && block_state.r#facing == Facing::North && block_state.r#powered == false && block_state.r#open == true { return 14315; }
        if block_state.r#facing == Facing::South && block_state.r#hinge == Hinge::Left && block_state.r#open == false && block_state.r#powered == false && block_state.r#half == Half::Lower { return 14333; }
        if block_state.r#powered == false && block_state.r#hinge == Hinge::Right && block_state.r#facing == Facing::North && block_state.r#half == Half::Lower && block_state.r#open == false { return 14321; }
        if block_state.r#powered == true && block_state.r#hinge == Hinge::Right && block_state.r#facing == Facing::South && block_state.r#open == false && block_state.r#half == Half::Upper { return 14328; }
        if block_state.r#hinge == Hinge::Left && block_state.r#powered == true && block_state.r#facing == Facing::East && block_state.r#half == Half::Upper && block_state.r#open == false { return 14356; }
        if block_state.r#powered == false && block_state.r#open == false && block_state.r#half == Half::Upper && block_state.r#facing == Facing::South && block_state.r#hinge == Hinge::Right { return 14329; }
        if block_state.r#facing == Facing::North && block_state.r#hinge == Hinge::Left && block_state.r#open == false && block_state.r#half == Half::Upper && block_state.r#powered == true { return 14308; }
        if block_state.r#open == false && block_state.r#half == Half::Lower && block_state.r#facing == Facing::West && block_state.r#hinge == Hinge::Left && block_state.r#powered == false { return 14349; }
        if block_state.r#powered == true && block_state.r#hinge == Hinge::Right && block_state.r#facing == Facing::West && block_state.r#half == Half::Upper && block_state.r#open == false { return 14344; }
        if block_state.r#open == false && block_state.r#hinge == Hinge::Right && block_state.r#facing == Facing::North && block_state.r#half == Half::Upper && block_state.r#powered == true { return 14312; }
        if block_state.r#facing == Facing::East && block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Left && block_state.r#open == true && block_state.r#powered == false { return 14355; }
        if block_state.r#hinge == Hinge::Right && block_state.r#open == true && block_state.r#half == Half::Lower && block_state.r#facing == Facing::North && block_state.r#powered == false { return 14319; }
        if block_state.r#half == Half::Upper && block_state.r#facing == Facing::East && block_state.r#hinge == Hinge::Left && block_state.r#powered == false && block_state.r#open == false { return 14357; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14335 {
            return Some(MangroveDoor {
                r#open: true,
                r#hinge: Hinge::Right,
                r#half: Half::Lower,
                r#powered: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 14313 {
            return Some(MangroveDoor {
                r#powered: false,
                r#hinge: Hinge::Right,
                r#facing: Facing::North,
                r#half: Half::Upper,
                r#open: false,
            });
        }
        if state_id == 14367 {
            return Some(MangroveDoor {
                r#hinge: Hinge::Right,
                r#facing: Facing::East,
                r#half: Half::Lower,
                r#powered: false,
                r#open: true,
            });
        }
        if state_id == 14351 {
            return Some(MangroveDoor {
                r#open: true,
                r#powered: false,
                r#hinge: Hinge::Right,
                r#half: Half::Lower,
                r#facing: Facing::West,
            });
        }
        if state_id == 14346 {
            return Some(MangroveDoor {
                r#hinge: Hinge::Left,
                r#powered: true,
                r#half: Half::Lower,
                r#facing: Facing::West,
                r#open: true,
            });
        }
        if state_id == 14311 {
            return Some(MangroveDoor {
                r#powered: false,
                r#open: true,
                r#facing: Facing::North,
                r#hinge: Hinge::Right,
                r#half: Half::Upper,
            });
        }
        if state_id == 14318 {
            return Some(MangroveDoor {
                r#powered: true,
                r#half: Half::Lower,
                r#facing: Facing::North,
                r#hinge: Hinge::Right,
                r#open: true,
            });
        }
        if state_id == 14320 {
            return Some(MangroveDoor {
                r#powered: true,
                r#open: false,
                r#hinge: Hinge::Right,
                r#facing: Facing::North,
                r#half: Half::Lower,
            });
        }
        if state_id == 14345 {
            return Some(MangroveDoor {
                r#hinge: Hinge::Right,
                r#facing: Facing::West,
                r#half: Half::Upper,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 14348 {
            return Some(MangroveDoor {
                r#open: false,
                r#half: Half::Lower,
                r#facing: Facing::West,
                r#powered: true,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 14360 {
            return Some(MangroveDoor {
                r#powered: true,
                r#half: Half::Upper,
                r#facing: Facing::East,
                r#open: false,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 14338 {
            return Some(MangroveDoor {
                r#facing: Facing::West,
                r#hinge: Hinge::Left,
                r#half: Half::Upper,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 14364 {
            return Some(MangroveDoor {
                r#facing: Facing::East,
                r#powered: true,
                r#hinge: Hinge::Left,
                r#open: false,
                r#half: Half::Lower,
            });
        }
        if state_id == 14337 {
            return Some(MangroveDoor {
                r#hinge: Hinge::Right,
                r#facing: Facing::South,
                r#open: false,
                r#powered: false,
                r#half: Half::Lower,
            });
        }
        if state_id == 14353 {
            return Some(MangroveDoor {
                r#facing: Facing::West,
                r#open: false,
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
                r#powered: false,
            });
        }
        if state_id == 14354 {
            return Some(MangroveDoor {
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
                r#open: true,
                r#facing: Facing::East,
                r#powered: true,
            });
        }
        if state_id == 14368 {
            return Some(MangroveDoor {
                r#facing: Facing::East,
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
                r#open: false,
                r#powered: true,
            });
        }
        if state_id == 14326 {
            return Some(MangroveDoor {
                r#open: true,
                r#powered: true,
                r#half: Half::Upper,
                r#facing: Facing::South,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 14310 {
            return Some(MangroveDoor {
                r#facing: Facing::North,
                r#open: true,
                r#half: Half::Upper,
                r#powered: true,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 14309 {
            return Some(MangroveDoor {
                r#open: false,
                r#half: Half::Upper,
                r#powered: false,
                r#facing: Facing::North,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 14361 {
            return Some(MangroveDoor {
                r#hinge: Hinge::Right,
                r#facing: Facing::East,
                r#open: false,
                r#half: Half::Upper,
                r#powered: false,
            });
        }
        if state_id == 14331 {
            return Some(MangroveDoor {
                r#powered: false,
                r#facing: Facing::South,
                r#open: true,
                r#hinge: Hinge::Left,
                r#half: Half::Lower,
            });
        }
        if state_id == 14359 {
            return Some(MangroveDoor {
                r#half: Half::Upper,
                r#open: true,
                r#powered: false,
                r#facing: Facing::East,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 14330 {
            return Some(MangroveDoor {
                r#hinge: Hinge::Left,
                r#powered: true,
                r#facing: Facing::South,
                r#open: true,
                r#half: Half::Lower,
            });
        }
        if state_id == 14362 {
            return Some(MangroveDoor {
                r#facing: Facing::East,
                r#hinge: Hinge::Left,
                r#open: true,
                r#half: Half::Lower,
                r#powered: true,
            });
        }
        if state_id == 14340 {
            return Some(MangroveDoor {
                r#facing: Facing::West,
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
                r#open: false,
                r#powered: true,
            });
        }
        if state_id == 14324 {
            return Some(MangroveDoor {
                r#hinge: Hinge::Left,
                r#powered: true,
                r#half: Half::Upper,
                r#open: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 14365 {
            return Some(MangroveDoor {
                r#hinge: Hinge::Left,
                r#facing: Facing::East,
                r#open: false,
                r#half: Half::Lower,
                r#powered: false,
            });
        }
        if state_id == 14306 {
            return Some(MangroveDoor {
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
                r#open: true,
                r#facing: Facing::North,
                r#powered: true,
            });
        }
        if state_id == 14339 {
            return Some(MangroveDoor {
                r#facing: Facing::West,
                r#open: true,
                r#hinge: Hinge::Left,
                r#half: Half::Upper,
                r#powered: false,
            });
        }
        if state_id == 14347 {
            return Some(MangroveDoor {
                r#hinge: Hinge::Left,
                r#open: true,
                r#half: Half::Lower,
                r#facing: Facing::West,
                r#powered: false,
            });
        }
        if state_id == 14363 {
            return Some(MangroveDoor {
                r#facing: Facing::East,
                r#powered: false,
                r#hinge: Hinge::Left,
                r#half: Half::Lower,
                r#open: true,
            });
        }
        if state_id == 14316 {
            return Some(MangroveDoor {
                r#powered: true,
                r#facing: Facing::North,
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
                r#open: false,
            });
        }
        if state_id == 14323 {
            return Some(MangroveDoor {
                r#facing: Facing::South,
                r#half: Half::Upper,
                r#powered: false,
                r#hinge: Hinge::Left,
                r#open: true,
            });
        }
        if state_id == 14325 {
            return Some(MangroveDoor {
                r#open: false,
                r#powered: false,
                r#hinge: Hinge::Left,
                r#half: Half::Upper,
                r#facing: Facing::South,
            });
        }
        if state_id == 14341 {
            return Some(MangroveDoor {
                r#facing: Facing::West,
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 14343 {
            return Some(MangroveDoor {
                r#facing: Facing::West,
                r#half: Half::Upper,
                r#open: true,
                r#powered: false,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 14342 {
            return Some(MangroveDoor {
                r#facing: Facing::West,
                r#powered: true,
                r#hinge: Hinge::Right,
                r#open: true,
                r#half: Half::Upper,
            });
        }
        if state_id == 14352 {
            return Some(MangroveDoor {
                r#hinge: Hinge::Right,
                r#open: false,
                r#facing: Facing::West,
                r#powered: true,
                r#half: Half::Lower,
            });
        }
        if state_id == 14358 {
            return Some(MangroveDoor {
                r#facing: Facing::East,
                r#powered: true,
                r#hinge: Hinge::Right,
                r#half: Half::Upper,
                r#open: true,
            });
        }
        if state_id == 14327 {
            return Some(MangroveDoor {
                r#powered: false,
                r#facing: Facing::South,
                r#half: Half::Upper,
                r#hinge: Hinge::Right,
                r#open: true,
            });
        }
        if state_id == 14334 {
            return Some(MangroveDoor {
                r#facing: Facing::South,
                r#hinge: Hinge::Right,
                r#powered: true,
                r#half: Half::Lower,
                r#open: true,
            });
        }
        if state_id == 14369 {
            return Some(MangroveDoor {
                r#open: false,
                r#half: Half::Lower,
                r#powered: false,
                r#facing: Facing::East,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 14336 {
            return Some(MangroveDoor {
                r#hinge: Hinge::Right,
                r#facing: Facing::South,
                r#half: Half::Lower,
                r#open: false,
                r#powered: true,
            });
        }
        if state_id == 14322 {
            return Some(MangroveDoor {
                r#half: Half::Upper,
                r#powered: true,
                r#facing: Facing::South,
                r#hinge: Hinge::Left,
                r#open: true,
            });
        }
        if state_id == 14307 {
            return Some(MangroveDoor {
                r#open: true,
                r#powered: false,
                r#hinge: Hinge::Left,
                r#facing: Facing::North,
                r#half: Half::Upper,
            });
        }
        if state_id == 14332 {
            return Some(MangroveDoor {
                r#open: false,
                r#facing: Facing::South,
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
                r#powered: true,
            });
        }
        if state_id == 14350 {
            return Some(MangroveDoor {
                r#open: true,
                r#half: Half::Lower,
                r#powered: true,
                r#hinge: Hinge::Right,
                r#facing: Facing::West,
            });
        }
        if state_id == 14317 {
            return Some(MangroveDoor {
                r#powered: false,
                r#half: Half::Lower,
                r#facing: Facing::North,
                r#hinge: Hinge::Left,
                r#open: false,
            });
        }
        if state_id == 14314 {
            return Some(MangroveDoor {
                r#powered: true,
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
                r#open: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 14366 {
            return Some(MangroveDoor {
                r#facing: Facing::East,
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 14315 {
            return Some(MangroveDoor {
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
                r#facing: Facing::North,
                r#powered: false,
                r#open: true,
            });
        }
        if state_id == 14333 {
            return Some(MangroveDoor {
                r#facing: Facing::South,
                r#hinge: Hinge::Left,
                r#open: false,
                r#powered: false,
                r#half: Half::Lower,
            });
        }
        if state_id == 14321 {
            return Some(MangroveDoor {
                r#powered: false,
                r#hinge: Hinge::Right,
                r#facing: Facing::North,
                r#half: Half::Lower,
                r#open: false,
            });
        }
        if state_id == 14328 {
            return Some(MangroveDoor {
                r#powered: true,
                r#hinge: Hinge::Right,
                r#facing: Facing::South,
                r#open: false,
                r#half: Half::Upper,
            });
        }
        if state_id == 14356 {
            return Some(MangroveDoor {
                r#hinge: Hinge::Left,
                r#powered: true,
                r#facing: Facing::East,
                r#half: Half::Upper,
                r#open: false,
            });
        }
        if state_id == 14329 {
            return Some(MangroveDoor {
                r#powered: false,
                r#open: false,
                r#half: Half::Upper,
                r#facing: Facing::South,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 14308 {
            return Some(MangroveDoor {
                r#facing: Facing::North,
                r#hinge: Hinge::Left,
                r#open: false,
                r#half: Half::Upper,
                r#powered: true,
            });
        }
        if state_id == 14349 {
            return Some(MangroveDoor {
                r#open: false,
                r#half: Half::Lower,
                r#facing: Facing::West,
                r#hinge: Hinge::Left,
                r#powered: false,
            });
        }
        if state_id == 14344 {
            return Some(MangroveDoor {
                r#powered: true,
                r#hinge: Hinge::Right,
                r#facing: Facing::West,
                r#half: Half::Upper,
                r#open: false,
            });
        }
        if state_id == 14312 {
            return Some(MangroveDoor {
                r#open: false,
                r#hinge: Hinge::Right,
                r#facing: Facing::North,
                r#half: Half::Upper,
                r#powered: true,
            });
        }
        if state_id == 14355 {
            return Some(MangroveDoor {
                r#facing: Facing::East,
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
                r#open: true,
                r#powered: false,
            });
        }
        if state_id == 14319 {
            return Some(MangroveDoor {
                r#hinge: Hinge::Right,
                r#open: true,
                r#half: Half::Lower,
                r#facing: Facing::North,
                r#powered: false,
            });
        }
        if state_id == 14357 {
            return Some(MangroveDoor {
                r#half: Half::Upper,
                r#facing: Facing::East,
                r#hinge: Hinge::Left,
                r#powered: false,
                r#open: false,
            });
        }
        return None;
    }
}

