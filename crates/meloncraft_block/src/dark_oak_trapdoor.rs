use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DarkOakTrapdoor {
    pub open: bool,
    pub powered: bool,
    pub waterlogged: bool,
    pub r#facing: Facing,
    pub r#half: Half,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Half {
    Top,
    Bottom,
}

impl BlockState for DarkOakTrapdoor {
    fn to_id(&self) -> i32 {
        if self.r#facing == Facing::West && self.r#open == true && self.r#powered == false && self.r#half == Half::Top && self.r#waterlogged == false { return 7332; }
        if self.r#powered == false && self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#open == true && self.r#facing == Facing::East { return 7355; }
        if self.r#half == Half::Top && self.r#open == true && self.r#facing == Facing::East && self.r#powered == false && self.r#waterlogged == false { return 7348; }
        if self.r#half == Half::Bottom && self.r#open == true && self.r#powered == true && self.r#waterlogged == false && self.r#facing == Facing::South { return 7322; }
        if self.r#open == false && self.r#waterlogged == true && self.r#facing == Facing::North && self.r#powered == false && self.r#half == Half::Top { return 7303; }
        if self.r#powered == true && self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#facing == Facing::East && self.r#open == false { return 7357; }
        if self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#open == false && self.r#facing == Facing::North && self.r#powered == false { return 7312; }
        if self.r#powered == true && self.r#half == Half::Top && self.r#facing == Facing::East && self.r#open == true && self.r#waterlogged == true { return 7345; }
        if self.r#open == true && self.r#facing == Facing::South && self.r#powered == false && self.r#waterlogged == true && self.r#half == Half::Bottom { return 7323; }
        if self.r#facing == Facing::West && self.r#powered == true && self.r#open == true && self.r#waterlogged == false && self.r#half == Half::Bottom { return 7338; }
        if self.r#waterlogged == true && self.r#open == false && self.r#half == Half::Bottom && self.r#powered == false && self.r#facing == Facing::South { return 7327; }
        if self.r#facing == Facing::East && self.r#half == Half::Bottom && self.r#open == false && self.r#waterlogged == true && self.r#powered == false { return 7359; }
        if self.r#powered == false && self.r#open == true && self.r#facing == Facing::North && self.r#waterlogged == true && self.r#half == Half::Bottom { return 7307; }
        if self.r#open == true && self.r#powered == false && self.r#waterlogged == false && self.r#facing == Facing::South && self.r#half == Half::Top { return 7316; }
        if self.r#powered == true && self.r#waterlogged == true && self.r#open == true && self.r#facing == Facing::North && self.r#half == Half::Bottom { return 7305; }
        if self.r#half == Half::Bottom && self.r#waterlogged == true && self.r#powered == true && self.r#facing == Facing::South && self.r#open == true { return 7321; }
        if self.r#waterlogged == true && self.r#powered == true && self.r#half == Half::Bottom && self.r#open == false && self.r#facing == Facing::South { return 7325; }
        if self.r#open == false && self.r#powered == true && self.r#half == Half::Top && self.r#waterlogged == true && self.r#facing == Facing::South { return 7317; }
        if self.r#waterlogged == true && self.r#facing == Facing::North && self.r#open == true && self.r#half == Half::Top && self.r#powered == true { return 7297; }
        if self.r#waterlogged == true && self.r#powered == false && self.r#facing == Facing::East && self.r#open == false && self.r#half == Half::Top { return 7351; }
        if self.r#waterlogged == false && self.r#powered == true && self.r#facing == Facing::West && self.r#open == true && self.r#half == Half::Top { return 7330; }
        if self.r#facing == Facing::North && self.r#open == false && self.r#half == Half::Top && self.r#waterlogged == false && self.r#powered == true { return 7302; }
        if self.r#facing == Facing::West && self.r#half == Half::Top && self.r#powered == true && self.r#waterlogged == true && self.r#open == true { return 7329; }
        if self.r#half == Half::Top && self.r#waterlogged == false && self.r#open == true && self.r#powered == false && self.r#facing == Facing::North { return 7300; }
        if self.r#open == true && self.r#half == Half::Bottom && self.r#powered == true && self.r#facing == Facing::East && self.r#waterlogged == false { return 7354; }
        if self.r#facing == Facing::East && self.r#half == Half::Top && self.r#powered == true && self.r#open == true && self.r#waterlogged == false { return 7346; }
        if self.r#half == Half::Bottom && self.r#powered == true && self.r#open == false && self.r#facing == Facing::East && self.r#waterlogged == false { return 7358; }
        if self.r#half == Half::Top && self.r#waterlogged == true && self.r#facing == Facing::West && self.r#open == false && self.r#powered == false { return 7335; }
        if self.r#facing == Facing::East && self.r#powered == true && self.r#half == Half::Top && self.r#open == false && self.r#waterlogged == false { return 7350; }
        if self.r#open == false && self.r#facing == Facing::North && self.r#waterlogged == true && self.r#powered == false && self.r#half == Half::Bottom { return 7311; }
        if self.r#powered == false && self.r#facing == Facing::East && self.r#waterlogged == false && self.r#half == Half::Top && self.r#open == false { return 7352; }
        if self.r#waterlogged == false && self.r#facing == Facing::North && self.r#open == true && self.r#powered == false && self.r#half == Half::Bottom { return 7308; }
        if self.r#waterlogged == false && self.r#open == false && self.r#facing == Facing::North && self.r#powered == false && self.r#half == Half::Top { return 7304; }
        if self.r#facing == Facing::South && self.r#half == Half::Top && self.r#powered == false && self.r#open == false && self.r#waterlogged == false { return 7320; }
        if self.r#half == Half::Top && self.r#waterlogged == true && self.r#powered == true && self.r#open == false && self.r#facing == Facing::East { return 7349; }
        if self.r#powered == false && self.r#waterlogged == true && self.r#facing == Facing::South && self.r#open == false && self.r#half == Half::Top { return 7319; }
        if self.r#facing == Facing::South && self.r#open == true && self.r#waterlogged == true && self.r#half == Half::Top && self.r#powered == true { return 7313; }
        if self.r#waterlogged == false && self.r#open == false && self.r#half == Half::Bottom && self.r#facing == Facing::North && self.r#powered == true { return 7310; }
        if self.r#open == false && self.r#facing == Facing::South && self.r#powered == false && self.r#half == Half::Bottom && self.r#waterlogged == false { return 7328; }
        if self.r#open == false && self.r#waterlogged == false && self.r#facing == Facing::South && self.r#half == Half::Top && self.r#powered == true { return 7318; }
        if self.r#facing == Facing::West && self.r#half == Half::Bottom && self.r#open == true && self.r#powered == true && self.r#waterlogged == true { return 7337; }
        if self.r#open == false && self.r#facing == Facing::West && self.r#waterlogged == false && self.r#powered == false && self.r#half == Half::Top { return 7336; }
        if self.r#powered == true && self.r#open == true && self.r#facing == Facing::South && self.r#waterlogged == false && self.r#half == Half::Top { return 7314; }
        if self.r#half == Half::Bottom && self.r#powered == false && self.r#waterlogged == false && self.r#open == false && self.r#facing == Facing::East { return 7360; }
        if self.r#facing == Facing::South && self.r#waterlogged == false && self.r#half == Half::Bottom && self.r#open == true && self.r#powered == false { return 7324; }
        if self.r#facing == Facing::North && self.r#half == Half::Top && self.r#open == false && self.r#powered == true && self.r#waterlogged == true { return 7301; }
        if self.r#facing == Facing::North && self.r#half == Half::Bottom && self.r#open == true && self.r#powered == true && self.r#waterlogged == false { return 7306; }
        if self.r#waterlogged == true && self.r#half == Half::Top && self.r#powered == false && self.r#facing == Facing::North && self.r#open == true { return 7299; }
        if self.r#waterlogged == true && self.r#facing == Facing::West && self.r#open == true && self.r#half == Half::Top && self.r#powered == false { return 7331; }
        if self.r#powered == false && self.r#facing == Facing::East && self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#open == true { return 7356; }
        if self.r#powered == true && self.r#half == Half::Bottom && self.r#open == false && self.r#waterlogged == true && self.r#facing == Facing::North { return 7309; }
        if self.r#open == true && self.r#facing == Facing::North && self.r#powered == true && self.r#waterlogged == false && self.r#half == Half::Top { return 7298; }
        if self.r#powered == false && self.r#half == Half::Top && self.r#waterlogged == true && self.r#open == true && self.r#facing == Facing::South { return 7315; }
        if self.r#half == Half::Bottom && self.r#waterlogged == true && self.r#powered == false && self.r#facing == Facing::West && self.r#open == true { return 7339; }
        if self.r#open == true && self.r#facing == Facing::East && self.r#half == Half::Bottom && self.r#powered == true && self.r#waterlogged == true { return 7353; }
        if self.r#waterlogged == false && self.r#open == false && self.r#facing == Facing::South && self.r#half == Half::Bottom && self.r#powered == true { return 7326; }
        if self.r#facing == Facing::West && self.r#open == false && self.r#waterlogged == false && self.r#half == Half::Top && self.r#powered == true { return 7334; }
        if self.r#waterlogged == true && self.r#open == false && self.r#half == Half::Bottom && self.r#facing == Facing::West && self.r#powered == true { return 7341; }
        if self.r#powered == false && self.r#waterlogged == true && self.r#facing == Facing::East && self.r#half == Half::Top && self.r#open == true { return 7347; }
        if self.r#open == false && self.r#facing == Facing::West && self.r#half == Half::Bottom && self.r#waterlogged == true && self.r#powered == false { return 7343; }
        if self.r#facing == Facing::West && self.r#open == true && self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#powered == false { return 7340; }
        if self.r#open == false && self.r#powered == true && self.r#waterlogged == false && self.r#facing == Facing::West && self.r#half == Half::Bottom { return 7342; }
        if self.r#open == false && self.r#half == Half::Bottom && self.r#facing == Facing::West && self.r#powered == false && self.r#waterlogged == false { return 7344; }
        if self.r#facing == Facing::West && self.r#powered == true && self.r#waterlogged == true && self.r#open == false && self.r#half == Half::Top { return 7333; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 7332 {
            return Some(DarkOakTrapdoor {
                r#facing: Facing::West,
                r#open: true,
                r#powered: false,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 7355 {
            return Some(DarkOakTrapdoor {
                r#powered: false,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#open: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 7348 {
            return Some(DarkOakTrapdoor {
                r#half: Half::Top,
                r#open: true,
                r#facing: Facing::East,
                r#powered: false,
                r#waterlogged: false,
            });
        }
        if state_id == 7322 {
            return Some(DarkOakTrapdoor {
                r#half: Half::Bottom,
                r#open: true,
                r#powered: true,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 7303 {
            return Some(DarkOakTrapdoor {
                r#open: false,
                r#waterlogged: true,
                r#facing: Facing::North,
                r#powered: false,
                r#half: Half::Top,
            });
        }
        if state_id == 7357 {
            return Some(DarkOakTrapdoor {
                r#powered: true,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#open: false,
            });
        }
        if state_id == 7312 {
            return Some(DarkOakTrapdoor {
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#open: false,
                r#facing: Facing::North,
                r#powered: false,
            });
        }
        if state_id == 7345 {
            return Some(DarkOakTrapdoor {
                r#powered: true,
                r#half: Half::Top,
                r#facing: Facing::East,
                r#open: true,
                r#waterlogged: true,
            });
        }
        if state_id == 7323 {
            return Some(DarkOakTrapdoor {
                r#open: true,
                r#facing: Facing::South,
                r#powered: false,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 7338 {
            return Some(DarkOakTrapdoor {
                r#facing: Facing::West,
                r#powered: true,
                r#open: true,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 7327 {
            return Some(DarkOakTrapdoor {
                r#waterlogged: true,
                r#open: false,
                r#half: Half::Bottom,
                r#powered: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 7359 {
            return Some(DarkOakTrapdoor {
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#open: false,
                r#waterlogged: true,
                r#powered: false,
            });
        }
        if state_id == 7307 {
            return Some(DarkOakTrapdoor {
                r#powered: false,
                r#open: true,
                r#facing: Facing::North,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 7316 {
            return Some(DarkOakTrapdoor {
                r#open: true,
                r#powered: false,
                r#waterlogged: false,
                r#facing: Facing::South,
                r#half: Half::Top,
            });
        }
        if state_id == 7305 {
            return Some(DarkOakTrapdoor {
                r#powered: true,
                r#waterlogged: true,
                r#open: true,
                r#facing: Facing::North,
                r#half: Half::Bottom,
            });
        }
        if state_id == 7321 {
            return Some(DarkOakTrapdoor {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#powered: true,
                r#facing: Facing::South,
                r#open: true,
            });
        }
        if state_id == 7325 {
            return Some(DarkOakTrapdoor {
                r#waterlogged: true,
                r#powered: true,
                r#half: Half::Bottom,
                r#open: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 7317 {
            return Some(DarkOakTrapdoor {
                r#open: false,
                r#powered: true,
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 7297 {
            return Some(DarkOakTrapdoor {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#open: true,
                r#half: Half::Top,
                r#powered: true,
            });
        }
        if state_id == 7351 {
            return Some(DarkOakTrapdoor {
                r#waterlogged: true,
                r#powered: false,
                r#facing: Facing::East,
                r#open: false,
                r#half: Half::Top,
            });
        }
        if state_id == 7330 {
            return Some(DarkOakTrapdoor {
                r#waterlogged: false,
                r#powered: true,
                r#facing: Facing::West,
                r#open: true,
                r#half: Half::Top,
            });
        }
        if state_id == 7302 {
            return Some(DarkOakTrapdoor {
                r#facing: Facing::North,
                r#open: false,
                r#half: Half::Top,
                r#waterlogged: false,
                r#powered: true,
            });
        }
        if state_id == 7329 {
            return Some(DarkOakTrapdoor {
                r#facing: Facing::West,
                r#half: Half::Top,
                r#powered: true,
                r#waterlogged: true,
                r#open: true,
            });
        }
        if state_id == 7300 {
            return Some(DarkOakTrapdoor {
                r#half: Half::Top,
                r#waterlogged: false,
                r#open: true,
                r#powered: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 7354 {
            return Some(DarkOakTrapdoor {
                r#open: true,
                r#half: Half::Bottom,
                r#powered: true,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 7346 {
            return Some(DarkOakTrapdoor {
                r#facing: Facing::East,
                r#half: Half::Top,
                r#powered: true,
                r#open: true,
                r#waterlogged: false,
            });
        }
        if state_id == 7358 {
            return Some(DarkOakTrapdoor {
                r#half: Half::Bottom,
                r#powered: true,
                r#open: false,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 7335 {
            return Some(DarkOakTrapdoor {
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 7350 {
            return Some(DarkOakTrapdoor {
                r#facing: Facing::East,
                r#powered: true,
                r#half: Half::Top,
                r#open: false,
                r#waterlogged: false,
            });
        }
        if state_id == 7311 {
            return Some(DarkOakTrapdoor {
                r#open: false,
                r#facing: Facing::North,
                r#waterlogged: true,
                r#powered: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 7352 {
            return Some(DarkOakTrapdoor {
                r#powered: false,
                r#facing: Facing::East,
                r#waterlogged: false,
                r#half: Half::Top,
                r#open: false,
            });
        }
        if state_id == 7308 {
            return Some(DarkOakTrapdoor {
                r#waterlogged: false,
                r#facing: Facing::North,
                r#open: true,
                r#powered: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 7304 {
            return Some(DarkOakTrapdoor {
                r#waterlogged: false,
                r#open: false,
                r#facing: Facing::North,
                r#powered: false,
                r#half: Half::Top,
            });
        }
        if state_id == 7320 {
            return Some(DarkOakTrapdoor {
                r#facing: Facing::South,
                r#half: Half::Top,
                r#powered: false,
                r#open: false,
                r#waterlogged: false,
            });
        }
        if state_id == 7349 {
            return Some(DarkOakTrapdoor {
                r#half: Half::Top,
                r#waterlogged: true,
                r#powered: true,
                r#open: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 7319 {
            return Some(DarkOakTrapdoor {
                r#powered: false,
                r#waterlogged: true,
                r#facing: Facing::South,
                r#open: false,
                r#half: Half::Top,
            });
        }
        if state_id == 7313 {
            return Some(DarkOakTrapdoor {
                r#facing: Facing::South,
                r#open: true,
                r#waterlogged: true,
                r#half: Half::Top,
                r#powered: true,
            });
        }
        if state_id == 7310 {
            return Some(DarkOakTrapdoor {
                r#waterlogged: false,
                r#open: false,
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#powered: true,
            });
        }
        if state_id == 7328 {
            return Some(DarkOakTrapdoor {
                r#open: false,
                r#facing: Facing::South,
                r#powered: false,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 7318 {
            return Some(DarkOakTrapdoor {
                r#open: false,
                r#waterlogged: false,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#powered: true,
            });
        }
        if state_id == 7337 {
            return Some(DarkOakTrapdoor {
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#open: true,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 7336 {
            return Some(DarkOakTrapdoor {
                r#open: false,
                r#facing: Facing::West,
                r#waterlogged: false,
                r#powered: false,
                r#half: Half::Top,
            });
        }
        if state_id == 7314 {
            return Some(DarkOakTrapdoor {
                r#powered: true,
                r#open: true,
                r#facing: Facing::South,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 7360 {
            return Some(DarkOakTrapdoor {
                r#half: Half::Bottom,
                r#powered: false,
                r#waterlogged: false,
                r#open: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 7324 {
            return Some(DarkOakTrapdoor {
                r#facing: Facing::South,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#open: true,
                r#powered: false,
            });
        }
        if state_id == 7301 {
            return Some(DarkOakTrapdoor {
                r#facing: Facing::North,
                r#half: Half::Top,
                r#open: false,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 7306 {
            return Some(DarkOakTrapdoor {
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#open: true,
                r#powered: true,
                r#waterlogged: false,
            });
        }
        if state_id == 7299 {
            return Some(DarkOakTrapdoor {
                r#waterlogged: true,
                r#half: Half::Top,
                r#powered: false,
                r#facing: Facing::North,
                r#open: true,
            });
        }
        if state_id == 7331 {
            return Some(DarkOakTrapdoor {
                r#waterlogged: true,
                r#facing: Facing::West,
                r#open: true,
                r#half: Half::Top,
                r#powered: false,
            });
        }
        if state_id == 7356 {
            return Some(DarkOakTrapdoor {
                r#powered: false,
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#open: true,
            });
        }
        if state_id == 7309 {
            return Some(DarkOakTrapdoor {
                r#powered: true,
                r#half: Half::Bottom,
                r#open: false,
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 7298 {
            return Some(DarkOakTrapdoor {
                r#open: true,
                r#facing: Facing::North,
                r#powered: true,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 7315 {
            return Some(DarkOakTrapdoor {
                r#powered: false,
                r#half: Half::Top,
                r#waterlogged: true,
                r#open: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 7339 {
            return Some(DarkOakTrapdoor {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#powered: false,
                r#facing: Facing::West,
                r#open: true,
            });
        }
        if state_id == 7353 {
            return Some(DarkOakTrapdoor {
                r#open: true,
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 7326 {
            return Some(DarkOakTrapdoor {
                r#waterlogged: false,
                r#open: false,
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#powered: true,
            });
        }
        if state_id == 7334 {
            return Some(DarkOakTrapdoor {
                r#facing: Facing::West,
                r#open: false,
                r#waterlogged: false,
                r#half: Half::Top,
                r#powered: true,
            });
        }
        if state_id == 7341 {
            return Some(DarkOakTrapdoor {
                r#waterlogged: true,
                r#open: false,
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#powered: true,
            });
        }
        if state_id == 7347 {
            return Some(DarkOakTrapdoor {
                r#powered: false,
                r#waterlogged: true,
                r#facing: Facing::East,
                r#half: Half::Top,
                r#open: true,
            });
        }
        if state_id == 7343 {
            return Some(DarkOakTrapdoor {
                r#open: false,
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#powered: false,
            });
        }
        if state_id == 7340 {
            return Some(DarkOakTrapdoor {
                r#facing: Facing::West,
                r#open: true,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#powered: false,
            });
        }
        if state_id == 7342 {
            return Some(DarkOakTrapdoor {
                r#open: false,
                r#powered: true,
                r#waterlogged: false,
                r#facing: Facing::West,
                r#half: Half::Bottom,
            });
        }
        if state_id == 7344 {
            return Some(DarkOakTrapdoor {
                r#open: false,
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#powered: false,
                r#waterlogged: false,
            });
        }
        if state_id == 7333 {
            return Some(DarkOakTrapdoor {
                r#facing: Facing::West,
                r#powered: true,
                r#waterlogged: true,
                r#open: false,
                r#half: Half::Top,
            });
        }
        return None;
    }
}

