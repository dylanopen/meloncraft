use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpruceButton {
    pub powered: bool,
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

impl BlockState for SpruceButton {
    fn to_id(&self) -> i32 {
        if self.r#powered == true && self.r#face == Face::Wall && self.r#facing == Facing::North {
            return 10505;
        }
        if self.r#powered == true && self.r#facing == Facing::West && self.r#face == Face::Ceiling {
            return 10517;
        }
        if self.r#face == Face::Ceiling && self.r#powered == false && self.r#facing == Facing::West
        {
            return 10518;
        }
        if self.r#powered == false && self.r#facing == Facing::North && self.r#face == Face::Floor {
            return 10498;
        }
        if self.r#face == Face::Floor && self.r#facing == Facing::East && self.r#powered == false {
            return 10504;
        }
        if self.r#powered == true && self.r#face == Face::Floor && self.r#facing == Facing::North {
            return 10497;
        }
        if self.r#powered == true && self.r#facing == Facing::South && self.r#face == Face::Floor {
            return 10499;
        }
        if self.r#face == Face::Wall && self.r#powered == false && self.r#facing == Facing::South {
            return 10508;
        }
        if self.r#powered == false && self.r#face == Face::Wall && self.r#facing == Facing::West {
            return 10510;
        }
        if self.r#face == Face::Ceiling && self.r#facing == Facing::South && self.r#powered == false
        {
            return 10516;
        }
        if self.r#face == Face::Wall && self.r#powered == false && self.r#facing == Facing::North {
            return 10506;
        }
        if self.r#facing == Facing::East && self.r#face == Face::Ceiling && self.r#powered == true {
            return 10519;
        }
        if self.r#face == Face::Wall && self.r#powered == true && self.r#facing == Facing::South {
            return 10507;
        }
        if self.r#face == Face::Floor && self.r#facing == Facing::West && self.r#powered == false {
            return 10502;
        }
        if self.r#facing == Facing::West && self.r#face == Face::Wall && self.r#powered == true {
            return 10509;
        }
        if self.r#face == Face::Floor && self.r#powered == true && self.r#facing == Facing::West {
            return 10501;
        }
        if self.r#facing == Facing::North && self.r#powered == false && self.r#face == Face::Ceiling
        {
            return 10514;
        }
        if self.r#powered == true && self.r#facing == Facing::North && self.r#face == Face::Ceiling
        {
            return 10513;
        }
        if self.r#powered == false && self.r#facing == Facing::East && self.r#face == Face::Ceiling
        {
            return 10520;
        }
        if self.r#powered == false && self.r#face == Face::Floor && self.r#facing == Facing::South {
            return 10500;
        }
        if self.r#face == Face::Wall && self.r#powered == true && self.r#facing == Facing::East {
            return 10511;
        }
        if self.r#face == Face::Wall && self.r#facing == Facing::East && self.r#powered == false {
            return 10512;
        }
        if self.r#powered == true && self.r#facing == Facing::South && self.r#face == Face::Ceiling
        {
            return 10515;
        }
        if self.r#powered == true && self.r#facing == Facing::East && self.r#face == Face::Floor {
            return 10503;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 10505 {
            return Some(SpruceButton {
                r#powered: true,
                r#face: Face::Wall,
                r#facing: Facing::North,
            });
        }
        if state_id == 10517 {
            return Some(SpruceButton {
                r#powered: true,
                r#facing: Facing::West,
                r#face: Face::Ceiling,
            });
        }
        if state_id == 10518 {
            return Some(SpruceButton {
                r#face: Face::Ceiling,
                r#powered: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 10498 {
            return Some(SpruceButton {
                r#powered: false,
                r#facing: Facing::North,
                r#face: Face::Floor,
            });
        }
        if state_id == 10504 {
            return Some(SpruceButton {
                r#face: Face::Floor,
                r#facing: Facing::East,
                r#powered: false,
            });
        }
        if state_id == 10497 {
            return Some(SpruceButton {
                r#powered: true,
                r#face: Face::Floor,
                r#facing: Facing::North,
            });
        }
        if state_id == 10499 {
            return Some(SpruceButton {
                r#powered: true,
                r#facing: Facing::South,
                r#face: Face::Floor,
            });
        }
        if state_id == 10508 {
            return Some(SpruceButton {
                r#face: Face::Wall,
                r#powered: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 10510 {
            return Some(SpruceButton {
                r#powered: false,
                r#face: Face::Wall,
                r#facing: Facing::West,
            });
        }
        if state_id == 10516 {
            return Some(SpruceButton {
                r#face: Face::Ceiling,
                r#facing: Facing::South,
                r#powered: false,
            });
        }
        if state_id == 10506 {
            return Some(SpruceButton {
                r#face: Face::Wall,
                r#powered: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 10519 {
            return Some(SpruceButton {
                r#facing: Facing::East,
                r#face: Face::Ceiling,
                r#powered: true,
            });
        }
        if state_id == 10507 {
            return Some(SpruceButton {
                r#face: Face::Wall,
                r#powered: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 10502 {
            return Some(SpruceButton {
                r#face: Face::Floor,
                r#facing: Facing::West,
                r#powered: false,
            });
        }
        if state_id == 10509 {
            return Some(SpruceButton {
                r#facing: Facing::West,
                r#face: Face::Wall,
                r#powered: true,
            });
        }
        if state_id == 10501 {
            return Some(SpruceButton {
                r#face: Face::Floor,
                r#powered: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 10514 {
            return Some(SpruceButton {
                r#facing: Facing::North,
                r#powered: false,
                r#face: Face::Ceiling,
            });
        }
        if state_id == 10513 {
            return Some(SpruceButton {
                r#powered: true,
                r#facing: Facing::North,
                r#face: Face::Ceiling,
            });
        }
        if state_id == 10520 {
            return Some(SpruceButton {
                r#powered: false,
                r#facing: Facing::East,
                r#face: Face::Ceiling,
            });
        }
        if state_id == 10500 {
            return Some(SpruceButton {
                r#powered: false,
                r#face: Face::Floor,
                r#facing: Facing::South,
            });
        }
        if state_id == 10511 {
            return Some(SpruceButton {
                r#face: Face::Wall,
                r#powered: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 10512 {
            return Some(SpruceButton {
                r#face: Face::Wall,
                r#facing: Facing::East,
                r#powered: false,
            });
        }
        if state_id == 10515 {
            return Some(SpruceButton {
                r#powered: true,
                r#facing: Facing::South,
                r#face: Face::Ceiling,
            });
        }
        if state_id == 10503 {
            return Some(SpruceButton {
                r#powered: true,
                r#facing: Facing::East,
                r#face: Face::Floor,
            });
        }
        return None;
    }
}
