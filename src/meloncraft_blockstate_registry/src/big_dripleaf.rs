use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BigDripleaf {
    pub r#facing: Facing,
    pub waterlogged: bool,
    pub r#tilt: Tilt,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tilt {
    None,
    Unstable,
    Partial,
    Full,
}

impl BlockState for BigDripleaf {
    fn to_id(&self) -> i32 {
        if self.r#facing == Facing::North && self.r#tilt == Tilt::Full && self.r#waterlogged == false { return 27668; }
        if self.r#tilt == Tilt::Partial && self.r#facing == Facing::East && self.r#waterlogged == true { return 27689; }
        if self.r#waterlogged == true && self.r#facing == Facing::North && self.r#tilt == Tilt::Unstable { return 27663; }
        if self.r#tilt == Tilt::None && self.r#facing == Facing::North && self.r#waterlogged == true { return 27661; }
        if self.r#waterlogged == false && self.r#facing == Facing::South && self.r#tilt == Tilt::Partial { return 27674; }
        if self.r#waterlogged == true && self.r#facing == Facing::North && self.r#tilt == Tilt::Partial { return 27665; }
        if self.r#tilt == Tilt::Full && self.r#facing == Facing::South && self.r#waterlogged == true { return 27675; }
        if self.r#tilt == Tilt::Partial && self.r#waterlogged == false && self.r#facing == Facing::East { return 27690; }
        if self.r#waterlogged == true && self.r#facing == Facing::East && self.r#tilt == Tilt::None { return 27685; }
        if self.r#tilt == Tilt::Unstable && self.r#waterlogged == true && self.r#facing == Facing::East { return 27687; }
        if self.r#tilt == Tilt::Partial && self.r#waterlogged == false && self.r#facing == Facing::North { return 27666; }
        if self.r#facing == Facing::West && self.r#waterlogged == false && self.r#tilt == Tilt::Partial { return 27682; }
        if self.r#waterlogged == true && self.r#facing == Facing::South && self.r#tilt == Tilt::Unstable { return 27671; }
        if self.r#tilt == Tilt::None && self.r#waterlogged == false && self.r#facing == Facing::North { return 27662; }
        if self.r#facing == Facing::South && self.r#tilt == Tilt::None && self.r#waterlogged == true { return 27669; }
        if self.r#facing == Facing::West && self.r#tilt == Tilt::Full && self.r#waterlogged == true { return 27683; }
        if self.r#facing == Facing::North && self.r#tilt == Tilt::Unstable && self.r#waterlogged == false { return 27664; }
        if self.r#tilt == Tilt::Full && self.r#waterlogged == false && self.r#facing == Facing::South { return 27676; }
        if self.r#tilt == Tilt::Partial && self.r#facing == Facing::West && self.r#waterlogged == true { return 27681; }
        if self.r#tilt == Tilt::Full && self.r#facing == Facing::West && self.r#waterlogged == false { return 27684; }
        if self.r#facing == Facing::South && self.r#tilt == Tilt::None && self.r#waterlogged == false { return 27670; }
        if self.r#tilt == Tilt::Unstable && self.r#facing == Facing::West && self.r#waterlogged == true { return 27679; }
        if self.r#waterlogged == true && self.r#tilt == Tilt::Partial && self.r#facing == Facing::South { return 27673; }
        if self.r#facing == Facing::North && self.r#tilt == Tilt::Full && self.r#waterlogged == true { return 27667; }
        if self.r#waterlogged == false && self.r#tilt == Tilt::Unstable && self.r#facing == Facing::South { return 27672; }
        if self.r#waterlogged == true && self.r#tilt == Tilt::None && self.r#facing == Facing::West { return 27677; }
        if self.r#tilt == Tilt::Unstable && self.r#waterlogged == false && self.r#facing == Facing::East { return 27688; }
        if self.r#facing == Facing::West && self.r#tilt == Tilt::Unstable && self.r#waterlogged == false { return 27680; }
        if self.r#facing == Facing::East && self.r#waterlogged == true && self.r#tilt == Tilt::Full { return 27691; }
        if self.r#facing == Facing::East && self.r#tilt == Tilt::None && self.r#waterlogged == false { return 27686; }
        if self.r#waterlogged == false && self.r#tilt == Tilt::None && self.r#facing == Facing::West { return 27678; }
        if self.r#waterlogged == false && self.r#tilt == Tilt::Full && self.r#facing == Facing::East { return 27692; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 27668 {
            return Some(BigDripleaf {
                r#facing: Facing::North,
                r#tilt: Tilt::Full,
                r#waterlogged: false,
            });
        }
        if state_id == 27689 {
            return Some(BigDripleaf {
                r#tilt: Tilt::Partial,
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 27663 {
            return Some(BigDripleaf {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#tilt: Tilt::Unstable,
            });
        }
        if state_id == 27661 {
            return Some(BigDripleaf {
                r#tilt: Tilt::None,
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 27674 {
            return Some(BigDripleaf {
                r#waterlogged: false,
                r#facing: Facing::South,
                r#tilt: Tilt::Partial,
            });
        }
        if state_id == 27665 {
            return Some(BigDripleaf {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#tilt: Tilt::Partial,
            });
        }
        if state_id == 27675 {
            return Some(BigDripleaf {
                r#tilt: Tilt::Full,
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 27690 {
            return Some(BigDripleaf {
                r#tilt: Tilt::Partial,
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 27685 {
            return Some(BigDripleaf {
                r#waterlogged: true,
                r#facing: Facing::East,
                r#tilt: Tilt::None,
            });
        }
        if state_id == 27687 {
            return Some(BigDripleaf {
                r#tilt: Tilt::Unstable,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 27666 {
            return Some(BigDripleaf {
                r#tilt: Tilt::Partial,
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 27682 {
            return Some(BigDripleaf {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#tilt: Tilt::Partial,
            });
        }
        if state_id == 27671 {
            return Some(BigDripleaf {
                r#waterlogged: true,
                r#facing: Facing::South,
                r#tilt: Tilt::Unstable,
            });
        }
        if state_id == 27662 {
            return Some(BigDripleaf {
                r#tilt: Tilt::None,
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 27669 {
            return Some(BigDripleaf {
                r#facing: Facing::South,
                r#tilt: Tilt::None,
                r#waterlogged: true,
            });
        }
        if state_id == 27683 {
            return Some(BigDripleaf {
                r#facing: Facing::West,
                r#tilt: Tilt::Full,
                r#waterlogged: true,
            });
        }
        if state_id == 27664 {
            return Some(BigDripleaf {
                r#facing: Facing::North,
                r#tilt: Tilt::Unstable,
                r#waterlogged: false,
            });
        }
        if state_id == 27676 {
            return Some(BigDripleaf {
                r#tilt: Tilt::Full,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 27681 {
            return Some(BigDripleaf {
                r#tilt: Tilt::Partial,
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 27684 {
            return Some(BigDripleaf {
                r#tilt: Tilt::Full,
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 27670 {
            return Some(BigDripleaf {
                r#facing: Facing::South,
                r#tilt: Tilt::None,
                r#waterlogged: false,
            });
        }
        if state_id == 27679 {
            return Some(BigDripleaf {
                r#tilt: Tilt::Unstable,
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 27673 {
            return Some(BigDripleaf {
                r#waterlogged: true,
                r#tilt: Tilt::Partial,
                r#facing: Facing::South,
            });
        }
        if state_id == 27667 {
            return Some(BigDripleaf {
                r#facing: Facing::North,
                r#tilt: Tilt::Full,
                r#waterlogged: true,
            });
        }
        if state_id == 27672 {
            return Some(BigDripleaf {
                r#waterlogged: false,
                r#tilt: Tilt::Unstable,
                r#facing: Facing::South,
            });
        }
        if state_id == 27677 {
            return Some(BigDripleaf {
                r#waterlogged: true,
                r#tilt: Tilt::None,
                r#facing: Facing::West,
            });
        }
        if state_id == 27688 {
            return Some(BigDripleaf {
                r#tilt: Tilt::Unstable,
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 27680 {
            return Some(BigDripleaf {
                r#facing: Facing::West,
                r#tilt: Tilt::Unstable,
                r#waterlogged: false,
            });
        }
        if state_id == 27691 {
            return Some(BigDripleaf {
                r#facing: Facing::East,
                r#waterlogged: true,
                r#tilt: Tilt::Full,
            });
        }
        if state_id == 27686 {
            return Some(BigDripleaf {
                r#facing: Facing::East,
                r#tilt: Tilt::None,
                r#waterlogged: false,
            });
        }
        if state_id == 27678 {
            return Some(BigDripleaf {
                r#waterlogged: false,
                r#tilt: Tilt::None,
                r#facing: Facing::West,
            });
        }
        if state_id == 27692 {
            return Some(BigDripleaf {
                r#waterlogged: false,
                r#tilt: Tilt::Full,
                r#facing: Facing::East,
            });
        }
        return None;
    }
}

