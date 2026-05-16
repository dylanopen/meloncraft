use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TripwireHook {
    pub r#facing: Facing,
    pub powered: bool,
    pub attached: bool,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for TripwireHook {
    fn to_id(&self) -> i32 {
        if self.r#facing == Facing::East && self.r#powered == true && self.r#attached == true { return 9388; }
        if self.r#powered == true && self.r#facing == Facing::South && self.r#attached == true { return 9384; }
        if self.r#powered == false && self.r#attached == false && self.r#facing == Facing::East { return 9397; }
        if self.r#facing == Facing::East && self.r#powered == false && self.r#attached == true { return 9389; }
        if self.r#attached == true && self.r#powered == true && self.r#facing == Facing::North { return 9382; }
        if self.r#attached == false && self.r#facing == Facing::North && self.r#powered == true { return 9390; }
        if self.r#attached == true && self.r#powered == true && self.r#facing == Facing::West { return 9386; }
        if self.r#attached == false && self.r#powered == false && self.r#facing == Facing::North { return 9391; }
        if self.r#powered == true && self.r#facing == Facing::West && self.r#attached == false { return 9394; }
        if self.r#attached == false && self.r#powered == false && self.r#facing == Facing::South { return 9393; }
        if self.r#attached == false && self.r#powered == false && self.r#facing == Facing::West { return 9395; }
        if self.r#facing == Facing::South && self.r#attached == true && self.r#powered == false { return 9385; }
        if self.r#attached == false && self.r#facing == Facing::East && self.r#powered == true { return 9396; }
        if self.r#attached == true && self.r#facing == Facing::North && self.r#powered == false { return 9383; }
        if self.r#attached == true && self.r#facing == Facing::West && self.r#powered == false { return 9387; }
        if self.r#powered == true && self.r#facing == Facing::South && self.r#attached == false { return 9392; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 9388 {
            return Some(TripwireHook {
                r#facing: Facing::East,
                r#powered: true,
                r#attached: true,
            });
        }
        if state_id == 9384 {
            return Some(TripwireHook {
                r#powered: true,
                r#facing: Facing::South,
                r#attached: true,
            });
        }
        if state_id == 9397 {
            return Some(TripwireHook {
                r#powered: false,
                r#attached: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 9389 {
            return Some(TripwireHook {
                r#facing: Facing::East,
                r#powered: false,
                r#attached: true,
            });
        }
        if state_id == 9382 {
            return Some(TripwireHook {
                r#attached: true,
                r#powered: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 9390 {
            return Some(TripwireHook {
                r#attached: false,
                r#facing: Facing::North,
                r#powered: true,
            });
        }
        if state_id == 9386 {
            return Some(TripwireHook {
                r#attached: true,
                r#powered: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 9391 {
            return Some(TripwireHook {
                r#attached: false,
                r#powered: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 9394 {
            return Some(TripwireHook {
                r#powered: true,
                r#facing: Facing::West,
                r#attached: false,
            });
        }
        if state_id == 9393 {
            return Some(TripwireHook {
                r#attached: false,
                r#powered: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 9395 {
            return Some(TripwireHook {
                r#attached: false,
                r#powered: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 9385 {
            return Some(TripwireHook {
                r#facing: Facing::South,
                r#attached: true,
                r#powered: false,
            });
        }
        if state_id == 9396 {
            return Some(TripwireHook {
                r#attached: false,
                r#facing: Facing::East,
                r#powered: true,
            });
        }
        if state_id == 9383 {
            return Some(TripwireHook {
                r#attached: true,
                r#facing: Facing::North,
                r#powered: false,
            });
        }
        if state_id == 9387 {
            return Some(TripwireHook {
                r#attached: true,
                r#facing: Facing::West,
                r#powered: false,
            });
        }
        if state_id == 9392 {
            return Some(TripwireHook {
                r#powered: true,
                r#facing: Facing::South,
                r#attached: false,
            });
        }
        return None;
    }
}

