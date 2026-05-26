use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MangroveTrapdoor {
    pub powered: bool,
    pub r#half: Half,
    pub open: bool,
    pub r#facing: Facing,
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

impl BlockState for MangroveTrapdoor {
    fn to_id(&self) -> i32 {
        if self.r#half == Half::Top
            && self.r#waterlogged == true
            && self.r#facing == Facing::North
            && self.r#open == true
            && self.r#powered == true
        {
            return 7425;
        }
        if self.r#half == Half::Bottom
            && self.r#powered == false
            && self.r#facing == Facing::South
            && self.r#open == false
            && self.r#waterlogged == true
        {
            return 7455;
        }
        if self.r#waterlogged == false
            && self.r#half == Half::Top
            && self.r#open == true
            && self.r#powered == false
            && self.r#facing == Facing::East
        {
            return 7476;
        }
        if self.r#open == false
            && self.r#half == Half::Top
            && self.r#waterlogged == true
            && self.r#facing == Facing::South
            && self.r#powered == true
        {
            return 7445;
        }
        if self.r#open == true
            && self.r#powered == false
            && self.r#waterlogged == true
            && self.r#half == Half::Bottom
            && self.r#facing == Facing::South
        {
            return 7451;
        }
        if self.r#half == Half::Bottom
            && self.r#waterlogged == false
            && self.r#powered == true
            && self.r#facing == Facing::West
            && self.r#open == true
        {
            return 7466;
        }
        if self.r#powered == false
            && self.r#facing == Facing::South
            && self.r#half == Half::Bottom
            && self.r#open == true
            && self.r#waterlogged == false
        {
            return 7452;
        }
        if self.r#open == false
            && self.r#waterlogged == true
            && self.r#powered == false
            && self.r#half == Half::Bottom
            && self.r#facing == Facing::West
        {
            return 7471;
        }
        if self.r#facing == Facing::East
            && self.r#waterlogged == false
            && self.r#powered == false
            && self.r#open == false
            && self.r#half == Half::Bottom
        {
            return 7488;
        }
        if self.r#powered == false
            && self.r#waterlogged == false
            && self.r#facing == Facing::West
            && self.r#half == Half::Bottom
            && self.r#open == false
        {
            return 7472;
        }
        if self.r#facing == Facing::South
            && self.r#waterlogged == true
            && self.r#half == Half::Top
            && self.r#open == false
            && self.r#powered == false
        {
            return 7447;
        }
        if self.r#half == Half::Bottom
            && self.r#waterlogged == false
            && self.r#powered == true
            && self.r#open == true
            && self.r#facing == Facing::South
        {
            return 7450;
        }
        if self.r#facing == Facing::West
            && self.r#open == false
            && self.r#half == Half::Bottom
            && self.r#powered == true
            && self.r#waterlogged == false
        {
            return 7470;
        }
        if self.r#waterlogged == true
            && self.r#facing == Facing::West
            && self.r#open == true
            && self.r#powered == false
            && self.r#half == Half::Bottom
        {
            return 7467;
        }
        if self.r#waterlogged == true
            && self.r#half == Half::Bottom
            && self.r#open == true
            && self.r#facing == Facing::North
            && self.r#powered == false
        {
            return 7435;
        }
        if self.r#facing == Facing::East
            && self.r#half == Half::Bottom
            && self.r#open == true
            && self.r#waterlogged == true
            && self.r#powered == true
        {
            return 7481;
        }
        if self.r#waterlogged == true
            && self.r#half == Half::Bottom
            && self.r#facing == Facing::North
            && self.r#open == false
            && self.r#powered == true
        {
            return 7437;
        }
        if self.r#open == false
            && self.r#waterlogged == false
            && self.r#facing == Facing::East
            && self.r#half == Half::Top
            && self.r#powered == true
        {
            return 7478;
        }
        if self.r#half == Half::Top
            && self.r#powered == false
            && self.r#facing == Facing::North
            && self.r#waterlogged == false
            && self.r#open == false
        {
            return 7432;
        }
        if self.r#powered == true
            && self.r#open == false
            && self.r#waterlogged == false
            && self.r#facing == Facing::North
            && self.r#half == Half::Bottom
        {
            return 7438;
        }
        if self.r#open == true
            && self.r#powered == true
            && self.r#half == Half::Top
            && self.r#facing == Facing::East
            && self.r#waterlogged == false
        {
            return 7474;
        }
        if self.r#half == Half::Bottom
            && self.r#open == true
            && self.r#powered == true
            && self.r#facing == Facing::South
            && self.r#waterlogged == true
        {
            return 7449;
        }
        if self.r#open == true
            && self.r#half == Half::Bottom
            && self.r#facing == Facing::North
            && self.r#waterlogged == true
            && self.r#powered == true
        {
            return 7433;
        }
        if self.r#half == Half::Bottom
            && self.r#waterlogged == true
            && self.r#facing == Facing::North
            && self.r#powered == false
            && self.r#open == false
        {
            return 7439;
        }
        if self.r#half == Half::Top
            && self.r#open == false
            && self.r#facing == Facing::West
            && self.r#powered == true
            && self.r#waterlogged == true
        {
            return 7461;
        }
        if self.r#waterlogged == false
            && self.r#facing == Facing::West
            && self.r#half == Half::Bottom
            && self.r#powered == false
            && self.r#open == true
        {
            return 7468;
        }
        if self.r#powered == false
            && self.r#open == true
            && self.r#half == Half::Top
            && self.r#waterlogged == true
            && self.r#facing == Facing::East
        {
            return 7475;
        }
        if self.r#half == Half::Top
            && self.r#waterlogged == true
            && self.r#facing == Facing::South
            && self.r#powered == true
            && self.r#open == true
        {
            return 7441;
        }
        if self.r#facing == Facing::South
            && self.r#powered == false
            && self.r#waterlogged == false
            && self.r#open == false
            && self.r#half == Half::Bottom
        {
            return 7456;
        }
        if self.r#open == true
            && self.r#facing == Facing::East
            && self.r#half == Half::Bottom
            && self.r#powered == true
            && self.r#waterlogged == false
        {
            return 7482;
        }
        if self.r#half == Half::Bottom
            && self.r#facing == Facing::East
            && self.r#powered == true
            && self.r#open == false
            && self.r#waterlogged == false
        {
            return 7486;
        }
        if self.r#facing == Facing::West
            && self.r#waterlogged == false
            && self.r#half == Half::Top
            && self.r#powered == false
            && self.r#open == false
        {
            return 7464;
        }
        if self.r#powered == true
            && self.r#waterlogged == true
            && self.r#half == Half::Top
            && self.r#facing == Facing::East
            && self.r#open == true
        {
            return 7473;
        }
        if self.r#waterlogged == false
            && self.r#open == true
            && self.r#half == Half::Bottom
            && self.r#facing == Facing::East
            && self.r#powered == false
        {
            return 7484;
        }
        if self.r#open == true
            && self.r#waterlogged == true
            && self.r#powered == false
            && self.r#half == Half::Top
            && self.r#facing == Facing::North
        {
            return 7427;
        }
        if self.r#powered == false
            && self.r#open == true
            && self.r#facing == Facing::North
            && self.r#waterlogged == false
            && self.r#half == Half::Top
        {
            return 7428;
        }
        if self.r#powered == true
            && self.r#waterlogged == true
            && self.r#half == Half::Top
            && self.r#open == false
            && self.r#facing == Facing::East
        {
            return 7477;
        }
        if self.r#waterlogged == false
            && self.r#powered == false
            && self.r#half == Half::Top
            && self.r#open == true
            && self.r#facing == Facing::West
        {
            return 7460;
        }
        if self.r#half == Half::Top
            && self.r#powered == true
            && self.r#waterlogged == false
            && self.r#open == false
            && self.r#facing == Facing::South
        {
            return 7446;
        }
        if self.r#facing == Facing::East
            && self.r#half == Half::Top
            && self.r#waterlogged == false
            && self.r#open == false
            && self.r#powered == false
        {
            return 7480;
        }
        if self.r#facing == Facing::North
            && self.r#open == false
            && self.r#powered == true
            && self.r#waterlogged == true
            && self.r#half == Half::Top
        {
            return 7429;
        }
        if self.r#half == Half::Bottom
            && self.r#powered == true
            && self.r#facing == Facing::South
            && self.r#open == false
            && self.r#waterlogged == true
        {
            return 7453;
        }
        if self.r#facing == Facing::West
            && self.r#powered == true
            && self.r#half == Half::Top
            && self.r#open == false
            && self.r#waterlogged == false
        {
            return 7462;
        }
        if self.r#half == Half::Bottom
            && self.r#waterlogged == true
            && self.r#open == true
            && self.r#powered == true
            && self.r#facing == Facing::West
        {
            return 7465;
        }
        if self.r#facing == Facing::East
            && self.r#powered == false
            && self.r#waterlogged == true
            && self.r#half == Half::Bottom
            && self.r#open == false
        {
            return 7487;
        }
        if self.r#facing == Facing::South
            && self.r#half == Half::Bottom
            && self.r#open == false
            && self.r#waterlogged == false
            && self.r#powered == true
        {
            return 7454;
        }
        if self.r#powered == true
            && self.r#half == Half::Top
            && self.r#waterlogged == false
            && self.r#facing == Facing::North
            && self.r#open == true
        {
            return 7426;
        }
        if self.r#waterlogged == false
            && self.r#open == false
            && self.r#half == Half::Top
            && self.r#facing == Facing::South
            && self.r#powered == false
        {
            return 7448;
        }
        if self.r#facing == Facing::North
            && self.r#open == false
            && self.r#half == Half::Top
            && self.r#powered == false
            && self.r#waterlogged == true
        {
            return 7431;
        }
        if self.r#powered == true
            && self.r#half == Half::Bottom
            && self.r#waterlogged == true
            && self.r#facing == Facing::East
            && self.r#open == false
        {
            return 7485;
        }
        if self.r#facing == Facing::East
            && self.r#open == false
            && self.r#half == Half::Top
            && self.r#waterlogged == true
            && self.r#powered == false
        {
            return 7479;
        }
        if self.r#half == Half::Bottom
            && self.r#open == true
            && self.r#facing == Facing::North
            && self.r#powered == false
            && self.r#waterlogged == false
        {
            return 7436;
        }
        if self.r#open == true
            && self.r#powered == false
            && self.r#facing == Facing::South
            && self.r#half == Half::Top
            && self.r#waterlogged == true
        {
            return 7443;
        }
        if self.r#powered == true
            && self.r#open == true
            && self.r#facing == Facing::North
            && self.r#waterlogged == false
            && self.r#half == Half::Bottom
        {
            return 7434;
        }
        if self.r#waterlogged == false
            && self.r#half == Half::Top
            && self.r#open == true
            && self.r#facing == Facing::West
            && self.r#powered == true
        {
            return 7458;
        }
        if self.r#facing == Facing::East
            && self.r#powered == false
            && self.r#open == true
            && self.r#waterlogged == true
            && self.r#half == Half::Bottom
        {
            return 7483;
        }
        if self.r#powered == false
            && self.r#half == Half::Bottom
            && self.r#open == false
            && self.r#facing == Facing::North
            && self.r#waterlogged == false
        {
            return 7440;
        }
        if self.r#powered == true
            && self.r#open == true
            && self.r#half == Half::Top
            && self.r#waterlogged == true
            && self.r#facing == Facing::West
        {
            return 7457;
        }
        if self.r#powered == false
            && self.r#waterlogged == true
            && self.r#open == false
            && self.r#facing == Facing::West
            && self.r#half == Half::Top
        {
            return 7463;
        }
        if self.r#half == Half::Top
            && self.r#open == true
            && self.r#waterlogged == true
            && self.r#facing == Facing::West
            && self.r#powered == false
        {
            return 7459;
        }
        if self.r#powered == true
            && self.r#facing == Facing::West
            && self.r#waterlogged == true
            && self.r#half == Half::Bottom
            && self.r#open == false
        {
            return 7469;
        }
        if self.r#open == true
            && self.r#facing == Facing::South
            && self.r#powered == false
            && self.r#waterlogged == false
            && self.r#half == Half::Top
        {
            return 7444;
        }
        if self.r#waterlogged == false
            && self.r#half == Half::Top
            && self.r#facing == Facing::South
            && self.r#open == true
            && self.r#powered == true
        {
            return 7442;
        }
        if self.r#powered == true
            && self.r#waterlogged == false
            && self.r#facing == Facing::North
            && self.r#half == Half::Top
            && self.r#open == false
        {
            return 7430;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 7425 {
            return Some(MangroveTrapdoor {
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::North,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 7455 {
            return Some(MangroveTrapdoor {
                r#half: Half::Bottom,
                r#powered: false,
                r#facing: Facing::South,
                r#open: false,
                r#waterlogged: true,
            });
        }
        if state_id == 7476 {
            return Some(MangroveTrapdoor {
                r#waterlogged: false,
                r#half: Half::Top,
                r#open: true,
                r#powered: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 7445 {
            return Some(MangroveTrapdoor {
                r#open: false,
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::South,
                r#powered: true,
            });
        }
        if state_id == 7451 {
            return Some(MangroveTrapdoor {
                r#open: true,
                r#powered: false,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::South,
            });
        }
        if state_id == 7466 {
            return Some(MangroveTrapdoor {
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#powered: true,
                r#facing: Facing::West,
                r#open: true,
            });
        }
        if state_id == 7452 {
            return Some(MangroveTrapdoor {
                r#powered: false,
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#open: true,
                r#waterlogged: false,
            });
        }
        if state_id == 7471 {
            return Some(MangroveTrapdoor {
                r#open: false,
                r#waterlogged: true,
                r#powered: false,
                r#half: Half::Bottom,
                r#facing: Facing::West,
            });
        }
        if state_id == 7488 {
            return Some(MangroveTrapdoor {
                r#facing: Facing::East,
                r#waterlogged: false,
                r#powered: false,
                r#open: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 7472 {
            return Some(MangroveTrapdoor {
                r#powered: false,
                r#waterlogged: false,
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#open: false,
            });
        }
        if state_id == 7447 {
            return Some(MangroveTrapdoor {
                r#facing: Facing::South,
                r#waterlogged: true,
                r#half: Half::Top,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 7450 {
            return Some(MangroveTrapdoor {
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#powered: true,
                r#open: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 7470 {
            return Some(MangroveTrapdoor {
                r#facing: Facing::West,
                r#open: false,
                r#half: Half::Bottom,
                r#powered: true,
                r#waterlogged: false,
            });
        }
        if state_id == 7467 {
            return Some(MangroveTrapdoor {
                r#waterlogged: true,
                r#facing: Facing::West,
                r#open: true,
                r#powered: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 7435 {
            return Some(MangroveTrapdoor {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#open: true,
                r#facing: Facing::North,
                r#powered: false,
            });
        }
        if state_id == 7481 {
            return Some(MangroveTrapdoor {
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#open: true,
                r#waterlogged: true,
                r#powered: true,
            });
        }
        if state_id == 7437 {
            return Some(MangroveTrapdoor {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#open: false,
                r#powered: true,
            });
        }
        if state_id == 7478 {
            return Some(MangroveTrapdoor {
                r#open: false,
                r#waterlogged: false,
                r#facing: Facing::East,
                r#half: Half::Top,
                r#powered: true,
            });
        }
        if state_id == 7432 {
            return Some(MangroveTrapdoor {
                r#half: Half::Top,
                r#powered: false,
                r#facing: Facing::North,
                r#waterlogged: false,
                r#open: false,
            });
        }
        if state_id == 7438 {
            return Some(MangroveTrapdoor {
                r#powered: true,
                r#open: false,
                r#waterlogged: false,
                r#facing: Facing::North,
                r#half: Half::Bottom,
            });
        }
        if state_id == 7474 {
            return Some(MangroveTrapdoor {
                r#open: true,
                r#powered: true,
                r#half: Half::Top,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 7449 {
            return Some(MangroveTrapdoor {
                r#half: Half::Bottom,
                r#open: true,
                r#powered: true,
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 7433 {
            return Some(MangroveTrapdoor {
                r#open: true,
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#waterlogged: true,
                r#powered: true,
            });
        }
        if state_id == 7439 {
            return Some(MangroveTrapdoor {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::North,
                r#powered: false,
                r#open: false,
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
        if state_id == 7468 {
            return Some(MangroveTrapdoor {
                r#waterlogged: false,
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#powered: false,
                r#open: true,
            });
        }
        if state_id == 7475 {
            return Some(MangroveTrapdoor {
                r#powered: false,
                r#open: true,
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 7441 {
            return Some(MangroveTrapdoor {
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::South,
                r#powered: true,
                r#open: true,
            });
        }
        if state_id == 7456 {
            return Some(MangroveTrapdoor {
                r#facing: Facing::South,
                r#powered: false,
                r#waterlogged: false,
                r#open: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 7482 {
            return Some(MangroveTrapdoor {
                r#open: true,
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#powered: true,
                r#waterlogged: false,
            });
        }
        if state_id == 7486 {
            return Some(MangroveTrapdoor {
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#powered: true,
                r#open: false,
                r#waterlogged: false,
            });
        }
        if state_id == 7464 {
            return Some(MangroveTrapdoor {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#half: Half::Top,
                r#powered: false,
                r#open: false,
            });
        }
        if state_id == 7473 {
            return Some(MangroveTrapdoor {
                r#powered: true,
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::East,
                r#open: true,
            });
        }
        if state_id == 7484 {
            return Some(MangroveTrapdoor {
                r#waterlogged: false,
                r#open: true,
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#powered: false,
            });
        }
        if state_id == 7427 {
            return Some(MangroveTrapdoor {
                r#open: true,
                r#waterlogged: true,
                r#powered: false,
                r#half: Half::Top,
                r#facing: Facing::North,
            });
        }
        if state_id == 7428 {
            return Some(MangroveTrapdoor {
                r#powered: false,
                r#open: true,
                r#facing: Facing::North,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 7477 {
            return Some(MangroveTrapdoor {
                r#powered: true,
                r#waterlogged: true,
                r#half: Half::Top,
                r#open: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 7460 {
            return Some(MangroveTrapdoor {
                r#waterlogged: false,
                r#powered: false,
                r#half: Half::Top,
                r#open: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 7446 {
            return Some(MangroveTrapdoor {
                r#half: Half::Top,
                r#powered: true,
                r#waterlogged: false,
                r#open: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 7480 {
            return Some(MangroveTrapdoor {
                r#facing: Facing::East,
                r#half: Half::Top,
                r#waterlogged: false,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 7429 {
            return Some(MangroveTrapdoor {
                r#facing: Facing::North,
                r#open: false,
                r#powered: true,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 7453 {
            return Some(MangroveTrapdoor {
                r#half: Half::Bottom,
                r#powered: true,
                r#facing: Facing::South,
                r#open: false,
                r#waterlogged: true,
            });
        }
        if state_id == 7462 {
            return Some(MangroveTrapdoor {
                r#facing: Facing::West,
                r#powered: true,
                r#half: Half::Top,
                r#open: false,
                r#waterlogged: false,
            });
        }
        if state_id == 7465 {
            return Some(MangroveTrapdoor {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#open: true,
                r#powered: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 7487 {
            return Some(MangroveTrapdoor {
                r#facing: Facing::East,
                r#powered: false,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#open: false,
            });
        }
        if state_id == 7454 {
            return Some(MangroveTrapdoor {
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#open: false,
                r#waterlogged: false,
                r#powered: true,
            });
        }
        if state_id == 7426 {
            return Some(MangroveTrapdoor {
                r#powered: true,
                r#half: Half::Top,
                r#waterlogged: false,
                r#facing: Facing::North,
                r#open: true,
            });
        }
        if state_id == 7448 {
            return Some(MangroveTrapdoor {
                r#waterlogged: false,
                r#open: false,
                r#half: Half::Top,
                r#facing: Facing::South,
                r#powered: false,
            });
        }
        if state_id == 7431 {
            return Some(MangroveTrapdoor {
                r#facing: Facing::North,
                r#open: false,
                r#half: Half::Top,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 7485 {
            return Some(MangroveTrapdoor {
                r#powered: true,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::East,
                r#open: false,
            });
        }
        if state_id == 7479 {
            return Some(MangroveTrapdoor {
                r#facing: Facing::East,
                r#open: false,
                r#half: Half::Top,
                r#waterlogged: true,
                r#powered: false,
            });
        }
        if state_id == 7436 {
            return Some(MangroveTrapdoor {
                r#half: Half::Bottom,
                r#open: true,
                r#facing: Facing::North,
                r#powered: false,
                r#waterlogged: false,
            });
        }
        if state_id == 7443 {
            return Some(MangroveTrapdoor {
                r#open: true,
                r#powered: false,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 7434 {
            return Some(MangroveTrapdoor {
                r#powered: true,
                r#open: true,
                r#facing: Facing::North,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 7458 {
            return Some(MangroveTrapdoor {
                r#waterlogged: false,
                r#half: Half::Top,
                r#open: true,
                r#facing: Facing::West,
                r#powered: true,
            });
        }
        if state_id == 7483 {
            return Some(MangroveTrapdoor {
                r#facing: Facing::East,
                r#powered: false,
                r#open: true,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 7440 {
            return Some(MangroveTrapdoor {
                r#powered: false,
                r#half: Half::Bottom,
                r#open: false,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 7457 {
            return Some(MangroveTrapdoor {
                r#powered: true,
                r#open: true,
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 7463 {
            return Some(MangroveTrapdoor {
                r#powered: false,
                r#waterlogged: true,
                r#open: false,
                r#facing: Facing::West,
                r#half: Half::Top,
            });
        }
        if state_id == 7459 {
            return Some(MangroveTrapdoor {
                r#half: Half::Top,
                r#open: true,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#powered: false,
            });
        }
        if state_id == 7469 {
            return Some(MangroveTrapdoor {
                r#powered: true,
                r#facing: Facing::West,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#open: false,
            });
        }
        if state_id == 7444 {
            return Some(MangroveTrapdoor {
                r#open: true,
                r#facing: Facing::South,
                r#powered: false,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 7442 {
            return Some(MangroveTrapdoor {
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::South,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 7430 {
            return Some(MangroveTrapdoor {
                r#powered: true,
                r#waterlogged: false,
                r#facing: Facing::North,
                r#half: Half::Top,
                r#open: false,
            });
        }
        return None;
    }
}
