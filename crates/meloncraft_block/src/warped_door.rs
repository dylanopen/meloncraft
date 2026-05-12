use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WarpedDoor {
    pub open: bool,
    pub r#facing: Facing,
    pub r#half: Half,
    pub powered: bool,
    pub r#hinge: Hinge,
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
    Upper,
    Lower,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Hinge {
    Left,
    Right,
}

impl BlockState for WarpedDoor {
    fn to_id(&self) -> i32 {
        if self.r#open == true && self.r#half == Half::Lower && self.r#hinge == Hinge::Right && self.r#facing == Facing::West && self.r#powered == false { return 21421; }
        if self.r#powered == false && self.r#half == Half::Upper && self.r#facing == Facing::West && self.r#hinge == Hinge::Right && self.r#open == true { return 21413; }
        if self.r#open == false && self.r#facing == Facing::East && self.r#powered == true && self.r#hinge == Hinge::Right && self.r#half == Half::Lower { return 21438; }
        if self.r#powered == false && self.r#facing == Facing::South && self.r#half == Half::Upper && self.r#hinge == Hinge::Right && self.r#open == false { return 21399; }
        if self.r#facing == Facing::West && self.r#half == Half::Upper && self.r#powered == true && self.r#hinge == Hinge::Left && self.r#open == false { return 21410; }
        if self.r#half == Half::Upper && self.r#hinge == Hinge::Left && self.r#open == true && self.r#powered == true && self.r#facing == Facing::West { return 21408; }
        if self.r#facing == Facing::South && self.r#hinge == Hinge::Right && self.r#powered == true && self.r#half == Half::Upper && self.r#open == true { return 21396; }
        if self.r#hinge == Hinge::Left && self.r#powered == true && self.r#facing == Facing::East && self.r#half == Half::Upper && self.r#open == true { return 21424; }
        if self.r#powered == false && self.r#half == Half::Upper && self.r#facing == Facing::South && self.r#open == true && self.r#hinge == Hinge::Right { return 21397; }
        if self.r#half == Half::Lower && self.r#hinge == Hinge::Left && self.r#facing == Facing::North && self.r#open == true && self.r#powered == true { return 21384; }
        if self.r#facing == Facing::West && self.r#hinge == Hinge::Left && self.r#half == Half::Upper && self.r#open == false && self.r#powered == false { return 21411; }
        if self.r#half == Half::Upper && self.r#hinge == Hinge::Left && self.r#open == false && self.r#facing == Facing::South && self.r#powered == true { return 21394; }
        if self.r#facing == Facing::West && self.r#open == false && self.r#half == Half::Lower && self.r#powered == false && self.r#hinge == Hinge::Left { return 21419; }
        if self.r#half == Half::Upper && self.r#open == false && self.r#powered == true && self.r#facing == Facing::East && self.r#hinge == Hinge::Right { return 21430; }
        if self.r#half == Half::Lower && self.r#powered == true && self.r#hinge == Hinge::Left && self.r#open == false && self.r#facing == Facing::West { return 21418; }
        if self.r#open == false && self.r#hinge == Hinge::Right && self.r#powered == true && self.r#facing == Facing::West && self.r#half == Half::Upper { return 21414; }
        if self.r#half == Half::Upper && self.r#hinge == Hinge::Left && self.r#open == true && self.r#powered == false && self.r#facing == Facing::South { return 21393; }
        if self.r#open == false && self.r#powered == true && self.r#facing == Facing::South && self.r#hinge == Hinge::Right && self.r#half == Half::Upper { return 21398; }
        if self.r#facing == Facing::East && self.r#half == Half::Upper && self.r#open == false && self.r#powered == true && self.r#hinge == Hinge::Left { return 21426; }
        if self.r#hinge == Hinge::Right && self.r#open == true && self.r#half == Half::Lower && self.r#facing == Facing::North && self.r#powered == false { return 21389; }
        if self.r#half == Half::Lower && self.r#open == false && self.r#powered == true && self.r#facing == Facing::North && self.r#hinge == Hinge::Left { return 21386; }
        if self.r#facing == Facing::East && self.r#half == Half::Lower && self.r#powered == false && self.r#open == true && self.r#hinge == Hinge::Right { return 21437; }
        if self.r#powered == false && self.r#facing == Facing::West && self.r#half == Half::Lower && self.r#hinge == Hinge::Left && self.r#open == true { return 21417; }
        if self.r#hinge == Hinge::Left && self.r#facing == Facing::North && self.r#half == Half::Lower && self.r#open == true && self.r#powered == false { return 21385; }
        if self.r#powered == false && self.r#facing == Facing::East && self.r#half == Half::Upper && self.r#hinge == Hinge::Left && self.r#open == true { return 21425; }
        if self.r#powered == false && self.r#open == false && self.r#facing == Facing::East && self.r#hinge == Hinge::Right && self.r#half == Half::Upper { return 21431; }
        if self.r#open == false && self.r#hinge == Hinge::Right && self.r#half == Half::Lower && self.r#powered == false && self.r#facing == Facing::East { return 21439; }
        if self.r#half == Half::Lower && self.r#hinge == Hinge::Right && self.r#powered == true && self.r#open == true && self.r#facing == Facing::West { return 21420; }
        if self.r#hinge == Hinge::Right && self.r#open == true && self.r#half == Half::Lower && self.r#powered == true && self.r#facing == Facing::North { return 21388; }
        if self.r#facing == Facing::East && self.r#hinge == Hinge::Left && self.r#powered == true && self.r#open == false && self.r#half == Half::Lower { return 21434; }
        if self.r#hinge == Hinge::Left && self.r#facing == Facing::East && self.r#open == true && self.r#half == Half::Lower && self.r#powered == true { return 21432; }
        if self.r#half == Half::Upper && self.r#open == false && self.r#powered == false && self.r#hinge == Hinge::Left && self.r#facing == Facing::North { return 21379; }
        if self.r#facing == Facing::North && self.r#open == true && self.r#powered == true && self.r#half == Half::Upper && self.r#hinge == Hinge::Left { return 21376; }
        if self.r#open == false && self.r#half == Half::Lower && self.r#powered == false && self.r#facing == Facing::South && self.r#hinge == Hinge::Left { return 21403; }
        if self.r#facing == Facing::West && self.r#half == Half::Upper && self.r#open == true && self.r#powered == true && self.r#hinge == Hinge::Right { return 21412; }
        if self.r#open == false && self.r#half == Half::Upper && self.r#facing == Facing::North && self.r#powered == true && self.r#hinge == Hinge::Right { return 21382; }
        if self.r#powered == true && self.r#half == Half::Upper && self.r#facing == Facing::North && self.r#hinge == Hinge::Left && self.r#open == false { return 21378; }
        if self.r#open == false && self.r#half == Half::Lower && self.r#powered == true && self.r#facing == Facing::South && self.r#hinge == Hinge::Left { return 21402; }
        if self.r#facing == Facing::South && self.r#open == false && self.r#hinge == Hinge::Right && self.r#powered == false && self.r#half == Half::Lower { return 21407; }
        if self.r#powered == false && self.r#hinge == Hinge::Left && self.r#open == true && self.r#half == Half::Lower && self.r#facing == Facing::East { return 21433; }
        if self.r#open == false && self.r#hinge == Hinge::Right && self.r#half == Half::Lower && self.r#powered == true && self.r#facing == Facing::West { return 21422; }
        if self.r#open == true && self.r#powered == true && self.r#half == Half::Upper && self.r#facing == Facing::North && self.r#hinge == Hinge::Right { return 21380; }
        if self.r#open == false && self.r#hinge == Hinge::Left && self.r#powered == false && self.r#facing == Facing::North && self.r#half == Half::Lower { return 21387; }
        if self.r#facing == Facing::South && self.r#half == Half::Lower && self.r#hinge == Hinge::Left && self.r#powered == true && self.r#open == true { return 21400; }
        if self.r#hinge == Hinge::Right && self.r#half == Half::Lower && self.r#open == true && self.r#powered == true && self.r#facing == Facing::East { return 21436; }
        if self.r#powered == false && self.r#half == Half::Lower && self.r#hinge == Hinge::Left && self.r#open == true && self.r#facing == Facing::South { return 21401; }
        if self.r#powered == false && self.r#hinge == Hinge::Left && self.r#open == true && self.r#facing == Facing::North && self.r#half == Half::Upper { return 21377; }
        if self.r#half == Half::Lower && self.r#facing == Facing::North && self.r#powered == false && self.r#hinge == Hinge::Right && self.r#open == false { return 21391; }
        if self.r#facing == Facing::North && self.r#hinge == Hinge::Right && self.r#half == Half::Upper && self.r#open == false && self.r#powered == false { return 21383; }
        if self.r#open == false && self.r#hinge == Hinge::Right && self.r#facing == Facing::North && self.r#powered == true && self.r#half == Half::Lower { return 21390; }
        if self.r#facing == Facing::South && self.r#hinge == Hinge::Right && self.r#powered == false && self.r#open == true && self.r#half == Half::Lower { return 21405; }
        if self.r#facing == Facing::East && self.r#powered == false && self.r#open == false && self.r#hinge == Hinge::Left && self.r#half == Half::Upper { return 21427; }
        if self.r#open == true && self.r#facing == Facing::South && self.r#half == Half::Lower && self.r#hinge == Hinge::Right && self.r#powered == true { return 21404; }
        if self.r#facing == Facing::West && self.r#half == Half::Upper && self.r#hinge == Hinge::Left && self.r#open == true && self.r#powered == false { return 21409; }
        if self.r#hinge == Hinge::Left && self.r#open == false && self.r#powered == false && self.r#half == Half::Lower && self.r#facing == Facing::East { return 21435; }
        if self.r#facing == Facing::West && self.r#hinge == Hinge::Left && self.r#half == Half::Lower && self.r#powered == true && self.r#open == true { return 21416; }
        if self.r#facing == Facing::South && self.r#powered == true && self.r#half == Half::Lower && self.r#hinge == Hinge::Right && self.r#open == false { return 21406; }
        if self.r#hinge == Hinge::Right && self.r#facing == Facing::North && self.r#half == Half::Upper && self.r#open == true && self.r#powered == false { return 21381; }
        if self.r#open == true && self.r#powered == true && self.r#facing == Facing::East && self.r#half == Half::Upper && self.r#hinge == Hinge::Right { return 21428; }
        if self.r#hinge == Hinge::Left && self.r#powered == true && self.r#facing == Facing::South && self.r#half == Half::Upper && self.r#open == true { return 21392; }
        if self.r#open == false && self.r#hinge == Hinge::Left && self.r#facing == Facing::South && self.r#powered == false && self.r#half == Half::Upper { return 21395; }
        if self.r#half == Half::Upper && self.r#hinge == Hinge::Right && self.r#powered == false && self.r#facing == Facing::East && self.r#open == true { return 21429; }
        if self.r#hinge == Hinge::Right && self.r#open == false && self.r#powered == false && self.r#half == Half::Lower && self.r#facing == Facing::West { return 21423; }
        if self.r#half == Half::Upper && self.r#facing == Facing::West && self.r#hinge == Hinge::Right && self.r#open == false && self.r#powered == false { return 21415; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 21421 {
            return Some(WarpedDoor {
                r#open: true,
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
                r#facing: Facing::West,
                r#powered: false,
            });
        }
        if state_id == 21413 {
            return Some(WarpedDoor {
                r#powered: false,
                r#half: Half::Upper,
                r#facing: Facing::West,
                r#hinge: Hinge::Right,
                r#open: true,
            });
        }
        if state_id == 21438 {
            return Some(WarpedDoor {
                r#open: false,
                r#facing: Facing::East,
                r#powered: true,
                r#hinge: Hinge::Right,
                r#half: Half::Lower,
            });
        }
        if state_id == 21399 {
            return Some(WarpedDoor {
                r#powered: false,
                r#facing: Facing::South,
                r#half: Half::Upper,
                r#hinge: Hinge::Right,
                r#open: false,
            });
        }
        if state_id == 21410 {
            return Some(WarpedDoor {
                r#facing: Facing::West,
                r#half: Half::Upper,
                r#powered: true,
                r#hinge: Hinge::Left,
                r#open: false,
            });
        }
        if state_id == 21408 {
            return Some(WarpedDoor {
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
                r#open: true,
                r#powered: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 21396 {
            return Some(WarpedDoor {
                r#facing: Facing::South,
                r#hinge: Hinge::Right,
                r#powered: true,
                r#half: Half::Upper,
                r#open: true,
            });
        }
        if state_id == 21424 {
            return Some(WarpedDoor {
                r#hinge: Hinge::Left,
                r#powered: true,
                r#facing: Facing::East,
                r#half: Half::Upper,
                r#open: true,
            });
        }
        if state_id == 21397 {
            return Some(WarpedDoor {
                r#powered: false,
                r#half: Half::Upper,
                r#facing: Facing::South,
                r#open: true,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 21384 {
            return Some(WarpedDoor {
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
                r#facing: Facing::North,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 21411 {
            return Some(WarpedDoor {
                r#facing: Facing::West,
                r#hinge: Hinge::Left,
                r#half: Half::Upper,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 21394 {
            return Some(WarpedDoor {
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
                r#open: false,
                r#facing: Facing::South,
                r#powered: true,
            });
        }
        if state_id == 21419 {
            return Some(WarpedDoor {
                r#facing: Facing::West,
                r#open: false,
                r#half: Half::Lower,
                r#powered: false,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 21430 {
            return Some(WarpedDoor {
                r#half: Half::Upper,
                r#open: false,
                r#powered: true,
                r#facing: Facing::East,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 21418 {
            return Some(WarpedDoor {
                r#half: Half::Lower,
                r#powered: true,
                r#hinge: Hinge::Left,
                r#open: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 21414 {
            return Some(WarpedDoor {
                r#open: false,
                r#hinge: Hinge::Right,
                r#powered: true,
                r#facing: Facing::West,
                r#half: Half::Upper,
            });
        }
        if state_id == 21393 {
            return Some(WarpedDoor {
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
                r#open: true,
                r#powered: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 21398 {
            return Some(WarpedDoor {
                r#open: false,
                r#powered: true,
                r#facing: Facing::South,
                r#hinge: Hinge::Right,
                r#half: Half::Upper,
            });
        }
        if state_id == 21426 {
            return Some(WarpedDoor {
                r#facing: Facing::East,
                r#half: Half::Upper,
                r#open: false,
                r#powered: true,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 21389 {
            return Some(WarpedDoor {
                r#hinge: Hinge::Right,
                r#open: true,
                r#half: Half::Lower,
                r#facing: Facing::North,
                r#powered: false,
            });
        }
        if state_id == 21386 {
            return Some(WarpedDoor {
                r#half: Half::Lower,
                r#open: false,
                r#powered: true,
                r#facing: Facing::North,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 21437 {
            return Some(WarpedDoor {
                r#facing: Facing::East,
                r#half: Half::Lower,
                r#powered: false,
                r#open: true,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 21417 {
            return Some(WarpedDoor {
                r#powered: false,
                r#facing: Facing::West,
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
                r#open: true,
            });
        }
        if state_id == 21385 {
            return Some(WarpedDoor {
                r#hinge: Hinge::Left,
                r#facing: Facing::North,
                r#half: Half::Lower,
                r#open: true,
                r#powered: false,
            });
        }
        if state_id == 21425 {
            return Some(WarpedDoor {
                r#powered: false,
                r#facing: Facing::East,
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
                r#open: true,
            });
        }
        if state_id == 21431 {
            return Some(WarpedDoor {
                r#powered: false,
                r#open: false,
                r#facing: Facing::East,
                r#hinge: Hinge::Right,
                r#half: Half::Upper,
            });
        }
        if state_id == 21439 {
            return Some(WarpedDoor {
                r#open: false,
                r#hinge: Hinge::Right,
                r#half: Half::Lower,
                r#powered: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 21420 {
            return Some(WarpedDoor {
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
                r#powered: true,
                r#open: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 21388 {
            return Some(WarpedDoor {
                r#hinge: Hinge::Right,
                r#open: true,
                r#half: Half::Lower,
                r#powered: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 21434 {
            return Some(WarpedDoor {
                r#facing: Facing::East,
                r#hinge: Hinge::Left,
                r#powered: true,
                r#open: false,
                r#half: Half::Lower,
            });
        }
        if state_id == 21432 {
            return Some(WarpedDoor {
                r#hinge: Hinge::Left,
                r#facing: Facing::East,
                r#open: true,
                r#half: Half::Lower,
                r#powered: true,
            });
        }
        if state_id == 21379 {
            return Some(WarpedDoor {
                r#half: Half::Upper,
                r#open: false,
                r#powered: false,
                r#hinge: Hinge::Left,
                r#facing: Facing::North,
            });
        }
        if state_id == 21376 {
            return Some(WarpedDoor {
                r#facing: Facing::North,
                r#open: true,
                r#powered: true,
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 21403 {
            return Some(WarpedDoor {
                r#open: false,
                r#half: Half::Lower,
                r#powered: false,
                r#facing: Facing::South,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 21412 {
            return Some(WarpedDoor {
                r#facing: Facing::West,
                r#half: Half::Upper,
                r#open: true,
                r#powered: true,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 21382 {
            return Some(WarpedDoor {
                r#open: false,
                r#half: Half::Upper,
                r#facing: Facing::North,
                r#powered: true,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 21378 {
            return Some(WarpedDoor {
                r#powered: true,
                r#half: Half::Upper,
                r#facing: Facing::North,
                r#hinge: Hinge::Left,
                r#open: false,
            });
        }
        if state_id == 21402 {
            return Some(WarpedDoor {
                r#open: false,
                r#half: Half::Lower,
                r#powered: true,
                r#facing: Facing::South,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 21407 {
            return Some(WarpedDoor {
                r#facing: Facing::South,
                r#open: false,
                r#hinge: Hinge::Right,
                r#powered: false,
                r#half: Half::Lower,
            });
        }
        if state_id == 21433 {
            return Some(WarpedDoor {
                r#powered: false,
                r#hinge: Hinge::Left,
                r#open: true,
                r#half: Half::Lower,
                r#facing: Facing::East,
            });
        }
        if state_id == 21422 {
            return Some(WarpedDoor {
                r#open: false,
                r#hinge: Hinge::Right,
                r#half: Half::Lower,
                r#powered: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 21380 {
            return Some(WarpedDoor {
                r#open: true,
                r#powered: true,
                r#half: Half::Upper,
                r#facing: Facing::North,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 21387 {
            return Some(WarpedDoor {
                r#open: false,
                r#hinge: Hinge::Left,
                r#powered: false,
                r#facing: Facing::North,
                r#half: Half::Lower,
            });
        }
        if state_id == 21400 {
            return Some(WarpedDoor {
                r#facing: Facing::South,
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
                r#powered: true,
                r#open: true,
            });
        }
        if state_id == 21436 {
            return Some(WarpedDoor {
                r#hinge: Hinge::Right,
                r#half: Half::Lower,
                r#open: true,
                r#powered: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 21401 {
            return Some(WarpedDoor {
                r#powered: false,
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
                r#open: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 21377 {
            return Some(WarpedDoor {
                r#powered: false,
                r#hinge: Hinge::Left,
                r#open: true,
                r#facing: Facing::North,
                r#half: Half::Upper,
            });
        }
        if state_id == 21391 {
            return Some(WarpedDoor {
                r#half: Half::Lower,
                r#facing: Facing::North,
                r#powered: false,
                r#hinge: Hinge::Right,
                r#open: false,
            });
        }
        if state_id == 21383 {
            return Some(WarpedDoor {
                r#facing: Facing::North,
                r#hinge: Hinge::Right,
                r#half: Half::Upper,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 21390 {
            return Some(WarpedDoor {
                r#open: false,
                r#hinge: Hinge::Right,
                r#facing: Facing::North,
                r#powered: true,
                r#half: Half::Lower,
            });
        }
        if state_id == 21405 {
            return Some(WarpedDoor {
                r#facing: Facing::South,
                r#hinge: Hinge::Right,
                r#powered: false,
                r#open: true,
                r#half: Half::Lower,
            });
        }
        if state_id == 21427 {
            return Some(WarpedDoor {
                r#facing: Facing::East,
                r#powered: false,
                r#open: false,
                r#hinge: Hinge::Left,
                r#half: Half::Upper,
            });
        }
        if state_id == 21404 {
            return Some(WarpedDoor {
                r#open: true,
                r#facing: Facing::South,
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
                r#powered: true,
            });
        }
        if state_id == 21409 {
            return Some(WarpedDoor {
                r#facing: Facing::West,
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
                r#open: true,
                r#powered: false,
            });
        }
        if state_id == 21435 {
            return Some(WarpedDoor {
                r#hinge: Hinge::Left,
                r#open: false,
                r#powered: false,
                r#half: Half::Lower,
                r#facing: Facing::East,
            });
        }
        if state_id == 21416 {
            return Some(WarpedDoor {
                r#facing: Facing::West,
                r#hinge: Hinge::Left,
                r#half: Half::Lower,
                r#powered: true,
                r#open: true,
            });
        }
        if state_id == 21406 {
            return Some(WarpedDoor {
                r#facing: Facing::South,
                r#powered: true,
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
                r#open: false,
            });
        }
        if state_id == 21381 {
            return Some(WarpedDoor {
                r#hinge: Hinge::Right,
                r#facing: Facing::North,
                r#half: Half::Upper,
                r#open: true,
                r#powered: false,
            });
        }
        if state_id == 21428 {
            return Some(WarpedDoor {
                r#open: true,
                r#powered: true,
                r#facing: Facing::East,
                r#half: Half::Upper,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 21392 {
            return Some(WarpedDoor {
                r#hinge: Hinge::Left,
                r#powered: true,
                r#facing: Facing::South,
                r#half: Half::Upper,
                r#open: true,
            });
        }
        if state_id == 21395 {
            return Some(WarpedDoor {
                r#open: false,
                r#hinge: Hinge::Left,
                r#facing: Facing::South,
                r#powered: false,
                r#half: Half::Upper,
            });
        }
        if state_id == 21429 {
            return Some(WarpedDoor {
                r#half: Half::Upper,
                r#hinge: Hinge::Right,
                r#powered: false,
                r#facing: Facing::East,
                r#open: true,
            });
        }
        if state_id == 21423 {
            return Some(WarpedDoor {
                r#hinge: Hinge::Right,
                r#open: false,
                r#powered: false,
                r#half: Half::Lower,
                r#facing: Facing::West,
            });
        }
        if state_id == 21415 {
            return Some(WarpedDoor {
                r#half: Half::Upper,
                r#facing: Facing::West,
                r#hinge: Hinge::Right,
                r#open: false,
                r#powered: false,
            });
        }
        return None;
    }
}

