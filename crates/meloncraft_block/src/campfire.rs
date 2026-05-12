use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Campfire {
    pub lit: bool,
    pub signal_fire: bool,
    pub waterlogged: bool,
    pub r#facing: Facing,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for Campfire {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == true && block_state.r#lit == false && block_state.r#signal_fire == true && block_state.r#facing == Facing::North { return 20679; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == false && block_state.r#lit == false && block_state.r#signal_fire == false { return 20706; }
        if block_state.r#waterlogged == true && block_state.r#lit == true && block_state.r#signal_fire == true && block_state.r#facing == Facing::North { return 20675; }
        if block_state.r#signal_fire == false && block_state.r#facing == Facing::North && block_state.r#lit == true && block_state.r#waterlogged == true { return 20677; }
        if block_state.r#waterlogged == false && block_state.r#signal_fire == false && block_state.r#lit == true && block_state.r#facing == Facing::West { return 20694; }
        if block_state.r#lit == true && block_state.r#facing == Facing::East && block_state.r#signal_fire == true && block_state.r#waterlogged == true { return 20699; }
        if block_state.r#facing == Facing::North && block_state.r#lit == false && block_state.r#signal_fire == false && block_state.r#waterlogged == true { return 20681; }
        if block_state.r#lit == false && block_state.r#waterlogged == false && block_state.r#facing == Facing::South && block_state.r#signal_fire == true { return 20688; }
        if block_state.r#lit == true && block_state.r#waterlogged == true && block_state.r#facing == Facing::East && block_state.r#signal_fire == false { return 20701; }
        if block_state.r#facing == Facing::East && block_state.r#signal_fire == true && block_state.r#lit == false && block_state.r#waterlogged == false { return 20704; }
        if block_state.r#lit == false && block_state.r#signal_fire == true && block_state.r#waterlogged == false && block_state.r#facing == Facing::North { return 20680; }
        if block_state.r#waterlogged == true && block_state.r#lit == false && block_state.r#facing == Facing::South && block_state.r#signal_fire == true { return 20687; }
        if block_state.r#waterlogged == false && block_state.r#signal_fire == false && block_state.r#facing == Facing::South && block_state.r#lit == false { return 20690; }
        if block_state.r#facing == Facing::West && block_state.r#lit == false && block_state.r#signal_fire == false && block_state.r#waterlogged == true { return 20697; }
        if block_state.r#lit == true && block_state.r#signal_fire == false && block_state.r#facing == Facing::East && block_state.r#waterlogged == false { return 20702; }
        if block_state.r#facing == Facing::South && block_state.r#lit == true && block_state.r#signal_fire == true && block_state.r#waterlogged == true { return 20683; }
        if block_state.r#lit == true && block_state.r#facing == Facing::East && block_state.r#signal_fire == true && block_state.r#waterlogged == false { return 20700; }
        if block_state.r#lit == false && block_state.r#waterlogged == true && block_state.r#facing == Facing::South && block_state.r#signal_fire == false { return 20689; }
        if block_state.r#waterlogged == true && block_state.r#signal_fire == false && block_state.r#lit == true && block_state.r#facing == Facing::West { return 20693; }
        if block_state.r#lit == false && block_state.r#waterlogged == false && block_state.r#signal_fire == false && block_state.r#facing == Facing::West { return 20698; }
        if block_state.r#signal_fire == false && block_state.r#waterlogged == false && block_state.r#lit == true && block_state.r#facing == Facing::South { return 20686; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::West && block_state.r#lit == true && block_state.r#signal_fire == true { return 20692; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == true && block_state.r#lit == false && block_state.r#signal_fire == false { return 20705; }
        if block_state.r#waterlogged == true && block_state.r#lit == false && block_state.r#facing == Facing::West && block_state.r#signal_fire == true { return 20695; }
        if block_state.r#signal_fire == false && block_state.r#facing == Facing::North && block_state.r#waterlogged == false && block_state.r#lit == false { return 20682; }
        if block_state.r#facing == Facing::North && block_state.r#lit == true && block_state.r#signal_fire == true && block_state.r#waterlogged == false { return 20676; }
        if block_state.r#lit == true && block_state.r#signal_fire == true && block_state.r#facing == Facing::West && block_state.r#waterlogged == true { return 20691; }
        if block_state.r#lit == false && block_state.r#facing == Facing::East && block_state.r#signal_fire == true && block_state.r#waterlogged == true { return 20703; }
        if block_state.r#facing == Facing::North && block_state.r#signal_fire == false && block_state.r#lit == true && block_state.r#waterlogged == false { return 20678; }
        if block_state.r#signal_fire == true && block_state.r#lit == true && block_state.r#waterlogged == false && block_state.r#facing == Facing::South { return 20684; }
        if block_state.r#signal_fire == true && block_state.r#waterlogged == false && block_state.r#lit == false && block_state.r#facing == Facing::West { return 20696; }
        if block_state.r#facing == Facing::South && block_state.r#signal_fire == false && block_state.r#waterlogged == true && block_state.r#lit == true { return 20685; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 20679 {
            return Some(Campfire {
                r#waterlogged: true,
                r#lit: false,
                r#signal_fire: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 20706 {
            return Some(Campfire {
                r#facing: Facing::East,
                r#waterlogged: false,
                r#lit: false,
                r#signal_fire: false,
            });
        }
        if state_id == 20675 {
            return Some(Campfire {
                r#waterlogged: true,
                r#lit: true,
                r#signal_fire: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 20677 {
            return Some(Campfire {
                r#signal_fire: false,
                r#facing: Facing::North,
                r#lit: true,
                r#waterlogged: true,
            });
        }
        if state_id == 20694 {
            return Some(Campfire {
                r#waterlogged: false,
                r#signal_fire: false,
                r#lit: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 20699 {
            return Some(Campfire {
                r#lit: true,
                r#facing: Facing::East,
                r#signal_fire: true,
                r#waterlogged: true,
            });
        }
        if state_id == 20681 {
            return Some(Campfire {
                r#facing: Facing::North,
                r#lit: false,
                r#signal_fire: false,
                r#waterlogged: true,
            });
        }
        if state_id == 20688 {
            return Some(Campfire {
                r#lit: false,
                r#waterlogged: false,
                r#facing: Facing::South,
                r#signal_fire: true,
            });
        }
        if state_id == 20701 {
            return Some(Campfire {
                r#lit: true,
                r#waterlogged: true,
                r#facing: Facing::East,
                r#signal_fire: false,
            });
        }
        if state_id == 20704 {
            return Some(Campfire {
                r#facing: Facing::East,
                r#signal_fire: true,
                r#lit: false,
                r#waterlogged: false,
            });
        }
        if state_id == 20680 {
            return Some(Campfire {
                r#lit: false,
                r#signal_fire: true,
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 20687 {
            return Some(Campfire {
                r#waterlogged: true,
                r#lit: false,
                r#facing: Facing::South,
                r#signal_fire: true,
            });
        }
        if state_id == 20690 {
            return Some(Campfire {
                r#waterlogged: false,
                r#signal_fire: false,
                r#facing: Facing::South,
                r#lit: false,
            });
        }
        if state_id == 20697 {
            return Some(Campfire {
                r#facing: Facing::West,
                r#lit: false,
                r#signal_fire: false,
                r#waterlogged: true,
            });
        }
        if state_id == 20702 {
            return Some(Campfire {
                r#lit: true,
                r#signal_fire: false,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 20683 {
            return Some(Campfire {
                r#facing: Facing::South,
                r#lit: true,
                r#signal_fire: true,
                r#waterlogged: true,
            });
        }
        if state_id == 20700 {
            return Some(Campfire {
                r#lit: true,
                r#facing: Facing::East,
                r#signal_fire: true,
                r#waterlogged: false,
            });
        }
        if state_id == 20689 {
            return Some(Campfire {
                r#lit: false,
                r#waterlogged: true,
                r#facing: Facing::South,
                r#signal_fire: false,
            });
        }
        if state_id == 20693 {
            return Some(Campfire {
                r#waterlogged: true,
                r#signal_fire: false,
                r#lit: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 20698 {
            return Some(Campfire {
                r#lit: false,
                r#waterlogged: false,
                r#signal_fire: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 20686 {
            return Some(Campfire {
                r#signal_fire: false,
                r#waterlogged: false,
                r#lit: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 20692 {
            return Some(Campfire {
                r#waterlogged: false,
                r#facing: Facing::West,
                r#lit: true,
                r#signal_fire: true,
            });
        }
        if state_id == 20705 {
            return Some(Campfire {
                r#facing: Facing::East,
                r#waterlogged: true,
                r#lit: false,
                r#signal_fire: false,
            });
        }
        if state_id == 20695 {
            return Some(Campfire {
                r#waterlogged: true,
                r#lit: false,
                r#facing: Facing::West,
                r#signal_fire: true,
            });
        }
        if state_id == 20682 {
            return Some(Campfire {
                r#signal_fire: false,
                r#facing: Facing::North,
                r#waterlogged: false,
                r#lit: false,
            });
        }
        if state_id == 20676 {
            return Some(Campfire {
                r#facing: Facing::North,
                r#lit: true,
                r#signal_fire: true,
                r#waterlogged: false,
            });
        }
        if state_id == 20691 {
            return Some(Campfire {
                r#lit: true,
                r#signal_fire: true,
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 20703 {
            return Some(Campfire {
                r#lit: false,
                r#facing: Facing::East,
                r#signal_fire: true,
                r#waterlogged: true,
            });
        }
        if state_id == 20678 {
            return Some(Campfire {
                r#facing: Facing::North,
                r#signal_fire: false,
                r#lit: true,
                r#waterlogged: false,
            });
        }
        if state_id == 20684 {
            return Some(Campfire {
                r#signal_fire: true,
                r#lit: true,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 20696 {
            return Some(Campfire {
                r#signal_fire: true,
                r#waterlogged: false,
                r#lit: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 20685 {
            return Some(Campfire {
                r#facing: Facing::South,
                r#signal_fire: false,
                r#waterlogged: true,
                r#lit: true,
            });
        }
        return None;
    }
}

