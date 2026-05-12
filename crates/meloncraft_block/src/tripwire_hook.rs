use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TripwireHook {
    pub r#facing: Facing,
    pub attached: bool,
    pub powered: bool,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for TripwireHook {
    fn to_id(self) -> i32 {
        if block_state.r#attached == false && block_state.r#powered == true && block_state.r#facing == Facing::West { return 9394; }
        if block_state.r#powered == true && block_state.r#facing == Facing::West && block_state.r#attached == true { return 9386; }
        if block_state.r#powered == false && block_state.r#facing == Facing::South && block_state.r#attached == false { return 9393; }
        if block_state.r#powered == true && block_state.r#attached == true && block_state.r#facing == Facing::East { return 9388; }
        if block_state.r#facing == Facing::South && block_state.r#attached == false && block_state.r#powered == true { return 9392; }
        if block_state.r#attached == false && block_state.r#facing == Facing::West && block_state.r#powered == false { return 9395; }
        if block_state.r#attached == true && block_state.r#facing == Facing::North && block_state.r#powered == false { return 9383; }
        if block_state.r#powered == false && block_state.r#facing == Facing::East && block_state.r#attached == true { return 9389; }
        if block_state.r#attached == false && block_state.r#powered == true && block_state.r#facing == Facing::North { return 9390; }
        if block_state.r#facing == Facing::West && block_state.r#attached == true && block_state.r#powered == false { return 9387; }
        if block_state.r#powered == false && block_state.r#facing == Facing::North && block_state.r#attached == false { return 9391; }
        if block_state.r#facing == Facing::East && block_state.r#attached == false && block_state.r#powered == true { return 9396; }
        if block_state.r#powered == true && block_state.r#attached == true && block_state.r#facing == Facing::North { return 9382; }
        if block_state.r#powered == false && block_state.r#attached == false && block_state.r#facing == Facing::East { return 9397; }
        if block_state.r#powered == true && block_state.r#facing == Facing::South && block_state.r#attached == true { return 9384; }
        if block_state.r#powered == false && block_state.r#facing == Facing::South && block_state.r#attached == true { return 9385; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 9394 {
            return Some(TripwireHook {
                r#attached: false,
                r#powered: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 9386 {
            return Some(TripwireHook {
                r#powered: true,
                r#facing: Facing::West,
                r#attached: true,
            });
        }
        if state_id == 9393 {
            return Some(TripwireHook {
                r#powered: false,
                r#facing: Facing::South,
                r#attached: false,
            });
        }
        if state_id == 9388 {
            return Some(TripwireHook {
                r#powered: true,
                r#attached: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 9392 {
            return Some(TripwireHook {
                r#facing: Facing::South,
                r#attached: false,
                r#powered: true,
            });
        }
        if state_id == 9395 {
            return Some(TripwireHook {
                r#attached: false,
                r#facing: Facing::West,
                r#powered: false,
            });
        }
        if state_id == 9383 {
            return Some(TripwireHook {
                r#attached: true,
                r#facing: Facing::North,
                r#powered: false,
            });
        }
        if state_id == 9389 {
            return Some(TripwireHook {
                r#powered: false,
                r#facing: Facing::East,
                r#attached: true,
            });
        }
        if state_id == 9390 {
            return Some(TripwireHook {
                r#attached: false,
                r#powered: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 9387 {
            return Some(TripwireHook {
                r#facing: Facing::West,
                r#attached: true,
                r#powered: false,
            });
        }
        if state_id == 9391 {
            return Some(TripwireHook {
                r#powered: false,
                r#facing: Facing::North,
                r#attached: false,
            });
        }
        if state_id == 9396 {
            return Some(TripwireHook {
                r#facing: Facing::East,
                r#attached: false,
                r#powered: true,
            });
        }
        if state_id == 9382 {
            return Some(TripwireHook {
                r#powered: true,
                r#attached: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 9397 {
            return Some(TripwireHook {
                r#powered: false,
                r#attached: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 9384 {
            return Some(TripwireHook {
                r#powered: true,
                r#facing: Facing::South,
                r#attached: true,
            });
        }
        if state_id == 9385 {
            return Some(TripwireHook {
                r#powered: false,
                r#facing: Facing::South,
                r#attached: true,
            });
        }
        return None;
    }
}

