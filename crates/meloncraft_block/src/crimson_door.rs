use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrimsonDoor {
    pub r#hinge: Hinge,
    pub r#facing: Facing,
    pub r#half: Half,
    pub open: bool,
    pub powered: bool,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Hinge {
    Left,
    Right,
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

impl BlockState for CrimsonDoor {
    fn to_id(self) -> i32 {
        if block_state.r#hinge == Hinge::Left && block_state.r#open == true && block_state.r#powered == false && block_state.r#half == Half::Upper && block_state.r#facing == Facing::North { return 21313; }
        if block_state.r#facing == Facing::West && block_state.r#hinge == Hinge::Left && block_state.r#half == Half::Lower && block_state.r#powered == true && block_state.r#open == false { return 21354; }
        if block_state.r#facing == Facing::South && block_state.r#hinge == Hinge::Right && block_state.r#half == Half::Lower && block_state.r#powered == true && block_state.r#open == true { return 21340; }
        if block_state.r#powered == true && block_state.r#facing == Facing::North && block_state.r#open == false && block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Right { return 21318; }
        if block_state.r#half == Half::Upper && block_state.r#facing == Facing::East && block_state.r#powered == true && block_state.r#hinge == Hinge::Right && block_state.r#open == false { return 21366; }
        if block_state.r#facing == Facing::South && block_state.r#half == Half::Lower && block_state.r#powered == true && block_state.r#open == true && block_state.r#hinge == Hinge::Left { return 21336; }
        if block_state.r#half == Half::Lower && block_state.r#facing == Facing::South && block_state.r#powered == false && block_state.r#open == true && block_state.r#hinge == Hinge::Right { return 21341; }
        if block_state.r#hinge == Hinge::Left && block_state.r#facing == Facing::North && block_state.r#half == Half::Upper && block_state.r#open == false && block_state.r#powered == true { return 21314; }
        if block_state.r#powered == false && block_state.r#hinge == Hinge::Left && block_state.r#half == Half::Upper && block_state.r#open == false && block_state.r#facing == Facing::West { return 21347; }
        if block_state.r#powered == true && block_state.r#facing == Facing::North && block_state.r#half == Half::Lower && block_state.r#open == true && block_state.r#hinge == Hinge::Right { return 21324; }
        if block_state.r#powered == false && block_state.r#open == true && block_state.r#facing == Facing::West && block_state.r#hinge == Hinge::Right && block_state.r#half == Half::Upper { return 21349; }
        if block_state.r#facing == Facing::East && block_state.r#hinge == Hinge::Left && block_state.r#open == false && block_state.r#powered == true && block_state.r#half == Half::Upper { return 21362; }
        if block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Left && block_state.r#facing == Facing::South && block_state.r#powered == false && block_state.r#open == true { return 21337; }
        if block_state.r#facing == Facing::East && block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Right && block_state.r#open == true && block_state.r#powered == true { return 21364; }
        if block_state.r#half == Half::Lower && block_state.r#facing == Facing::East && block_state.r#open == true && block_state.r#powered == true && block_state.r#hinge == Hinge::Right { return 21372; }
        if block_state.r#open == true && block_state.r#powered == true && block_state.r#half == Half::Lower && block_state.r#facing == Facing::West && block_state.r#hinge == Hinge::Right { return 21356; }
        if block_state.r#half == Half::Upper && block_state.r#open == false && block_state.r#facing == Facing::East && block_state.r#hinge == Hinge::Right && block_state.r#powered == false { return 21367; }
        if block_state.r#facing == Facing::East && block_state.r#open == false && block_state.r#half == Half::Lower && block_state.r#powered == true && block_state.r#hinge == Hinge::Left { return 21370; }
        if block_state.r#open == false && block_state.r#hinge == Hinge::Right && block_state.r#powered == false && block_state.r#half == Half::Lower && block_state.r#facing == Facing::East { return 21375; }
        if block_state.r#hinge == Hinge::Right && block_state.r#facing == Facing::West && block_state.r#half == Half::Lower && block_state.r#open == false && block_state.r#powered == false { return 21359; }
        if block_state.r#hinge == Hinge::Left && block_state.r#facing == Facing::North && block_state.r#half == Half::Upper && block_state.r#open == true && block_state.r#powered == true { return 21312; }
        if block_state.r#open == true && block_state.r#half == Half::Lower && block_state.r#powered == true && block_state.r#facing == Facing::West && block_state.r#hinge == Hinge::Left { return 21352; }
        if block_state.r#facing == Facing::West && block_state.r#half == Half::Lower && block_state.r#open == true && block_state.r#powered == false && block_state.r#hinge == Hinge::Right { return 21357; }
        if block_state.r#hinge == Hinge::Right && block_state.r#facing == Facing::South && block_state.r#powered == true && block_state.r#half == Half::Upper && block_state.r#open == true { return 21332; }
        if block_state.r#facing == Facing::South && block_state.r#powered == false && block_state.r#half == Half::Upper && block_state.r#open == true && block_state.r#hinge == Hinge::Right { return 21333; }
        if block_state.r#powered == true && block_state.r#open == true && block_state.r#facing == Facing::West && block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Left { return 21344; }
        if block_state.r#half == Half::Lower && block_state.r#powered == false && block_state.r#open == true && block_state.r#facing == Facing::West && block_state.r#hinge == Hinge::Left { return 21353; }
        if block_state.r#facing == Facing::East && block_state.r#hinge == Hinge::Right && block_state.r#half == Half::Upper && block_state.r#open == true && block_state.r#powered == false { return 21365; }
        if block_state.r#half == Half::Lower && block_state.r#powered == true && block_state.r#facing == Facing::East && block_state.r#open == true && block_state.r#hinge == Hinge::Left { return 21368; }
        if block_state.r#facing == Facing::South && block_state.r#open == false && block_state.r#powered == true && block_state.r#hinge == Hinge::Left && block_state.r#half == Half::Upper { return 21330; }
        if block_state.r#powered == true && block_state.r#facing == Facing::West && block_state.r#hinge == Hinge::Right && block_state.r#half == Half::Upper && block_state.r#open == false { return 21350; }
        if block_state.r#open == false && block_state.r#facing == Facing::South && block_state.r#hinge == Hinge::Right && block_state.r#half == Half::Lower && block_state.r#powered == false { return 21343; }
        if block_state.r#open == false && block_state.r#hinge == Hinge::Left && block_state.r#powered == true && block_state.r#facing == Facing::West && block_state.r#half == Half::Upper { return 21346; }
        if block_state.r#powered == true && block_state.r#facing == Facing::North && block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Right && block_state.r#open == true { return 21316; }
        if block_state.r#open == true && block_state.r#powered == true && block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Left && block_state.r#facing == Facing::North { return 21320; }
        if block_state.r#facing == Facing::South && block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Left && block_state.r#open == true && block_state.r#powered == true { return 21328; }
        if block_state.r#half == Half::Upper && block_state.r#open == true && block_state.r#powered == false && block_state.r#facing == Facing::North && block_state.r#hinge == Hinge::Right { return 21317; }
        if block_state.r#hinge == Hinge::Left && block_state.r#half == Half::Lower && block_state.r#facing == Facing::South && block_state.r#powered == true && block_state.r#open == false { return 21338; }
        if block_state.r#open == true && block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Left && block_state.r#facing == Facing::East && block_state.r#powered == false { return 21369; }
        if block_state.r#hinge == Hinge::Left && block_state.r#half == Half::Lower && block_state.r#open == false && block_state.r#powered == false && block_state.r#facing == Facing::South { return 21339; }
        if block_state.r#powered == false && block_state.r#half == Half::Upper && block_state.r#facing == Facing::East && block_state.r#hinge == Hinge::Left && block_state.r#open == true { return 21361; }
        if block_state.r#facing == Facing::East && block_state.r#powered == false && block_state.r#half == Half::Upper && block_state.r#open == false && block_state.r#hinge == Hinge::Left { return 21363; }
        if block_state.r#facing == Facing::West && block_state.r#powered == true && block_state.r#hinge == Hinge::Right && block_state.r#open == false && block_state.r#half == Half::Lower { return 21358; }
        if block_state.r#facing == Facing::North && block_state.r#powered == false && block_state.r#open == true && block_state.r#hinge == Hinge::Right && block_state.r#half == Half::Lower { return 21325; }
        if block_state.r#facing == Facing::West && block_state.r#hinge == Hinge::Right && block_state.r#powered == true && block_state.r#half == Half::Upper && block_state.r#open == true { return 21348; }
        if block_state.r#powered == true && block_state.r#hinge == Hinge::Right && block_state.r#open == false && block_state.r#facing == Facing::North && block_state.r#half == Half::Lower { return 21326; }
        if block_state.r#half == Half::Lower && block_state.r#powered == false && block_state.r#open == false && block_state.r#hinge == Hinge::Left && block_state.r#facing == Facing::West { return 21355; }
        if block_state.r#hinge == Hinge::Left && block_state.r#powered == false && block_state.r#half == Half::Lower && block_state.r#open == true && block_state.r#facing == Facing::North { return 21321; }
        if block_state.r#open == false && block_state.r#facing == Facing::North && block_state.r#half == Half::Lower && block_state.r#powered == true && block_state.r#hinge == Hinge::Left { return 21322; }
        if block_state.r#hinge == Hinge::Right && block_state.r#half == Half::Upper && block_state.r#open == false && block_state.r#facing == Facing::North && block_state.r#powered == false { return 21319; }
        if block_state.r#half == Half::Lower && block_state.r#facing == Facing::North && block_state.r#open == false && block_state.r#hinge == Hinge::Right && block_state.r#powered == false { return 21327; }
        if block_state.r#facing == Facing::South && block_state.r#half == Half::Lower && block_state.r#powered == true && block_state.r#hinge == Hinge::Right && block_state.r#open == false { return 21342; }
        if block_state.r#facing == Facing::North && block_state.r#hinge == Hinge::Left && block_state.r#half == Half::Lower && block_state.r#open == false && block_state.r#powered == false { return 21323; }
        if block_state.r#powered == false && block_state.r#hinge == Hinge::Left && block_state.r#facing == Facing::West && block_state.r#half == Half::Upper && block_state.r#open == true { return 21345; }
        if block_state.r#powered == true && block_state.r#half == Half::Lower && block_state.r#open == false && block_state.r#facing == Facing::East && block_state.r#hinge == Hinge::Right { return 21374; }
        if block_state.r#hinge == Hinge::Right && block_state.r#facing == Facing::South && block_state.r#half == Half::Upper && block_state.r#open == false && block_state.r#powered == true { return 21334; }
        if block_state.r#open == true && block_state.r#powered == false && block_state.r#half == Half::Lower && block_state.r#facing == Facing::East && block_state.r#hinge == Hinge::Right { return 21373; }
        if block_state.r#facing == Facing::East && block_state.r#open == true && block_state.r#hinge == Hinge::Left && block_state.r#powered == true && block_state.r#half == Half::Upper { return 21360; }
        if block_state.r#facing == Facing::East && block_state.r#hinge == Hinge::Left && block_state.r#half == Half::Lower && block_state.r#powered == false && block_state.r#open == false { return 21371; }
        if block_state.r#hinge == Hinge::Left && block_state.r#facing == Facing::South && block_state.r#half == Half::Upper && block_state.r#powered == false && block_state.r#open == false { return 21331; }
        if block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Right && block_state.r#open == false && block_state.r#powered == false && block_state.r#facing == Facing::West { return 21351; }
        if block_state.r#half == Half::Upper && block_state.r#facing == Facing::North && block_state.r#hinge == Hinge::Left && block_state.r#open == false && block_state.r#powered == false { return 21315; }
        if block_state.r#facing == Facing::South && block_state.r#hinge == Hinge::Right && block_state.r#open == false && block_state.r#powered == false && block_state.r#half == Half::Upper { return 21335; }
        if block_state.r#facing == Facing::South && block_state.r#open == true && block_state.r#powered == false && block_state.r#hinge == Hinge::Left && block_state.r#half == Half::Upper { return 21329; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 21313 {
            return Some(CrimsonDoor {
                r#hinge: Hinge::Left,
                r#open: true,
                r#powered: false,
                r#half: Half::Upper,
                r#facing: Facing::North,
            });
        }
        if state_id == 21354 {
            return Some(CrimsonDoor {
                r#facing: Facing::West,
                r#hinge: Hinge::Left,
                r#half: Half::Lower,
                r#powered: true,
                r#open: false,
            });
        }
        if state_id == 21340 {
            return Some(CrimsonDoor {
                r#facing: Facing::South,
                r#hinge: Hinge::Right,
                r#half: Half::Lower,
                r#powered: true,
                r#open: true,
            });
        }
        if state_id == 21318 {
            return Some(CrimsonDoor {
                r#powered: true,
                r#facing: Facing::North,
                r#open: false,
                r#half: Half::Upper,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 21366 {
            return Some(CrimsonDoor {
                r#half: Half::Upper,
                r#facing: Facing::East,
                r#powered: true,
                r#hinge: Hinge::Right,
                r#open: false,
            });
        }
        if state_id == 21336 {
            return Some(CrimsonDoor {
                r#facing: Facing::South,
                r#half: Half::Lower,
                r#powered: true,
                r#open: true,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 21341 {
            return Some(CrimsonDoor {
                r#half: Half::Lower,
                r#facing: Facing::South,
                r#powered: false,
                r#open: true,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 21314 {
            return Some(CrimsonDoor {
                r#hinge: Hinge::Left,
                r#facing: Facing::North,
                r#half: Half::Upper,
                r#open: false,
                r#powered: true,
            });
        }
        if state_id == 21347 {
            return Some(CrimsonDoor {
                r#powered: false,
                r#hinge: Hinge::Left,
                r#half: Half::Upper,
                r#open: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 21324 {
            return Some(CrimsonDoor {
                r#powered: true,
                r#facing: Facing::North,
                r#half: Half::Lower,
                r#open: true,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 21349 {
            return Some(CrimsonDoor {
                r#powered: false,
                r#open: true,
                r#facing: Facing::West,
                r#hinge: Hinge::Right,
                r#half: Half::Upper,
            });
        }
        if state_id == 21362 {
            return Some(CrimsonDoor {
                r#facing: Facing::East,
                r#hinge: Hinge::Left,
                r#open: false,
                r#powered: true,
                r#half: Half::Upper,
            });
        }
        if state_id == 21337 {
            return Some(CrimsonDoor {
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
                r#facing: Facing::South,
                r#powered: false,
                r#open: true,
            });
        }
        if state_id == 21364 {
            return Some(CrimsonDoor {
                r#facing: Facing::East,
                r#half: Half::Upper,
                r#hinge: Hinge::Right,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 21372 {
            return Some(CrimsonDoor {
                r#half: Half::Lower,
                r#facing: Facing::East,
                r#open: true,
                r#powered: true,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 21356 {
            return Some(CrimsonDoor {
                r#open: true,
                r#powered: true,
                r#half: Half::Lower,
                r#facing: Facing::West,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 21367 {
            return Some(CrimsonDoor {
                r#half: Half::Upper,
                r#open: false,
                r#facing: Facing::East,
                r#hinge: Hinge::Right,
                r#powered: false,
            });
        }
        if state_id == 21370 {
            return Some(CrimsonDoor {
                r#facing: Facing::East,
                r#open: false,
                r#half: Half::Lower,
                r#powered: true,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 21375 {
            return Some(CrimsonDoor {
                r#open: false,
                r#hinge: Hinge::Right,
                r#powered: false,
                r#half: Half::Lower,
                r#facing: Facing::East,
            });
        }
        if state_id == 21359 {
            return Some(CrimsonDoor {
                r#hinge: Hinge::Right,
                r#facing: Facing::West,
                r#half: Half::Lower,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 21312 {
            return Some(CrimsonDoor {
                r#hinge: Hinge::Left,
                r#facing: Facing::North,
                r#half: Half::Upper,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 21352 {
            return Some(CrimsonDoor {
                r#open: true,
                r#half: Half::Lower,
                r#powered: true,
                r#facing: Facing::West,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 21357 {
            return Some(CrimsonDoor {
                r#facing: Facing::West,
                r#half: Half::Lower,
                r#open: true,
                r#powered: false,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 21332 {
            return Some(CrimsonDoor {
                r#hinge: Hinge::Right,
                r#facing: Facing::South,
                r#powered: true,
                r#half: Half::Upper,
                r#open: true,
            });
        }
        if state_id == 21333 {
            return Some(CrimsonDoor {
                r#facing: Facing::South,
                r#powered: false,
                r#half: Half::Upper,
                r#open: true,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 21344 {
            return Some(CrimsonDoor {
                r#powered: true,
                r#open: true,
                r#facing: Facing::West,
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 21353 {
            return Some(CrimsonDoor {
                r#half: Half::Lower,
                r#powered: false,
                r#open: true,
                r#facing: Facing::West,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 21365 {
            return Some(CrimsonDoor {
                r#facing: Facing::East,
                r#hinge: Hinge::Right,
                r#half: Half::Upper,
                r#open: true,
                r#powered: false,
            });
        }
        if state_id == 21368 {
            return Some(CrimsonDoor {
                r#half: Half::Lower,
                r#powered: true,
                r#facing: Facing::East,
                r#open: true,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 21330 {
            return Some(CrimsonDoor {
                r#facing: Facing::South,
                r#open: false,
                r#powered: true,
                r#hinge: Hinge::Left,
                r#half: Half::Upper,
            });
        }
        if state_id == 21350 {
            return Some(CrimsonDoor {
                r#powered: true,
                r#facing: Facing::West,
                r#hinge: Hinge::Right,
                r#half: Half::Upper,
                r#open: false,
            });
        }
        if state_id == 21343 {
            return Some(CrimsonDoor {
                r#open: false,
                r#facing: Facing::South,
                r#hinge: Hinge::Right,
                r#half: Half::Lower,
                r#powered: false,
            });
        }
        if state_id == 21346 {
            return Some(CrimsonDoor {
                r#open: false,
                r#hinge: Hinge::Left,
                r#powered: true,
                r#facing: Facing::West,
                r#half: Half::Upper,
            });
        }
        if state_id == 21316 {
            return Some(CrimsonDoor {
                r#powered: true,
                r#facing: Facing::North,
                r#half: Half::Upper,
                r#hinge: Hinge::Right,
                r#open: true,
            });
        }
        if state_id == 21320 {
            return Some(CrimsonDoor {
                r#open: true,
                r#powered: true,
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
                r#facing: Facing::North,
            });
        }
        if state_id == 21328 {
            return Some(CrimsonDoor {
                r#facing: Facing::South,
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 21317 {
            return Some(CrimsonDoor {
                r#half: Half::Upper,
                r#open: true,
                r#powered: false,
                r#facing: Facing::North,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 21338 {
            return Some(CrimsonDoor {
                r#hinge: Hinge::Left,
                r#half: Half::Lower,
                r#facing: Facing::South,
                r#powered: true,
                r#open: false,
            });
        }
        if state_id == 21369 {
            return Some(CrimsonDoor {
                r#open: true,
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
                r#facing: Facing::East,
                r#powered: false,
            });
        }
        if state_id == 21339 {
            return Some(CrimsonDoor {
                r#hinge: Hinge::Left,
                r#half: Half::Lower,
                r#open: false,
                r#powered: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 21361 {
            return Some(CrimsonDoor {
                r#powered: false,
                r#half: Half::Upper,
                r#facing: Facing::East,
                r#hinge: Hinge::Left,
                r#open: true,
            });
        }
        if state_id == 21363 {
            return Some(CrimsonDoor {
                r#facing: Facing::East,
                r#powered: false,
                r#half: Half::Upper,
                r#open: false,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 21358 {
            return Some(CrimsonDoor {
                r#facing: Facing::West,
                r#powered: true,
                r#hinge: Hinge::Right,
                r#open: false,
                r#half: Half::Lower,
            });
        }
        if state_id == 21325 {
            return Some(CrimsonDoor {
                r#facing: Facing::North,
                r#powered: false,
                r#open: true,
                r#hinge: Hinge::Right,
                r#half: Half::Lower,
            });
        }
        if state_id == 21348 {
            return Some(CrimsonDoor {
                r#facing: Facing::West,
                r#hinge: Hinge::Right,
                r#powered: true,
                r#half: Half::Upper,
                r#open: true,
            });
        }
        if state_id == 21326 {
            return Some(CrimsonDoor {
                r#powered: true,
                r#hinge: Hinge::Right,
                r#open: false,
                r#facing: Facing::North,
                r#half: Half::Lower,
            });
        }
        if state_id == 21355 {
            return Some(CrimsonDoor {
                r#half: Half::Lower,
                r#powered: false,
                r#open: false,
                r#hinge: Hinge::Left,
                r#facing: Facing::West,
            });
        }
        if state_id == 21321 {
            return Some(CrimsonDoor {
                r#hinge: Hinge::Left,
                r#powered: false,
                r#half: Half::Lower,
                r#open: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 21322 {
            return Some(CrimsonDoor {
                r#open: false,
                r#facing: Facing::North,
                r#half: Half::Lower,
                r#powered: true,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 21319 {
            return Some(CrimsonDoor {
                r#hinge: Hinge::Right,
                r#half: Half::Upper,
                r#open: false,
                r#facing: Facing::North,
                r#powered: false,
            });
        }
        if state_id == 21327 {
            return Some(CrimsonDoor {
                r#half: Half::Lower,
                r#facing: Facing::North,
                r#open: false,
                r#hinge: Hinge::Right,
                r#powered: false,
            });
        }
        if state_id == 21342 {
            return Some(CrimsonDoor {
                r#facing: Facing::South,
                r#half: Half::Lower,
                r#powered: true,
                r#hinge: Hinge::Right,
                r#open: false,
            });
        }
        if state_id == 21323 {
            return Some(CrimsonDoor {
                r#facing: Facing::North,
                r#hinge: Hinge::Left,
                r#half: Half::Lower,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 21345 {
            return Some(CrimsonDoor {
                r#powered: false,
                r#hinge: Hinge::Left,
                r#facing: Facing::West,
                r#half: Half::Upper,
                r#open: true,
            });
        }
        if state_id == 21374 {
            return Some(CrimsonDoor {
                r#powered: true,
                r#half: Half::Lower,
                r#open: false,
                r#facing: Facing::East,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 21334 {
            return Some(CrimsonDoor {
                r#hinge: Hinge::Right,
                r#facing: Facing::South,
                r#half: Half::Upper,
                r#open: false,
                r#powered: true,
            });
        }
        if state_id == 21373 {
            return Some(CrimsonDoor {
                r#open: true,
                r#powered: false,
                r#half: Half::Lower,
                r#facing: Facing::East,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 21360 {
            return Some(CrimsonDoor {
                r#facing: Facing::East,
                r#open: true,
                r#hinge: Hinge::Left,
                r#powered: true,
                r#half: Half::Upper,
            });
        }
        if state_id == 21371 {
            return Some(CrimsonDoor {
                r#facing: Facing::East,
                r#hinge: Hinge::Left,
                r#half: Half::Lower,
                r#powered: false,
                r#open: false,
            });
        }
        if state_id == 21331 {
            return Some(CrimsonDoor {
                r#hinge: Hinge::Left,
                r#facing: Facing::South,
                r#half: Half::Upper,
                r#powered: false,
                r#open: false,
            });
        }
        if state_id == 21351 {
            return Some(CrimsonDoor {
                r#half: Half::Upper,
                r#hinge: Hinge::Right,
                r#open: false,
                r#powered: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 21315 {
            return Some(CrimsonDoor {
                r#half: Half::Upper,
                r#facing: Facing::North,
                r#hinge: Hinge::Left,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 21335 {
            return Some(CrimsonDoor {
                r#facing: Facing::South,
                r#hinge: Hinge::Right,
                r#open: false,
                r#powered: false,
                r#half: Half::Upper,
            });
        }
        if state_id == 21329 {
            return Some(CrimsonDoor {
                r#facing: Facing::South,
                r#open: true,
                r#powered: false,
                r#hinge: Hinge::Left,
                r#half: Half::Upper,
            });
        }
        return None;
    }
}

