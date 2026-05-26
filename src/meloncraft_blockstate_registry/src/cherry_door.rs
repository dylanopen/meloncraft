use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CherryDoor {
    pub open: bool,
    pub powered: bool,
    pub r#facing: Facing,
    pub r#hinge: Hinge,
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
pub enum Hinge {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Half {
    Upper,
    Lower,
}

impl BlockState for CherryDoor {
    fn to_id(&self) -> i32 {
        if self.r#hinge == Hinge::Right
            && self.r#powered == true
            && self.r#half == Half::Lower
            && self.r#facing == Facing::East
            && self.r#open == false
        {
            return 14176;
        }
        if self.r#open == true
            && self.r#hinge == Hinge::Right
            && self.r#half == Half::Upper
            && self.r#facing == Facing::South
            && self.r#powered == false
        {
            return 14135;
        }
        if self.r#facing == Facing::North
            && self.r#half == Half::Lower
            && self.r#powered == false
            && self.r#hinge == Hinge::Right
            && self.r#open == false
        {
            return 14129;
        }
        if self.r#half == Half::Upper
            && self.r#open == false
            && self.r#powered == true
            && self.r#hinge == Hinge::Left
            && self.r#facing == Facing::East
        {
            return 14164;
        }
        if self.r#hinge == Hinge::Right
            && self.r#powered == true
            && self.r#half == Half::Upper
            && self.r#open == true
            && self.r#facing == Facing::East
        {
            return 14166;
        }
        if self.r#facing == Facing::North
            && self.r#hinge == Hinge::Right
            && self.r#open == false
            && self.r#powered == false
            && self.r#half == Half::Upper
        {
            return 14121;
        }
        if self.r#facing == Facing::South
            && self.r#open == false
            && self.r#powered == false
            && self.r#half == Half::Upper
            && self.r#hinge == Hinge::Right
        {
            return 14137;
        }
        if self.r#powered == true
            && self.r#open == false
            && self.r#facing == Facing::West
            && self.r#half == Half::Upper
            && self.r#hinge == Hinge::Left
        {
            return 14148;
        }
        if self.r#half == Half::Lower
            && self.r#open == false
            && self.r#hinge == Hinge::Left
            && self.r#facing == Facing::South
            && self.r#powered == true
        {
            return 14140;
        }
        if self.r#facing == Facing::North
            && self.r#half == Half::Lower
            && self.r#open == true
            && self.r#hinge == Hinge::Left
            && self.r#powered == true
        {
            return 14122;
        }
        if self.r#facing == Facing::West
            && self.r#open == false
            && self.r#powered == false
            && self.r#hinge == Hinge::Left
            && self.r#half == Half::Lower
        {
            return 14157;
        }
        if self.r#half == Half::Upper
            && self.r#hinge == Hinge::Left
            && self.r#facing == Facing::North
            && self.r#open == true
            && self.r#powered == true
        {
            return 14114;
        }
        if self.r#open == false
            && self.r#powered == false
            && self.r#half == Half::Upper
            && self.r#facing == Facing::East
            && self.r#hinge == Hinge::Right
        {
            return 14169;
        }
        if self.r#open == true
            && self.r#facing == Facing::West
            && self.r#powered == true
            && self.r#half == Half::Upper
            && self.r#hinge == Hinge::Left
        {
            return 14146;
        }
        if self.r#facing == Facing::South
            && self.r#open == true
            && self.r#hinge == Hinge::Left
            && self.r#powered == false
            && self.r#half == Half::Upper
        {
            return 14131;
        }
        if self.r#powered == true
            && self.r#open == true
            && self.r#half == Half::Lower
            && self.r#facing == Facing::West
            && self.r#hinge == Hinge::Right
        {
            return 14158;
        }
        if self.r#open == true
            && self.r#powered == false
            && self.r#facing == Facing::North
            && self.r#hinge == Hinge::Right
            && self.r#half == Half::Lower
        {
            return 14127;
        }
        if self.r#open == true
            && self.r#facing == Facing::West
            && self.r#powered == false
            && self.r#half == Half::Lower
            && self.r#hinge == Hinge::Left
        {
            return 14155;
        }
        if self.r#powered == true
            && self.r#hinge == Hinge::Right
            && self.r#open == false
            && self.r#half == Half::Upper
            && self.r#facing == Facing::East
        {
            return 14168;
        }
        if self.r#hinge == Hinge::Left
            && self.r#open == true
            && self.r#powered == true
            && self.r#facing == Facing::East
            && self.r#half == Half::Upper
        {
            return 14162;
        }
        if self.r#facing == Facing::West
            && self.r#hinge == Hinge::Right
            && self.r#half == Half::Upper
            && self.r#open == true
            && self.r#powered == true
        {
            return 14150;
        }
        if self.r#half == Half::Upper
            && self.r#hinge == Hinge::Right
            && self.r#powered == true
            && self.r#facing == Facing::West
            && self.r#open == false
        {
            return 14152;
        }
        if self.r#facing == Facing::North
            && self.r#powered == false
            && self.r#hinge == Hinge::Right
            && self.r#open == true
            && self.r#half == Half::Upper
        {
            return 14119;
        }
        if self.r#hinge == Hinge::Left
            && self.r#facing == Facing::West
            && self.r#half == Half::Lower
            && self.r#open == true
            && self.r#powered == true
        {
            return 14154;
        }
        if self.r#open == false
            && self.r#half == Half::Lower
            && self.r#facing == Facing::West
            && self.r#powered == true
            && self.r#hinge == Hinge::Left
        {
            return 14156;
        }
        if self.r#facing == Facing::South
            && self.r#hinge == Hinge::Right
            && self.r#half == Half::Lower
            && self.r#open == false
            && self.r#powered == false
        {
            return 14145;
        }
        if self.r#half == Half::Lower
            && self.r#open == true
            && self.r#hinge == Hinge::Left
            && self.r#facing == Facing::North
            && self.r#powered == false
        {
            return 14123;
        }
        if self.r#open == true
            && self.r#hinge == Hinge::Left
            && self.r#half == Half::Lower
            && self.r#facing == Facing::South
            && self.r#powered == true
        {
            return 14138;
        }
        if self.r#open == false
            && self.r#powered == true
            && self.r#hinge == Hinge::Left
            && self.r#half == Half::Upper
            && self.r#facing == Facing::North
        {
            return 14116;
        }
        if self.r#open == false
            && self.r#hinge == Hinge::Right
            && self.r#facing == Facing::North
            && self.r#powered == true
            && self.r#half == Half::Lower
        {
            return 14128;
        }
        if self.r#powered == false
            && self.r#facing == Facing::West
            && self.r#open == true
            && self.r#half == Half::Upper
            && self.r#hinge == Hinge::Right
        {
            return 14151;
        }
        if self.r#hinge == Hinge::Right
            && self.r#facing == Facing::West
            && self.r#open == false
            && self.r#half == Half::Lower
            && self.r#powered == true
        {
            return 14160;
        }
        if self.r#facing == Facing::South
            && self.r#half == Half::Lower
            && self.r#powered == false
            && self.r#hinge == Hinge::Left
            && self.r#open == false
        {
            return 14141;
        }
        if self.r#powered == true
            && self.r#open == true
            && self.r#hinge == Hinge::Right
            && self.r#facing == Facing::North
            && self.r#half == Half::Lower
        {
            return 14126;
        }
        if self.r#half == Half::Upper
            && self.r#hinge == Hinge::Right
            && self.r#powered == true
            && self.r#open == true
            && self.r#facing == Facing::South
        {
            return 14134;
        }
        if self.r#facing == Facing::East
            && self.r#half == Half::Lower
            && self.r#powered == true
            && self.r#open == true
            && self.r#hinge == Hinge::Left
        {
            return 14170;
        }
        if self.r#facing == Facing::South
            && self.r#half == Half::Lower
            && self.r#hinge == Hinge::Right
            && self.r#open == true
            && self.r#powered == true
        {
            return 14142;
        }
        if self.r#half == Half::Upper
            && self.r#powered == false
            && self.r#open == true
            && self.r#hinge == Hinge::Right
            && self.r#facing == Facing::East
        {
            return 14167;
        }
        if self.r#powered == false
            && self.r#half == Half::Upper
            && self.r#facing == Facing::North
            && self.r#hinge == Hinge::Left
            && self.r#open == false
        {
            return 14117;
        }
        if self.r#half == Half::Lower
            && self.r#hinge == Hinge::Left
            && self.r#powered == false
            && self.r#open == false
            && self.r#facing == Facing::East
        {
            return 14173;
        }
        if self.r#open == false
            && self.r#hinge == Hinge::Left
            && self.r#powered == true
            && self.r#facing == Facing::East
            && self.r#half == Half::Lower
        {
            return 14172;
        }
        if self.r#powered == false
            && self.r#half == Half::Lower
            && self.r#open == true
            && self.r#hinge == Hinge::Left
            && self.r#facing == Facing::South
        {
            return 14139;
        }
        if self.r#facing == Facing::North
            && self.r#open == false
            && self.r#half == Half::Upper
            && self.r#powered == true
            && self.r#hinge == Hinge::Right
        {
            return 14120;
        }
        if self.r#facing == Facing::West
            && self.r#hinge == Hinge::Right
            && self.r#open == true
            && self.r#half == Half::Lower
            && self.r#powered == false
        {
            return 14159;
        }
        if self.r#facing == Facing::West
            && self.r#half == Half::Upper
            && self.r#open == true
            && self.r#hinge == Hinge::Left
            && self.r#powered == false
        {
            return 14147;
        }
        if self.r#open == true
            && self.r#powered == true
            && self.r#half == Half::Upper
            && self.r#facing == Facing::South
            && self.r#hinge == Hinge::Left
        {
            return 14130;
        }
        if self.r#half == Half::Upper
            && self.r#open == false
            && self.r#powered == true
            && self.r#facing == Facing::South
            && self.r#hinge == Hinge::Left
        {
            return 14132;
        }
        if self.r#powered == false
            && self.r#facing == Facing::West
            && self.r#open == false
            && self.r#hinge == Hinge::Left
            && self.r#half == Half::Upper
        {
            return 14149;
        }
        if self.r#open == false
            && self.r#powered == false
            && self.r#hinge == Hinge::Left
            && self.r#half == Half::Upper
            && self.r#facing == Facing::South
        {
            return 14133;
        }
        if self.r#facing == Facing::West
            && self.r#half == Half::Lower
            && self.r#powered == false
            && self.r#open == false
            && self.r#hinge == Hinge::Right
        {
            return 14161;
        }
        if self.r#hinge == Hinge::Left
            && self.r#facing == Facing::North
            && self.r#half == Half::Lower
            && self.r#powered == false
            && self.r#open == false
        {
            return 14125;
        }
        if self.r#hinge == Hinge::Left
            && self.r#facing == Facing::North
            && self.r#half == Half::Upper
            && self.r#open == true
            && self.r#powered == false
        {
            return 14115;
        }
        if self.r#powered == false
            && self.r#facing == Facing::East
            && self.r#half == Half::Lower
            && self.r#hinge == Hinge::Right
            && self.r#open == false
        {
            return 14177;
        }
        if self.r#facing == Facing::South
            && self.r#hinge == Hinge::Right
            && self.r#open == false
            && self.r#half == Half::Upper
            && self.r#powered == true
        {
            return 14136;
        }
        if self.r#facing == Facing::East
            && self.r#hinge == Hinge::Left
            && self.r#powered == false
            && self.r#open == false
            && self.r#half == Half::Upper
        {
            return 14165;
        }
        if self.r#open == false
            && self.r#facing == Facing::North
            && self.r#hinge == Hinge::Left
            && self.r#powered == true
            && self.r#half == Half::Lower
        {
            return 14124;
        }
        if self.r#half == Half::Upper
            && self.r#facing == Facing::West
            && self.r#powered == false
            && self.r#hinge == Hinge::Right
            && self.r#open == false
        {
            return 14153;
        }
        if self.r#powered == true
            && self.r#half == Half::Lower
            && self.r#hinge == Hinge::Right
            && self.r#open == false
            && self.r#facing == Facing::South
        {
            return 14144;
        }
        if self.r#hinge == Hinge::Left
            && self.r#half == Half::Upper
            && self.r#facing == Facing::East
            && self.r#open == true
            && self.r#powered == false
        {
            return 14163;
        }
        if self.r#open == true
            && self.r#hinge == Hinge::Left
            && self.r#powered == false
            && self.r#facing == Facing::East
            && self.r#half == Half::Lower
        {
            return 14171;
        }
        if self.r#facing == Facing::North
            && self.r#powered == true
            && self.r#half == Half::Upper
            && self.r#open == true
            && self.r#hinge == Hinge::Right
        {
            return 14118;
        }
        if self.r#half == Half::Lower
            && self.r#open == true
            && self.r#powered == false
            && self.r#facing == Facing::South
            && self.r#hinge == Hinge::Right
        {
            return 14143;
        }
        if self.r#powered == false
            && self.r#half == Half::Lower
            && self.r#facing == Facing::East
            && self.r#hinge == Hinge::Right
            && self.r#open == true
        {
            return 14175;
        }
        if self.r#hinge == Hinge::Right
            && self.r#facing == Facing::East
            && self.r#open == true
            && self.r#half == Half::Lower
            && self.r#powered == true
        {
            return 14174;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14176 {
            return Some(CherryDoor {
                r#hinge: Hinge::Right,
                r#powered: true,
                r#half: Half::Lower,
                r#facing: Facing::East,
                r#open: false,
            });
        }
        if state_id == 14135 {
            return Some(CherryDoor {
                r#open: true,
                r#hinge: Hinge::Right,
                r#half: Half::Upper,
                r#facing: Facing::South,
                r#powered: false,
            });
        }
        if state_id == 14129 {
            return Some(CherryDoor {
                r#facing: Facing::North,
                r#half: Half::Lower,
                r#powered: false,
                r#hinge: Hinge::Right,
                r#open: false,
            });
        }
        if state_id == 14164 {
            return Some(CherryDoor {
                r#half: Half::Upper,
                r#open: false,
                r#powered: true,
                r#hinge: Hinge::Left,
                r#facing: Facing::East,
            });
        }
        if state_id == 14166 {
            return Some(CherryDoor {
                r#hinge: Hinge::Right,
                r#powered: true,
                r#half: Half::Upper,
                r#open: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 14121 {
            return Some(CherryDoor {
                r#facing: Facing::North,
                r#hinge: Hinge::Right,
                r#open: false,
                r#powered: false,
                r#half: Half::Upper,
            });
        }
        if state_id == 14137 {
            return Some(CherryDoor {
                r#facing: Facing::South,
                r#open: false,
                r#powered: false,
                r#half: Half::Upper,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 14148 {
            return Some(CherryDoor {
                r#powered: true,
                r#open: false,
                r#facing: Facing::West,
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 14140 {
            return Some(CherryDoor {
                r#half: Half::Lower,
                r#open: false,
                r#hinge: Hinge::Left,
                r#facing: Facing::South,
                r#powered: true,
            });
        }
        if state_id == 14122 {
            return Some(CherryDoor {
                r#facing: Facing::North,
                r#half: Half::Lower,
                r#open: true,
                r#hinge: Hinge::Left,
                r#powered: true,
            });
        }
        if state_id == 14157 {
            return Some(CherryDoor {
                r#facing: Facing::West,
                r#open: false,
                r#powered: false,
                r#hinge: Hinge::Left,
                r#half: Half::Lower,
            });
        }
        if state_id == 14114 {
            return Some(CherryDoor {
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
                r#facing: Facing::North,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 14169 {
            return Some(CherryDoor {
                r#open: false,
                r#powered: false,
                r#half: Half::Upper,
                r#facing: Facing::East,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 14146 {
            return Some(CherryDoor {
                r#open: true,
                r#facing: Facing::West,
                r#powered: true,
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 14131 {
            return Some(CherryDoor {
                r#facing: Facing::South,
                r#open: true,
                r#hinge: Hinge::Left,
                r#powered: false,
                r#half: Half::Upper,
            });
        }
        if state_id == 14158 {
            return Some(CherryDoor {
                r#powered: true,
                r#open: true,
                r#half: Half::Lower,
                r#facing: Facing::West,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 14127 {
            return Some(CherryDoor {
                r#open: true,
                r#powered: false,
                r#facing: Facing::North,
                r#hinge: Hinge::Right,
                r#half: Half::Lower,
            });
        }
        if state_id == 14155 {
            return Some(CherryDoor {
                r#open: true,
                r#facing: Facing::West,
                r#powered: false,
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 14168 {
            return Some(CherryDoor {
                r#powered: true,
                r#hinge: Hinge::Right,
                r#open: false,
                r#half: Half::Upper,
                r#facing: Facing::East,
            });
        }
        if state_id == 14162 {
            return Some(CherryDoor {
                r#hinge: Hinge::Left,
                r#open: true,
                r#powered: true,
                r#facing: Facing::East,
                r#half: Half::Upper,
            });
        }
        if state_id == 14150 {
            return Some(CherryDoor {
                r#facing: Facing::West,
                r#hinge: Hinge::Right,
                r#half: Half::Upper,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 14152 {
            return Some(CherryDoor {
                r#half: Half::Upper,
                r#hinge: Hinge::Right,
                r#powered: true,
                r#facing: Facing::West,
                r#open: false,
            });
        }
        if state_id == 14119 {
            return Some(CherryDoor {
                r#facing: Facing::North,
                r#powered: false,
                r#hinge: Hinge::Right,
                r#open: true,
                r#half: Half::Upper,
            });
        }
        if state_id == 14154 {
            return Some(CherryDoor {
                r#hinge: Hinge::Left,
                r#facing: Facing::West,
                r#half: Half::Lower,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 14156 {
            return Some(CherryDoor {
                r#open: false,
                r#half: Half::Lower,
                r#facing: Facing::West,
                r#powered: true,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 14145 {
            return Some(CherryDoor {
                r#facing: Facing::South,
                r#hinge: Hinge::Right,
                r#half: Half::Lower,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 14123 {
            return Some(CherryDoor {
                r#half: Half::Lower,
                r#open: true,
                r#hinge: Hinge::Left,
                r#facing: Facing::North,
                r#powered: false,
            });
        }
        if state_id == 14138 {
            return Some(CherryDoor {
                r#open: true,
                r#hinge: Hinge::Left,
                r#half: Half::Lower,
                r#facing: Facing::South,
                r#powered: true,
            });
        }
        if state_id == 14116 {
            return Some(CherryDoor {
                r#open: false,
                r#powered: true,
                r#hinge: Hinge::Left,
                r#half: Half::Upper,
                r#facing: Facing::North,
            });
        }
        if state_id == 14128 {
            return Some(CherryDoor {
                r#open: false,
                r#hinge: Hinge::Right,
                r#facing: Facing::North,
                r#powered: true,
                r#half: Half::Lower,
            });
        }
        if state_id == 14151 {
            return Some(CherryDoor {
                r#powered: false,
                r#facing: Facing::West,
                r#open: true,
                r#half: Half::Upper,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 14160 {
            return Some(CherryDoor {
                r#hinge: Hinge::Right,
                r#facing: Facing::West,
                r#open: false,
                r#half: Half::Lower,
                r#powered: true,
            });
        }
        if state_id == 14141 {
            return Some(CherryDoor {
                r#facing: Facing::South,
                r#half: Half::Lower,
                r#powered: false,
                r#hinge: Hinge::Left,
                r#open: false,
            });
        }
        if state_id == 14126 {
            return Some(CherryDoor {
                r#powered: true,
                r#open: true,
                r#hinge: Hinge::Right,
                r#facing: Facing::North,
                r#half: Half::Lower,
            });
        }
        if state_id == 14134 {
            return Some(CherryDoor {
                r#half: Half::Upper,
                r#hinge: Hinge::Right,
                r#powered: true,
                r#open: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 14170 {
            return Some(CherryDoor {
                r#facing: Facing::East,
                r#half: Half::Lower,
                r#powered: true,
                r#open: true,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 14142 {
            return Some(CherryDoor {
                r#facing: Facing::South,
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 14167 {
            return Some(CherryDoor {
                r#half: Half::Upper,
                r#powered: false,
                r#open: true,
                r#hinge: Hinge::Right,
                r#facing: Facing::East,
            });
        }
        if state_id == 14117 {
            return Some(CherryDoor {
                r#powered: false,
                r#half: Half::Upper,
                r#facing: Facing::North,
                r#hinge: Hinge::Left,
                r#open: false,
            });
        }
        if state_id == 14173 {
            return Some(CherryDoor {
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
                r#powered: false,
                r#open: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 14172 {
            return Some(CherryDoor {
                r#open: false,
                r#hinge: Hinge::Left,
                r#powered: true,
                r#facing: Facing::East,
                r#half: Half::Lower,
            });
        }
        if state_id == 14139 {
            return Some(CherryDoor {
                r#powered: false,
                r#half: Half::Lower,
                r#open: true,
                r#hinge: Hinge::Left,
                r#facing: Facing::South,
            });
        }
        if state_id == 14120 {
            return Some(CherryDoor {
                r#facing: Facing::North,
                r#open: false,
                r#half: Half::Upper,
                r#powered: true,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 14159 {
            return Some(CherryDoor {
                r#facing: Facing::West,
                r#hinge: Hinge::Right,
                r#open: true,
                r#half: Half::Lower,
                r#powered: false,
            });
        }
        if state_id == 14147 {
            return Some(CherryDoor {
                r#facing: Facing::West,
                r#half: Half::Upper,
                r#open: true,
                r#hinge: Hinge::Left,
                r#powered: false,
            });
        }
        if state_id == 14130 {
            return Some(CherryDoor {
                r#open: true,
                r#powered: true,
                r#half: Half::Upper,
                r#facing: Facing::South,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 14132 {
            return Some(CherryDoor {
                r#half: Half::Upper,
                r#open: false,
                r#powered: true,
                r#facing: Facing::South,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 14149 {
            return Some(CherryDoor {
                r#powered: false,
                r#facing: Facing::West,
                r#open: false,
                r#hinge: Hinge::Left,
                r#half: Half::Upper,
            });
        }
        if state_id == 14133 {
            return Some(CherryDoor {
                r#open: false,
                r#powered: false,
                r#hinge: Hinge::Left,
                r#half: Half::Upper,
                r#facing: Facing::South,
            });
        }
        if state_id == 14161 {
            return Some(CherryDoor {
                r#facing: Facing::West,
                r#half: Half::Lower,
                r#powered: false,
                r#open: false,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 14125 {
            return Some(CherryDoor {
                r#hinge: Hinge::Left,
                r#facing: Facing::North,
                r#half: Half::Lower,
                r#powered: false,
                r#open: false,
            });
        }
        if state_id == 14115 {
            return Some(CherryDoor {
                r#hinge: Hinge::Left,
                r#facing: Facing::North,
                r#half: Half::Upper,
                r#open: true,
                r#powered: false,
            });
        }
        if state_id == 14177 {
            return Some(CherryDoor {
                r#powered: false,
                r#facing: Facing::East,
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
                r#open: false,
            });
        }
        if state_id == 14136 {
            return Some(CherryDoor {
                r#facing: Facing::South,
                r#hinge: Hinge::Right,
                r#open: false,
                r#half: Half::Upper,
                r#powered: true,
            });
        }
        if state_id == 14165 {
            return Some(CherryDoor {
                r#facing: Facing::East,
                r#hinge: Hinge::Left,
                r#powered: false,
                r#open: false,
                r#half: Half::Upper,
            });
        }
        if state_id == 14124 {
            return Some(CherryDoor {
                r#open: false,
                r#facing: Facing::North,
                r#hinge: Hinge::Left,
                r#powered: true,
                r#half: Half::Lower,
            });
        }
        if state_id == 14153 {
            return Some(CherryDoor {
                r#half: Half::Upper,
                r#facing: Facing::West,
                r#powered: false,
                r#hinge: Hinge::Right,
                r#open: false,
            });
        }
        if state_id == 14144 {
            return Some(CherryDoor {
                r#powered: true,
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
                r#open: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 14163 {
            return Some(CherryDoor {
                r#hinge: Hinge::Left,
                r#half: Half::Upper,
                r#facing: Facing::East,
                r#open: true,
                r#powered: false,
            });
        }
        if state_id == 14171 {
            return Some(CherryDoor {
                r#open: true,
                r#hinge: Hinge::Left,
                r#powered: false,
                r#facing: Facing::East,
                r#half: Half::Lower,
            });
        }
        if state_id == 14118 {
            return Some(CherryDoor {
                r#facing: Facing::North,
                r#powered: true,
                r#half: Half::Upper,
                r#open: true,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 14143 {
            return Some(CherryDoor {
                r#half: Half::Lower,
                r#open: true,
                r#powered: false,
                r#facing: Facing::South,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 14175 {
            return Some(CherryDoor {
                r#powered: false,
                r#half: Half::Lower,
                r#facing: Facing::East,
                r#hinge: Hinge::Right,
                r#open: true,
            });
        }
        if state_id == 14174 {
            return Some(CherryDoor {
                r#hinge: Hinge::Right,
                r#facing: Facing::East,
                r#open: true,
                r#half: Half::Lower,
                r#powered: true,
            });
        }
        return None;
    }
}
