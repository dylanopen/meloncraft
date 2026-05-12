use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MangroveTrapdoor {
    pub waterlogged: bool,
    pub r#facing: Facing,
    pub open: bool,
    pub r#half: Half,
    pub powered: bool,
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

impl BlockState for MangroveTrapdoor {
    fn to_id(self) -> i32 {
        if block_state.r#facing == Facing::West && block_state.r#open == false && block_state.r#half == Half::Top && block_state.r#waterlogged == true && block_state.r#powered == false { return 7463; }
        if block_state.r#half == Half::Top && block_state.r#powered == false && block_state.r#open == false && block_state.r#facing == Facing::South && block_state.r#waterlogged == true { return 7447; }
        if block_state.r#open == true && block_state.r#waterlogged == false && block_state.r#facing == Facing::North && block_state.r#powered == false && block_state.r#half == Half::Bottom { return 7436; }
        if block_state.r#open == true && block_state.r#powered == true && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::North && block_state.r#waterlogged == false { return 7434; }
        if block_state.r#waterlogged == true && block_state.r#half == Half::Top && block_state.r#facing == Facing::South && block_state.r#powered == true && block_state.r#open == true { return 7441; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::South && block_state.r#half == Half::Top && block_state.r#powered == false && block_state.r#open == false { return 7448; }
        if block_state.r#waterlogged == true && block_state.r#open == true && block_state.r#powered == false && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::North { return 7435; }
        if block_state.r#half == Half::Top && block_state.r#powered == false && block_state.r#facing == Facing::East && block_state.r#open == true && block_state.r#waterlogged == true { return 7475; }
        if block_state.r#half == Half::Bottom && block_state.r#waterlogged == true && block_state.r#open == true && block_state.r#facing == Facing::North && block_state.r#powered == true { return 7433; }
        if block_state.r#powered == true && block_state.r#half == Half::Top && block_state.r#waterlogged == true && block_state.r#open == false && block_state.r#facing == Facing::East { return 7477; }
        if block_state.r#half == Half::Top && block_state.r#open == false && block_state.r#powered == true && block_state.r#facing == Facing::East && block_state.r#waterlogged == false { return 7478; }
        if block_state.r#facing == Facing::East && block_state.r#half == Half::Top && block_state.r#powered == false && block_state.r#open == false && block_state.r#waterlogged == true { return 7479; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::West && block_state.r#open == false && block_state.r#half == Half::Top && block_state.r#powered == false { return 7464; }
        if block_state.r#half == Half::Top && block_state.r#powered == true && block_state.r#open == true && block_state.r#waterlogged == true && block_state.r#facing == Facing::North { return 7425; }
        if block_state.r#facing == Facing::North && block_state.r#open == false && block_state.r#powered == false && block_state.r#half == Half::Top && block_state.r#waterlogged == true { return 7431; }
        if block_state.r#open == false && block_state.r#powered == true && block_state.r#facing == Facing::West && block_state.r#waterlogged == false && block_state.r#half == Half::Top { return 7462; }
        if block_state.r#powered == false && block_state.r#half == Half::Bottom && block_state.r#open == true && block_state.r#facing == Facing::West && block_state.r#waterlogged == false { return 7468; }
        if block_state.r#open == true && block_state.r#facing == Facing::East && block_state.r#powered == false && block_state.r#half == Half::Top && block_state.r#waterlogged == false { return 7476; }
        if block_state.r#open == true && block_state.r#powered == false && block_state.r#facing == Facing::West && block_state.r#waterlogged == true && block_state.r#half == Half::Top { return 7459; }
        if block_state.r#half == Half::Bottom && block_state.r#open == false && block_state.r#waterlogged == true && block_state.r#facing == Facing::South && block_state.r#powered == false { return 7455; }
        if block_state.r#half == Half::Top && block_state.r#open == false && block_state.r#facing == Facing::West && block_state.r#powered == true && block_state.r#waterlogged == true { return 7461; }
        if block_state.r#open == true && block_state.r#waterlogged == false && block_state.r#half == Half::Top && block_state.r#facing == Facing::North && block_state.r#powered == false { return 7428; }
        if block_state.r#half == Half::Bottom && block_state.r#facing == Facing::North && block_state.r#powered == false && block_state.r#waterlogged == true && block_state.r#open == false { return 7439; }
        if block_state.r#waterlogged == false && block_state.r#open == false && block_state.r#facing == Facing::East && block_state.r#half == Half::Bottom && block_state.r#powered == true { return 7486; }
        if block_state.r#half == Half::Bottom && block_state.r#open == true && block_state.r#waterlogged == false && block_state.r#powered == false && block_state.r#facing == Facing::South { return 7452; }
        if block_state.r#powered == true && block_state.r#open == false && block_state.r#facing == Facing::South && block_state.r#waterlogged == false && block_state.r#half == Half::Top { return 7446; }
        if block_state.r#half == Half::Top && block_state.r#powered == true && block_state.r#facing == Facing::North && block_state.r#waterlogged == true && block_state.r#open == false { return 7429; }
        if block_state.r#open == true && block_state.r#powered == false && block_state.r#waterlogged == true && block_state.r#facing == Facing::South && block_state.r#half == Half::Top { return 7443; }
        if block_state.r#facing == Facing::North && block_state.r#open == false && block_state.r#waterlogged == false && block_state.r#powered == true && block_state.r#half == Half::Bottom { return 7438; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::South && block_state.r#powered == false && block_state.r#half == Half::Bottom && block_state.r#open == true { return 7451; }
        if block_state.r#powered == true && block_state.r#half == Half::Top && block_state.r#facing == Facing::North && block_state.r#open == true && block_state.r#waterlogged == false { return 7426; }
        if block_state.r#open == false && block_state.r#waterlogged == false && block_state.r#facing == Facing::East && block_state.r#powered == false && block_state.r#half == Half::Bottom { return 7488; }
        if block_state.r#powered == false && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::East && block_state.r#waterlogged == true && block_state.r#open == true { return 7483; }
        if block_state.r#half == Half::Top && block_state.r#waterlogged == false && block_state.r#powered == true && block_state.r#facing == Facing::West && block_state.r#open == true { return 7458; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::West && block_state.r#half == Half::Top && block_state.r#powered == true && block_state.r#open == true { return 7457; }
        if block_state.r#waterlogged == false && block_state.r#open == true && block_state.r#facing == Facing::South && block_state.r#half == Half::Bottom && block_state.r#powered == true { return 7450; }
        if block_state.r#half == Half::Bottom && block_state.r#powered == true && block_state.r#waterlogged == true && block_state.r#facing == Facing::West && block_state.r#open == true { return 7465; }
        if block_state.r#open == false && block_state.r#facing == Facing::East && block_state.r#powered == true && block_state.r#waterlogged == true && block_state.r#half == Half::Bottom { return 7485; }
        if block_state.r#half == Half::Top && block_state.r#facing == Facing::South && block_state.r#waterlogged == false && block_state.r#powered == true && block_state.r#open == true { return 7442; }
        if block_state.r#powered == true && block_state.r#waterlogged == true && block_state.r#open == false && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::South { return 7453; }
        if block_state.r#waterlogged == false && block_state.r#powered == true && block_state.r#open == true && block_state.r#facing == Facing::East && block_state.r#half == Half::Bottom { return 7482; }
        if block_state.r#half == Half::Bottom && block_state.r#facing == Facing::West && block_state.r#powered == false && block_state.r#waterlogged == true && block_state.r#open == false { return 7471; }
        if block_state.r#open == true && block_state.r#powered == false && block_state.r#waterlogged == false && block_state.r#facing == Facing::East && block_state.r#half == Half::Bottom { return 7484; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::South && block_state.r#open == false && block_state.r#powered == true && block_state.r#half == Half::Bottom { return 7454; }
        if block_state.r#facing == Facing::South && block_state.r#powered == false && block_state.r#half == Half::Top && block_state.r#waterlogged == false && block_state.r#open == true { return 7444; }
        if block_state.r#half == Half::Top && block_state.r#powered == false && block_state.r#open == true && block_state.r#facing == Facing::North && block_state.r#waterlogged == true { return 7427; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::West && block_state.r#half == Half::Top && block_state.r#open == true && block_state.r#powered == false { return 7460; }
        if block_state.r#facing == Facing::East && block_state.r#open == true && block_state.r#waterlogged == false && block_state.r#powered == true && block_state.r#half == Half::Top { return 7474; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::East && block_state.r#half == Half::Bottom && block_state.r#open == false && block_state.r#powered == false { return 7487; }
        if block_state.r#powered == false && block_state.r#open == false && block_state.r#waterlogged == false && block_state.r#facing == Facing::South && block_state.r#half == Half::Bottom { return 7456; }
        if block_state.r#waterlogged == false && block_state.r#powered == true && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::West && block_state.r#open == true { return 7466; }
        if block_state.r#facing == Facing::West && block_state.r#half == Half::Bottom && block_state.r#powered == false && block_state.r#open == false && block_state.r#waterlogged == false { return 7472; }
        if block_state.r#powered == false && block_state.r#half == Half::Bottom && block_state.r#open == false && block_state.r#waterlogged == false && block_state.r#facing == Facing::North { return 7440; }
        if block_state.r#powered == false && block_state.r#open == true && block_state.r#waterlogged == true && block_state.r#facing == Facing::West && block_state.r#half == Half::Bottom { return 7467; }
        if block_state.r#half == Half::Top && block_state.r#powered == false && block_state.r#open == false && block_state.r#waterlogged == false && block_state.r#facing == Facing::North { return 7432; }
        if block_state.r#half == Half::Bottom && block_state.r#open == true && block_state.r#waterlogged == true && block_state.r#powered == true && block_state.r#facing == Facing::East { return 7481; }
        if block_state.r#facing == Facing::North && block_state.r#open == false && block_state.r#powered == true && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true { return 7437; }
        if block_state.r#facing == Facing::West && block_state.r#powered == true && block_state.r#half == Half::Bottom && block_state.r#waterlogged == false && block_state.r#open == false { return 7470; }
        if block_state.r#facing == Facing::South && block_state.r#open == true && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true && block_state.r#powered == true { return 7449; }
        if block_state.r#facing == Facing::West && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true && block_state.r#open == false && block_state.r#powered == true { return 7469; }
        if block_state.r#facing == Facing::East && block_state.r#powered == true && block_state.r#half == Half::Top && block_state.r#waterlogged == true && block_state.r#open == true { return 7473; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == true && block_state.r#half == Half::Top && block_state.r#open == false && block_state.r#powered == true { return 7445; }
        if block_state.r#open == false && block_state.r#facing == Facing::North && block_state.r#half == Half::Top && block_state.r#waterlogged == false && block_state.r#powered == true { return 7430; }
        if block_state.r#half == Half::Top && block_state.r#powered == false && block_state.r#waterlogged == false && block_state.r#facing == Facing::East && block_state.r#open == false { return 7480; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 7463 {
            return Some(MangroveTrapdoor {
                r#facing: Facing::West,
                r#open: false,
                r#half: Half::Top,
                r#waterlogged: true,
                r#powered: false,
            });
        }
        if state_id == 7447 {
            return Some(MangroveTrapdoor {
                r#half: Half::Top,
                r#powered: false,
                r#open: false,
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 7436 {
            return Some(MangroveTrapdoor {
                r#open: true,
                r#waterlogged: false,
                r#facing: Facing::North,
                r#powered: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 7434 {
            return Some(MangroveTrapdoor {
                r#open: true,
                r#powered: true,
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 7441 {
            return Some(MangroveTrapdoor {
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::South,
                r#powered: true,
                r#open: true,
            });
        }
        if state_id == 7448 {
            return Some(MangroveTrapdoor {
                r#waterlogged: false,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#powered: false,
                r#open: false,
            });
        }
        if state_id == 7435 {
            return Some(MangroveTrapdoor {
                r#waterlogged: true,
                r#open: true,
                r#powered: false,
                r#half: Half::Bottom,
                r#facing: Facing::North,
            });
        }
        if state_id == 7475 {
            return Some(MangroveTrapdoor {
                r#half: Half::Top,
                r#powered: false,
                r#facing: Facing::East,
                r#open: true,
                r#waterlogged: true,
            });
        }
        if state_id == 7433 {
            return Some(MangroveTrapdoor {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#open: true,
                r#facing: Facing::North,
                r#powered: true,
            });
        }
        if state_id == 7477 {
            return Some(MangroveTrapdoor {
                r#powered: true,
                r#half: Half::Top,
                r#waterlogged: true,
                r#open: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 7478 {
            return Some(MangroveTrapdoor {
                r#half: Half::Top,
                r#open: false,
                r#powered: true,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 7479 {
            return Some(MangroveTrapdoor {
                r#facing: Facing::East,
                r#half: Half::Top,
                r#powered: false,
                r#open: false,
                r#waterlogged: true,
            });
        }
        if state_id == 7464 {
            return Some(MangroveTrapdoor {
                r#waterlogged: false,
                r#facing: Facing::West,
                r#open: false,
                r#half: Half::Top,
                r#powered: false,
            });
        }
        if state_id == 7425 {
            return Some(MangroveTrapdoor {
                r#half: Half::Top,
                r#powered: true,
                r#open: true,
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 7431 {
            return Some(MangroveTrapdoor {
                r#facing: Facing::North,
                r#open: false,
                r#powered: false,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 7462 {
            return Some(MangroveTrapdoor {
                r#open: false,
                r#powered: true,
                r#facing: Facing::West,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 7468 {
            return Some(MangroveTrapdoor {
                r#powered: false,
                r#half: Half::Bottom,
                r#open: true,
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 7476 {
            return Some(MangroveTrapdoor {
                r#open: true,
                r#facing: Facing::East,
                r#powered: false,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 7459 {
            return Some(MangroveTrapdoor {
                r#open: true,
                r#powered: false,
                r#facing: Facing::West,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 7455 {
            return Some(MangroveTrapdoor {
                r#half: Half::Bottom,
                r#open: false,
                r#waterlogged: true,
                r#facing: Facing::South,
                r#powered: false,
            });
        }
        if state_id == 7461 {
            return Some(MangroveTrapdoor {
                r#half: Half::Top,
                r#open: false,
                r#facing: Facing::West,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 7428 {
            return Some(MangroveTrapdoor {
                r#open: true,
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::North,
                r#powered: false,
            });
        }
        if state_id == 7439 {
            return Some(MangroveTrapdoor {
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#powered: false,
                r#waterlogged: true,
                r#open: false,
            });
        }
        if state_id == 7486 {
            return Some(MangroveTrapdoor {
                r#waterlogged: false,
                r#open: false,
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#powered: true,
            });
        }
        if state_id == 7452 {
            return Some(MangroveTrapdoor {
                r#half: Half::Bottom,
                r#open: true,
                r#waterlogged: false,
                r#powered: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 7446 {
            return Some(MangroveTrapdoor {
                r#powered: true,
                r#open: false,
                r#facing: Facing::South,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 7429 {
            return Some(MangroveTrapdoor {
                r#half: Half::Top,
                r#powered: true,
                r#facing: Facing::North,
                r#waterlogged: true,
                r#open: false,
            });
        }
        if state_id == 7443 {
            return Some(MangroveTrapdoor {
                r#open: true,
                r#powered: false,
                r#waterlogged: true,
                r#facing: Facing::South,
                r#half: Half::Top,
            });
        }
        if state_id == 7438 {
            return Some(MangroveTrapdoor {
                r#facing: Facing::North,
                r#open: false,
                r#waterlogged: false,
                r#powered: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 7451 {
            return Some(MangroveTrapdoor {
                r#waterlogged: true,
                r#facing: Facing::South,
                r#powered: false,
                r#half: Half::Bottom,
                r#open: true,
            });
        }
        if state_id == 7426 {
            return Some(MangroveTrapdoor {
                r#powered: true,
                r#half: Half::Top,
                r#facing: Facing::North,
                r#open: true,
                r#waterlogged: false,
            });
        }
        if state_id == 7488 {
            return Some(MangroveTrapdoor {
                r#open: false,
                r#waterlogged: false,
                r#facing: Facing::East,
                r#powered: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 7483 {
            return Some(MangroveTrapdoor {
                r#powered: false,
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#waterlogged: true,
                r#open: true,
            });
        }
        if state_id == 7458 {
            return Some(MangroveTrapdoor {
                r#half: Half::Top,
                r#waterlogged: false,
                r#powered: true,
                r#facing: Facing::West,
                r#open: true,
            });
        }
        if state_id == 7457 {
            return Some(MangroveTrapdoor {
                r#waterlogged: true,
                r#facing: Facing::West,
                r#half: Half::Top,
                r#powered: true,
                r#open: true,
            });
        }
        if state_id == 7450 {
            return Some(MangroveTrapdoor {
                r#waterlogged: false,
                r#open: true,
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#powered: true,
            });
        }
        if state_id == 7465 {
            return Some(MangroveTrapdoor {
                r#half: Half::Bottom,
                r#powered: true,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#open: true,
            });
        }
        if state_id == 7485 {
            return Some(MangroveTrapdoor {
                r#open: false,
                r#facing: Facing::East,
                r#powered: true,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 7442 {
            return Some(MangroveTrapdoor {
                r#half: Half::Top,
                r#facing: Facing::South,
                r#waterlogged: false,
                r#powered: true,
                r#open: true,
            });
        }
        if state_id == 7453 {
            return Some(MangroveTrapdoor {
                r#powered: true,
                r#waterlogged: true,
                r#open: false,
                r#half: Half::Bottom,
                r#facing: Facing::South,
            });
        }
        if state_id == 7482 {
            return Some(MangroveTrapdoor {
                r#waterlogged: false,
                r#powered: true,
                r#open: true,
                r#facing: Facing::East,
                r#half: Half::Bottom,
            });
        }
        if state_id == 7471 {
            return Some(MangroveTrapdoor {
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#powered: false,
                r#waterlogged: true,
                r#open: false,
            });
        }
        if state_id == 7484 {
            return Some(MangroveTrapdoor {
                r#open: true,
                r#powered: false,
                r#waterlogged: false,
                r#facing: Facing::East,
                r#half: Half::Bottom,
            });
        }
        if state_id == 7454 {
            return Some(MangroveTrapdoor {
                r#waterlogged: false,
                r#facing: Facing::South,
                r#open: false,
                r#powered: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 7444 {
            return Some(MangroveTrapdoor {
                r#facing: Facing::South,
                r#powered: false,
                r#half: Half::Top,
                r#waterlogged: false,
                r#open: true,
            });
        }
        if state_id == 7427 {
            return Some(MangroveTrapdoor {
                r#half: Half::Top,
                r#powered: false,
                r#open: true,
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 7460 {
            return Some(MangroveTrapdoor {
                r#waterlogged: false,
                r#facing: Facing::West,
                r#half: Half::Top,
                r#open: true,
                r#powered: false,
            });
        }
        if state_id == 7474 {
            return Some(MangroveTrapdoor {
                r#facing: Facing::East,
                r#open: true,
                r#waterlogged: false,
                r#powered: true,
                r#half: Half::Top,
            });
        }
        if state_id == 7487 {
            return Some(MangroveTrapdoor {
                r#waterlogged: true,
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 7456 {
            return Some(MangroveTrapdoor {
                r#powered: false,
                r#open: false,
                r#waterlogged: false,
                r#facing: Facing::South,
                r#half: Half::Bottom,
            });
        }
        if state_id == 7466 {
            return Some(MangroveTrapdoor {
                r#waterlogged: false,
                r#powered: true,
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#open: true,
            });
        }
        if state_id == 7472 {
            return Some(MangroveTrapdoor {
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#powered: false,
                r#open: false,
                r#waterlogged: false,
            });
        }
        if state_id == 7440 {
            return Some(MangroveTrapdoor {
                r#powered: false,
                r#half: Half::Bottom,
                r#open: false,
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 7467 {
            return Some(MangroveTrapdoor {
                r#powered: false,
                r#open: true,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#half: Half::Bottom,
            });
        }
        if state_id == 7432 {
            return Some(MangroveTrapdoor {
                r#half: Half::Top,
                r#powered: false,
                r#open: false,
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 7481 {
            return Some(MangroveTrapdoor {
                r#half: Half::Bottom,
                r#open: true,
                r#waterlogged: true,
                r#powered: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 7437 {
            return Some(MangroveTrapdoor {
                r#facing: Facing::North,
                r#open: false,
                r#powered: true,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 7470 {
            return Some(MangroveTrapdoor {
                r#facing: Facing::West,
                r#powered: true,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#open: false,
            });
        }
        if state_id == 7449 {
            return Some(MangroveTrapdoor {
                r#facing: Facing::South,
                r#open: true,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#powered: true,
            });
        }
        if state_id == 7469 {
            return Some(MangroveTrapdoor {
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#open: false,
                r#powered: true,
            });
        }
        if state_id == 7473 {
            return Some(MangroveTrapdoor {
                r#facing: Facing::East,
                r#powered: true,
                r#half: Half::Top,
                r#waterlogged: true,
                r#open: true,
            });
        }
        if state_id == 7445 {
            return Some(MangroveTrapdoor {
                r#facing: Facing::South,
                r#waterlogged: true,
                r#half: Half::Top,
                r#open: false,
                r#powered: true,
            });
        }
        if state_id == 7430 {
            return Some(MangroveTrapdoor {
                r#open: false,
                r#facing: Facing::North,
                r#half: Half::Top,
                r#waterlogged: false,
                r#powered: true,
            });
        }
        if state_id == 7480 {
            return Some(MangroveTrapdoor {
                r#half: Half::Top,
                r#powered: false,
                r#waterlogged: false,
                r#facing: Facing::East,
                r#open: false,
            });
        }
        return None;
    }
}

