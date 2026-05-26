use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ZombieWallHead {
    pub r#facing: Facing,
    pub powered: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for ZombieWallHead {
    fn to_id(&self) -> i32 {
        if self.r#facing == Facing::West && self.r#powered == false {
            return 10830;
        }
        if self.r#facing == Facing::East && self.r#powered == false {
            return 10832;
        }
        if self.r#powered == false && self.r#facing == Facing::South {
            return 10828;
        }
        if self.r#facing == Facing::North && self.r#powered == true {
            return 10825;
        }
        if self.r#facing == Facing::South && self.r#powered == true {
            return 10827;
        }
        if self.r#powered == true && self.r#facing == Facing::West {
            return 10829;
        }
        if self.r#facing == Facing::East && self.r#powered == true {
            return 10831;
        }
        if self.r#powered == false && self.r#facing == Facing::North {
            return 10826;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 10830 {
            return Some(ZombieWallHead {
                r#facing: Facing::West,
                r#powered: false,
            });
        }
        if state_id == 10832 {
            return Some(ZombieWallHead {
                r#facing: Facing::East,
                r#powered: false,
            });
        }
        if state_id == 10828 {
            return Some(ZombieWallHead {
                r#powered: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 10825 {
            return Some(ZombieWallHead {
                r#facing: Facing::North,
                r#powered: true,
            });
        }
        if state_id == 10827 {
            return Some(ZombieWallHead {
                r#facing: Facing::South,
                r#powered: true,
            });
        }
        if state_id == 10829 {
            return Some(ZombieWallHead {
                r#powered: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 10831 {
            return Some(ZombieWallHead {
                r#facing: Facing::East,
                r#powered: true,
            });
        }
        if state_id == 10826 {
            return Some(ZombieWallHead {
                r#powered: false,
                r#facing: Facing::North,
            });
        }
        return None;
    }
}
