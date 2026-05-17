use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OakSign {
    pub rotation: i32,
    pub waterlogged: bool,
}


impl BlockState for OakSign {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == false && self.r#rotation == 3 { return 5141; }
        if self.r#waterlogged == true && self.r#rotation == 1 { return 5136; }
        if self.r#rotation == 10 && self.r#waterlogged == false { return 5155; }
        if self.r#rotation == 14 && self.r#waterlogged == true { return 5162; }
        if self.r#rotation == 3 && self.r#waterlogged == true { return 5140; }
        if self.r#waterlogged == false && self.r#rotation == 15 { return 5165; }
        if self.r#rotation == 8 && self.r#waterlogged == false { return 5151; }
        if self.r#rotation == 13 && self.r#waterlogged == true { return 5160; }
        if self.r#rotation == 5 && self.r#waterlogged == false { return 5145; }
        if self.r#rotation == 5 && self.r#waterlogged == true { return 5144; }
        if self.r#waterlogged == true && self.r#rotation == 7 { return 5148; }
        if self.r#rotation == 13 && self.r#waterlogged == false { return 5161; }
        if self.r#waterlogged == false && self.r#rotation == 2 { return 5139; }
        if self.r#waterlogged == true && self.r#rotation == 0 { return 5134; }
        if self.r#rotation == 4 && self.r#waterlogged == true { return 5142; }
        if self.r#waterlogged == false && self.r#rotation == 11 { return 5157; }
        if self.r#waterlogged == false && self.r#rotation == 1 { return 5137; }
        if self.r#waterlogged == false && self.r#rotation == 9 { return 5153; }
        if self.r#waterlogged == false && self.r#rotation == 14 { return 5163; }
        if self.r#rotation == 4 && self.r#waterlogged == false { return 5143; }
        if self.r#waterlogged == true && self.r#rotation == 8 { return 5150; }
        if self.r#waterlogged == true && self.r#rotation == 6 { return 5146; }
        if self.r#rotation == 2 && self.r#waterlogged == true { return 5138; }
        if self.r#rotation == 7 && self.r#waterlogged == false { return 5149; }
        if self.r#rotation == 10 && self.r#waterlogged == true { return 5154; }
        if self.r#waterlogged == true && self.r#rotation == 11 { return 5156; }
        if self.r#rotation == 12 && self.r#waterlogged == true { return 5158; }
        if self.r#rotation == 12 && self.r#waterlogged == false { return 5159; }
        if self.r#rotation == 15 && self.r#waterlogged == true { return 5164; }
        if self.r#waterlogged == false && self.r#rotation == 6 { return 5147; }
        if self.r#waterlogged == true && self.r#rotation == 9 { return 5152; }
        if self.r#rotation == 0 && self.r#waterlogged == false { return 5135; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 5141 {
            return Some(OakSign {
                r#waterlogged: false,
                r#rotation: 3,
            });
        }
        if state_id == 5136 {
            return Some(OakSign {
                r#waterlogged: true,
                r#rotation: 1,
            });
        }
        if state_id == 5155 {
            return Some(OakSign {
                r#rotation: 10,
                r#waterlogged: false,
            });
        }
        if state_id == 5162 {
            return Some(OakSign {
                r#rotation: 14,
                r#waterlogged: true,
            });
        }
        if state_id == 5140 {
            return Some(OakSign {
                r#rotation: 3,
                r#waterlogged: true,
            });
        }
        if state_id == 5165 {
            return Some(OakSign {
                r#waterlogged: false,
                r#rotation: 15,
            });
        }
        if state_id == 5151 {
            return Some(OakSign {
                r#rotation: 8,
                r#waterlogged: false,
            });
        }
        if state_id == 5160 {
            return Some(OakSign {
                r#rotation: 13,
                r#waterlogged: true,
            });
        }
        if state_id == 5145 {
            return Some(OakSign {
                r#rotation: 5,
                r#waterlogged: false,
            });
        }
        if state_id == 5144 {
            return Some(OakSign {
                r#rotation: 5,
                r#waterlogged: true,
            });
        }
        if state_id == 5148 {
            return Some(OakSign {
                r#waterlogged: true,
                r#rotation: 7,
            });
        }
        if state_id == 5161 {
            return Some(OakSign {
                r#rotation: 13,
                r#waterlogged: false,
            });
        }
        if state_id == 5139 {
            return Some(OakSign {
                r#waterlogged: false,
                r#rotation: 2,
            });
        }
        if state_id == 5134 {
            return Some(OakSign {
                r#waterlogged: true,
                r#rotation: 0,
            });
        }
        if state_id == 5142 {
            return Some(OakSign {
                r#rotation: 4,
                r#waterlogged: true,
            });
        }
        if state_id == 5157 {
            return Some(OakSign {
                r#waterlogged: false,
                r#rotation: 11,
            });
        }
        if state_id == 5137 {
            return Some(OakSign {
                r#waterlogged: false,
                r#rotation: 1,
            });
        }
        if state_id == 5153 {
            return Some(OakSign {
                r#waterlogged: false,
                r#rotation: 9,
            });
        }
        if state_id == 5163 {
            return Some(OakSign {
                r#waterlogged: false,
                r#rotation: 14,
            });
        }
        if state_id == 5143 {
            return Some(OakSign {
                r#rotation: 4,
                r#waterlogged: false,
            });
        }
        if state_id == 5150 {
            return Some(OakSign {
                r#waterlogged: true,
                r#rotation: 8,
            });
        }
        if state_id == 5146 {
            return Some(OakSign {
                r#waterlogged: true,
                r#rotation: 6,
            });
        }
        if state_id == 5138 {
            return Some(OakSign {
                r#rotation: 2,
                r#waterlogged: true,
            });
        }
        if state_id == 5149 {
            return Some(OakSign {
                r#rotation: 7,
                r#waterlogged: false,
            });
        }
        if state_id == 5154 {
            return Some(OakSign {
                r#rotation: 10,
                r#waterlogged: true,
            });
        }
        if state_id == 5156 {
            return Some(OakSign {
                r#waterlogged: true,
                r#rotation: 11,
            });
        }
        if state_id == 5158 {
            return Some(OakSign {
                r#rotation: 12,
                r#waterlogged: true,
            });
        }
        if state_id == 5159 {
            return Some(OakSign {
                r#rotation: 12,
                r#waterlogged: false,
            });
        }
        if state_id == 5164 {
            return Some(OakSign {
                r#rotation: 15,
                r#waterlogged: true,
            });
        }
        if state_id == 5147 {
            return Some(OakSign {
                r#waterlogged: false,
                r#rotation: 6,
            });
        }
        if state_id == 5152 {
            return Some(OakSign {
                r#waterlogged: true,
                r#rotation: 9,
            });
        }
        if state_id == 5135 {
            return Some(OakSign {
                r#rotation: 0,
                r#waterlogged: false,
            });
        }
        return None;
    }
}

