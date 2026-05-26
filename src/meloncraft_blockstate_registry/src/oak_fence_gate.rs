use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OakFenceGate {
    pub open: bool,
    pub in_wall: bool,
    pub powered: bool,
    pub r#facing: Facing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for OakFenceGate {
    fn to_id(&self) -> i32 {
        if self.r#facing == Facing::North
            && self.r#in_wall == true
            && self.r#open == false
            && self.r#powered == true
        {
            return 8447;
        }
        if self.r#powered == true
            && self.r#in_wall == true
            && self.r#open == true
            && self.r#facing == Facing::South
        {
            return 8453;
        }
        if self.r#in_wall == false
            && self.r#facing == Facing::West
            && self.r#open == true
            && self.r#powered == false
        {
            return 8466;
        }
        if self.r#powered == true
            && self.r#open == false
            && self.r#in_wall == false
            && self.r#facing == Facing::South
        {
            return 8459;
        }
        if self.r#open == false
            && self.r#in_wall == false
            && self.r#powered == true
            && self.r#facing == Facing::North
        {
            return 8451;
        }
        if self.r#open == true
            && self.r#in_wall == true
            && self.r#facing == Facing::West
            && self.r#powered == true
        {
            return 8461;
        }
        if self.r#facing == Facing::West
            && self.r#in_wall == false
            && self.r#powered == false
            && self.r#open == false
        {
            return 8468;
        }
        if self.r#open == true
            && self.r#in_wall == true
            && self.r#powered == false
            && self.r#facing == Facing::North
        {
            return 8446;
        }
        if self.r#open == true
            && self.r#in_wall == true
            && self.r#powered == true
            && self.r#facing == Facing::East
        {
            return 8469;
        }
        if self.r#facing == Facing::East
            && self.r#powered == true
            && self.r#open == false
            && self.r#in_wall == false
        {
            return 8475;
        }
        if self.r#facing == Facing::North
            && self.r#powered == true
            && self.r#in_wall == false
            && self.r#open == true
        {
            return 8449;
        }
        if self.r#in_wall == false
            && self.r#open == true
            && self.r#powered == false
            && self.r#facing == Facing::South
        {
            return 8458;
        }
        if self.r#facing == Facing::East
            && self.r#in_wall == false
            && self.r#open == true
            && self.r#powered == false
        {
            return 8474;
        }
        if self.r#powered == true
            && self.r#facing == Facing::West
            && self.r#in_wall == false
            && self.r#open == false
        {
            return 8467;
        }
        if self.r#powered == true
            && self.r#in_wall == false
            && self.r#facing == Facing::West
            && self.r#open == true
        {
            return 8465;
        }
        if self.r#in_wall == true
            && self.r#open == true
            && self.r#powered == false
            && self.r#facing == Facing::West
        {
            return 8462;
        }
        if self.r#powered == false
            && self.r#in_wall == true
            && self.r#open == true
            && self.r#facing == Facing::South
        {
            return 8454;
        }
        if self.r#powered == true
            && self.r#open == true
            && self.r#facing == Facing::North
            && self.r#in_wall == true
        {
            return 8445;
        }
        if self.r#in_wall == true
            && self.r#facing == Facing::North
            && self.r#open == false
            && self.r#powered == false
        {
            return 8448;
        }
        if self.r#powered == false
            && self.r#facing == Facing::South
            && self.r#in_wall == false
            && self.r#open == false
        {
            return 8460;
        }
        if self.r#in_wall == true
            && self.r#facing == Facing::South
            && self.r#open == false
            && self.r#powered == true
        {
            return 8455;
        }
        if self.r#powered == false
            && self.r#open == false
            && self.r#facing == Facing::East
            && self.r#in_wall == false
        {
            return 8476;
        }
        if self.r#open == false
            && self.r#in_wall == true
            && self.r#powered == false
            && self.r#facing == Facing::East
        {
            return 8472;
        }
        if self.r#in_wall == true
            && self.r#powered == false
            && self.r#facing == Facing::West
            && self.r#open == false
        {
            return 8464;
        }
        if self.r#open == false
            && self.r#powered == true
            && self.r#in_wall == true
            && self.r#facing == Facing::East
        {
            return 8471;
        }
        if self.r#open == true
            && self.r#facing == Facing::South
            && self.r#powered == true
            && self.r#in_wall == false
        {
            return 8457;
        }
        if self.r#powered == false
            && self.r#in_wall == false
            && self.r#facing == Facing::North
            && self.r#open == true
        {
            return 8450;
        }
        if self.r#powered == false
            && self.r#open == false
            && self.r#facing == Facing::North
            && self.r#in_wall == false
        {
            return 8452;
        }
        if self.r#facing == Facing::East
            && self.r#powered == true
            && self.r#in_wall == false
            && self.r#open == true
        {
            return 8473;
        }
        if self.r#in_wall == true
            && self.r#open == true
            && self.r#facing == Facing::East
            && self.r#powered == false
        {
            return 8470;
        }
        if self.r#facing == Facing::West
            && self.r#powered == true
            && self.r#in_wall == true
            && self.r#open == false
        {
            return 8463;
        }
        if self.r#in_wall == true
            && self.r#facing == Facing::South
            && self.r#powered == false
            && self.r#open == false
        {
            return 8456;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 8447 {
            return Some(OakFenceGate {
                r#facing: Facing::North,
                r#in_wall: true,
                r#open: false,
                r#powered: true,
            });
        }
        if state_id == 8453 {
            return Some(OakFenceGate {
                r#powered: true,
                r#in_wall: true,
                r#open: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 8466 {
            return Some(OakFenceGate {
                r#in_wall: false,
                r#facing: Facing::West,
                r#open: true,
                r#powered: false,
            });
        }
        if state_id == 8459 {
            return Some(OakFenceGate {
                r#powered: true,
                r#open: false,
                r#in_wall: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 8451 {
            return Some(OakFenceGate {
                r#open: false,
                r#in_wall: false,
                r#powered: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 8461 {
            return Some(OakFenceGate {
                r#open: true,
                r#in_wall: true,
                r#facing: Facing::West,
                r#powered: true,
            });
        }
        if state_id == 8468 {
            return Some(OakFenceGate {
                r#facing: Facing::West,
                r#in_wall: false,
                r#powered: false,
                r#open: false,
            });
        }
        if state_id == 8446 {
            return Some(OakFenceGate {
                r#open: true,
                r#in_wall: true,
                r#powered: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 8469 {
            return Some(OakFenceGate {
                r#open: true,
                r#in_wall: true,
                r#powered: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 8475 {
            return Some(OakFenceGate {
                r#facing: Facing::East,
                r#powered: true,
                r#open: false,
                r#in_wall: false,
            });
        }
        if state_id == 8449 {
            return Some(OakFenceGate {
                r#facing: Facing::North,
                r#powered: true,
                r#in_wall: false,
                r#open: true,
            });
        }
        if state_id == 8458 {
            return Some(OakFenceGate {
                r#in_wall: false,
                r#open: true,
                r#powered: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 8474 {
            return Some(OakFenceGate {
                r#facing: Facing::East,
                r#in_wall: false,
                r#open: true,
                r#powered: false,
            });
        }
        if state_id == 8467 {
            return Some(OakFenceGate {
                r#powered: true,
                r#facing: Facing::West,
                r#in_wall: false,
                r#open: false,
            });
        }
        if state_id == 8465 {
            return Some(OakFenceGate {
                r#powered: true,
                r#in_wall: false,
                r#facing: Facing::West,
                r#open: true,
            });
        }
        if state_id == 8462 {
            return Some(OakFenceGate {
                r#in_wall: true,
                r#open: true,
                r#powered: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 8454 {
            return Some(OakFenceGate {
                r#powered: false,
                r#in_wall: true,
                r#open: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 8445 {
            return Some(OakFenceGate {
                r#powered: true,
                r#open: true,
                r#facing: Facing::North,
                r#in_wall: true,
            });
        }
        if state_id == 8448 {
            return Some(OakFenceGate {
                r#in_wall: true,
                r#facing: Facing::North,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 8460 {
            return Some(OakFenceGate {
                r#powered: false,
                r#facing: Facing::South,
                r#in_wall: false,
                r#open: false,
            });
        }
        if state_id == 8455 {
            return Some(OakFenceGate {
                r#in_wall: true,
                r#facing: Facing::South,
                r#open: false,
                r#powered: true,
            });
        }
        if state_id == 8476 {
            return Some(OakFenceGate {
                r#powered: false,
                r#open: false,
                r#facing: Facing::East,
                r#in_wall: false,
            });
        }
        if state_id == 8472 {
            return Some(OakFenceGate {
                r#open: false,
                r#in_wall: true,
                r#powered: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 8464 {
            return Some(OakFenceGate {
                r#in_wall: true,
                r#powered: false,
                r#facing: Facing::West,
                r#open: false,
            });
        }
        if state_id == 8471 {
            return Some(OakFenceGate {
                r#open: false,
                r#powered: true,
                r#in_wall: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 8457 {
            return Some(OakFenceGate {
                r#open: true,
                r#facing: Facing::South,
                r#powered: true,
                r#in_wall: false,
            });
        }
        if state_id == 8450 {
            return Some(OakFenceGate {
                r#powered: false,
                r#in_wall: false,
                r#facing: Facing::North,
                r#open: true,
            });
        }
        if state_id == 8452 {
            return Some(OakFenceGate {
                r#powered: false,
                r#open: false,
                r#facing: Facing::North,
                r#in_wall: false,
            });
        }
        if state_id == 8473 {
            return Some(OakFenceGate {
                r#facing: Facing::East,
                r#powered: true,
                r#in_wall: false,
                r#open: true,
            });
        }
        if state_id == 8470 {
            return Some(OakFenceGate {
                r#in_wall: true,
                r#open: true,
                r#facing: Facing::East,
                r#powered: false,
            });
        }
        if state_id == 8463 {
            return Some(OakFenceGate {
                r#facing: Facing::West,
                r#powered: true,
                r#in_wall: true,
                r#open: false,
            });
        }
        if state_id == 8456 {
            return Some(OakFenceGate {
                r#in_wall: true,
                r#facing: Facing::South,
                r#powered: false,
                r#open: false,
            });
        }
        return None;
    }
}
