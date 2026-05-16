use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PaleOakTrapdoor {
    pub r#half: Half,
    pub r#facing: Facing,
    pub powered: bool,
    pub waterlogged: bool,
    pub open: bool,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Half {
    Top,
    Bottom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for PaleOakTrapdoor {
    fn to_id(&self) -> i32 {
        if self.r#half == Half::Bottom && self.r#powered == false && self.r#waterlogged == true && self.r#facing == Facing::North && self.r#open == false { return 7375; }
        if self.r#open == false && self.r#half == Half::Top && self.r#powered == false && self.r#facing == Facing::West && self.r#waterlogged == false { return 7400; }
        if self.r#half == Half::Top && self.r#waterlogged == false && self.r#facing == Facing::East && self.r#open == false && self.r#powered == false { return 7416; }
        if self.r#facing == Facing::North && self.r#half == Half::Top && self.r#open == false && self.r#powered == false && self.r#waterlogged == true { return 7367; }
        if self.r#half == Half::Top && self.r#powered == false && self.r#waterlogged == false && self.r#facing == Facing::South && self.r#open == false { return 7384; }
        if self.r#powered == true && self.r#waterlogged == true && self.r#facing == Facing::South && self.r#half == Half::Top && self.r#open == true { return 7377; }
        if self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#facing == Facing::North && self.r#open == true && self.r#powered == false { return 7371; }
        if self.r#facing == Facing::South && self.r#open == false && self.r#half == Half::Bottom && self.r#powered == false && self.r#waterlogged == false { return 7392; }
        if self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#open == false && self.r#facing == Facing::West && self.r#powered == true { return 7405; }
        if self.r#facing == Facing::East && self.r#open == false && self.r#powered == true && self.r#waterlogged == false && self.r#half == Half::Top { return 7414; }
        if self.r#half == Half::Top && self.r#facing == Facing::West && self.r#powered == false && self.r#open == true && self.r#waterlogged == false { return 7396; }
        if self.r#waterlogged == true && self.r#facing == Facing::West && self.r#open == true && self.r#powered == true && self.r#half == Half::Top { return 7393; }
        if self.r#half == Half::Top && self.r#facing == Facing::East && self.r#powered == false && self.r#waterlogged == false && self.r#open == true { return 7412; }
        if self.r#open == true && self.r#powered == false && self.r#half == Half::Top && self.r#facing == Facing::East && self.r#waterlogged == true { return 7411; }
        if self.r#facing == Facing::East && self.r#waterlogged == true && self.r#open == false && self.r#half == Half::Bottom && self.r#powered == false { return 7423; }
        if self.r#facing == Facing::East && self.r#half == Half::Top && self.r#waterlogged == true && self.r#open == false && self.r#powered == true { return 7413; }
        if self.r#waterlogged == true && self.r#facing == Facing::North && self.r#powered == false && self.r#open == true && self.r#half == Half::Top { return 7363; }
        if self.r#facing == Facing::North && self.r#waterlogged == false && self.r#open == true && self.r#powered == false && self.r#half == Half::Bottom { return 7372; }
        if self.r#half == Half::Top && self.r#powered == false && self.r#facing == Facing::South && self.r#open == false && self.r#waterlogged == true { return 7383; }
        if self.r#waterlogged == true && self.r#open == true && self.r#facing == Facing::South && self.r#half == Half::Top && self.r#powered == false { return 7379; }
        if self.r#waterlogged == false && self.r#half == Half::Bottom && self.r#facing == Facing::North && self.r#open == true && self.r#powered == true { return 7370; }
        if self.r#facing == Facing::West && self.r#half == Half::Bottom && self.r#powered == false && self.r#waterlogged == true && self.r#open == false { return 7407; }
        if self.r#waterlogged == true && self.r#powered == true && self.r#open == true && self.r#facing == Facing::East && self.r#half == Half::Top { return 7409; }
        if self.r#waterlogged == false && self.r#open == true && self.r#half == Half::Bottom && self.r#powered == false && self.r#facing == Facing::South { return 7388; }
        if self.r#half == Half::Top && self.r#open == true && self.r#powered == true && self.r#facing == Facing::South && self.r#waterlogged == false { return 7378; }
        if self.r#facing == Facing::East && self.r#waterlogged == false && self.r#half == Half::Bottom && self.r#powered == false && self.r#open == true { return 7420; }
        if self.r#facing == Facing::West && self.r#half == Half::Top && self.r#powered == false && self.r#open == false && self.r#waterlogged == true { return 7399; }
        if self.r#facing == Facing::North && self.r#powered == true && self.r#open == false && self.r#half == Half::Top && self.r#waterlogged == true { return 7365; }
        if self.r#waterlogged == false && self.r#half == Half::Top && self.r#open == true && self.r#powered == false && self.r#facing == Facing::North { return 7364; }
        if self.r#facing == Facing::South && self.r#open == false && self.r#powered == true && self.r#half == Half::Top && self.r#waterlogged == true { return 7381; }
        if self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#open == false && self.r#powered == true && self.r#facing == Facing::South { return 7390; }
        if self.r#facing == Facing::West && self.r#waterlogged == false && self.r#powered == false && self.r#half == Half::Bottom && self.r#open == false { return 7408; }
        if self.r#waterlogged == false && self.r#powered == true && self.r#half == Half::Top && self.r#open == false && self.r#facing == Facing::West { return 7398; }
        if self.r#facing == Facing::East && self.r#open == false && self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#powered == false { return 7424; }
        if self.r#open == true && self.r#waterlogged == true && self.r#powered == true && self.r#facing == Facing::South && self.r#half == Half::Bottom { return 7385; }
        if self.r#powered == false && self.r#waterlogged == false && self.r#facing == Facing::North && self.r#half == Half::Top && self.r#open == false { return 7368; }
        if self.r#open == false && self.r#half == Half::Bottom && self.r#facing == Facing::South && self.r#powered == true && self.r#waterlogged == true { return 7389; }
        if self.r#facing == Facing::West && self.r#half == Half::Top && self.r#waterlogged == false && self.r#powered == true && self.r#open == true { return 7394; }
        if self.r#powered == false && self.r#waterlogged == true && self.r#half == Half::Top && self.r#open == true && self.r#facing == Facing::West { return 7395; }
        if self.r#powered == false && self.r#facing == Facing::South && self.r#half == Half::Bottom && self.r#waterlogged == true && self.r#open == true { return 7387; }
        if self.r#powered == true && self.r#waterlogged == false && self.r#facing == Facing::South && self.r#half == Half::Top && self.r#open == false { return 7382; }
        if self.r#powered == false && self.r#open == false && self.r#waterlogged == false && self.r#half == Half::Bottom && self.r#facing == Facing::North { return 7376; }
        if self.r#facing == Facing::East && self.r#powered == false && self.r#waterlogged == true && self.r#open == false && self.r#half == Half::Top { return 7415; }
        if self.r#half == Half::Bottom && self.r#open == true && self.r#facing == Facing::East && self.r#powered == false && self.r#waterlogged == true { return 7419; }
        if self.r#facing == Facing::North && self.r#powered == true && self.r#half == Half::Top && self.r#open == true && self.r#waterlogged == false { return 7362; }
        if self.r#facing == Facing::North && self.r#powered == true && self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#open == false { return 7373; }
        if self.r#facing == Facing::North && self.r#waterlogged == true && self.r#half == Half::Top && self.r#open == true && self.r#powered == true { return 7361; }
        if self.r#powered == false && self.r#open == true && self.r#facing == Facing::West && self.r#half == Half::Bottom && self.r#waterlogged == true { return 7403; }
        if self.r#open == true && self.r#waterlogged == false && self.r#half == Half::Bottom && self.r#powered == true && self.r#facing == Facing::East { return 7418; }
        if self.r#half == Half::Bottom && self.r#open == false && self.r#facing == Facing::South && self.r#powered == false && self.r#waterlogged == true { return 7391; }
        if self.r#facing == Facing::East && self.r#powered == true && self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#open == true { return 7417; }
        if self.r#powered == true && self.r#open == false && self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#facing == Facing::West { return 7406; }
        if self.r#half == Half::Bottom && self.r#powered == true && self.r#open == false && self.r#waterlogged == true && self.r#facing == Facing::East { return 7421; }
        if self.r#half == Half::Top && self.r#open == true && self.r#powered == false && self.r#facing == Facing::South && self.r#waterlogged == false { return 7380; }
        if self.r#facing == Facing::East && self.r#half == Half::Bottom && self.r#open == false && self.r#powered == true && self.r#waterlogged == false { return 7422; }
        if self.r#facing == Facing::North && self.r#open == false && self.r#powered == true && self.r#waterlogged == false && self.r#half == Half::Top { return 7366; }
        if self.r#powered == false && self.r#waterlogged == false && self.r#facing == Facing::West && self.r#half == Half::Bottom && self.r#open == true { return 7404; }
        if self.r#open == true && self.r#powered == true && self.r#half == Half::Bottom && self.r#facing == Facing::West && self.r#waterlogged == true { return 7401; }
        if self.r#open == false && self.r#waterlogged == false && self.r#powered == true && self.r#facing == Facing::North && self.r#half == Half::Bottom { return 7374; }
        if self.r#waterlogged == true && self.r#facing == Facing::West && self.r#half == Half::Top && self.r#open == false && self.r#powered == true { return 7397; }
        if self.r#open == true && self.r#waterlogged == false && self.r#powered == true && self.r#facing == Facing::East && self.r#half == Half::Top { return 7410; }
        if self.r#facing == Facing::North && self.r#half == Half::Bottom && self.r#open == true && self.r#waterlogged == true && self.r#powered == true { return 7369; }
        if self.r#open == true && self.r#half == Half::Bottom && self.r#powered == true && self.r#waterlogged == false && self.r#facing == Facing::West { return 7402; }
        if self.r#waterlogged == false && self.r#half == Half::Bottom && self.r#powered == true && self.r#facing == Facing::South && self.r#open == true { return 7386; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 7375 {
            return Some(PaleOakTrapdoor {
                r#half: Half::Bottom,
                r#powered: false,
                r#waterlogged: true,
                r#facing: Facing::North,
                r#open: false,
            });
        }
        if state_id == 7400 {
            return Some(PaleOakTrapdoor {
                r#open: false,
                r#half: Half::Top,
                r#powered: false,
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 7416 {
            return Some(PaleOakTrapdoor {
                r#half: Half::Top,
                r#waterlogged: false,
                r#facing: Facing::East,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 7367 {
            return Some(PaleOakTrapdoor {
                r#facing: Facing::North,
                r#half: Half::Top,
                r#open: false,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 7384 {
            return Some(PaleOakTrapdoor {
                r#half: Half::Top,
                r#powered: false,
                r#waterlogged: false,
                r#facing: Facing::South,
                r#open: false,
            });
        }
        if state_id == 7377 {
            return Some(PaleOakTrapdoor {
                r#powered: true,
                r#waterlogged: true,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#open: true,
            });
        }
        if state_id == 7371 {
            return Some(PaleOakTrapdoor {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#open: true,
                r#powered: false,
            });
        }
        if state_id == 7392 {
            return Some(PaleOakTrapdoor {
                r#facing: Facing::South,
                r#open: false,
                r#half: Half::Bottom,
                r#powered: false,
                r#waterlogged: false,
            });
        }
        if state_id == 7405 {
            return Some(PaleOakTrapdoor {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#open: false,
                r#facing: Facing::West,
                r#powered: true,
            });
        }
        if state_id == 7414 {
            return Some(PaleOakTrapdoor {
                r#facing: Facing::East,
                r#open: false,
                r#powered: true,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 7396 {
            return Some(PaleOakTrapdoor {
                r#half: Half::Top,
                r#facing: Facing::West,
                r#powered: false,
                r#open: true,
                r#waterlogged: false,
            });
        }
        if state_id == 7393 {
            return Some(PaleOakTrapdoor {
                r#waterlogged: true,
                r#facing: Facing::West,
                r#open: true,
                r#powered: true,
                r#half: Half::Top,
            });
        }
        if state_id == 7412 {
            return Some(PaleOakTrapdoor {
                r#half: Half::Top,
                r#facing: Facing::East,
                r#powered: false,
                r#waterlogged: false,
                r#open: true,
            });
        }
        if state_id == 7411 {
            return Some(PaleOakTrapdoor {
                r#open: true,
                r#powered: false,
                r#half: Half::Top,
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 7423 {
            return Some(PaleOakTrapdoor {
                r#facing: Facing::East,
                r#waterlogged: true,
                r#open: false,
                r#half: Half::Bottom,
                r#powered: false,
            });
        }
        if state_id == 7413 {
            return Some(PaleOakTrapdoor {
                r#facing: Facing::East,
                r#half: Half::Top,
                r#waterlogged: true,
                r#open: false,
                r#powered: true,
            });
        }
        if state_id == 7363 {
            return Some(PaleOakTrapdoor {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#powered: false,
                r#open: true,
                r#half: Half::Top,
            });
        }
        if state_id == 7372 {
            return Some(PaleOakTrapdoor {
                r#facing: Facing::North,
                r#waterlogged: false,
                r#open: true,
                r#powered: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 7383 {
            return Some(PaleOakTrapdoor {
                r#half: Half::Top,
                r#powered: false,
                r#facing: Facing::South,
                r#open: false,
                r#waterlogged: true,
            });
        }
        if state_id == 7379 {
            return Some(PaleOakTrapdoor {
                r#waterlogged: true,
                r#open: true,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#powered: false,
            });
        }
        if state_id == 7370 {
            return Some(PaleOakTrapdoor {
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 7407 {
            return Some(PaleOakTrapdoor {
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#powered: false,
                r#waterlogged: true,
                r#open: false,
            });
        }
        if state_id == 7409 {
            return Some(PaleOakTrapdoor {
                r#waterlogged: true,
                r#powered: true,
                r#open: true,
                r#facing: Facing::East,
                r#half: Half::Top,
            });
        }
        if state_id == 7388 {
            return Some(PaleOakTrapdoor {
                r#waterlogged: false,
                r#open: true,
                r#half: Half::Bottom,
                r#powered: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 7378 {
            return Some(PaleOakTrapdoor {
                r#half: Half::Top,
                r#open: true,
                r#powered: true,
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 7420 {
            return Some(PaleOakTrapdoor {
                r#facing: Facing::East,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#powered: false,
                r#open: true,
            });
        }
        if state_id == 7399 {
            return Some(PaleOakTrapdoor {
                r#facing: Facing::West,
                r#half: Half::Top,
                r#powered: false,
                r#open: false,
                r#waterlogged: true,
            });
        }
        if state_id == 7365 {
            return Some(PaleOakTrapdoor {
                r#facing: Facing::North,
                r#powered: true,
                r#open: false,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 7364 {
            return Some(PaleOakTrapdoor {
                r#waterlogged: false,
                r#half: Half::Top,
                r#open: true,
                r#powered: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 7381 {
            return Some(PaleOakTrapdoor {
                r#facing: Facing::South,
                r#open: false,
                r#powered: true,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 7390 {
            return Some(PaleOakTrapdoor {
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#open: false,
                r#powered: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 7408 {
            return Some(PaleOakTrapdoor {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#powered: false,
                r#half: Half::Bottom,
                r#open: false,
            });
        }
        if state_id == 7398 {
            return Some(PaleOakTrapdoor {
                r#waterlogged: false,
                r#powered: true,
                r#half: Half::Top,
                r#open: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 7424 {
            return Some(PaleOakTrapdoor {
                r#facing: Facing::East,
                r#open: false,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#powered: false,
            });
        }
        if state_id == 7385 {
            return Some(PaleOakTrapdoor {
                r#open: true,
                r#waterlogged: true,
                r#powered: true,
                r#facing: Facing::South,
                r#half: Half::Bottom,
            });
        }
        if state_id == 7368 {
            return Some(PaleOakTrapdoor {
                r#powered: false,
                r#waterlogged: false,
                r#facing: Facing::North,
                r#half: Half::Top,
                r#open: false,
            });
        }
        if state_id == 7389 {
            return Some(PaleOakTrapdoor {
                r#open: false,
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 7394 {
            return Some(PaleOakTrapdoor {
                r#facing: Facing::West,
                r#half: Half::Top,
                r#waterlogged: false,
                r#powered: true,
                r#open: true,
            });
        }
        if state_id == 7395 {
            return Some(PaleOakTrapdoor {
                r#powered: false,
                r#waterlogged: true,
                r#half: Half::Top,
                r#open: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 7387 {
            return Some(PaleOakTrapdoor {
                r#powered: false,
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#open: true,
            });
        }
        if state_id == 7382 {
            return Some(PaleOakTrapdoor {
                r#powered: true,
                r#waterlogged: false,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#open: false,
            });
        }
        if state_id == 7376 {
            return Some(PaleOakTrapdoor {
                r#powered: false,
                r#open: false,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#facing: Facing::North,
            });
        }
        if state_id == 7415 {
            return Some(PaleOakTrapdoor {
                r#facing: Facing::East,
                r#powered: false,
                r#waterlogged: true,
                r#open: false,
                r#half: Half::Top,
            });
        }
        if state_id == 7419 {
            return Some(PaleOakTrapdoor {
                r#half: Half::Bottom,
                r#open: true,
                r#facing: Facing::East,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 7362 {
            return Some(PaleOakTrapdoor {
                r#facing: Facing::North,
                r#powered: true,
                r#half: Half::Top,
                r#open: true,
                r#waterlogged: false,
            });
        }
        if state_id == 7373 {
            return Some(PaleOakTrapdoor {
                r#facing: Facing::North,
                r#powered: true,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#open: false,
            });
        }
        if state_id == 7361 {
            return Some(PaleOakTrapdoor {
                r#facing: Facing::North,
                r#waterlogged: true,
                r#half: Half::Top,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 7403 {
            return Some(PaleOakTrapdoor {
                r#powered: false,
                r#open: true,
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 7418 {
            return Some(PaleOakTrapdoor {
                r#open: true,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#powered: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 7391 {
            return Some(PaleOakTrapdoor {
                r#half: Half::Bottom,
                r#open: false,
                r#facing: Facing::South,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 7417 {
            return Some(PaleOakTrapdoor {
                r#facing: Facing::East,
                r#powered: true,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#open: true,
            });
        }
        if state_id == 7406 {
            return Some(PaleOakTrapdoor {
                r#powered: true,
                r#open: false,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 7421 {
            return Some(PaleOakTrapdoor {
                r#half: Half::Bottom,
                r#powered: true,
                r#open: false,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 7380 {
            return Some(PaleOakTrapdoor {
                r#half: Half::Top,
                r#open: true,
                r#powered: false,
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 7422 {
            return Some(PaleOakTrapdoor {
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#open: false,
                r#powered: true,
                r#waterlogged: false,
            });
        }
        if state_id == 7366 {
            return Some(PaleOakTrapdoor {
                r#facing: Facing::North,
                r#open: false,
                r#powered: true,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 7404 {
            return Some(PaleOakTrapdoor {
                r#powered: false,
                r#waterlogged: false,
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#open: true,
            });
        }
        if state_id == 7401 {
            return Some(PaleOakTrapdoor {
                r#open: true,
                r#powered: true,
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 7374 {
            return Some(PaleOakTrapdoor {
                r#open: false,
                r#waterlogged: false,
                r#powered: true,
                r#facing: Facing::North,
                r#half: Half::Bottom,
            });
        }
        if state_id == 7397 {
            return Some(PaleOakTrapdoor {
                r#waterlogged: true,
                r#facing: Facing::West,
                r#half: Half::Top,
                r#open: false,
                r#powered: true,
            });
        }
        if state_id == 7410 {
            return Some(PaleOakTrapdoor {
                r#open: true,
                r#waterlogged: false,
                r#powered: true,
                r#facing: Facing::East,
                r#half: Half::Top,
            });
        }
        if state_id == 7369 {
            return Some(PaleOakTrapdoor {
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#open: true,
                r#waterlogged: true,
                r#powered: true,
            });
        }
        if state_id == 7402 {
            return Some(PaleOakTrapdoor {
                r#open: true,
                r#half: Half::Bottom,
                r#powered: true,
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 7386 {
            return Some(PaleOakTrapdoor {
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#powered: true,
                r#facing: Facing::South,
                r#open: true,
            });
        }
        return None;
    }
}

