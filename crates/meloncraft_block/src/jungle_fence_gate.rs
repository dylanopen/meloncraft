use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JungleFenceGate {
    pub open: bool,
    pub r#facing: Facing,
    pub powered: bool,
    pub in_wall: bool,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for JungleFenceGate {
    fn to_id(self) -> i32 {
        if block_state.r#powered == false && block_state.r#in_wall == false && block_state.r#facing == Facing::East && block_state.r#open == false { return 13377; }
        if block_state.r#open == true && block_state.r#facing == Facing::North && block_state.r#powered == false && block_state.r#in_wall == false { return 13351; }
        if block_state.r#powered == true && block_state.r#open == false && block_state.r#facing == Facing::North && block_state.r#in_wall == false { return 13352; }
        if block_state.r#in_wall == true && block_state.r#facing == Facing::South && block_state.r#open == false && block_state.r#powered == false { return 13357; }
        if block_state.r#facing == Facing::West && block_state.r#powered == false && block_state.r#in_wall == true && block_state.r#open == true { return 13363; }
        if block_state.r#open == true && block_state.r#in_wall == true && block_state.r#facing == Facing::South && block_state.r#powered == false { return 13355; }
        if block_state.r#powered == true && block_state.r#in_wall == false && block_state.r#open == true && block_state.r#facing == Facing::North { return 13350; }
        if block_state.r#open == true && block_state.r#in_wall == false && block_state.r#facing == Facing::South && block_state.r#powered == true { return 13358; }
        if block_state.r#open == true && block_state.r#in_wall == true && block_state.r#powered == false && block_state.r#facing == Facing::North { return 13347; }
        if block_state.r#open == true && block_state.r#powered == true && block_state.r#in_wall == true && block_state.r#facing == Facing::South { return 13354; }
        if block_state.r#powered == true && block_state.r#in_wall == false && block_state.r#facing == Facing::South && block_state.r#open == false { return 13360; }
        if block_state.r#powered == true && block_state.r#in_wall == true && block_state.r#facing == Facing::East && block_state.r#open == true { return 13370; }
        if block_state.r#facing == Facing::East && block_state.r#powered == true && block_state.r#open == false && block_state.r#in_wall == false { return 13376; }
        if block_state.r#powered == false && block_state.r#in_wall == false && block_state.r#open == false && block_state.r#facing == Facing::North { return 13353; }
        if block_state.r#facing == Facing::East && block_state.r#in_wall == false && block_state.r#open == true && block_state.r#powered == true { return 13374; }
        if block_state.r#in_wall == false && block_state.r#powered == false && block_state.r#open == true && block_state.r#facing == Facing::South { return 13359; }
        if block_state.r#facing == Facing::West && block_state.r#in_wall == false && block_state.r#open == false && block_state.r#powered == false { return 13369; }
        if block_state.r#facing == Facing::West && block_state.r#powered == false && block_state.r#in_wall == false && block_state.r#open == true { return 13367; }
        if block_state.r#facing == Facing::West && block_state.r#in_wall == true && block_state.r#open == true && block_state.r#powered == true { return 13362; }
        if block_state.r#in_wall == false && block_state.r#facing == Facing::West && block_state.r#open == true && block_state.r#powered == true { return 13366; }
        if block_state.r#open == true && block_state.r#powered == true && block_state.r#facing == Facing::North && block_state.r#in_wall == true { return 13346; }
        if block_state.r#open == false && block_state.r#facing == Facing::West && block_state.r#powered == true && block_state.r#in_wall == true { return 13364; }
        if block_state.r#facing == Facing::West && block_state.r#in_wall == false && block_state.r#powered == true && block_state.r#open == false { return 13368; }
        if block_state.r#open == true && block_state.r#facing == Facing::East && block_state.r#in_wall == true && block_state.r#powered == false { return 13371; }
        if block_state.r#open == false && block_state.r#in_wall == false && block_state.r#facing == Facing::South && block_state.r#powered == false { return 13361; }
        if block_state.r#facing == Facing::North && block_state.r#open == false && block_state.r#in_wall == true && block_state.r#powered == false { return 13349; }
        if block_state.r#in_wall == true && block_state.r#powered == true && block_state.r#facing == Facing::East && block_state.r#open == false { return 13372; }
        if block_state.r#facing == Facing::East && block_state.r#open == true && block_state.r#in_wall == false && block_state.r#powered == false { return 13375; }
        if block_state.r#facing == Facing::West && block_state.r#in_wall == true && block_state.r#powered == false && block_state.r#open == false { return 13365; }
        if block_state.r#powered == true && block_state.r#open == false && block_state.r#in_wall == true && block_state.r#facing == Facing::North { return 13348; }
        if block_state.r#in_wall == true && block_state.r#facing == Facing::East && block_state.r#open == false && block_state.r#powered == false { return 13373; }
        if block_state.r#facing == Facing::South && block_state.r#open == false && block_state.r#in_wall == true && block_state.r#powered == true { return 13356; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 13377 {
            return Some(JungleFenceGate {
                r#powered: false,
                r#in_wall: false,
                r#facing: Facing::East,
                r#open: false,
            });
        }
        if state_id == 13351 {
            return Some(JungleFenceGate {
                r#open: true,
                r#facing: Facing::North,
                r#powered: false,
                r#in_wall: false,
            });
        }
        if state_id == 13352 {
            return Some(JungleFenceGate {
                r#powered: true,
                r#open: false,
                r#facing: Facing::North,
                r#in_wall: false,
            });
        }
        if state_id == 13357 {
            return Some(JungleFenceGate {
                r#in_wall: true,
                r#facing: Facing::South,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 13363 {
            return Some(JungleFenceGate {
                r#facing: Facing::West,
                r#powered: false,
                r#in_wall: true,
                r#open: true,
            });
        }
        if state_id == 13355 {
            return Some(JungleFenceGate {
                r#open: true,
                r#in_wall: true,
                r#facing: Facing::South,
                r#powered: false,
            });
        }
        if state_id == 13350 {
            return Some(JungleFenceGate {
                r#powered: true,
                r#in_wall: false,
                r#open: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 13358 {
            return Some(JungleFenceGate {
                r#open: true,
                r#in_wall: false,
                r#facing: Facing::South,
                r#powered: true,
            });
        }
        if state_id == 13347 {
            return Some(JungleFenceGate {
                r#open: true,
                r#in_wall: true,
                r#powered: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 13354 {
            return Some(JungleFenceGate {
                r#open: true,
                r#powered: true,
                r#in_wall: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 13360 {
            return Some(JungleFenceGate {
                r#powered: true,
                r#in_wall: false,
                r#facing: Facing::South,
                r#open: false,
            });
        }
        if state_id == 13370 {
            return Some(JungleFenceGate {
                r#powered: true,
                r#in_wall: true,
                r#facing: Facing::East,
                r#open: true,
            });
        }
        if state_id == 13376 {
            return Some(JungleFenceGate {
                r#facing: Facing::East,
                r#powered: true,
                r#open: false,
                r#in_wall: false,
            });
        }
        if state_id == 13353 {
            return Some(JungleFenceGate {
                r#powered: false,
                r#in_wall: false,
                r#open: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 13374 {
            return Some(JungleFenceGate {
                r#facing: Facing::East,
                r#in_wall: false,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 13359 {
            return Some(JungleFenceGate {
                r#in_wall: false,
                r#powered: false,
                r#open: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 13369 {
            return Some(JungleFenceGate {
                r#facing: Facing::West,
                r#in_wall: false,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 13367 {
            return Some(JungleFenceGate {
                r#facing: Facing::West,
                r#powered: false,
                r#in_wall: false,
                r#open: true,
            });
        }
        if state_id == 13362 {
            return Some(JungleFenceGate {
                r#facing: Facing::West,
                r#in_wall: true,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 13366 {
            return Some(JungleFenceGate {
                r#in_wall: false,
                r#facing: Facing::West,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 13346 {
            return Some(JungleFenceGate {
                r#open: true,
                r#powered: true,
                r#facing: Facing::North,
                r#in_wall: true,
            });
        }
        if state_id == 13364 {
            return Some(JungleFenceGate {
                r#open: false,
                r#facing: Facing::West,
                r#powered: true,
                r#in_wall: true,
            });
        }
        if state_id == 13368 {
            return Some(JungleFenceGate {
                r#facing: Facing::West,
                r#in_wall: false,
                r#powered: true,
                r#open: false,
            });
        }
        if state_id == 13371 {
            return Some(JungleFenceGate {
                r#open: true,
                r#facing: Facing::East,
                r#in_wall: true,
                r#powered: false,
            });
        }
        if state_id == 13361 {
            return Some(JungleFenceGate {
                r#open: false,
                r#in_wall: false,
                r#facing: Facing::South,
                r#powered: false,
            });
        }
        if state_id == 13349 {
            return Some(JungleFenceGate {
                r#facing: Facing::North,
                r#open: false,
                r#in_wall: true,
                r#powered: false,
            });
        }
        if state_id == 13372 {
            return Some(JungleFenceGate {
                r#in_wall: true,
                r#powered: true,
                r#facing: Facing::East,
                r#open: false,
            });
        }
        if state_id == 13375 {
            return Some(JungleFenceGate {
                r#facing: Facing::East,
                r#open: true,
                r#in_wall: false,
                r#powered: false,
            });
        }
        if state_id == 13365 {
            return Some(JungleFenceGate {
                r#facing: Facing::West,
                r#in_wall: true,
                r#powered: false,
                r#open: false,
            });
        }
        if state_id == 13348 {
            return Some(JungleFenceGate {
                r#powered: true,
                r#open: false,
                r#in_wall: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 13373 {
            return Some(JungleFenceGate {
                r#in_wall: true,
                r#facing: Facing::East,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 13356 {
            return Some(JungleFenceGate {
                r#facing: Facing::South,
                r#open: false,
                r#in_wall: true,
                r#powered: true,
            });
        }
        return None;
    }
}

