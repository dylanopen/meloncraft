use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BambooTrapdoor {
    pub r#facing: Facing,
    pub powered: bool,
    pub r#half: Half,
    pub waterlogged: bool,
    pub open: bool,
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

impl BlockState for BambooTrapdoor {
    fn to_id(self) -> i32 {
        if block_state.r#facing == Facing::South && block_state.r#powered == false && block_state.r#waterlogged == false && block_state.r#open == true && block_state.r#half == Half::Top { return 7508; }
        if block_state.r#powered == true && block_state.r#half == Half::Bottom && block_state.r#open == true && block_state.r#waterlogged == true && block_state.r#facing == Facing::South { return 7513; }
        if block_state.r#facing == Facing::East && block_state.r#powered == false && block_state.r#waterlogged == true && block_state.r#half == Half::Top && block_state.r#open == true { return 7539; }
        if block_state.r#open == true && block_state.r#facing == Facing::East && block_state.r#powered == true && block_state.r#half == Half::Top && block_state.r#waterlogged == false { return 7538; }
        if block_state.r#powered == false && block_state.r#waterlogged == false && block_state.r#half == Half::Top && block_state.r#open == false && block_state.r#facing == Facing::North { return 7496; }
        if block_state.r#facing == Facing::West && block_state.r#half == Half::Top && block_state.r#open == true && block_state.r#powered == false && block_state.r#waterlogged == true { return 7523; }
        if block_state.r#facing == Facing::East && block_state.r#half == Half::Bottom && block_state.r#open == true && block_state.r#powered == false && block_state.r#waterlogged == true { return 7547; }
        if block_state.r#powered == true && block_state.r#facing == Facing::North && block_state.r#waterlogged == false && block_state.r#half == Half::Bottom && block_state.r#open == false { return 7502; }
        if block_state.r#facing == Facing::South && block_state.r#half == Half::Bottom && block_state.r#open == true && block_state.r#waterlogged == false && block_state.r#powered == false { return 7516; }
        if block_state.r#waterlogged == false && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::West && block_state.r#powered == true && block_state.r#open == true { return 7530; }
        if block_state.r#facing == Facing::North && block_state.r#half == Half::Top && block_state.r#waterlogged == false && block_state.r#open == true && block_state.r#powered == true { return 7490; }
        if block_state.r#open == true && block_state.r#powered == true && block_state.r#waterlogged == true && block_state.r#facing == Facing::West && block_state.r#half == Half::Top { return 7521; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == true && block_state.r#powered == false && block_state.r#half == Half::Bottom && block_state.r#open == false { return 7519; }
        if block_state.r#half == Half::Top && block_state.r#waterlogged == true && block_state.r#open == false && block_state.r#facing == Facing::South && block_state.r#powered == true { return 7509; }
        if block_state.r#waterlogged == false && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::West && block_state.r#open == false && block_state.r#powered == false { return 7536; }
        if block_state.r#open == false && block_state.r#powered == false && block_state.r#waterlogged == false && block_state.r#facing == Facing::West && block_state.r#half == Half::Top { return 7528; }
        if block_state.r#open == false && block_state.r#powered == false && block_state.r#waterlogged == true && block_state.r#facing == Facing::West && block_state.r#half == Half::Bottom { return 7535; }
        if block_state.r#powered == true && block_state.r#facing == Facing::West && block_state.r#half == Half::Top && block_state.r#waterlogged == true && block_state.r#open == false { return 7525; }
        if block_state.r#powered == false && block_state.r#facing == Facing::North && block_state.r#half == Half::Top && block_state.r#open == false && block_state.r#waterlogged == true { return 7495; }
        if block_state.r#facing == Facing::East && block_state.r#half == Half::Top && block_state.r#open == false && block_state.r#powered == true && block_state.r#waterlogged == true { return 7541; }
        if block_state.r#half == Half::Top && block_state.r#waterlogged == false && block_state.r#open == true && block_state.r#powered == true && block_state.r#facing == Facing::South { return 7506; }
        if block_state.r#facing == Facing::North && block_state.r#powered == true && block_state.r#half == Half::Bottom && block_state.r#open == true && block_state.r#waterlogged == false { return 7498; }
        if block_state.r#half == Half::Bottom && block_state.r#open == false && block_state.r#powered == false && block_state.r#facing == Facing::North && block_state.r#waterlogged == true { return 7503; }
        if block_state.r#facing == Facing::South && block_state.r#powered == true && block_state.r#waterlogged == false && block_state.r#open == false && block_state.r#half == Half::Top { return 7510; }
        if block_state.r#open == false && block_state.r#waterlogged == true && block_state.r#half == Half::Top && block_state.r#facing == Facing::South && block_state.r#powered == false { return 7511; }
        if block_state.r#facing == Facing::South && block_state.r#half == Half::Top && block_state.r#open == false && block_state.r#powered == false && block_state.r#waterlogged == false { return 7512; }
        if block_state.r#half == Half::Bottom && block_state.r#powered == true && block_state.r#facing == Facing::South && block_state.r#open == false && block_state.r#waterlogged == true { return 7517; }
        if block_state.r#waterlogged == true && block_state.r#open == true && block_state.r#facing == Facing::North && block_state.r#powered == true && block_state.r#half == Half::Bottom { return 7497; }
        if block_state.r#waterlogged == false && block_state.r#open == true && block_state.r#facing == Facing::East && block_state.r#half == Half::Bottom && block_state.r#powered == false { return 7548; }
        if block_state.r#powered == false && block_state.r#facing == Facing::West && block_state.r#half == Half::Bottom && block_state.r#open == true && block_state.r#waterlogged == true { return 7531; }
        if block_state.r#open == true && block_state.r#waterlogged == false && block_state.r#facing == Facing::North && block_state.r#half == Half::Top && block_state.r#powered == false { return 7492; }
        if block_state.r#open == true && block_state.r#powered == false && block_state.r#facing == Facing::West && block_state.r#waterlogged == false && block_state.r#half == Half::Bottom { return 7532; }
        if block_state.r#powered == true && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::East && block_state.r#waterlogged == false && block_state.r#open == true { return 7546; }
        if block_state.r#facing == Facing::North && block_state.r#half == Half::Bottom && block_state.r#open == false && block_state.r#powered == true && block_state.r#waterlogged == true { return 7501; }
        if block_state.r#waterlogged == true && block_state.r#powered == true && block_state.r#open == false && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::East { return 7549; }
        if block_state.r#powered == false && block_state.r#waterlogged == false && block_state.r#facing == Facing::North && block_state.r#half == Half::Bottom && block_state.r#open == true { return 7500; }
        if block_state.r#waterlogged == false && block_state.r#open == false && block_state.r#powered == true && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::West { return 7534; }
        if block_state.r#open == true && block_state.r#half == Half::Top && block_state.r#waterlogged == true && block_state.r#facing == Facing::North && block_state.r#powered == false { return 7491; }
        if block_state.r#waterlogged == false && block_state.r#open == false && block_state.r#facing == Facing::West && block_state.r#powered == true && block_state.r#half == Half::Top { return 7526; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::East && block_state.r#half == Half::Bottom && block_state.r#open == false && block_state.r#powered == false { return 7552; }
        if block_state.r#powered == true && block_state.r#open == true && block_state.r#facing == Facing::South && block_state.r#half == Half::Bottom && block_state.r#waterlogged == false { return 7514; }
        if block_state.r#facing == Facing::East && block_state.r#open == true && block_state.r#half == Half::Top && block_state.r#powered == true && block_state.r#waterlogged == true { return 7537; }
        if block_state.r#half == Half::Top && block_state.r#facing == Facing::South && block_state.r#open == true && block_state.r#powered == true && block_state.r#waterlogged == true { return 7505; }
        if block_state.r#half == Half::Bottom && block_state.r#powered == false && block_state.r#facing == Facing::North && block_state.r#waterlogged == true && block_state.r#open == true { return 7499; }
        if block_state.r#waterlogged == false && block_state.r#half == Half::Top && block_state.r#powered == true && block_state.r#facing == Facing::East && block_state.r#open == false { return 7542; }
        if block_state.r#facing == Facing::West && block_state.r#open == false && block_state.r#powered == false && block_state.r#waterlogged == true && block_state.r#half == Half::Top { return 7527; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == false && block_state.r#half == Half::Top && block_state.r#open == false && block_state.r#powered == true { return 7494; }
        if block_state.r#waterlogged == false && block_state.r#powered == false && block_state.r#facing == Facing::West && block_state.r#open == true && block_state.r#half == Half::Top { return 7524; }
        if block_state.r#waterlogged == true && block_state.r#powered == true && block_state.r#facing == Facing::West && block_state.r#half == Half::Bottom && block_state.r#open == true { return 7529; }
        if block_state.r#half == Half::Top && block_state.r#facing == Facing::North && block_state.r#open == true && block_state.r#waterlogged == true && block_state.r#powered == true { return 7489; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::West && block_state.r#open == true && block_state.r#half == Half::Top && block_state.r#powered == true { return 7522; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == true && block_state.r#open == false && block_state.r#powered == false && block_state.r#half == Half::Bottom { return 7551; }
        if block_state.r#waterlogged == false && block_state.r#half == Half::Top && block_state.r#powered == false && block_state.r#facing == Facing::East && block_state.r#open == false { return 7544; }
        if block_state.r#facing == Facing::East && block_state.r#half == Half::Top && block_state.r#powered == false && block_state.r#open == true && block_state.r#waterlogged == false { return 7540; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::East && block_state.r#open == false && block_state.r#half == Half::Top && block_state.r#powered == false { return 7543; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::North && block_state.r#half == Half::Top && block_state.r#open == false && block_state.r#powered == true { return 7493; }
        if block_state.r#waterlogged == false && block_state.r#open == false && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::East && block_state.r#powered == true { return 7550; }
        if block_state.r#open == false && block_state.r#facing == Facing::South && block_state.r#half == Half::Bottom && block_state.r#waterlogged == false && block_state.r#powered == true { return 7518; }
        if block_state.r#waterlogged == true && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::East && block_state.r#powered == true && block_state.r#open == true { return 7545; }
        if block_state.r#powered == false && block_state.r#facing == Facing::North && block_state.r#half == Half::Bottom && block_state.r#waterlogged == false && block_state.r#open == false { return 7504; }
        if block_state.r#half == Half::Bottom && block_state.r#open == false && block_state.r#facing == Facing::West && block_state.r#powered == true && block_state.r#waterlogged == true { return 7533; }
        if block_state.r#facing == Facing::South && block_state.r#open == false && block_state.r#half == Half::Bottom && block_state.r#waterlogged == false && block_state.r#powered == false { return 7520; }
        if block_state.r#half == Half::Bottom && block_state.r#waterlogged == true && block_state.r#open == true && block_state.r#facing == Facing::South && block_state.r#powered == false { return 7515; }
        if block_state.r#open == true && block_state.r#facing == Facing::South && block_state.r#half == Half::Top && block_state.r#powered == false && block_state.r#waterlogged == true { return 7507; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 7508 {
            return Some(BambooTrapdoor {
                r#facing: Facing::South,
                r#powered: false,
                r#waterlogged: false,
                r#open: true,
                r#half: Half::Top,
            });
        }
        if state_id == 7513 {
            return Some(BambooTrapdoor {
                r#powered: true,
                r#half: Half::Bottom,
                r#open: true,
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 7539 {
            return Some(BambooTrapdoor {
                r#facing: Facing::East,
                r#powered: false,
                r#waterlogged: true,
                r#half: Half::Top,
                r#open: true,
            });
        }
        if state_id == 7538 {
            return Some(BambooTrapdoor {
                r#open: true,
                r#facing: Facing::East,
                r#powered: true,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 7496 {
            return Some(BambooTrapdoor {
                r#powered: false,
                r#waterlogged: false,
                r#half: Half::Top,
                r#open: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 7523 {
            return Some(BambooTrapdoor {
                r#facing: Facing::West,
                r#half: Half::Top,
                r#open: true,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 7547 {
            return Some(BambooTrapdoor {
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#open: true,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 7502 {
            return Some(BambooTrapdoor {
                r#powered: true,
                r#facing: Facing::North,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#open: false,
            });
        }
        if state_id == 7516 {
            return Some(BambooTrapdoor {
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#open: true,
                r#waterlogged: false,
                r#powered: false,
            });
        }
        if state_id == 7530 {
            return Some(BambooTrapdoor {
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#powered: true,
                r#open: true,
            });
        }
        if state_id == 7490 {
            return Some(BambooTrapdoor {
                r#facing: Facing::North,
                r#half: Half::Top,
                r#waterlogged: false,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 7521 {
            return Some(BambooTrapdoor {
                r#open: true,
                r#powered: true,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#half: Half::Top,
            });
        }
        if state_id == 7519 {
            return Some(BambooTrapdoor {
                r#facing: Facing::South,
                r#waterlogged: true,
                r#powered: false,
                r#half: Half::Bottom,
                r#open: false,
            });
        }
        if state_id == 7509 {
            return Some(BambooTrapdoor {
                r#half: Half::Top,
                r#waterlogged: true,
                r#open: false,
                r#facing: Facing::South,
                r#powered: true,
            });
        }
        if state_id == 7536 {
            return Some(BambooTrapdoor {
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 7528 {
            return Some(BambooTrapdoor {
                r#open: false,
                r#powered: false,
                r#waterlogged: false,
                r#facing: Facing::West,
                r#half: Half::Top,
            });
        }
        if state_id == 7535 {
            return Some(BambooTrapdoor {
                r#open: false,
                r#powered: false,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#half: Half::Bottom,
            });
        }
        if state_id == 7525 {
            return Some(BambooTrapdoor {
                r#powered: true,
                r#facing: Facing::West,
                r#half: Half::Top,
                r#waterlogged: true,
                r#open: false,
            });
        }
        if state_id == 7495 {
            return Some(BambooTrapdoor {
                r#powered: false,
                r#facing: Facing::North,
                r#half: Half::Top,
                r#open: false,
                r#waterlogged: true,
            });
        }
        if state_id == 7541 {
            return Some(BambooTrapdoor {
                r#facing: Facing::East,
                r#half: Half::Top,
                r#open: false,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 7506 {
            return Some(BambooTrapdoor {
                r#half: Half::Top,
                r#waterlogged: false,
                r#open: true,
                r#powered: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 7498 {
            return Some(BambooTrapdoor {
                r#facing: Facing::North,
                r#powered: true,
                r#half: Half::Bottom,
                r#open: true,
                r#waterlogged: false,
            });
        }
        if state_id == 7503 {
            return Some(BambooTrapdoor {
                r#half: Half::Bottom,
                r#open: false,
                r#powered: false,
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 7510 {
            return Some(BambooTrapdoor {
                r#facing: Facing::South,
                r#powered: true,
                r#waterlogged: false,
                r#open: false,
                r#half: Half::Top,
            });
        }
        if state_id == 7511 {
            return Some(BambooTrapdoor {
                r#open: false,
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::South,
                r#powered: false,
            });
        }
        if state_id == 7512 {
            return Some(BambooTrapdoor {
                r#facing: Facing::South,
                r#half: Half::Top,
                r#open: false,
                r#powered: false,
                r#waterlogged: false,
            });
        }
        if state_id == 7517 {
            return Some(BambooTrapdoor {
                r#half: Half::Bottom,
                r#powered: true,
                r#facing: Facing::South,
                r#open: false,
                r#waterlogged: true,
            });
        }
        if state_id == 7497 {
            return Some(BambooTrapdoor {
                r#waterlogged: true,
                r#open: true,
                r#facing: Facing::North,
                r#powered: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 7548 {
            return Some(BambooTrapdoor {
                r#waterlogged: false,
                r#open: true,
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#powered: false,
            });
        }
        if state_id == 7531 {
            return Some(BambooTrapdoor {
                r#powered: false,
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#open: true,
                r#waterlogged: true,
            });
        }
        if state_id == 7492 {
            return Some(BambooTrapdoor {
                r#open: true,
                r#waterlogged: false,
                r#facing: Facing::North,
                r#half: Half::Top,
                r#powered: false,
            });
        }
        if state_id == 7532 {
            return Some(BambooTrapdoor {
                r#open: true,
                r#powered: false,
                r#facing: Facing::West,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 7546 {
            return Some(BambooTrapdoor {
                r#powered: true,
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#waterlogged: false,
                r#open: true,
            });
        }
        if state_id == 7501 {
            return Some(BambooTrapdoor {
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#open: false,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 7549 {
            return Some(BambooTrapdoor {
                r#waterlogged: true,
                r#powered: true,
                r#open: false,
                r#half: Half::Bottom,
                r#facing: Facing::East,
            });
        }
        if state_id == 7500 {
            return Some(BambooTrapdoor {
                r#powered: false,
                r#waterlogged: false,
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#open: true,
            });
        }
        if state_id == 7534 {
            return Some(BambooTrapdoor {
                r#waterlogged: false,
                r#open: false,
                r#powered: true,
                r#half: Half::Bottom,
                r#facing: Facing::West,
            });
        }
        if state_id == 7491 {
            return Some(BambooTrapdoor {
                r#open: true,
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::North,
                r#powered: false,
            });
        }
        if state_id == 7526 {
            return Some(BambooTrapdoor {
                r#waterlogged: false,
                r#open: false,
                r#facing: Facing::West,
                r#powered: true,
                r#half: Half::Top,
            });
        }
        if state_id == 7552 {
            return Some(BambooTrapdoor {
                r#waterlogged: false,
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 7514 {
            return Some(BambooTrapdoor {
                r#powered: true,
                r#open: true,
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 7537 {
            return Some(BambooTrapdoor {
                r#facing: Facing::East,
                r#open: true,
                r#half: Half::Top,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 7505 {
            return Some(BambooTrapdoor {
                r#half: Half::Top,
                r#facing: Facing::South,
                r#open: true,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 7499 {
            return Some(BambooTrapdoor {
                r#half: Half::Bottom,
                r#powered: false,
                r#facing: Facing::North,
                r#waterlogged: true,
                r#open: true,
            });
        }
        if state_id == 7542 {
            return Some(BambooTrapdoor {
                r#waterlogged: false,
                r#half: Half::Top,
                r#powered: true,
                r#facing: Facing::East,
                r#open: false,
            });
        }
        if state_id == 7527 {
            return Some(BambooTrapdoor {
                r#facing: Facing::West,
                r#open: false,
                r#powered: false,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 7494 {
            return Some(BambooTrapdoor {
                r#facing: Facing::North,
                r#waterlogged: false,
                r#half: Half::Top,
                r#open: false,
                r#powered: true,
            });
        }
        if state_id == 7524 {
            return Some(BambooTrapdoor {
                r#waterlogged: false,
                r#powered: false,
                r#facing: Facing::West,
                r#open: true,
                r#half: Half::Top,
            });
        }
        if state_id == 7529 {
            return Some(BambooTrapdoor {
                r#waterlogged: true,
                r#powered: true,
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#open: true,
            });
        }
        if state_id == 7489 {
            return Some(BambooTrapdoor {
                r#half: Half::Top,
                r#facing: Facing::North,
                r#open: true,
                r#waterlogged: true,
                r#powered: true,
            });
        }
        if state_id == 7522 {
            return Some(BambooTrapdoor {
                r#waterlogged: false,
                r#facing: Facing::West,
                r#open: true,
                r#half: Half::Top,
                r#powered: true,
            });
        }
        if state_id == 7551 {
            return Some(BambooTrapdoor {
                r#facing: Facing::East,
                r#waterlogged: true,
                r#open: false,
                r#powered: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 7544 {
            return Some(BambooTrapdoor {
                r#waterlogged: false,
                r#half: Half::Top,
                r#powered: false,
                r#facing: Facing::East,
                r#open: false,
            });
        }
        if state_id == 7540 {
            return Some(BambooTrapdoor {
                r#facing: Facing::East,
                r#half: Half::Top,
                r#powered: false,
                r#open: true,
                r#waterlogged: false,
            });
        }
        if state_id == 7543 {
            return Some(BambooTrapdoor {
                r#waterlogged: true,
                r#facing: Facing::East,
                r#open: false,
                r#half: Half::Top,
                r#powered: false,
            });
        }
        if state_id == 7493 {
            return Some(BambooTrapdoor {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#half: Half::Top,
                r#open: false,
                r#powered: true,
            });
        }
        if state_id == 7550 {
            return Some(BambooTrapdoor {
                r#waterlogged: false,
                r#open: false,
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#powered: true,
            });
        }
        if state_id == 7518 {
            return Some(BambooTrapdoor {
                r#open: false,
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#powered: true,
            });
        }
        if state_id == 7545 {
            return Some(BambooTrapdoor {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#powered: true,
                r#open: true,
            });
        }
        if state_id == 7504 {
            return Some(BambooTrapdoor {
                r#powered: false,
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#open: false,
            });
        }
        if state_id == 7533 {
            return Some(BambooTrapdoor {
                r#half: Half::Bottom,
                r#open: false,
                r#facing: Facing::West,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 7520 {
            return Some(BambooTrapdoor {
                r#facing: Facing::South,
                r#open: false,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#powered: false,
            });
        }
        if state_id == 7515 {
            return Some(BambooTrapdoor {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#open: true,
                r#facing: Facing::South,
                r#powered: false,
            });
        }
        if state_id == 7507 {
            return Some(BambooTrapdoor {
                r#open: true,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        return None;
    }
}

