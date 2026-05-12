use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LightningRod {
    pub r#facing: Facing,
    pub powered: bool,
    pub waterlogged: bool,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    East,
    South,
    West,
    Up,
    Down,
}

impl BlockState for LightningRod {
    fn to_id(self) -> i32 {
        if block_state.r#facing == Facing::East && block_state.r#powered == false && block_state.r#waterlogged == false { return 27348; }
        if block_state.r#facing == Facing::South && block_state.r#powered == false && block_state.r#waterlogged == false { return 27352; }
        if block_state.r#powered == true && block_state.r#facing == Facing::Up && block_state.r#waterlogged == true { return 27357; }
        if block_state.r#powered == true && block_state.r#waterlogged == false && block_state.r#facing == Facing::Up { return 27358; }
        if block_state.r#powered == false && block_state.r#waterlogged == true && block_state.r#facing == Facing::Up { return 27359; }
        if block_state.r#facing == Facing::Down && block_state.r#powered == true && block_state.r#waterlogged == true { return 27361; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::Up && block_state.r#powered == false { return 27360; }
        if block_state.r#powered == false && block_state.r#waterlogged == true && block_state.r#facing == Facing::Down { return 27363; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::Down && block_state.r#powered == false { return 27364; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == false && block_state.r#powered == false { return 27344; }
        if block_state.r#powered == true && block_state.r#waterlogged == false && block_state.r#facing == Facing::East { return 27346; }
        if block_state.r#powered == false && block_state.r#waterlogged == true && block_state.r#facing == Facing::South { return 27351; }
        if block_state.r#powered == true && block_state.r#facing == Facing::North && block_state.r#waterlogged == false { return 27342; }
        if block_state.r#waterlogged == false && block_state.r#powered == false && block_state.r#facing == Facing::West { return 27356; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::West && block_state.r#powered == true { return 27353; }
        if block_state.r#facing == Facing::West && block_state.r#powered == true && block_state.r#waterlogged == false { return 27354; }
        if block_state.r#waterlogged == true && block_state.r#powered == false && block_state.r#facing == Facing::East { return 27347; }
        if block_state.r#powered == false && block_state.r#waterlogged == true && block_state.r#facing == Facing::West { return 27355; }
        if block_state.r#powered == true && block_state.r#waterlogged == false && block_state.r#facing == Facing::South { return 27350; }
        if block_state.r#powered == true && block_state.r#waterlogged == false && block_state.r#facing == Facing::Down { return 27362; }
        if block_state.r#waterlogged == true && block_state.r#powered == true && block_state.r#facing == Facing::South { return 27349; }
        if block_state.r#powered == false && block_state.r#facing == Facing::North && block_state.r#waterlogged == true { return 27343; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::East && block_state.r#powered == true { return 27345; }
        if block_state.r#powered == true && block_state.r#waterlogged == true && block_state.r#facing == Facing::North { return 27341; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 27348 {
            return Some(LightningRod {
                r#facing: Facing::East,
                r#powered: false,
                r#waterlogged: false,
            });
        }
        if state_id == 27352 {
            return Some(LightningRod {
                r#facing: Facing::South,
                r#powered: false,
                r#waterlogged: false,
            });
        }
        if state_id == 27357 {
            return Some(LightningRod {
                r#powered: true,
                r#facing: Facing::Up,
                r#waterlogged: true,
            });
        }
        if state_id == 27358 {
            return Some(LightningRod {
                r#powered: true,
                r#waterlogged: false,
                r#facing: Facing::Up,
            });
        }
        if state_id == 27359 {
            return Some(LightningRod {
                r#powered: false,
                r#waterlogged: true,
                r#facing: Facing::Up,
            });
        }
        if state_id == 27361 {
            return Some(LightningRod {
                r#facing: Facing::Down,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 27360 {
            return Some(LightningRod {
                r#waterlogged: false,
                r#facing: Facing::Up,
                r#powered: false,
            });
        }
        if state_id == 27363 {
            return Some(LightningRod {
                r#powered: false,
                r#waterlogged: true,
                r#facing: Facing::Down,
            });
        }
        if state_id == 27364 {
            return Some(LightningRod {
                r#waterlogged: false,
                r#facing: Facing::Down,
                r#powered: false,
            });
        }
        if state_id == 27344 {
            return Some(LightningRod {
                r#facing: Facing::North,
                r#waterlogged: false,
                r#powered: false,
            });
        }
        if state_id == 27346 {
            return Some(LightningRod {
                r#powered: true,
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 27351 {
            return Some(LightningRod {
                r#powered: false,
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 27342 {
            return Some(LightningRod {
                r#powered: true,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 27356 {
            return Some(LightningRod {
                r#waterlogged: false,
                r#powered: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 27353 {
            return Some(LightningRod {
                r#waterlogged: true,
                r#facing: Facing::West,
                r#powered: true,
            });
        }
        if state_id == 27354 {
            return Some(LightningRod {
                r#facing: Facing::West,
                r#powered: true,
                r#waterlogged: false,
            });
        }
        if state_id == 27347 {
            return Some(LightningRod {
                r#waterlogged: true,
                r#powered: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 27355 {
            return Some(LightningRod {
                r#powered: false,
                r#waterlogged: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 27350 {
            return Some(LightningRod {
                r#powered: true,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 27362 {
            return Some(LightningRod {
                r#powered: true,
                r#waterlogged: false,
                r#facing: Facing::Down,
            });
        }
        if state_id == 27349 {
            return Some(LightningRod {
                r#waterlogged: true,
                r#powered: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 27343 {
            return Some(LightningRod {
                r#powered: false,
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 27345 {
            return Some(LightningRod {
                r#waterlogged: true,
                r#facing: Facing::East,
                r#powered: true,
            });
        }
        if state_id == 27341 {
            return Some(LightningRod {
                r#powered: true,
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        return None;
    }
}

