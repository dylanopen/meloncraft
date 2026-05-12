use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JungleFenceGate {
    pub open: bool,
    pub powered: bool,
    pub in_wall: bool,
    pub r#facing: Facing,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for JungleFenceGate {
    fn to_id(&self) -> i32 {
        if self.r#open == true && self.r#in_wall == false && self.r#facing == Facing::South && self.r#powered == true { return 13358; }
        if self.r#in_wall == true && self.r#open == true && self.r#powered == true && self.r#facing == Facing::North { return 13346; }
        if self.r#facing == Facing::East && self.r#powered == true && self.r#open == false && self.r#in_wall == false { return 13376; }
        if self.r#powered == false && self.r#facing == Facing::South && self.r#in_wall == false && self.r#open == true { return 13359; }
        if self.r#open == true && self.r#in_wall == false && self.r#powered == true && self.r#facing == Facing::North { return 13350; }
        if self.r#in_wall == true && self.r#powered == true && self.r#facing == Facing::East && self.r#open == false { return 13372; }
        if self.r#facing == Facing::South && self.r#powered == false && self.r#in_wall == true && self.r#open == true { return 13355; }
        if self.r#powered == false && self.r#open == true && self.r#in_wall == false && self.r#facing == Facing::East { return 13375; }
        if self.r#powered == false && self.r#in_wall == false && self.r#open == false && self.r#facing == Facing::East { return 13377; }
        if self.r#powered == true && self.r#facing == Facing::South && self.r#open == true && self.r#in_wall == true { return 13354; }
        if self.r#in_wall == true && self.r#powered == false && self.r#open == false && self.r#facing == Facing::North { return 13349; }
        if self.r#in_wall == true && self.r#powered == true && self.r#open == true && self.r#facing == Facing::West { return 13362; }
        if self.r#in_wall == false && self.r#powered == true && self.r#open == false && self.r#facing == Facing::West { return 13368; }
        if self.r#facing == Facing::West && self.r#in_wall == true && self.r#powered == false && self.r#open == true { return 13363; }
        if self.r#facing == Facing::East && self.r#in_wall == true && self.r#open == true && self.r#powered == true { return 13370; }
        if self.r#open == false && self.r#in_wall == false && self.r#facing == Facing::West && self.r#powered == false { return 13369; }
        if self.r#in_wall == false && self.r#open == true && self.r#powered == true && self.r#facing == Facing::West { return 13366; }
        if self.r#in_wall == true && self.r#open == false && self.r#powered == true && self.r#facing == Facing::South { return 13356; }
        if self.r#open == false && self.r#facing == Facing::North && self.r#in_wall == false && self.r#powered == true { return 13352; }
        if self.r#facing == Facing::South && self.r#powered == true && self.r#open == false && self.r#in_wall == false { return 13360; }
        if self.r#powered == false && self.r#facing == Facing::East && self.r#in_wall == true && self.r#open == false { return 13373; }
        if self.r#open == true && self.r#powered == false && self.r#facing == Facing::East && self.r#in_wall == true { return 13371; }
        if self.r#open == false && self.r#powered == false && self.r#facing == Facing::West && self.r#in_wall == true { return 13365; }
        if self.r#in_wall == false && self.r#facing == Facing::North && self.r#powered == false && self.r#open == false { return 13353; }
        if self.r#facing == Facing::South && self.r#in_wall == false && self.r#powered == false && self.r#open == false { return 13361; }
        if self.r#in_wall == true && self.r#facing == Facing::West && self.r#powered == true && self.r#open == false { return 13364; }
        if self.r#open == true && self.r#in_wall == true && self.r#powered == false && self.r#facing == Facing::North { return 13347; }
        if self.r#open == false && self.r#facing == Facing::North && self.r#in_wall == true && self.r#powered == true { return 13348; }
        if self.r#facing == Facing::West && self.r#open == true && self.r#powered == false && self.r#in_wall == false { return 13367; }
        if self.r#open == true && self.r#facing == Facing::East && self.r#powered == true && self.r#in_wall == false { return 13374; }
        if self.r#facing == Facing::South && self.r#powered == false && self.r#in_wall == true && self.r#open == false { return 13357; }
        if self.r#powered == false && self.r#in_wall == false && self.r#facing == Facing::North && self.r#open == true { return 13351; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 13358 {
            return Some(JungleFenceGate {
                r#open: true,
                r#in_wall: false,
                r#facing: Facing::South,
                r#powered: true,
            });
        }
        if state_id == 13346 {
            return Some(JungleFenceGate {
                r#in_wall: true,
                r#open: true,
                r#powered: true,
                r#facing: Facing::North,
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
        if state_id == 13359 {
            return Some(JungleFenceGate {
                r#powered: false,
                r#facing: Facing::South,
                r#in_wall: false,
                r#open: true,
            });
        }
        if state_id == 13350 {
            return Some(JungleFenceGate {
                r#open: true,
                r#in_wall: false,
                r#powered: true,
                r#facing: Facing::North,
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
        if state_id == 13355 {
            return Some(JungleFenceGate {
                r#facing: Facing::South,
                r#powered: false,
                r#in_wall: true,
                r#open: true,
            });
        }
        if state_id == 13375 {
            return Some(JungleFenceGate {
                r#powered: false,
                r#open: true,
                r#in_wall: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 13377 {
            return Some(JungleFenceGate {
                r#powered: false,
                r#in_wall: false,
                r#open: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 13354 {
            return Some(JungleFenceGate {
                r#powered: true,
                r#facing: Facing::South,
                r#open: true,
                r#in_wall: true,
            });
        }
        if state_id == 13349 {
            return Some(JungleFenceGate {
                r#in_wall: true,
                r#powered: false,
                r#open: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 13362 {
            return Some(JungleFenceGate {
                r#in_wall: true,
                r#powered: true,
                r#open: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 13368 {
            return Some(JungleFenceGate {
                r#in_wall: false,
                r#powered: true,
                r#open: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 13363 {
            return Some(JungleFenceGate {
                r#facing: Facing::West,
                r#in_wall: true,
                r#powered: false,
                r#open: true,
            });
        }
        if state_id == 13370 {
            return Some(JungleFenceGate {
                r#facing: Facing::East,
                r#in_wall: true,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 13369 {
            return Some(JungleFenceGate {
                r#open: false,
                r#in_wall: false,
                r#facing: Facing::West,
                r#powered: false,
            });
        }
        if state_id == 13366 {
            return Some(JungleFenceGate {
                r#in_wall: false,
                r#open: true,
                r#powered: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 13356 {
            return Some(JungleFenceGate {
                r#in_wall: true,
                r#open: false,
                r#powered: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 13352 {
            return Some(JungleFenceGate {
                r#open: false,
                r#facing: Facing::North,
                r#in_wall: false,
                r#powered: true,
            });
        }
        if state_id == 13360 {
            return Some(JungleFenceGate {
                r#facing: Facing::South,
                r#powered: true,
                r#open: false,
                r#in_wall: false,
            });
        }
        if state_id == 13373 {
            return Some(JungleFenceGate {
                r#powered: false,
                r#facing: Facing::East,
                r#in_wall: true,
                r#open: false,
            });
        }
        if state_id == 13371 {
            return Some(JungleFenceGate {
                r#open: true,
                r#powered: false,
                r#facing: Facing::East,
                r#in_wall: true,
            });
        }
        if state_id == 13365 {
            return Some(JungleFenceGate {
                r#open: false,
                r#powered: false,
                r#facing: Facing::West,
                r#in_wall: true,
            });
        }
        if state_id == 13353 {
            return Some(JungleFenceGate {
                r#in_wall: false,
                r#facing: Facing::North,
                r#powered: false,
                r#open: false,
            });
        }
        if state_id == 13361 {
            return Some(JungleFenceGate {
                r#facing: Facing::South,
                r#in_wall: false,
                r#powered: false,
                r#open: false,
            });
        }
        if state_id == 13364 {
            return Some(JungleFenceGate {
                r#in_wall: true,
                r#facing: Facing::West,
                r#powered: true,
                r#open: false,
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
        if state_id == 13348 {
            return Some(JungleFenceGate {
                r#open: false,
                r#facing: Facing::North,
                r#in_wall: true,
                r#powered: true,
            });
        }
        if state_id == 13367 {
            return Some(JungleFenceGate {
                r#facing: Facing::West,
                r#open: true,
                r#powered: false,
                r#in_wall: false,
            });
        }
        if state_id == 13374 {
            return Some(JungleFenceGate {
                r#open: true,
                r#facing: Facing::East,
                r#powered: true,
                r#in_wall: false,
            });
        }
        if state_id == 13357 {
            return Some(JungleFenceGate {
                r#facing: Facing::South,
                r#powered: false,
                r#in_wall: true,
                r#open: false,
            });
        }
        if state_id == 13351 {
            return Some(JungleFenceGate {
                r#powered: false,
                r#in_wall: false,
                r#facing: Facing::North,
                r#open: true,
            });
        }
        return None;
    }
}

