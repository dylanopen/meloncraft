use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AmethystCluster {
    pub waterlogged: bool,
    pub r#facing: Facing,
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

impl BlockState for AmethystCluster {
    fn to_id(self) -> i32 {
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == false { return 23209; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::East { return 23204; }
        if block_state.r#facing == Facing::Down && block_state.r#waterlogged == true { return 23212; }
        if block_state.r#facing == Facing::Down && block_state.r#waterlogged == false { return 23213; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == true { return 23202; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == true { return 23206; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::Up { return 23211; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::West { return 23208; }
        if block_state.r#facing == Facing::Up && block_state.r#waterlogged == true { return 23210; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == false { return 23203; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == false { return 23205; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == false { return 23207; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 23209 {
            return Some(AmethystCluster {
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 23204 {
            return Some(AmethystCluster {
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 23212 {
            return Some(AmethystCluster {
                r#facing: Facing::Down,
                r#waterlogged: true,
            });
        }
        if state_id == 23213 {
            return Some(AmethystCluster {
                r#facing: Facing::Down,
                r#waterlogged: false,
            });
        }
        if state_id == 23202 {
            return Some(AmethystCluster {
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 23206 {
            return Some(AmethystCluster {
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 23211 {
            return Some(AmethystCluster {
                r#waterlogged: false,
                r#facing: Facing::Up,
            });
        }
        if state_id == 23208 {
            return Some(AmethystCluster {
                r#waterlogged: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 23210 {
            return Some(AmethystCluster {
                r#facing: Facing::Up,
                r#waterlogged: true,
            });
        }
        if state_id == 23203 {
            return Some(AmethystCluster {
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 23205 {
            return Some(AmethystCluster {
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 23207 {
            return Some(AmethystCluster {
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        return None;
    }
}

