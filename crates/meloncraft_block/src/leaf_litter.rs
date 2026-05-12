use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LeafLitter {
    pub segment_amount: i32,
    pub r#facing: Facing,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for LeafLitter {
    fn to_id(self) -> i32 {
        if block_state.r#facing == Facing::North && block_state.r#segment_amount == 3 { return 27646; }
        if block_state.r#facing == Facing::South && block_state.r#segment_amount == 1 { return 27648; }
        if block_state.r#facing == Facing::East && block_state.r#segment_amount == 1 { return 27656; }
        if block_state.r#segment_amount == 2 && block_state.r#facing == Facing::East { return 27657; }
        if block_state.r#segment_amount == 2 && block_state.r#facing == Facing::West { return 27653; }
        if block_state.r#facing == Facing::South && block_state.r#segment_amount == 3 { return 27650; }
        if block_state.r#facing == Facing::West && block_state.r#segment_amount == 4 { return 27655; }
        if block_state.r#segment_amount == 3 && block_state.r#facing == Facing::West { return 27654; }
        if block_state.r#segment_amount == 1 && block_state.r#facing == Facing::West { return 27652; }
        if block_state.r#facing == Facing::North && block_state.r#segment_amount == 1 { return 27644; }
        if block_state.r#facing == Facing::East && block_state.r#segment_amount == 4 { return 27659; }
        if block_state.r#segment_amount == 4 && block_state.r#facing == Facing::South { return 27651; }
        if block_state.r#segment_amount == 4 && block_state.r#facing == Facing::North { return 27647; }
        if block_state.r#facing == Facing::East && block_state.r#segment_amount == 3 { return 27658; }
        if block_state.r#segment_amount == 2 && block_state.r#facing == Facing::North { return 27645; }
        if block_state.r#segment_amount == 2 && block_state.r#facing == Facing::South { return 27649; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 27646 {
            return Some(LeafLitter {
                r#facing: Facing::North,
                r#segment_amount: 3,
            });
        }
        if state_id == 27648 {
            return Some(LeafLitter {
                r#facing: Facing::South,
                r#segment_amount: 1,
            });
        }
        if state_id == 27656 {
            return Some(LeafLitter {
                r#facing: Facing::East,
                r#segment_amount: 1,
            });
        }
        if state_id == 27657 {
            return Some(LeafLitter {
                r#segment_amount: 2,
                r#facing: Facing::East,
            });
        }
        if state_id == 27653 {
            return Some(LeafLitter {
                r#segment_amount: 2,
                r#facing: Facing::West,
            });
        }
        if state_id == 27650 {
            return Some(LeafLitter {
                r#facing: Facing::South,
                r#segment_amount: 3,
            });
        }
        if state_id == 27655 {
            return Some(LeafLitter {
                r#facing: Facing::West,
                r#segment_amount: 4,
            });
        }
        if state_id == 27654 {
            return Some(LeafLitter {
                r#segment_amount: 3,
                r#facing: Facing::West,
            });
        }
        if state_id == 27652 {
            return Some(LeafLitter {
                r#segment_amount: 1,
                r#facing: Facing::West,
            });
        }
        if state_id == 27644 {
            return Some(LeafLitter {
                r#facing: Facing::North,
                r#segment_amount: 1,
            });
        }
        if state_id == 27659 {
            return Some(LeafLitter {
                r#facing: Facing::East,
                r#segment_amount: 4,
            });
        }
        if state_id == 27651 {
            return Some(LeafLitter {
                r#segment_amount: 4,
                r#facing: Facing::South,
            });
        }
        if state_id == 27647 {
            return Some(LeafLitter {
                r#segment_amount: 4,
                r#facing: Facing::North,
            });
        }
        if state_id == 27658 {
            return Some(LeafLitter {
                r#facing: Facing::East,
                r#segment_amount: 3,
            });
        }
        if state_id == 27645 {
            return Some(LeafLitter {
                r#segment_amount: 2,
                r#facing: Facing::North,
            });
        }
        if state_id == 27649 {
            return Some(LeafLitter {
                r#segment_amount: 2,
                r#facing: Facing::South,
            });
        }
        return None;
    }
}

