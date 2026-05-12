use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DriedGhast {
    pub hydration: i32,
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

impl BlockState for DriedGhast {
    fn to_id(self) -> i32 {
        if block_state.r#facing == Facing::North && block_state.r#hydration == 3 && block_state.r#waterlogged == true { return 14909; }
        if block_state.r#facing == Facing::West && block_state.r#hydration == 0 && block_state.r#waterlogged == true { return 14919; }
        if block_state.r#waterlogged == false && block_state.r#hydration == 2 && block_state.r#facing == Facing::East { return 14932; }
        if block_state.r#hydration == 0 && block_state.r#facing == Facing::North && block_state.r#waterlogged == false { return 14904; }
        if block_state.r#hydration == 0 && block_state.r#waterlogged == true && block_state.r#facing == Facing::South { return 14911; }
        if block_state.r#facing == Facing::South && block_state.r#hydration == 3 && block_state.r#waterlogged == false { return 14918; }
        if block_state.r#hydration == 1 && block_state.r#waterlogged == false && block_state.r#facing == Facing::East { return 14930; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == true && block_state.r#hydration == 1 { return 14905; }
        if block_state.r#hydration == 3 && block_state.r#waterlogged == false && block_state.r#facing == Facing::East { return 14934; }
        if block_state.r#waterlogged == true && block_state.r#hydration == 2 && block_state.r#facing == Facing::South { return 14915; }
        if block_state.r#waterlogged == true && block_state.r#hydration == 3 && block_state.r#facing == Facing::South { return 14917; }
        if block_state.r#hydration == 0 && block_state.r#waterlogged == false && block_state.r#facing == Facing::South { return 14912; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == false && block_state.r#hydration == 3 { return 14926; }
        if block_state.r#hydration == 2 && block_state.r#facing == Facing::West && block_state.r#waterlogged == false { return 14924; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == true && block_state.r#hydration == 1 { return 14929; }
        if block_state.r#facing == Facing::East && block_state.r#hydration == 0 && block_state.r#waterlogged == false { return 14928; }
        if block_state.r#facing == Facing::North && block_state.r#hydration == 2 && block_state.r#waterlogged == false { return 14908; }
        if block_state.r#facing == Facing::North && block_state.r#hydration == 0 && block_state.r#waterlogged == true { return 14903; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::East && block_state.r#hydration == 3 { return 14933; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::South && block_state.r#hydration == 1 { return 14913; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::West && block_state.r#hydration == 1 { return 14922; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == true && block_state.r#hydration == 3 { return 14925; }
        if block_state.r#facing == Facing::South && block_state.r#hydration == 2 && block_state.r#waterlogged == false { return 14916; }
        if block_state.r#hydration == 1 && block_state.r#waterlogged == true && block_state.r#facing == Facing::West { return 14921; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::West && block_state.r#hydration == 2 { return 14923; }
        if block_state.r#waterlogged == false && block_state.r#hydration == 1 && block_state.r#facing == Facing::South { return 14914; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::North && block_state.r#hydration == 3 { return 14910; }
        if block_state.r#hydration == 0 && block_state.r#facing == Facing::East && block_state.r#waterlogged == true { return 14927; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == false && block_state.r#hydration == 0 { return 14920; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::East && block_state.r#hydration == 2 { return 14931; }
        if block_state.r#facing == Facing::North && block_state.r#hydration == 1 && block_state.r#waterlogged == false { return 14906; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::North && block_state.r#hydration == 2 { return 14907; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14909 {
            return Some(DriedGhast {
                r#facing: Facing::North,
                r#hydration: 3,
                r#waterlogged: true,
            });
        }
        if state_id == 14919 {
            return Some(DriedGhast {
                r#facing: Facing::West,
                r#hydration: 0,
                r#waterlogged: true,
            });
        }
        if state_id == 14932 {
            return Some(DriedGhast {
                r#waterlogged: false,
                r#hydration: 2,
                r#facing: Facing::East,
            });
        }
        if state_id == 14904 {
            return Some(DriedGhast {
                r#hydration: 0,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 14911 {
            return Some(DriedGhast {
                r#hydration: 0,
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 14918 {
            return Some(DriedGhast {
                r#facing: Facing::South,
                r#hydration: 3,
                r#waterlogged: false,
            });
        }
        if state_id == 14930 {
            return Some(DriedGhast {
                r#hydration: 1,
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 14905 {
            return Some(DriedGhast {
                r#facing: Facing::North,
                r#waterlogged: true,
                r#hydration: 1,
            });
        }
        if state_id == 14934 {
            return Some(DriedGhast {
                r#hydration: 3,
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 14915 {
            return Some(DriedGhast {
                r#waterlogged: true,
                r#hydration: 2,
                r#facing: Facing::South,
            });
        }
        if state_id == 14917 {
            return Some(DriedGhast {
                r#waterlogged: true,
                r#hydration: 3,
                r#facing: Facing::South,
            });
        }
        if state_id == 14912 {
            return Some(DriedGhast {
                r#hydration: 0,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 14926 {
            return Some(DriedGhast {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#hydration: 3,
            });
        }
        if state_id == 14924 {
            return Some(DriedGhast {
                r#hydration: 2,
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 14929 {
            return Some(DriedGhast {
                r#facing: Facing::East,
                r#waterlogged: true,
                r#hydration: 1,
            });
        }
        if state_id == 14928 {
            return Some(DriedGhast {
                r#facing: Facing::East,
                r#hydration: 0,
                r#waterlogged: false,
            });
        }
        if state_id == 14908 {
            return Some(DriedGhast {
                r#facing: Facing::North,
                r#hydration: 2,
                r#waterlogged: false,
            });
        }
        if state_id == 14903 {
            return Some(DriedGhast {
                r#facing: Facing::North,
                r#hydration: 0,
                r#waterlogged: true,
            });
        }
        if state_id == 14933 {
            return Some(DriedGhast {
                r#waterlogged: true,
                r#facing: Facing::East,
                r#hydration: 3,
            });
        }
        if state_id == 14913 {
            return Some(DriedGhast {
                r#waterlogged: true,
                r#facing: Facing::South,
                r#hydration: 1,
            });
        }
        if state_id == 14922 {
            return Some(DriedGhast {
                r#waterlogged: false,
                r#facing: Facing::West,
                r#hydration: 1,
            });
        }
        if state_id == 14925 {
            return Some(DriedGhast {
                r#facing: Facing::West,
                r#waterlogged: true,
                r#hydration: 3,
            });
        }
        if state_id == 14916 {
            return Some(DriedGhast {
                r#facing: Facing::South,
                r#hydration: 2,
                r#waterlogged: false,
            });
        }
        if state_id == 14921 {
            return Some(DriedGhast {
                r#hydration: 1,
                r#waterlogged: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 14923 {
            return Some(DriedGhast {
                r#waterlogged: true,
                r#facing: Facing::West,
                r#hydration: 2,
            });
        }
        if state_id == 14914 {
            return Some(DriedGhast {
                r#waterlogged: false,
                r#hydration: 1,
                r#facing: Facing::South,
            });
        }
        if state_id == 14910 {
            return Some(DriedGhast {
                r#waterlogged: false,
                r#facing: Facing::North,
                r#hydration: 3,
            });
        }
        if state_id == 14927 {
            return Some(DriedGhast {
                r#hydration: 0,
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 14920 {
            return Some(DriedGhast {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#hydration: 0,
            });
        }
        if state_id == 14931 {
            return Some(DriedGhast {
                r#waterlogged: true,
                r#facing: Facing::East,
                r#hydration: 2,
            });
        }
        if state_id == 14906 {
            return Some(DriedGhast {
                r#facing: Facing::North,
                r#hydration: 1,
                r#waterlogged: false,
            });
        }
        if state_id == 14907 {
            return Some(DriedGhast {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#hydration: 2,
            });
        }
        return None;
    }
}

