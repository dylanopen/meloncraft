use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaxedWeatheredCopperDoor {
    pub r#half: Half,
    pub r#hinge: Hinge,
    pub open: bool,
    pub powered: bool,
    pub r#facing: Facing,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Half {
    Upper,
    Lower,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Hinge {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for WaxedWeatheredCopperDoor {
    fn to_id(&self) -> i32 {
        if self.r#half == Half::Upper && self.r#facing == Facing::West && self.r#hinge == Hinge::Right && self.r#powered == false && self.r#open == false { return 26308; }
        if self.r#hinge == Hinge::Left && self.r#facing == Facing::West && self.r#open == true && self.r#powered == false && self.r#half == Half::Upper { return 26302; }
        if self.r#hinge == Hinge::Left && self.r#facing == Facing::West && self.r#open == true && self.r#powered == true && self.r#half == Half::Lower { return 26309; }
        if self.r#half == Half::Upper && self.r#hinge == Hinge::Left && self.r#facing == Facing::East && self.r#open == false && self.r#powered == true { return 26319; }
        if self.r#facing == Facing::South && self.r#half == Half::Upper && self.r#hinge == Hinge::Right && self.r#open == false && self.r#powered == true { return 26291; }
        if self.r#facing == Facing::West && self.r#hinge == Hinge::Left && self.r#powered == true && self.r#half == Half::Upper && self.r#open == true { return 26301; }
        if self.r#half == Half::Upper && self.r#facing == Facing::South && self.r#open == true && self.r#hinge == Hinge::Right && self.r#powered == true { return 26289; }
        if self.r#open == false && self.r#hinge == Hinge::Right && self.r#powered == true && self.r#half == Half::Lower && self.r#facing == Facing::West { return 26315; }
        if self.r#half == Half::Upper && self.r#hinge == Hinge::Left && self.r#facing == Facing::West && self.r#open == false && self.r#powered == true { return 26303; }
        if self.r#powered == false && self.r#half == Half::Upper && self.r#open == false && self.r#hinge == Hinge::Left && self.r#facing == Facing::North { return 26272; }
        if self.r#facing == Facing::West && self.r#open == false && self.r#half == Half::Lower && self.r#hinge == Hinge::Left && self.r#powered == false { return 26312; }
        if self.r#hinge == Hinge::Right && self.r#facing == Facing::West && self.r#half == Half::Lower && self.r#open == true && self.r#powered == true { return 26313; }
        if self.r#hinge == Hinge::Right && self.r#powered == false && self.r#facing == Facing::North && self.r#half == Half::Lower && self.r#open == true { return 26282; }
        if self.r#hinge == Hinge::Right && self.r#open == true && self.r#half == Half::Lower && self.r#powered == true && self.r#facing == Facing::East { return 26329; }
        if self.r#open == false && self.r#hinge == Hinge::Right && self.r#half == Half::Lower && self.r#facing == Facing::North && self.r#powered == true { return 26283; }
        if self.r#facing == Facing::South && self.r#half == Half::Lower && self.r#hinge == Hinge::Right && self.r#open == true && self.r#powered == true { return 26297; }
        if self.r#facing == Facing::South && self.r#open == false && self.r#half == Half::Upper && self.r#powered == true && self.r#hinge == Hinge::Left { return 26287; }
        if self.r#powered == true && self.r#facing == Facing::West && self.r#hinge == Hinge::Left && self.r#open == false && self.r#half == Half::Lower { return 26311; }
        if self.r#facing == Facing::East && self.r#hinge == Hinge::Left && self.r#powered == false && self.r#half == Half::Upper && self.r#open == true { return 26318; }
        if self.r#facing == Facing::East && self.r#hinge == Hinge::Right && self.r#powered == false && self.r#half == Half::Upper && self.r#open == true { return 26322; }
        if self.r#open == false && self.r#powered == false && self.r#hinge == Hinge::Left && self.r#facing == Facing::East && self.r#half == Half::Lower { return 26328; }
        if self.r#open == false && self.r#powered == false && self.r#hinge == Hinge::Right && self.r#facing == Facing::East && self.r#half == Half::Lower { return 26332; }
        if self.r#open == true && self.r#hinge == Hinge::Right && self.r#powered == false && self.r#facing == Facing::South && self.r#half == Half::Upper { return 26290; }
        if self.r#half == Half::Lower && self.r#open == false && self.r#facing == Facing::South && self.r#powered == false && self.r#hinge == Hinge::Left { return 26296; }
        if self.r#half == Half::Upper && self.r#facing == Facing::North && self.r#hinge == Hinge::Right && self.r#open == false && self.r#powered == true { return 26275; }
        if self.r#half == Half::Lower && self.r#open == true && self.r#hinge == Hinge::Right && self.r#powered == true && self.r#facing == Facing::North { return 26281; }
        if self.r#facing == Facing::West && self.r#powered == true && self.r#half == Half::Upper && self.r#hinge == Hinge::Right && self.r#open == true { return 26305; }
        if self.r#facing == Facing::East && self.r#hinge == Hinge::Left && self.r#open == true && self.r#half == Half::Lower && self.r#powered == true { return 26325; }
        if self.r#facing == Facing::East && self.r#hinge == Hinge::Left && self.r#open == true && self.r#half == Half::Lower && self.r#powered == false { return 26326; }
        if self.r#half == Half::Lower && self.r#facing == Facing::East && self.r#powered == false && self.r#open == true && self.r#hinge == Hinge::Right { return 26330; }
        if self.r#open == true && self.r#facing == Facing::South && self.r#powered == true && self.r#half == Half::Lower && self.r#hinge == Hinge::Left { return 26293; }
        if self.r#open == false && self.r#powered == true && self.r#hinge == Hinge::Right && self.r#facing == Facing::West && self.r#half == Half::Upper { return 26307; }
        if self.r#hinge == Hinge::Right && self.r#facing == Facing::South && self.r#open == false && self.r#powered == true && self.r#half == Half::Lower { return 26299; }
        if self.r#half == Half::Lower && self.r#open == true && self.r#facing == Facing::North && self.r#hinge == Hinge::Left && self.r#powered == true { return 26277; }
        if self.r#hinge == Hinge::Right && self.r#half == Half::Upper && self.r#facing == Facing::North && self.r#powered == false && self.r#open == true { return 26274; }
        if self.r#facing == Facing::East && self.r#hinge == Hinge::Left && self.r#open == false && self.r#powered == true && self.r#half == Half::Lower { return 26327; }
        if self.r#powered == false && self.r#half == Half::Upper && self.r#hinge == Hinge::Left && self.r#open == true && self.r#facing == Facing::North { return 26270; }
        if self.r#half == Half::Upper && self.r#open == true && self.r#powered == false && self.r#hinge == Hinge::Left && self.r#facing == Facing::South { return 26286; }
        if self.r#open == false && self.r#powered == false && self.r#facing == Facing::South && self.r#half == Half::Lower && self.r#hinge == Hinge::Right { return 26300; }
        if self.r#facing == Facing::North && self.r#powered == true && self.r#half == Half::Upper && self.r#hinge == Hinge::Left && self.r#open == false { return 26271; }
        if self.r#facing == Facing::North && self.r#half == Half::Lower && self.r#open == true && self.r#powered == false && self.r#hinge == Hinge::Left { return 26278; }
        if self.r#facing == Facing::West && self.r#powered == false && self.r#hinge == Hinge::Right && self.r#half == Half::Lower && self.r#open == true { return 26314; }
        if self.r#facing == Facing::North && self.r#half == Half::Upper && self.r#open == false && self.r#hinge == Hinge::Right && self.r#powered == false { return 26276; }
        if self.r#half == Half::Upper && self.r#hinge == Hinge::Left && self.r#powered == false && self.r#facing == Facing::West && self.r#open == false { return 26304; }
        if self.r#hinge == Hinge::Left && self.r#open == true && self.r#powered == false && self.r#facing == Facing::South && self.r#half == Half::Lower { return 26294; }
        if self.r#half == Half::Lower && self.r#powered == false && self.r#open == false && self.r#facing == Facing::North && self.r#hinge == Hinge::Left { return 26280; }
        if self.r#hinge == Hinge::Left && self.r#facing == Facing::North && self.r#powered == true && self.r#open == false && self.r#half == Half::Lower { return 26279; }
        if self.r#facing == Facing::South && self.r#hinge == Hinge::Left && self.r#open == false && self.r#powered == false && self.r#half == Half::Upper { return 26288; }
        if self.r#powered == true && self.r#facing == Facing::East && self.r#half == Half::Upper && self.r#hinge == Hinge::Right && self.r#open == false { return 26323; }
        if self.r#hinge == Hinge::Right && self.r#powered == false && self.r#open == false && self.r#facing == Facing::North && self.r#half == Half::Lower { return 26284; }
        if self.r#facing == Facing::North && self.r#hinge == Hinge::Right && self.r#powered == true && self.r#open == true && self.r#half == Half::Upper { return 26273; }
        if self.r#facing == Facing::South && self.r#powered == false && self.r#half == Half::Upper && self.r#open == false && self.r#hinge == Hinge::Right { return 26292; }
        if self.r#facing == Facing::South && self.r#half == Half::Lower && self.r#hinge == Hinge::Left && self.r#open == false && self.r#powered == true { return 26295; }
        if self.r#open == true && self.r#powered == false && self.r#half == Half::Lower && self.r#hinge == Hinge::Left && self.r#facing == Facing::West { return 26310; }
        if self.r#facing == Facing::East && self.r#open == false && self.r#half == Half::Upper && self.r#powered == false && self.r#hinge == Hinge::Right { return 26324; }
        if self.r#facing == Facing::North && self.r#hinge == Hinge::Left && self.r#powered == true && self.r#half == Half::Upper && self.r#open == true { return 26269; }
        if self.r#facing == Facing::East && self.r#open == true && self.r#hinge == Hinge::Right && self.r#half == Half::Upper && self.r#powered == true { return 26321; }
        if self.r#powered == true && self.r#hinge == Hinge::Left && self.r#open == true && self.r#half == Half::Upper && self.r#facing == Facing::South { return 26285; }
        if self.r#facing == Facing::East && self.r#half == Half::Lower && self.r#powered == true && self.r#open == false && self.r#hinge == Hinge::Right { return 26331; }
        if self.r#half == Half::Lower && self.r#hinge == Hinge::Right && self.r#open == true && self.r#facing == Facing::South && self.r#powered == false { return 26298; }
        if self.r#half == Half::Lower && self.r#facing == Facing::West && self.r#hinge == Hinge::Right && self.r#powered == false && self.r#open == false { return 26316; }
        if self.r#facing == Facing::West && self.r#half == Half::Upper && self.r#hinge == Hinge::Right && self.r#open == true && self.r#powered == false { return 26306; }
        if self.r#facing == Facing::East && self.r#half == Half::Upper && self.r#open == true && self.r#hinge == Hinge::Left && self.r#powered == true { return 26317; }
        if self.r#half == Half::Upper && self.r#hinge == Hinge::Left && self.r#facing == Facing::East && self.r#open == false && self.r#powered == false { return 26320; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 26308 {
            return Some(WaxedWeatheredCopperDoor {
                r#half: Half::Upper,
                r#facing: Facing::West,
                r#hinge: Hinge::Right,
                r#powered: false,
                r#open: false,
            });
        }
        if state_id == 26302 {
            return Some(WaxedWeatheredCopperDoor {
                r#hinge: Hinge::Left,
                r#facing: Facing::West,
                r#open: true,
                r#powered: false,
                r#half: Half::Upper,
            });
        }
        if state_id == 26309 {
            return Some(WaxedWeatheredCopperDoor {
                r#hinge: Hinge::Left,
                r#facing: Facing::West,
                r#open: true,
                r#powered: true,
                r#half: Half::Lower,
            });
        }
        if state_id == 26319 {
            return Some(WaxedWeatheredCopperDoor {
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
                r#facing: Facing::East,
                r#open: false,
                r#powered: true,
            });
        }
        if state_id == 26291 {
            return Some(WaxedWeatheredCopperDoor {
                r#facing: Facing::South,
                r#half: Half::Upper,
                r#hinge: Hinge::Right,
                r#open: false,
                r#powered: true,
            });
        }
        if state_id == 26301 {
            return Some(WaxedWeatheredCopperDoor {
                r#facing: Facing::West,
                r#hinge: Hinge::Left,
                r#powered: true,
                r#half: Half::Upper,
                r#open: true,
            });
        }
        if state_id == 26289 {
            return Some(WaxedWeatheredCopperDoor {
                r#half: Half::Upper,
                r#facing: Facing::South,
                r#open: true,
                r#hinge: Hinge::Right,
                r#powered: true,
            });
        }
        if state_id == 26315 {
            return Some(WaxedWeatheredCopperDoor {
                r#open: false,
                r#hinge: Hinge::Right,
                r#powered: true,
                r#half: Half::Lower,
                r#facing: Facing::West,
            });
        }
        if state_id == 26303 {
            return Some(WaxedWeatheredCopperDoor {
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
                r#facing: Facing::West,
                r#open: false,
                r#powered: true,
            });
        }
        if state_id == 26272 {
            return Some(WaxedWeatheredCopperDoor {
                r#powered: false,
                r#half: Half::Upper,
                r#open: false,
                r#hinge: Hinge::Left,
                r#facing: Facing::North,
            });
        }
        if state_id == 26312 {
            return Some(WaxedWeatheredCopperDoor {
                r#facing: Facing::West,
                r#open: false,
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
                r#powered: false,
            });
        }
        if state_id == 26313 {
            return Some(WaxedWeatheredCopperDoor {
                r#hinge: Hinge::Right,
                r#facing: Facing::West,
                r#half: Half::Lower,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 26282 {
            return Some(WaxedWeatheredCopperDoor {
                r#hinge: Hinge::Right,
                r#powered: false,
                r#facing: Facing::North,
                r#half: Half::Lower,
                r#open: true,
            });
        }
        if state_id == 26329 {
            return Some(WaxedWeatheredCopperDoor {
                r#hinge: Hinge::Right,
                r#open: true,
                r#half: Half::Lower,
                r#powered: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 26283 {
            return Some(WaxedWeatheredCopperDoor {
                r#open: false,
                r#hinge: Hinge::Right,
                r#half: Half::Lower,
                r#facing: Facing::North,
                r#powered: true,
            });
        }
        if state_id == 26297 {
            return Some(WaxedWeatheredCopperDoor {
                r#facing: Facing::South,
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 26287 {
            return Some(WaxedWeatheredCopperDoor {
                r#facing: Facing::South,
                r#open: false,
                r#half: Half::Upper,
                r#powered: true,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 26311 {
            return Some(WaxedWeatheredCopperDoor {
                r#powered: true,
                r#facing: Facing::West,
                r#hinge: Hinge::Left,
                r#open: false,
                r#half: Half::Lower,
            });
        }
        if state_id == 26318 {
            return Some(WaxedWeatheredCopperDoor {
                r#facing: Facing::East,
                r#hinge: Hinge::Left,
                r#powered: false,
                r#half: Half::Upper,
                r#open: true,
            });
        }
        if state_id == 26322 {
            return Some(WaxedWeatheredCopperDoor {
                r#facing: Facing::East,
                r#hinge: Hinge::Right,
                r#powered: false,
                r#half: Half::Upper,
                r#open: true,
            });
        }
        if state_id == 26328 {
            return Some(WaxedWeatheredCopperDoor {
                r#open: false,
                r#powered: false,
                r#hinge: Hinge::Left,
                r#facing: Facing::East,
                r#half: Half::Lower,
            });
        }
        if state_id == 26332 {
            return Some(WaxedWeatheredCopperDoor {
                r#open: false,
                r#powered: false,
                r#hinge: Hinge::Right,
                r#facing: Facing::East,
                r#half: Half::Lower,
            });
        }
        if state_id == 26290 {
            return Some(WaxedWeatheredCopperDoor {
                r#open: true,
                r#hinge: Hinge::Right,
                r#powered: false,
                r#facing: Facing::South,
                r#half: Half::Upper,
            });
        }
        if state_id == 26296 {
            return Some(WaxedWeatheredCopperDoor {
                r#half: Half::Lower,
                r#open: false,
                r#facing: Facing::South,
                r#powered: false,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 26275 {
            return Some(WaxedWeatheredCopperDoor {
                r#half: Half::Upper,
                r#facing: Facing::North,
                r#hinge: Hinge::Right,
                r#open: false,
                r#powered: true,
            });
        }
        if state_id == 26281 {
            return Some(WaxedWeatheredCopperDoor {
                r#half: Half::Lower,
                r#open: true,
                r#hinge: Hinge::Right,
                r#powered: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 26305 {
            return Some(WaxedWeatheredCopperDoor {
                r#facing: Facing::West,
                r#powered: true,
                r#half: Half::Upper,
                r#hinge: Hinge::Right,
                r#open: true,
            });
        }
        if state_id == 26325 {
            return Some(WaxedWeatheredCopperDoor {
                r#facing: Facing::East,
                r#hinge: Hinge::Left,
                r#open: true,
                r#half: Half::Lower,
                r#powered: true,
            });
        }
        if state_id == 26326 {
            return Some(WaxedWeatheredCopperDoor {
                r#facing: Facing::East,
                r#hinge: Hinge::Left,
                r#open: true,
                r#half: Half::Lower,
                r#powered: false,
            });
        }
        if state_id == 26330 {
            return Some(WaxedWeatheredCopperDoor {
                r#half: Half::Lower,
                r#facing: Facing::East,
                r#powered: false,
                r#open: true,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 26293 {
            return Some(WaxedWeatheredCopperDoor {
                r#open: true,
                r#facing: Facing::South,
                r#powered: true,
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 26307 {
            return Some(WaxedWeatheredCopperDoor {
                r#open: false,
                r#powered: true,
                r#hinge: Hinge::Right,
                r#facing: Facing::West,
                r#half: Half::Upper,
            });
        }
        if state_id == 26299 {
            return Some(WaxedWeatheredCopperDoor {
                r#hinge: Hinge::Right,
                r#facing: Facing::South,
                r#open: false,
                r#powered: true,
                r#half: Half::Lower,
            });
        }
        if state_id == 26277 {
            return Some(WaxedWeatheredCopperDoor {
                r#half: Half::Lower,
                r#open: true,
                r#facing: Facing::North,
                r#hinge: Hinge::Left,
                r#powered: true,
            });
        }
        if state_id == 26274 {
            return Some(WaxedWeatheredCopperDoor {
                r#hinge: Hinge::Right,
                r#half: Half::Upper,
                r#facing: Facing::North,
                r#powered: false,
                r#open: true,
            });
        }
        if state_id == 26327 {
            return Some(WaxedWeatheredCopperDoor {
                r#facing: Facing::East,
                r#hinge: Hinge::Left,
                r#open: false,
                r#powered: true,
                r#half: Half::Lower,
            });
        }
        if state_id == 26270 {
            return Some(WaxedWeatheredCopperDoor {
                r#powered: false,
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
                r#open: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 26286 {
            return Some(WaxedWeatheredCopperDoor {
                r#half: Half::Upper,
                r#open: true,
                r#powered: false,
                r#hinge: Hinge::Left,
                r#facing: Facing::South,
            });
        }
        if state_id == 26300 {
            return Some(WaxedWeatheredCopperDoor {
                r#open: false,
                r#powered: false,
                r#facing: Facing::South,
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 26271 {
            return Some(WaxedWeatheredCopperDoor {
                r#facing: Facing::North,
                r#powered: true,
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
                r#open: false,
            });
        }
        if state_id == 26278 {
            return Some(WaxedWeatheredCopperDoor {
                r#facing: Facing::North,
                r#half: Half::Lower,
                r#open: true,
                r#powered: false,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 26314 {
            return Some(WaxedWeatheredCopperDoor {
                r#facing: Facing::West,
                r#powered: false,
                r#hinge: Hinge::Right,
                r#half: Half::Lower,
                r#open: true,
            });
        }
        if state_id == 26276 {
            return Some(WaxedWeatheredCopperDoor {
                r#facing: Facing::North,
                r#half: Half::Upper,
                r#open: false,
                r#hinge: Hinge::Right,
                r#powered: false,
            });
        }
        if state_id == 26304 {
            return Some(WaxedWeatheredCopperDoor {
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
                r#powered: false,
                r#facing: Facing::West,
                r#open: false,
            });
        }
        if state_id == 26294 {
            return Some(WaxedWeatheredCopperDoor {
                r#hinge: Hinge::Left,
                r#open: true,
                r#powered: false,
                r#facing: Facing::South,
                r#half: Half::Lower,
            });
        }
        if state_id == 26280 {
            return Some(WaxedWeatheredCopperDoor {
                r#half: Half::Lower,
                r#powered: false,
                r#open: false,
                r#facing: Facing::North,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 26279 {
            return Some(WaxedWeatheredCopperDoor {
                r#hinge: Hinge::Left,
                r#facing: Facing::North,
                r#powered: true,
                r#open: false,
                r#half: Half::Lower,
            });
        }
        if state_id == 26288 {
            return Some(WaxedWeatheredCopperDoor {
                r#facing: Facing::South,
                r#hinge: Hinge::Left,
                r#open: false,
                r#powered: false,
                r#half: Half::Upper,
            });
        }
        if state_id == 26323 {
            return Some(WaxedWeatheredCopperDoor {
                r#powered: true,
                r#facing: Facing::East,
                r#half: Half::Upper,
                r#hinge: Hinge::Right,
                r#open: false,
            });
        }
        if state_id == 26284 {
            return Some(WaxedWeatheredCopperDoor {
                r#hinge: Hinge::Right,
                r#powered: false,
                r#open: false,
                r#facing: Facing::North,
                r#half: Half::Lower,
            });
        }
        if state_id == 26273 {
            return Some(WaxedWeatheredCopperDoor {
                r#facing: Facing::North,
                r#hinge: Hinge::Right,
                r#powered: true,
                r#open: true,
                r#half: Half::Upper,
            });
        }
        if state_id == 26292 {
            return Some(WaxedWeatheredCopperDoor {
                r#facing: Facing::South,
                r#powered: false,
                r#half: Half::Upper,
                r#open: false,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 26295 {
            return Some(WaxedWeatheredCopperDoor {
                r#facing: Facing::South,
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
                r#open: false,
                r#powered: true,
            });
        }
        if state_id == 26310 {
            return Some(WaxedWeatheredCopperDoor {
                r#open: true,
                r#powered: false,
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
                r#facing: Facing::West,
            });
        }
        if state_id == 26324 {
            return Some(WaxedWeatheredCopperDoor {
                r#facing: Facing::East,
                r#open: false,
                r#half: Half::Upper,
                r#powered: false,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 26269 {
            return Some(WaxedWeatheredCopperDoor {
                r#facing: Facing::North,
                r#hinge: Hinge::Left,
                r#powered: true,
                r#half: Half::Upper,
                r#open: true,
            });
        }
        if state_id == 26321 {
            return Some(WaxedWeatheredCopperDoor {
                r#facing: Facing::East,
                r#open: true,
                r#hinge: Hinge::Right,
                r#half: Half::Upper,
                r#powered: true,
            });
        }
        if state_id == 26285 {
            return Some(WaxedWeatheredCopperDoor {
                r#powered: true,
                r#hinge: Hinge::Left,
                r#open: true,
                r#half: Half::Upper,
                r#facing: Facing::South,
            });
        }
        if state_id == 26331 {
            return Some(WaxedWeatheredCopperDoor {
                r#facing: Facing::East,
                r#half: Half::Lower,
                r#powered: true,
                r#open: false,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 26298 {
            return Some(WaxedWeatheredCopperDoor {
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
                r#open: true,
                r#facing: Facing::South,
                r#powered: false,
            });
        }
        if state_id == 26316 {
            return Some(WaxedWeatheredCopperDoor {
                r#half: Half::Lower,
                r#facing: Facing::West,
                r#hinge: Hinge::Right,
                r#powered: false,
                r#open: false,
            });
        }
        if state_id == 26306 {
            return Some(WaxedWeatheredCopperDoor {
                r#facing: Facing::West,
                r#half: Half::Upper,
                r#hinge: Hinge::Right,
                r#open: true,
                r#powered: false,
            });
        }
        if state_id == 26317 {
            return Some(WaxedWeatheredCopperDoor {
                r#facing: Facing::East,
                r#half: Half::Upper,
                r#open: true,
                r#hinge: Hinge::Left,
                r#powered: true,
            });
        }
        if state_id == 26320 {
            return Some(WaxedWeatheredCopperDoor {
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
                r#facing: Facing::East,
                r#open: false,
                r#powered: false,
            });
        }
        return None;
    }
}

