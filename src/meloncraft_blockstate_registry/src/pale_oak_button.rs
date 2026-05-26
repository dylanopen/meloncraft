use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PaleOakButton {
    pub r#face: Face,
    pub r#facing: Facing,
    pub powered: bool,
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

impl BlockState for PaleOakButton {
    fn to_id(&self) -> i32 {
        if self.r#face == Face::Floor && self.r#powered == false && self.r#facing == Facing::South {
            return 10644;
        }
        if self.r#powered == false && self.r#face == Face::Wall && self.r#facing == Facing::West {
            return 10654;
        }
        if self.r#powered == false && self.r#face == Face::Wall && self.r#facing == Facing::South {
            return 10652;
        }
        if self.r#face == Face::Ceiling && self.r#facing == Facing::West && self.r#powered == false
        {
            return 10662;
        }
        if self.r#face == Face::Ceiling && self.r#facing == Facing::South && self.r#powered == true
        {
            return 10659;
        }
        if self.r#face == Face::Floor && self.r#facing == Facing::North && self.r#powered == false {
            return 10642;
        }
        if self.r#facing == Facing::North && self.r#face == Face::Wall && self.r#powered == false {
            return 10650;
        }
        if self.r#face == Face::Floor && self.r#facing == Facing::East && self.r#powered == true {
            return 10647;
        }
        if self.r#facing == Facing::North && self.r#face == Face::Floor && self.r#powered == true {
            return 10641;
        }
        if self.r#face == Face::Wall && self.r#facing == Facing::East && self.r#powered == false {
            return 10656;
        }
        if self.r#powered == true && self.r#facing == Facing::East && self.r#face == Face::Ceiling {
            return 10663;
        }
        if self.r#facing == Facing::East && self.r#powered == false && self.r#face == Face::Ceiling
        {
            return 10664;
        }
        if self.r#face == Face::Wall && self.r#powered == true && self.r#facing == Facing::South {
            return 10651;
        }
        if self.r#powered == true && self.r#face == Face::Floor && self.r#facing == Facing::West {
            return 10645;
        }
        if self.r#face == Face::Wall && self.r#powered == true && self.r#facing == Facing::East {
            return 10655;
        }
        if self.r#powered == false && self.r#facing == Facing::West && self.r#face == Face::Floor {
            return 10646;
        }
        if self.r#facing == Facing::North && self.r#powered == false && self.r#face == Face::Ceiling
        {
            return 10658;
        }
        if self.r#face == Face::Ceiling && self.r#powered == false && self.r#facing == Facing::South
        {
            return 10660;
        }
        if self.r#face == Face::Floor && self.r#powered == true && self.r#facing == Facing::South {
            return 10643;
        }
        if self.r#facing == Facing::East && self.r#face == Face::Floor && self.r#powered == false {
            return 10648;
        }
        if self.r#powered == true && self.r#facing == Facing::North && self.r#face == Face::Wall {
            return 10649;
        }
        if self.r#face == Face::Wall && self.r#powered == true && self.r#facing == Facing::West {
            return 10653;
        }
        if self.r#powered == true && self.r#face == Face::Ceiling && self.r#facing == Facing::West {
            return 10661;
        }
        if self.r#powered == true && self.r#facing == Facing::North && self.r#face == Face::Ceiling
        {
            return 10657;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 10644 {
            return Some(PaleOakButton {
                r#face: Face::Floor,
                r#powered: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 10654 {
            return Some(PaleOakButton {
                r#powered: false,
                r#face: Face::Wall,
                r#facing: Facing::West,
            });
        }
        if state_id == 10652 {
            return Some(PaleOakButton {
                r#powered: false,
                r#face: Face::Wall,
                r#facing: Facing::South,
            });
        }
        if state_id == 10662 {
            return Some(PaleOakButton {
                r#face: Face::Ceiling,
                r#facing: Facing::West,
                r#powered: false,
            });
        }
        if state_id == 10659 {
            return Some(PaleOakButton {
                r#face: Face::Ceiling,
                r#facing: Facing::South,
                r#powered: true,
            });
        }
        if state_id == 10642 {
            return Some(PaleOakButton {
                r#face: Face::Floor,
                r#facing: Facing::North,
                r#powered: false,
            });
        }
        if state_id == 10650 {
            return Some(PaleOakButton {
                r#facing: Facing::North,
                r#face: Face::Wall,
                r#powered: false,
            });
        }
        if state_id == 10647 {
            return Some(PaleOakButton {
                r#face: Face::Floor,
                r#facing: Facing::East,
                r#powered: true,
            });
        }
        if state_id == 10641 {
            return Some(PaleOakButton {
                r#facing: Facing::North,
                r#face: Face::Floor,
                r#powered: true,
            });
        }
        if state_id == 10656 {
            return Some(PaleOakButton {
                r#face: Face::Wall,
                r#facing: Facing::East,
                r#powered: false,
            });
        }
        if state_id == 10663 {
            return Some(PaleOakButton {
                r#powered: true,
                r#facing: Facing::East,
                r#face: Face::Ceiling,
            });
        }
        if state_id == 10664 {
            return Some(PaleOakButton {
                r#facing: Facing::East,
                r#powered: false,
                r#face: Face::Ceiling,
            });
        }
        if state_id == 10651 {
            return Some(PaleOakButton {
                r#face: Face::Wall,
                r#powered: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 10645 {
            return Some(PaleOakButton {
                r#powered: true,
                r#face: Face::Floor,
                r#facing: Facing::West,
            });
        }
        if state_id == 10655 {
            return Some(PaleOakButton {
                r#face: Face::Wall,
                r#powered: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 10646 {
            return Some(PaleOakButton {
                r#powered: false,
                r#facing: Facing::West,
                r#face: Face::Floor,
            });
        }
        if state_id == 10658 {
            return Some(PaleOakButton {
                r#facing: Facing::North,
                r#powered: false,
                r#face: Face::Ceiling,
            });
        }
        if state_id == 10660 {
            return Some(PaleOakButton {
                r#face: Face::Ceiling,
                r#powered: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 10643 {
            return Some(PaleOakButton {
                r#face: Face::Floor,
                r#powered: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 10648 {
            return Some(PaleOakButton {
                r#facing: Facing::East,
                r#face: Face::Floor,
                r#powered: false,
            });
        }
        if state_id == 10649 {
            return Some(PaleOakButton {
                r#powered: true,
                r#facing: Facing::North,
                r#face: Face::Wall,
            });
        }
        if state_id == 10653 {
            return Some(PaleOakButton {
                r#face: Face::Wall,
                r#powered: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 10661 {
            return Some(PaleOakButton {
                r#powered: true,
                r#face: Face::Ceiling,
                r#facing: Facing::West,
            });
        }
        if state_id == 10657 {
            return Some(PaleOakButton {
                r#powered: true,
                r#facing: Facing::North,
                r#face: Face::Ceiling,
            });
        }
        return None;
    }
}
