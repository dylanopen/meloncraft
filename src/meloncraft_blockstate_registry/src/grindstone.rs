use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grindstone {
    pub r#face: Face,
    pub r#facing: Facing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Face {
    Floor,
    Wall,
    Ceiling,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for Grindstone {
    fn to_id(&self) -> i32 {
        if self.r#face == Face::Ceiling && self.r#facing == Facing::North {
            return 20578;
        }
        if self.r#face == Face::Ceiling && self.r#facing == Facing::East {
            return 20581;
        }
        if self.r#face == Face::Wall && self.r#facing == Facing::West {
            return 20576;
        }
        if self.r#face == Face::Ceiling && self.r#facing == Facing::South {
            return 20579;
        }
        if self.r#face == Face::Floor && self.r#facing == Facing::North {
            return 20570;
        }
        if self.r#face == Face::Wall && self.r#facing == Facing::North {
            return 20574;
        }
        if self.r#facing == Facing::West && self.r#face == Face::Floor {
            return 20572;
        }
        if self.r#face == Face::Ceiling && self.r#facing == Facing::West {
            return 20580;
        }
        if self.r#face == Face::Floor && self.r#facing == Facing::East {
            return 20573;
        }
        if self.r#face == Face::Floor && self.r#facing == Facing::South {
            return 20571;
        }
        if self.r#facing == Facing::South && self.r#face == Face::Wall {
            return 20575;
        }
        if self.r#facing == Facing::East && self.r#face == Face::Wall {
            return 20577;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 20578 {
            return Some(Grindstone {
                r#face: Face::Ceiling,
                r#facing: Facing::North,
            });
        }
        if state_id == 20581 {
            return Some(Grindstone {
                r#face: Face::Ceiling,
                r#facing: Facing::East,
            });
        }
        if state_id == 20576 {
            return Some(Grindstone {
                r#face: Face::Wall,
                r#facing: Facing::West,
            });
        }
        if state_id == 20579 {
            return Some(Grindstone {
                r#face: Face::Ceiling,
                r#facing: Facing::South,
            });
        }
        if state_id == 20570 {
            return Some(Grindstone {
                r#face: Face::Floor,
                r#facing: Facing::North,
            });
        }
        if state_id == 20574 {
            return Some(Grindstone {
                r#face: Face::Wall,
                r#facing: Facing::North,
            });
        }
        if state_id == 20572 {
            return Some(Grindstone {
                r#facing: Facing::West,
                r#face: Face::Floor,
            });
        }
        if state_id == 20580 {
            return Some(Grindstone {
                r#face: Face::Ceiling,
                r#facing: Facing::West,
            });
        }
        if state_id == 20573 {
            return Some(Grindstone {
                r#face: Face::Floor,
                r#facing: Facing::East,
            });
        }
        if state_id == 20571 {
            return Some(Grindstone {
                r#face: Face::Floor,
                r#facing: Facing::South,
            });
        }
        if state_id == 20575 {
            return Some(Grindstone {
                r#facing: Facing::South,
                r#face: Face::Wall,
            });
        }
        if state_id == 20577 {
            return Some(Grindstone {
                r#facing: Facing::East,
                r#face: Face::Wall,
            });
        }
        return None;
    }
}
