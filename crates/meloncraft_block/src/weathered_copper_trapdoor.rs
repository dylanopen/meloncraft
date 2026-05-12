use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WeatheredCopperTrapdoor {
    pub r#half: Half,
    pub open: bool,
    pub powered: bool,
    pub waterlogged: bool,
    pub r#facing: Facing,
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

impl BlockState for WeatheredCopperTrapdoor {
    fn to_id(self) -> i32 {
        if block_state.r#half == Half::Bottom && block_state.r#facing == Facing::South && block_state.r#powered == true && block_state.r#open == true && block_state.r#waterlogged == false { return 26550; }
        if block_state.r#open == true && block_state.r#powered == true && block_state.r#half == Half::Top && block_state.r#facing == Facing::North && block_state.r#waterlogged == true { return 26525; }
        if block_state.r#half == Half::Bottom && block_state.r#powered == false && block_state.r#waterlogged == false && block_state.r#facing == Facing::South && block_state.r#open == false { return 26556; }
        if block_state.r#open == false && block_state.r#half == Half::Bottom && block_state.r#powered == false && block_state.r#waterlogged == true && block_state.r#facing == Facing::South { return 26555; }
        if block_state.r#facing == Facing::North && block_state.r#half == Half::Bottom && block_state.r#powered == true && block_state.r#open == false && block_state.r#waterlogged == false { return 26538; }
        if block_state.r#waterlogged == true && block_state.r#open == true && block_state.r#powered == false && block_state.r#facing == Facing::North && block_state.r#half == Half::Top { return 26527; }
        if block_state.r#waterlogged == true && block_state.r#open == false && block_state.r#facing == Facing::North && block_state.r#half == Half::Top && block_state.r#powered == true { return 26529; }
        if block_state.r#powered == true && block_state.r#waterlogged == true && block_state.r#half == Half::Bottom && block_state.r#open == true && block_state.r#facing == Facing::South { return 26549; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == false && block_state.r#powered == false && block_state.r#open == true && block_state.r#half == Half::Top { return 26544; }
        if block_state.r#open == true && block_state.r#facing == Facing::West && block_state.r#powered == true && block_state.r#half == Half::Bottom && block_state.r#waterlogged == false { return 26566; }
        if block_state.r#facing == Facing::North && block_state.r#half == Half::Bottom && block_state.r#powered == false && block_state.r#waterlogged == false && block_state.r#open == false { return 26540; }
        if block_state.r#facing == Facing::South && block_state.r#powered == true && block_state.r#half == Half::Top && block_state.r#waterlogged == false && block_state.r#open == true { return 26542; }
        if block_state.r#half == Half::Bottom && block_state.r#powered == true && block_state.r#waterlogged == true && block_state.r#facing == Facing::North && block_state.r#open == false { return 26537; }
        if block_state.r#powered == true && block_state.r#open == false && block_state.r#half == Half::Bottom && block_state.r#waterlogged == false && block_state.r#facing == Facing::South { return 26554; }
        if block_state.r#half == Half::Bottom && block_state.r#powered == true && block_state.r#waterlogged == true && block_state.r#open == false && block_state.r#facing == Facing::West { return 26569; }
        if block_state.r#facing == Facing::North && block_state.r#open == true && block_state.r#powered == true && block_state.r#waterlogged == false && block_state.r#half == Half::Top { return 26526; }
        if block_state.r#powered == false && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::West && block_state.r#open == false && block_state.r#waterlogged == true { return 26571; }
        if block_state.r#open == false && block_state.r#facing == Facing::East && block_state.r#half == Half::Top && block_state.r#powered == false && block_state.r#waterlogged == false { return 26580; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::East && block_state.r#half == Half::Bottom && block_state.r#open == true && block_state.r#powered == true { return 26582; }
        if block_state.r#powered == false && block_state.r#open == false && block_state.r#waterlogged == true && block_state.r#facing == Facing::South && block_state.r#half == Half::Top { return 26547; }
        if block_state.r#open == true && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true && block_state.r#powered == false && block_state.r#facing == Facing::South { return 26551; }
        if block_state.r#half == Half::Top && block_state.r#powered == true && block_state.r#open == false && block_state.r#waterlogged == true && block_state.r#facing == Facing::West { return 26561; }
        if block_state.r#powered == false && block_state.r#facing == Facing::East && block_state.r#half == Half::Top && block_state.r#waterlogged == false && block_state.r#open == true { return 26576; }
        if block_state.r#open == true && block_state.r#waterlogged == false && block_state.r#facing == Facing::East && block_state.r#powered == false && block_state.r#half == Half::Bottom { return 26584; }
        if block_state.r#powered == true && block_state.r#facing == Facing::East && block_state.r#waterlogged == false && block_state.r#half == Half::Top && block_state.r#open == true { return 26574; }
        if block_state.r#powered == false && block_state.r#open == true && block_state.r#waterlogged == true && block_state.r#facing == Facing::North && block_state.r#half == Half::Bottom { return 26535; }
        if block_state.r#half == Half::Top && block_state.r#open == false && block_state.r#powered == true && block_state.r#facing == Facing::South && block_state.r#waterlogged == false { return 26546; }
        if block_state.r#powered == true && block_state.r#facing == Facing::West && block_state.r#waterlogged == false && block_state.r#open == true && block_state.r#half == Half::Top { return 26558; }
        if block_state.r#facing == Facing::West && block_state.r#half == Half::Top && block_state.r#powered == true && block_state.r#open == false && block_state.r#waterlogged == false { return 26562; }
        if block_state.r#half == Half::Top && block_state.r#waterlogged == false && block_state.r#facing == Facing::North && block_state.r#open == false && block_state.r#powered == false { return 26532; }
        if block_state.r#powered == false && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::West && block_state.r#waterlogged == false && block_state.r#open == true { return 26568; }
        if block_state.r#powered == false && block_state.r#half == Half::Bottom && block_state.r#waterlogged == false && block_state.r#facing == Facing::West && block_state.r#open == false { return 26572; }
        if block_state.r#open == true && block_state.r#half == Half::Top && block_state.r#facing == Facing::East && block_state.r#powered == false && block_state.r#waterlogged == true { return 26575; }
        if block_state.r#open == false && block_state.r#waterlogged == false && block_state.r#powered == true && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::East { return 26586; }
        if block_state.r#open == false && block_state.r#waterlogged == true && block_state.r#half == Half::Bottom && block_state.r#powered == false && block_state.r#facing == Facing::East { return 26587; }
        if block_state.r#waterlogged == true && block_state.r#half == Half::Top && block_state.r#facing == Facing::West && block_state.r#open == false && block_state.r#powered == false { return 26563; }
        if block_state.r#half == Half::Top && block_state.r#open == false && block_state.r#powered == false && block_state.r#waterlogged == false && block_state.r#facing == Facing::West { return 26564; }
        if block_state.r#half == Half::Bottom && block_state.r#powered == true && block_state.r#waterlogged == true && block_state.r#facing == Facing::South && block_state.r#open == false { return 26553; }
        if block_state.r#facing == Facing::North && block_state.r#powered == false && block_state.r#waterlogged == false && block_state.r#half == Half::Bottom && block_state.r#open == true { return 26536; }
        if block_state.r#half == Half::Top && block_state.r#open == true && block_state.r#facing == Facing::South && block_state.r#powered == false && block_state.r#waterlogged == true { return 26543; }
        if block_state.r#facing == Facing::North && block_state.r#half == Half::Bottom && block_state.r#powered == true && block_state.r#waterlogged == false && block_state.r#open == true { return 26534; }
        if block_state.r#half == Half::Top && block_state.r#facing == Facing::South && block_state.r#powered == true && block_state.r#open == true && block_state.r#waterlogged == true { return 26541; }
        if block_state.r#waterlogged == false && block_state.r#half == Half::Top && block_state.r#powered == false && block_state.r#open == true && block_state.r#facing == Facing::West { return 26560; }
        if block_state.r#facing == Facing::South && block_state.r#powered == false && block_state.r#half == Half::Top && block_state.r#open == false && block_state.r#waterlogged == false { return 26548; }
        if block_state.r#facing == Facing::East && block_state.r#open == false && block_state.r#waterlogged == false && block_state.r#half == Half::Bottom && block_state.r#powered == false { return 26588; }
        if block_state.r#half == Half::Bottom && block_state.r#waterlogged == true && block_state.r#powered == true && block_state.r#facing == Facing::East && block_state.r#open == true { return 26581; }
        if block_state.r#facing == Facing::West && block_state.r#half == Half::Bottom && block_state.r#powered == true && block_state.r#waterlogged == false && block_state.r#open == false { return 26570; }
        if block_state.r#open == false && block_state.r#powered == true && block_state.r#facing == Facing::South && block_state.r#half == Half::Top && block_state.r#waterlogged == true { return 26545; }
        if block_state.r#half == Half::Top && block_state.r#open == true && block_state.r#facing == Facing::East && block_state.r#powered == true && block_state.r#waterlogged == true { return 26573; }
        if block_state.r#open == false && block_state.r#powered == false && block_state.r#facing == Facing::East && block_state.r#half == Half::Top && block_state.r#waterlogged == true { return 26579; }
        if block_state.r#facing == Facing::North && block_state.r#powered == false && block_state.r#half == Half::Top && block_state.r#waterlogged == true && block_state.r#open == false { return 26531; }
        if block_state.r#powered == true && block_state.r#open == false && block_state.r#facing == Facing::North && block_state.r#waterlogged == false && block_state.r#half == Half::Top { return 26530; }
        if block_state.r#half == Half::Bottom && block_state.r#facing == Facing::East && block_state.r#open == false && block_state.r#waterlogged == true && block_state.r#powered == true { return 26585; }
        if block_state.r#half == Half::Bottom && block_state.r#facing == Facing::South && block_state.r#powered == false && block_state.r#waterlogged == false && block_state.r#open == true { return 26552; }
        if block_state.r#waterlogged == true && block_state.r#open == false && block_state.r#half == Half::Top && block_state.r#facing == Facing::East && block_state.r#powered == true { return 26577; }
        if block_state.r#half == Half::Bottom && block_state.r#powered == true && block_state.r#open == true && block_state.r#waterlogged == true && block_state.r#facing == Facing::North { return 26533; }
        if block_state.r#waterlogged == true && block_state.r#open == true && block_state.r#facing == Facing::West && block_state.r#powered == false && block_state.r#half == Half::Top { return 26559; }
        if block_state.r#open == true && block_state.r#waterlogged == true && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::West && block_state.r#powered == true { return 26565; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == true && block_state.r#half == Half::Bottom && block_state.r#powered == false && block_state.r#open == true { return 26583; }
        if block_state.r#facing == Facing::West && block_state.r#half == Half::Bottom && block_state.r#open == true && block_state.r#powered == false && block_state.r#waterlogged == true { return 26567; }
        if block_state.r#powered == true && block_state.r#half == Half::Top && block_state.r#facing == Facing::West && block_state.r#open == true && block_state.r#waterlogged == true { return 26557; }
        if block_state.r#powered == false && block_state.r#waterlogged == false && block_state.r#half == Half::Top && block_state.r#facing == Facing::North && block_state.r#open == true { return 26528; }
        if block_state.r#half == Half::Top && block_state.r#open == false && block_state.r#powered == true && block_state.r#waterlogged == false && block_state.r#facing == Facing::East { return 26578; }
        if block_state.r#open == false && block_state.r#powered == false && block_state.r#facing == Facing::North && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true { return 26539; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 26550 {
            return Some(WeatheredCopperTrapdoor {
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#powered: true,
                r#open: true,
                r#waterlogged: false,
            });
        }
        if state_id == 26525 {
            return Some(WeatheredCopperTrapdoor {
                r#open: true,
                r#powered: true,
                r#half: Half::Top,
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 26556 {
            return Some(WeatheredCopperTrapdoor {
                r#half: Half::Bottom,
                r#powered: false,
                r#waterlogged: false,
                r#facing: Facing::South,
                r#open: false,
            });
        }
        if state_id == 26555 {
            return Some(WeatheredCopperTrapdoor {
                r#open: false,
                r#half: Half::Bottom,
                r#powered: false,
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 26538 {
            return Some(WeatheredCopperTrapdoor {
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#powered: true,
                r#open: false,
                r#waterlogged: false,
            });
        }
        if state_id == 26527 {
            return Some(WeatheredCopperTrapdoor {
                r#waterlogged: true,
                r#open: true,
                r#powered: false,
                r#facing: Facing::North,
                r#half: Half::Top,
            });
        }
        if state_id == 26529 {
            return Some(WeatheredCopperTrapdoor {
                r#waterlogged: true,
                r#open: false,
                r#facing: Facing::North,
                r#half: Half::Top,
                r#powered: true,
            });
        }
        if state_id == 26549 {
            return Some(WeatheredCopperTrapdoor {
                r#powered: true,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#open: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 26544 {
            return Some(WeatheredCopperTrapdoor {
                r#facing: Facing::South,
                r#waterlogged: false,
                r#powered: false,
                r#open: true,
                r#half: Half::Top,
            });
        }
        if state_id == 26566 {
            return Some(WeatheredCopperTrapdoor {
                r#open: true,
                r#facing: Facing::West,
                r#powered: true,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 26540 {
            return Some(WeatheredCopperTrapdoor {
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#powered: false,
                r#waterlogged: false,
                r#open: false,
            });
        }
        if state_id == 26542 {
            return Some(WeatheredCopperTrapdoor {
                r#facing: Facing::South,
                r#powered: true,
                r#half: Half::Top,
                r#waterlogged: false,
                r#open: true,
            });
        }
        if state_id == 26537 {
            return Some(WeatheredCopperTrapdoor {
                r#half: Half::Bottom,
                r#powered: true,
                r#waterlogged: true,
                r#facing: Facing::North,
                r#open: false,
            });
        }
        if state_id == 26554 {
            return Some(WeatheredCopperTrapdoor {
                r#powered: true,
                r#open: false,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 26569 {
            return Some(WeatheredCopperTrapdoor {
                r#half: Half::Bottom,
                r#powered: true,
                r#waterlogged: true,
                r#open: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 26526 {
            return Some(WeatheredCopperTrapdoor {
                r#facing: Facing::North,
                r#open: true,
                r#powered: true,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 26571 {
            return Some(WeatheredCopperTrapdoor {
                r#powered: false,
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#open: false,
                r#waterlogged: true,
            });
        }
        if state_id == 26580 {
            return Some(WeatheredCopperTrapdoor {
                r#open: false,
                r#facing: Facing::East,
                r#half: Half::Top,
                r#powered: false,
                r#waterlogged: false,
            });
        }
        if state_id == 26582 {
            return Some(WeatheredCopperTrapdoor {
                r#waterlogged: false,
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 26547 {
            return Some(WeatheredCopperTrapdoor {
                r#powered: false,
                r#open: false,
                r#waterlogged: true,
                r#facing: Facing::South,
                r#half: Half::Top,
            });
        }
        if state_id == 26551 {
            return Some(WeatheredCopperTrapdoor {
                r#open: true,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#powered: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 26561 {
            return Some(WeatheredCopperTrapdoor {
                r#half: Half::Top,
                r#powered: true,
                r#open: false,
                r#waterlogged: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 26576 {
            return Some(WeatheredCopperTrapdoor {
                r#powered: false,
                r#facing: Facing::East,
                r#half: Half::Top,
                r#waterlogged: false,
                r#open: true,
            });
        }
        if state_id == 26584 {
            return Some(WeatheredCopperTrapdoor {
                r#open: true,
                r#waterlogged: false,
                r#facing: Facing::East,
                r#powered: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 26574 {
            return Some(WeatheredCopperTrapdoor {
                r#powered: true,
                r#facing: Facing::East,
                r#waterlogged: false,
                r#half: Half::Top,
                r#open: true,
            });
        }
        if state_id == 26535 {
            return Some(WeatheredCopperTrapdoor {
                r#powered: false,
                r#open: true,
                r#waterlogged: true,
                r#facing: Facing::North,
                r#half: Half::Bottom,
            });
        }
        if state_id == 26546 {
            return Some(WeatheredCopperTrapdoor {
                r#half: Half::Top,
                r#open: false,
                r#powered: true,
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 26558 {
            return Some(WeatheredCopperTrapdoor {
                r#powered: true,
                r#facing: Facing::West,
                r#waterlogged: false,
                r#open: true,
                r#half: Half::Top,
            });
        }
        if state_id == 26562 {
            return Some(WeatheredCopperTrapdoor {
                r#facing: Facing::West,
                r#half: Half::Top,
                r#powered: true,
                r#open: false,
                r#waterlogged: false,
            });
        }
        if state_id == 26532 {
            return Some(WeatheredCopperTrapdoor {
                r#half: Half::Top,
                r#waterlogged: false,
                r#facing: Facing::North,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 26568 {
            return Some(WeatheredCopperTrapdoor {
                r#powered: false,
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#waterlogged: false,
                r#open: true,
            });
        }
        if state_id == 26572 {
            return Some(WeatheredCopperTrapdoor {
                r#powered: false,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#facing: Facing::West,
                r#open: false,
            });
        }
        if state_id == 26575 {
            return Some(WeatheredCopperTrapdoor {
                r#open: true,
                r#half: Half::Top,
                r#facing: Facing::East,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 26586 {
            return Some(WeatheredCopperTrapdoor {
                r#open: false,
                r#waterlogged: false,
                r#powered: true,
                r#half: Half::Bottom,
                r#facing: Facing::East,
            });
        }
        if state_id == 26587 {
            return Some(WeatheredCopperTrapdoor {
                r#open: false,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#powered: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 26563 {
            return Some(WeatheredCopperTrapdoor {
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::West,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 26564 {
            return Some(WeatheredCopperTrapdoor {
                r#half: Half::Top,
                r#open: false,
                r#powered: false,
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 26553 {
            return Some(WeatheredCopperTrapdoor {
                r#half: Half::Bottom,
                r#powered: true,
                r#waterlogged: true,
                r#facing: Facing::South,
                r#open: false,
            });
        }
        if state_id == 26536 {
            return Some(WeatheredCopperTrapdoor {
                r#facing: Facing::North,
                r#powered: false,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#open: true,
            });
        }
        if state_id == 26543 {
            return Some(WeatheredCopperTrapdoor {
                r#half: Half::Top,
                r#open: true,
                r#facing: Facing::South,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 26534 {
            return Some(WeatheredCopperTrapdoor {
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#powered: true,
                r#waterlogged: false,
                r#open: true,
            });
        }
        if state_id == 26541 {
            return Some(WeatheredCopperTrapdoor {
                r#half: Half::Top,
                r#facing: Facing::South,
                r#powered: true,
                r#open: true,
                r#waterlogged: true,
            });
        }
        if state_id == 26560 {
            return Some(WeatheredCopperTrapdoor {
                r#waterlogged: false,
                r#half: Half::Top,
                r#powered: false,
                r#open: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 26548 {
            return Some(WeatheredCopperTrapdoor {
                r#facing: Facing::South,
                r#powered: false,
                r#half: Half::Top,
                r#open: false,
                r#waterlogged: false,
            });
        }
        if state_id == 26588 {
            return Some(WeatheredCopperTrapdoor {
                r#facing: Facing::East,
                r#open: false,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#powered: false,
            });
        }
        if state_id == 26581 {
            return Some(WeatheredCopperTrapdoor {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#powered: true,
                r#facing: Facing::East,
                r#open: true,
            });
        }
        if state_id == 26570 {
            return Some(WeatheredCopperTrapdoor {
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#powered: true,
                r#waterlogged: false,
                r#open: false,
            });
        }
        if state_id == 26545 {
            return Some(WeatheredCopperTrapdoor {
                r#open: false,
                r#powered: true,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 26573 {
            return Some(WeatheredCopperTrapdoor {
                r#half: Half::Top,
                r#open: true,
                r#facing: Facing::East,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 26579 {
            return Some(WeatheredCopperTrapdoor {
                r#open: false,
                r#powered: false,
                r#facing: Facing::East,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 26531 {
            return Some(WeatheredCopperTrapdoor {
                r#facing: Facing::North,
                r#powered: false,
                r#half: Half::Top,
                r#waterlogged: true,
                r#open: false,
            });
        }
        if state_id == 26530 {
            return Some(WeatheredCopperTrapdoor {
                r#powered: true,
                r#open: false,
                r#facing: Facing::North,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 26585 {
            return Some(WeatheredCopperTrapdoor {
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#open: false,
                r#waterlogged: true,
                r#powered: true,
            });
        }
        if state_id == 26552 {
            return Some(WeatheredCopperTrapdoor {
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#powered: false,
                r#waterlogged: false,
                r#open: true,
            });
        }
        if state_id == 26577 {
            return Some(WeatheredCopperTrapdoor {
                r#waterlogged: true,
                r#open: false,
                r#half: Half::Top,
                r#facing: Facing::East,
                r#powered: true,
            });
        }
        if state_id == 26533 {
            return Some(WeatheredCopperTrapdoor {
                r#half: Half::Bottom,
                r#powered: true,
                r#open: true,
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 26559 {
            return Some(WeatheredCopperTrapdoor {
                r#waterlogged: true,
                r#open: true,
                r#facing: Facing::West,
                r#powered: false,
                r#half: Half::Top,
            });
        }
        if state_id == 26565 {
            return Some(WeatheredCopperTrapdoor {
                r#open: true,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#powered: true,
            });
        }
        if state_id == 26583 {
            return Some(WeatheredCopperTrapdoor {
                r#facing: Facing::East,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#powered: false,
                r#open: true,
            });
        }
        if state_id == 26567 {
            return Some(WeatheredCopperTrapdoor {
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#open: true,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 26557 {
            return Some(WeatheredCopperTrapdoor {
                r#powered: true,
                r#half: Half::Top,
                r#facing: Facing::West,
                r#open: true,
                r#waterlogged: true,
            });
        }
        if state_id == 26528 {
            return Some(WeatheredCopperTrapdoor {
                r#powered: false,
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::North,
                r#open: true,
            });
        }
        if state_id == 26578 {
            return Some(WeatheredCopperTrapdoor {
                r#half: Half::Top,
                r#open: false,
                r#powered: true,
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 26539 {
            return Some(WeatheredCopperTrapdoor {
                r#open: false,
                r#powered: false,
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        return None;
    }
}

