use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OakSign {
    pub waterlogged: bool,
    pub rotation: i32,
}


impl BlockState for OakSign {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == false && block_state.r#rotation == 15 { return 5165; }
        if block_state.r#rotation == 12 && block_state.r#waterlogged == true { return 5158; }
        if block_state.r#waterlogged == false && block_state.r#rotation == 11 { return 5157; }
        if block_state.r#rotation == 6 && block_state.r#waterlogged == true { return 5146; }
        if block_state.r#rotation == 10 && block_state.r#waterlogged == true { return 5154; }
        if block_state.r#rotation == 12 && block_state.r#waterlogged == false { return 5159; }
        if block_state.r#waterlogged == false && block_state.r#rotation == 2 { return 5139; }
        if block_state.r#rotation == 1 && block_state.r#waterlogged == false { return 5137; }
        if block_state.r#rotation == 9 && block_state.r#waterlogged == false { return 5153; }
        if block_state.r#waterlogged == true && block_state.r#rotation == 11 { return 5156; }
        if block_state.r#rotation == 14 && block_state.r#waterlogged == true { return 5162; }
        if block_state.r#rotation == 10 && block_state.r#waterlogged == false { return 5155; }
        if block_state.r#rotation == 14 && block_state.r#waterlogged == false { return 5163; }
        if block_state.r#rotation == 1 && block_state.r#waterlogged == true { return 5136; }
        if block_state.r#rotation == 8 && block_state.r#waterlogged == true { return 5150; }
        if block_state.r#rotation == 7 && block_state.r#waterlogged == true { return 5148; }
        if block_state.r#waterlogged == true && block_state.r#rotation == 3 { return 5140; }
        if block_state.r#waterlogged == false && block_state.r#rotation == 3 { return 5141; }
        if block_state.r#rotation == 4 && block_state.r#waterlogged == true { return 5142; }
        if block_state.r#rotation == 0 && block_state.r#waterlogged == true { return 5134; }
        if block_state.r#rotation == 0 && block_state.r#waterlogged == false { return 5135; }
        if block_state.r#waterlogged == true && block_state.r#rotation == 2 { return 5138; }
        if block_state.r#waterlogged == false && block_state.r#rotation == 5 { return 5145; }
        if block_state.r#waterlogged == false && block_state.r#rotation == 4 { return 5143; }
        if block_state.r#waterlogged == false && block_state.r#rotation == 8 { return 5151; }
        if block_state.r#waterlogged == true && block_state.r#rotation == 13 { return 5160; }
        if block_state.r#waterlogged == true && block_state.r#rotation == 15 { return 5164; }
        if block_state.r#rotation == 5 && block_state.r#waterlogged == true { return 5144; }
        if block_state.r#rotation == 9 && block_state.r#waterlogged == true { return 5152; }
        if block_state.r#waterlogged == false && block_state.r#rotation == 13 { return 5161; }
        if block_state.r#waterlogged == false && block_state.r#rotation == 7 { return 5149; }
        if block_state.r#rotation == 6 && block_state.r#waterlogged == false { return 5147; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 5165 {
            return Some(OakSign {
                r#waterlogged: false,
                r#rotation: 15,
            });
        }
        if state_id == 5158 {
            return Some(OakSign {
                r#rotation: 12,
                r#waterlogged: true,
            });
        }
        if state_id == 5157 {
            return Some(OakSign {
                r#waterlogged: false,
                r#rotation: 11,
            });
        }
        if state_id == 5146 {
            return Some(OakSign {
                r#rotation: 6,
                r#waterlogged: true,
            });
        }
        if state_id == 5154 {
            return Some(OakSign {
                r#rotation: 10,
                r#waterlogged: true,
            });
        }
        if state_id == 5159 {
            return Some(OakSign {
                r#rotation: 12,
                r#waterlogged: false,
            });
        }
        if state_id == 5139 {
            return Some(OakSign {
                r#waterlogged: false,
                r#rotation: 2,
            });
        }
        if state_id == 5137 {
            return Some(OakSign {
                r#rotation: 1,
                r#waterlogged: false,
            });
        }
        if state_id == 5153 {
            return Some(OakSign {
                r#rotation: 9,
                r#waterlogged: false,
            });
        }
        if state_id == 5156 {
            return Some(OakSign {
                r#waterlogged: true,
                r#rotation: 11,
            });
        }
        if state_id == 5162 {
            return Some(OakSign {
                r#rotation: 14,
                r#waterlogged: true,
            });
        }
        if state_id == 5155 {
            return Some(OakSign {
                r#rotation: 10,
                r#waterlogged: false,
            });
        }
        if state_id == 5163 {
            return Some(OakSign {
                r#rotation: 14,
                r#waterlogged: false,
            });
        }
        if state_id == 5136 {
            return Some(OakSign {
                r#rotation: 1,
                r#waterlogged: true,
            });
        }
        if state_id == 5150 {
            return Some(OakSign {
                r#rotation: 8,
                r#waterlogged: true,
            });
        }
        if state_id == 5148 {
            return Some(OakSign {
                r#rotation: 7,
                r#waterlogged: true,
            });
        }
        if state_id == 5140 {
            return Some(OakSign {
                r#waterlogged: true,
                r#rotation: 3,
            });
        }
        if state_id == 5141 {
            return Some(OakSign {
                r#waterlogged: false,
                r#rotation: 3,
            });
        }
        if state_id == 5142 {
            return Some(OakSign {
                r#rotation: 4,
                r#waterlogged: true,
            });
        }
        if state_id == 5134 {
            return Some(OakSign {
                r#rotation: 0,
                r#waterlogged: true,
            });
        }
        if state_id == 5135 {
            return Some(OakSign {
                r#rotation: 0,
                r#waterlogged: false,
            });
        }
        if state_id == 5138 {
            return Some(OakSign {
                r#waterlogged: true,
                r#rotation: 2,
            });
        }
        if state_id == 5145 {
            return Some(OakSign {
                r#waterlogged: false,
                r#rotation: 5,
            });
        }
        if state_id == 5143 {
            return Some(OakSign {
                r#waterlogged: false,
                r#rotation: 4,
            });
        }
        if state_id == 5151 {
            return Some(OakSign {
                r#waterlogged: false,
                r#rotation: 8,
            });
        }
        if state_id == 5160 {
            return Some(OakSign {
                r#waterlogged: true,
                r#rotation: 13,
            });
        }
        if state_id == 5164 {
            return Some(OakSign {
                r#waterlogged: true,
                r#rotation: 15,
            });
        }
        if state_id == 5144 {
            return Some(OakSign {
                r#rotation: 5,
                r#waterlogged: true,
            });
        }
        if state_id == 5152 {
            return Some(OakSign {
                r#rotation: 9,
                r#waterlogged: true,
            });
        }
        if state_id == 5161 {
            return Some(OakSign {
                r#waterlogged: false,
                r#rotation: 13,
            });
        }
        if state_id == 5149 {
            return Some(OakSign {
                r#waterlogged: false,
                r#rotation: 7,
            });
        }
        if state_id == 5147 {
            return Some(OakSign {
                r#rotation: 6,
                r#waterlogged: false,
            });
        }
        return None;
    }
}

