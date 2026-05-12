use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AcaciaFenceGate {
    pub in_wall: bool,
    pub powered: bool,
    pub open: bool,
    pub r#facing: Facing,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for AcaciaFenceGate {
    fn to_id(self) -> i32 {
        if block_state.r#facing == Facing::East && block_state.r#open == true && block_state.r#in_wall == true && block_state.r#powered == false { return 13403; }
        if block_state.r#facing == Facing::East && block_state.r#in_wall == false && block_state.r#powered == false && block_state.r#open == false { return 13409; }
        if block_state.r#open == false && block_state.r#in_wall == false && block_state.r#facing == Facing::North && block_state.r#powered == false { return 13385; }
        if block_state.r#open == true && block_state.r#powered == true && block_state.r#facing == Facing::North && block_state.r#in_wall == false { return 13382; }
        if block_state.r#in_wall == true && block_state.r#powered == false && block_state.r#open == false && block_state.r#facing == Facing::West { return 13397; }
        if block_state.r#facing == Facing::South && block_state.r#in_wall == false && block_state.r#open == true && block_state.r#powered == true { return 13390; }
        if block_state.r#facing == Facing::West && block_state.r#powered == true && block_state.r#in_wall == false && block_state.r#open == false { return 13400; }
        if block_state.r#facing == Facing::South && block_state.r#powered == true && block_state.r#in_wall == false && block_state.r#open == false { return 13392; }
        if block_state.r#open == true && block_state.r#facing == Facing::South && block_state.r#in_wall == true && block_state.r#powered == false { return 13387; }
        if block_state.r#open == true && block_state.r#in_wall == true && block_state.r#powered == true && block_state.r#facing == Facing::North { return 13378; }
        if block_state.r#facing == Facing::South && block_state.r#in_wall == true && block_state.r#powered == false && block_state.r#open == false { return 13389; }
        if block_state.r#in_wall == true && block_state.r#powered == true && block_state.r#open == true && block_state.r#facing == Facing::East { return 13402; }
        if block_state.r#open == false && block_state.r#facing == Facing::North && block_state.r#in_wall == true && block_state.r#powered == false { return 13381; }
        if block_state.r#facing == Facing::South && block_state.r#in_wall == true && block_state.r#powered == true && block_state.r#open == false { return 13388; }
        if block_state.r#in_wall == true && block_state.r#open == false && block_state.r#facing == Facing::North && block_state.r#powered == true { return 13380; }
        if block_state.r#facing == Facing::West && block_state.r#powered == false && block_state.r#in_wall == true && block_state.r#open == true { return 13395; }
        if block_state.r#facing == Facing::North && block_state.r#in_wall == false && block_state.r#powered == true && block_state.r#open == false { return 13384; }
        if block_state.r#in_wall == false && block_state.r#facing == Facing::South && block_state.r#open == false && block_state.r#powered == false { return 13393; }
        if block_state.r#powered == true && block_state.r#in_wall == true && block_state.r#open == true && block_state.r#facing == Facing::West { return 13394; }
        if block_state.r#in_wall == false && block_state.r#facing == Facing::North && block_state.r#open == true && block_state.r#powered == false { return 13383; }
        if block_state.r#facing == Facing::West && block_state.r#in_wall == false && block_state.r#powered == true && block_state.r#open == true { return 13398; }
        if block_state.r#open == false && block_state.r#facing == Facing::East && block_state.r#in_wall == true && block_state.r#powered == false { return 13405; }
        if block_state.r#in_wall == false && block_state.r#open == true && block_state.r#powered == false && block_state.r#facing == Facing::West { return 13399; }
        if block_state.r#in_wall == true && block_state.r#facing == Facing::South && block_state.r#open == true && block_state.r#powered == true { return 13386; }
        if block_state.r#open == true && block_state.r#in_wall == false && block_state.r#facing == Facing::East && block_state.r#powered == true { return 13406; }
        if block_state.r#in_wall == false && block_state.r#facing == Facing::East && block_state.r#powered == false && block_state.r#open == true { return 13407; }
        if block_state.r#powered == true && block_state.r#in_wall == true && block_state.r#facing == Facing::West && block_state.r#open == false { return 13396; }
        if block_state.r#facing == Facing::West && block_state.r#open == false && block_state.r#powered == false && block_state.r#in_wall == false { return 13401; }
        if block_state.r#in_wall == true && block_state.r#open == true && block_state.r#facing == Facing::North && block_state.r#powered == false { return 13379; }
        if block_state.r#open == false && block_state.r#powered == true && block_state.r#in_wall == true && block_state.r#facing == Facing::East { return 13404; }
        if block_state.r#in_wall == false && block_state.r#facing == Facing::East && block_state.r#open == false && block_state.r#powered == true { return 13408; }
        if block_state.r#facing == Facing::South && block_state.r#in_wall == false && block_state.r#open == true && block_state.r#powered == false { return 13391; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 13403 {
            return Some(AcaciaFenceGate {
                r#facing: Facing::East,
                r#open: true,
                r#in_wall: true,
                r#powered: false,
            });
        }
        if state_id == 13409 {
            return Some(AcaciaFenceGate {
                r#facing: Facing::East,
                r#in_wall: false,
                r#powered: false,
                r#open: false,
            });
        }
        if state_id == 13385 {
            return Some(AcaciaFenceGate {
                r#open: false,
                r#in_wall: false,
                r#facing: Facing::North,
                r#powered: false,
            });
        }
        if state_id == 13382 {
            return Some(AcaciaFenceGate {
                r#open: true,
                r#powered: true,
                r#facing: Facing::North,
                r#in_wall: false,
            });
        }
        if state_id == 13397 {
            return Some(AcaciaFenceGate {
                r#in_wall: true,
                r#powered: false,
                r#open: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 13390 {
            return Some(AcaciaFenceGate {
                r#facing: Facing::South,
                r#in_wall: false,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 13400 {
            return Some(AcaciaFenceGate {
                r#facing: Facing::West,
                r#powered: true,
                r#in_wall: false,
                r#open: false,
            });
        }
        if state_id == 13392 {
            return Some(AcaciaFenceGate {
                r#facing: Facing::South,
                r#powered: true,
                r#in_wall: false,
                r#open: false,
            });
        }
        if state_id == 13387 {
            return Some(AcaciaFenceGate {
                r#open: true,
                r#facing: Facing::South,
                r#in_wall: true,
                r#powered: false,
            });
        }
        if state_id == 13378 {
            return Some(AcaciaFenceGate {
                r#open: true,
                r#in_wall: true,
                r#powered: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 13389 {
            return Some(AcaciaFenceGate {
                r#facing: Facing::South,
                r#in_wall: true,
                r#powered: false,
                r#open: false,
            });
        }
        if state_id == 13402 {
            return Some(AcaciaFenceGate {
                r#in_wall: true,
                r#powered: true,
                r#open: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 13381 {
            return Some(AcaciaFenceGate {
                r#open: false,
                r#facing: Facing::North,
                r#in_wall: true,
                r#powered: false,
            });
        }
        if state_id == 13388 {
            return Some(AcaciaFenceGate {
                r#facing: Facing::South,
                r#in_wall: true,
                r#powered: true,
                r#open: false,
            });
        }
        if state_id == 13380 {
            return Some(AcaciaFenceGate {
                r#in_wall: true,
                r#open: false,
                r#facing: Facing::North,
                r#powered: true,
            });
        }
        if state_id == 13395 {
            return Some(AcaciaFenceGate {
                r#facing: Facing::West,
                r#powered: false,
                r#in_wall: true,
                r#open: true,
            });
        }
        if state_id == 13384 {
            return Some(AcaciaFenceGate {
                r#facing: Facing::North,
                r#in_wall: false,
                r#powered: true,
                r#open: false,
            });
        }
        if state_id == 13393 {
            return Some(AcaciaFenceGate {
                r#in_wall: false,
                r#facing: Facing::South,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 13394 {
            return Some(AcaciaFenceGate {
                r#powered: true,
                r#in_wall: true,
                r#open: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 13383 {
            return Some(AcaciaFenceGate {
                r#in_wall: false,
                r#facing: Facing::North,
                r#open: true,
                r#powered: false,
            });
        }
        if state_id == 13398 {
            return Some(AcaciaFenceGate {
                r#facing: Facing::West,
                r#in_wall: false,
                r#powered: true,
                r#open: true,
            });
        }
        if state_id == 13405 {
            return Some(AcaciaFenceGate {
                r#open: false,
                r#facing: Facing::East,
                r#in_wall: true,
                r#powered: false,
            });
        }
        if state_id == 13399 {
            return Some(AcaciaFenceGate {
                r#in_wall: false,
                r#open: true,
                r#powered: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 13386 {
            return Some(AcaciaFenceGate {
                r#in_wall: true,
                r#facing: Facing::South,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 13406 {
            return Some(AcaciaFenceGate {
                r#open: true,
                r#in_wall: false,
                r#facing: Facing::East,
                r#powered: true,
            });
        }
        if state_id == 13407 {
            return Some(AcaciaFenceGate {
                r#in_wall: false,
                r#facing: Facing::East,
                r#powered: false,
                r#open: true,
            });
        }
        if state_id == 13396 {
            return Some(AcaciaFenceGate {
                r#powered: true,
                r#in_wall: true,
                r#facing: Facing::West,
                r#open: false,
            });
        }
        if state_id == 13401 {
            return Some(AcaciaFenceGate {
                r#facing: Facing::West,
                r#open: false,
                r#powered: false,
                r#in_wall: false,
            });
        }
        if state_id == 13379 {
            return Some(AcaciaFenceGate {
                r#in_wall: true,
                r#open: true,
                r#facing: Facing::North,
                r#powered: false,
            });
        }
        if state_id == 13404 {
            return Some(AcaciaFenceGate {
                r#open: false,
                r#powered: true,
                r#in_wall: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 13408 {
            return Some(AcaciaFenceGate {
                r#in_wall: false,
                r#facing: Facing::East,
                r#open: false,
                r#powered: true,
            });
        }
        if state_id == 13391 {
            return Some(AcaciaFenceGate {
                r#facing: Facing::South,
                r#in_wall: false,
                r#open: true,
                r#powered: false,
            });
        }
        return None;
    }
}

