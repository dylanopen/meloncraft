use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OxidizedLightningRod {
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

impl BlockState for OxidizedLightningRod {
    fn to_id(&self) -> i32 {
        if self.r#powered == false && self.r#facing == Facing::East && self.r#waterlogged == false { return 27420; }
        if self.r#powered == false && self.r#waterlogged == true && self.r#facing == Facing::East { return 27419; }
        if self.r#waterlogged == true && self.r#powered == true && self.r#facing == Facing::East { return 27417; }
        if self.r#waterlogged == true && self.r#powered == false && self.r#facing == Facing::Up { return 27431; }
        if self.r#waterlogged == false && self.r#facing == Facing::South && self.r#powered == false { return 27424; }
        if self.r#facing == Facing::North && self.r#waterlogged == false && self.r#powered == false { return 27416; }
        if self.r#powered == false && self.r#waterlogged == false && self.r#facing == Facing::West { return 27428; }
        if self.r#facing == Facing::West && self.r#powered == true && self.r#waterlogged == true { return 27425; }
        if self.r#powered == true && self.r#waterlogged == true && self.r#facing == Facing::South { return 27421; }
        if self.r#facing == Facing::South && self.r#powered == false && self.r#waterlogged == true { return 27423; }
        if self.r#facing == Facing::North && self.r#powered == true && self.r#waterlogged == true { return 27413; }
        if self.r#waterlogged == false && self.r#powered == true && self.r#facing == Facing::East { return 27418; }
        if self.r#powered == true && self.r#waterlogged == false && self.r#facing == Facing::West { return 27426; }
        if self.r#waterlogged == false && self.r#facing == Facing::North && self.r#powered == true { return 27414; }
        if self.r#waterlogged == true && self.r#powered == true && self.r#facing == Facing::Up { return 27429; }
        if self.r#facing == Facing::Up && self.r#powered == false && self.r#waterlogged == false { return 27432; }
        if self.r#powered == false && self.r#facing == Facing::North && self.r#waterlogged == true { return 27415; }
        if self.r#waterlogged == true && self.r#facing == Facing::West && self.r#powered == false { return 27427; }
        if self.r#facing == Facing::Down && self.r#powered == true && self.r#waterlogged == false { return 27434; }
        if self.r#powered == false && self.r#facing == Facing::Down && self.r#waterlogged == true { return 27435; }
        if self.r#waterlogged == false && self.r#facing == Facing::Down && self.r#powered == false { return 27436; }
        if self.r#facing == Facing::Up && self.r#powered == true && self.r#waterlogged == false { return 27430; }
        if self.r#waterlogged == true && self.r#facing == Facing::Down && self.r#powered == true { return 27433; }
        if self.r#waterlogged == false && self.r#facing == Facing::South && self.r#powered == true { return 27422; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 27420 {
            return Some(OxidizedLightningRod {
                r#powered: false,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 27419 {
            return Some(OxidizedLightningRod {
                r#powered: false,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 27417 {
            return Some(OxidizedLightningRod {
                r#waterlogged: true,
                r#powered: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 27431 {
            return Some(OxidizedLightningRod {
                r#waterlogged: true,
                r#powered: false,
                r#facing: Facing::Up,
            });
        }
        if state_id == 27424 {
            return Some(OxidizedLightningRod {
                r#waterlogged: false,
                r#facing: Facing::South,
                r#powered: false,
            });
        }
        if state_id == 27416 {
            return Some(OxidizedLightningRod {
                r#facing: Facing::North,
                r#waterlogged: false,
                r#powered: false,
            });
        }
        if state_id == 27428 {
            return Some(OxidizedLightningRod {
                r#powered: false,
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 27425 {
            return Some(OxidizedLightningRod {
                r#facing: Facing::West,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 27421 {
            return Some(OxidizedLightningRod {
                r#powered: true,
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 27423 {
            return Some(OxidizedLightningRod {
                r#facing: Facing::South,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 27413 {
            return Some(OxidizedLightningRod {
                r#facing: Facing::North,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 27418 {
            return Some(OxidizedLightningRod {
                r#waterlogged: false,
                r#powered: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 27426 {
            return Some(OxidizedLightningRod {
                r#powered: true,
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 27414 {
            return Some(OxidizedLightningRod {
                r#waterlogged: false,
                r#facing: Facing::North,
                r#powered: true,
            });
        }
        if state_id == 27429 {
            return Some(OxidizedLightningRod {
                r#waterlogged: true,
                r#powered: true,
                r#facing: Facing::Up,
            });
        }
        if state_id == 27432 {
            return Some(OxidizedLightningRod {
                r#facing: Facing::Up,
                r#powered: false,
                r#waterlogged: false,
            });
        }
        if state_id == 27415 {
            return Some(OxidizedLightningRod {
                r#powered: false,
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 27427 {
            return Some(OxidizedLightningRod {
                r#waterlogged: true,
                r#facing: Facing::West,
                r#powered: false,
            });
        }
        if state_id == 27434 {
            return Some(OxidizedLightningRod {
                r#facing: Facing::Down,
                r#powered: true,
                r#waterlogged: false,
            });
        }
        if state_id == 27435 {
            return Some(OxidizedLightningRod {
                r#powered: false,
                r#facing: Facing::Down,
                r#waterlogged: true,
            });
        }
        if state_id == 27436 {
            return Some(OxidizedLightningRod {
                r#waterlogged: false,
                r#facing: Facing::Down,
                r#powered: false,
            });
        }
        if state_id == 27430 {
            return Some(OxidizedLightningRod {
                r#facing: Facing::Up,
                r#powered: true,
                r#waterlogged: false,
            });
        }
        if state_id == 27433 {
            return Some(OxidizedLightningRod {
                r#waterlogged: true,
                r#facing: Facing::Down,
                r#powered: true,
            });
        }
        if state_id == 27422 {
            return Some(OxidizedLightningRod {
                r#waterlogged: false,
                r#facing: Facing::South,
                r#powered: true,
            });
        }
        return None;
    }
}

