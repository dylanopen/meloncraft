use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SoulCampfire {
    pub waterlogged: bool,
    pub r#facing: Facing,
    pub lit: bool,
    pub signal_fire: bool,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for SoulCampfire {
    fn to_id(&self) -> i32 {
        if self.r#signal_fire == true && self.r#waterlogged == true && self.r#facing == Facing::South && self.r#lit == false { return 20719; }
        if self.r#waterlogged == false && self.r#signal_fire == true && self.r#facing == Facing::East && self.r#lit == true { return 20732; }
        if self.r#lit == false && self.r#waterlogged == false && self.r#signal_fire == false && self.r#facing == Facing::East { return 20738; }
        if self.r#signal_fire == false && self.r#lit == true && self.r#facing == Facing::North && self.r#waterlogged == false { return 20710; }
        if self.r#facing == Facing::North && self.r#signal_fire == false && self.r#waterlogged == true && self.r#lit == false { return 20713; }
        if self.r#facing == Facing::West && self.r#lit == true && self.r#signal_fire == true && self.r#waterlogged == true { return 20723; }
        if self.r#lit == false && self.r#waterlogged == false && self.r#facing == Facing::East && self.r#signal_fire == true { return 20736; }
        if self.r#facing == Facing::North && self.r#signal_fire == true && self.r#lit == true && self.r#waterlogged == true { return 20707; }
        if self.r#lit == false && self.r#waterlogged == true && self.r#facing == Facing::West && self.r#signal_fire == true { return 20727; }
        if self.r#lit == true && self.r#facing == Facing::South && self.r#signal_fire == false && self.r#waterlogged == false { return 20718; }
        if self.r#signal_fire == true && self.r#facing == Facing::North && self.r#lit == false && self.r#waterlogged == true { return 20711; }
        if self.r#lit == true && self.r#facing == Facing::West && self.r#signal_fire == false && self.r#waterlogged == true { return 20725; }
        if self.r#facing == Facing::North && self.r#lit == false && self.r#signal_fire == true && self.r#waterlogged == false { return 20712; }
        if self.r#waterlogged == true && self.r#facing == Facing::East && self.r#signal_fire == true && self.r#lit == false { return 20735; }
        if self.r#waterlogged == true && self.r#lit == true && self.r#facing == Facing::East && self.r#signal_fire == false { return 20733; }
        if self.r#waterlogged == true && self.r#lit == false && self.r#signal_fire == false && self.r#facing == Facing::South { return 20721; }
        if self.r#waterlogged == true && self.r#facing == Facing::South && self.r#lit == true && self.r#signal_fire == true { return 20715; }
        if self.r#facing == Facing::North && self.r#signal_fire == true && self.r#lit == true && self.r#waterlogged == false { return 20708; }
        if self.r#lit == true && self.r#signal_fire == false && self.r#facing == Facing::North && self.r#waterlogged == true { return 20709; }
        if self.r#signal_fire == true && self.r#facing == Facing::East && self.r#lit == true && self.r#waterlogged == true { return 20731; }
        if self.r#lit == false && self.r#facing == Facing::West && self.r#waterlogged == true && self.r#signal_fire == false { return 20729; }
        if self.r#facing == Facing::South && self.r#waterlogged == true && self.r#signal_fire == false && self.r#lit == true { return 20717; }
        if self.r#waterlogged == false && self.r#signal_fire == false && self.r#lit == false && self.r#facing == Facing::West { return 20730; }
        if self.r#facing == Facing::West && self.r#lit == true && self.r#waterlogged == false && self.r#signal_fire == false { return 20726; }
        if self.r#lit == true && self.r#facing == Facing::South && self.r#waterlogged == false && self.r#signal_fire == true { return 20716; }
        if self.r#facing == Facing::East && self.r#lit == false && self.r#waterlogged == true && self.r#signal_fire == false { return 20737; }
        if self.r#waterlogged == false && self.r#facing == Facing::West && self.r#lit == false && self.r#signal_fire == true { return 20728; }
        if self.r#lit == false && self.r#facing == Facing::South && self.r#signal_fire == true && self.r#waterlogged == false { return 20720; }
        if self.r#lit == false && self.r#waterlogged == false && self.r#signal_fire == false && self.r#facing == Facing::North { return 20714; }
        if self.r#facing == Facing::East && self.r#lit == true && self.r#signal_fire == false && self.r#waterlogged == false { return 20734; }
        if self.r#signal_fire == true && self.r#waterlogged == false && self.r#lit == true && self.r#facing == Facing::West { return 20724; }
        if self.r#signal_fire == false && self.r#facing == Facing::South && self.r#waterlogged == false && self.r#lit == false { return 20722; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 20719 {
            return Some(SoulCampfire {
                r#signal_fire: true,
                r#waterlogged: true,
                r#facing: Facing::South,
                r#lit: false,
            });
        }
        if state_id == 20732 {
            return Some(SoulCampfire {
                r#waterlogged: false,
                r#signal_fire: true,
                r#facing: Facing::East,
                r#lit: true,
            });
        }
        if state_id == 20738 {
            return Some(SoulCampfire {
                r#lit: false,
                r#waterlogged: false,
                r#signal_fire: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 20710 {
            return Some(SoulCampfire {
                r#signal_fire: false,
                r#lit: true,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 20713 {
            return Some(SoulCampfire {
                r#facing: Facing::North,
                r#signal_fire: false,
                r#waterlogged: true,
                r#lit: false,
            });
        }
        if state_id == 20723 {
            return Some(SoulCampfire {
                r#facing: Facing::West,
                r#lit: true,
                r#signal_fire: true,
                r#waterlogged: true,
            });
        }
        if state_id == 20736 {
            return Some(SoulCampfire {
                r#lit: false,
                r#waterlogged: false,
                r#facing: Facing::East,
                r#signal_fire: true,
            });
        }
        if state_id == 20707 {
            return Some(SoulCampfire {
                r#facing: Facing::North,
                r#signal_fire: true,
                r#lit: true,
                r#waterlogged: true,
            });
        }
        if state_id == 20727 {
            return Some(SoulCampfire {
                r#lit: false,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#signal_fire: true,
            });
        }
        if state_id == 20718 {
            return Some(SoulCampfire {
                r#lit: true,
                r#facing: Facing::South,
                r#signal_fire: false,
                r#waterlogged: false,
            });
        }
        if state_id == 20711 {
            return Some(SoulCampfire {
                r#signal_fire: true,
                r#facing: Facing::North,
                r#lit: false,
                r#waterlogged: true,
            });
        }
        if state_id == 20725 {
            return Some(SoulCampfire {
                r#lit: true,
                r#facing: Facing::West,
                r#signal_fire: false,
                r#waterlogged: true,
            });
        }
        if state_id == 20712 {
            return Some(SoulCampfire {
                r#facing: Facing::North,
                r#lit: false,
                r#signal_fire: true,
                r#waterlogged: false,
            });
        }
        if state_id == 20735 {
            return Some(SoulCampfire {
                r#waterlogged: true,
                r#facing: Facing::East,
                r#signal_fire: true,
                r#lit: false,
            });
        }
        if state_id == 20733 {
            return Some(SoulCampfire {
                r#waterlogged: true,
                r#lit: true,
                r#facing: Facing::East,
                r#signal_fire: false,
            });
        }
        if state_id == 20721 {
            return Some(SoulCampfire {
                r#waterlogged: true,
                r#lit: false,
                r#signal_fire: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 20715 {
            return Some(SoulCampfire {
                r#waterlogged: true,
                r#facing: Facing::South,
                r#lit: true,
                r#signal_fire: true,
            });
        }
        if state_id == 20708 {
            return Some(SoulCampfire {
                r#facing: Facing::North,
                r#signal_fire: true,
                r#lit: true,
                r#waterlogged: false,
            });
        }
        if state_id == 20709 {
            return Some(SoulCampfire {
                r#lit: true,
                r#signal_fire: false,
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 20731 {
            return Some(SoulCampfire {
                r#signal_fire: true,
                r#facing: Facing::East,
                r#lit: true,
                r#waterlogged: true,
            });
        }
        if state_id == 20729 {
            return Some(SoulCampfire {
                r#lit: false,
                r#facing: Facing::West,
                r#waterlogged: true,
                r#signal_fire: false,
            });
        }
        if state_id == 20717 {
            return Some(SoulCampfire {
                r#facing: Facing::South,
                r#waterlogged: true,
                r#signal_fire: false,
                r#lit: true,
            });
        }
        if state_id == 20730 {
            return Some(SoulCampfire {
                r#waterlogged: false,
                r#signal_fire: false,
                r#lit: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 20726 {
            return Some(SoulCampfire {
                r#facing: Facing::West,
                r#lit: true,
                r#waterlogged: false,
                r#signal_fire: false,
            });
        }
        if state_id == 20716 {
            return Some(SoulCampfire {
                r#lit: true,
                r#facing: Facing::South,
                r#waterlogged: false,
                r#signal_fire: true,
            });
        }
        if state_id == 20737 {
            return Some(SoulCampfire {
                r#facing: Facing::East,
                r#lit: false,
                r#waterlogged: true,
                r#signal_fire: false,
            });
        }
        if state_id == 20728 {
            return Some(SoulCampfire {
                r#waterlogged: false,
                r#facing: Facing::West,
                r#lit: false,
                r#signal_fire: true,
            });
        }
        if state_id == 20720 {
            return Some(SoulCampfire {
                r#lit: false,
                r#facing: Facing::South,
                r#signal_fire: true,
                r#waterlogged: false,
            });
        }
        if state_id == 20714 {
            return Some(SoulCampfire {
                r#lit: false,
                r#waterlogged: false,
                r#signal_fire: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 20734 {
            return Some(SoulCampfire {
                r#facing: Facing::East,
                r#lit: true,
                r#signal_fire: false,
                r#waterlogged: false,
            });
        }
        if state_id == 20724 {
            return Some(SoulCampfire {
                r#signal_fire: true,
                r#waterlogged: false,
                r#lit: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 20722 {
            return Some(SoulCampfire {
                r#signal_fire: false,
                r#facing: Facing::South,
                r#waterlogged: false,
                r#lit: false,
            });
        }
        return None;
    }
}

