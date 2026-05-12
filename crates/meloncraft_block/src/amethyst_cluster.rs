use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AmethystCluster {
    pub r#facing: Facing,
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

impl BlockState for AmethystCluster {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == false && self.r#facing == Facing::Down { return 23213; }
        if self.r#facing == Facing::East && self.r#waterlogged == true { return 23204; }
        if self.r#facing == Facing::South && self.r#waterlogged == true { return 23206; }
        if self.r#facing == Facing::South && self.r#waterlogged == false { return 23207; }
        if self.r#facing == Facing::East && self.r#waterlogged == false { return 23205; }
        if self.r#waterlogged == true && self.r#facing == Facing::North { return 23202; }
        if self.r#facing == Facing::West && self.r#waterlogged == true { return 23208; }
        if self.r#waterlogged == false && self.r#facing == Facing::West { return 23209; }
        if self.r#facing == Facing::Up && self.r#waterlogged == true { return 23210; }
        if self.r#facing == Facing::Down && self.r#waterlogged == true { return 23212; }
        if self.r#waterlogged == false && self.r#facing == Facing::Up { return 23211; }
        if self.r#facing == Facing::North && self.r#waterlogged == false { return 23203; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 23213 {
            return Some(AmethystCluster {
                r#waterlogged: false,
                r#facing: Facing::Down,
            });
        }
        if state_id == 23204 {
            return Some(AmethystCluster {
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 23206 {
            return Some(AmethystCluster {
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 23207 {
            return Some(AmethystCluster {
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 23205 {
            return Some(AmethystCluster {
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 23202 {
            return Some(AmethystCluster {
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 23208 {
            return Some(AmethystCluster {
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 23209 {
            return Some(AmethystCluster {
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 23210 {
            return Some(AmethystCluster {
                r#facing: Facing::Up,
                r#waterlogged: true,
            });
        }
        if state_id == 23212 {
            return Some(AmethystCluster {
                r#facing: Facing::Down,
                r#waterlogged: true,
            });
        }
        if state_id == 23211 {
            return Some(AmethystCluster {
                r#waterlogged: false,
                r#facing: Facing::Up,
            });
        }
        if state_id == 23203 {
            return Some(AmethystCluster {
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        return None;
    }
}

