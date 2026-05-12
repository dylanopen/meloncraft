use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpruceSign {
    pub waterlogged: bool,
    pub rotation: i32,
}


impl BlockState for SpruceSign {
    fn to_id(self) -> i32 {
        if block_state.r#rotation == 10 && block_state.r#waterlogged == false { return 5187; }
        if block_state.r#waterlogged == true && block_state.r#rotation == 11 { return 5188; }
        if block_state.r#rotation == 14 && block_state.r#waterlogged == false { return 5195; }
        if block_state.r#waterlogged == true && block_state.r#rotation == 6 { return 5178; }
        if block_state.r#rotation == 7 && block_state.r#waterlogged == true { return 5180; }
        if block_state.r#rotation == 1 && block_state.r#waterlogged == true { return 5168; }
        if block_state.r#waterlogged == false && block_state.r#rotation == 7 { return 5181; }
        if block_state.r#rotation == 0 && block_state.r#waterlogged == false { return 5167; }
        if block_state.r#waterlogged == true && block_state.r#rotation == 9 { return 5184; }
        if block_state.r#rotation == 13 && block_state.r#waterlogged == true { return 5192; }
        if block_state.r#rotation == 3 && block_state.r#waterlogged == true { return 5172; }
        if block_state.r#rotation == 5 && block_state.r#waterlogged == false { return 5177; }
        if block_state.r#waterlogged == false && block_state.r#rotation == 8 { return 5183; }
        if block_state.r#waterlogged == true && block_state.r#rotation == 8 { return 5182; }
        if block_state.r#rotation == 5 && block_state.r#waterlogged == true { return 5176; }
        if block_state.r#waterlogged == true && block_state.r#rotation == 0 { return 5166; }
        if block_state.r#rotation == 12 && block_state.r#waterlogged == false { return 5191; }
        if block_state.r#waterlogged == false && block_state.r#rotation == 13 { return 5193; }
        if block_state.r#waterlogged == false && block_state.r#rotation == 15 { return 5197; }
        if block_state.r#rotation == 10 && block_state.r#waterlogged == true { return 5186; }
        if block_state.r#waterlogged == false && block_state.r#rotation == 2 { return 5171; }
        if block_state.r#waterlogged == true && block_state.r#rotation == 2 { return 5170; }
        if block_state.r#waterlogged == false && block_state.r#rotation == 9 { return 5185; }
        if block_state.r#rotation == 4 && block_state.r#waterlogged == true { return 5174; }
        if block_state.r#waterlogged == false && block_state.r#rotation == 11 { return 5189; }
        if block_state.r#rotation == 12 && block_state.r#waterlogged == true { return 5190; }
        if block_state.r#rotation == 15 && block_state.r#waterlogged == true { return 5196; }
        if block_state.r#rotation == 1 && block_state.r#waterlogged == false { return 5169; }
        if block_state.r#waterlogged == false && block_state.r#rotation == 4 { return 5175; }
        if block_state.r#waterlogged == false && block_state.r#rotation == 3 { return 5173; }
        if block_state.r#rotation == 14 && block_state.r#waterlogged == true { return 5194; }
        if block_state.r#waterlogged == false && block_state.r#rotation == 6 { return 5179; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 5187 {
            return Some(SpruceSign {
                r#rotation: 10,
                r#waterlogged: false,
            });
        }
        if state_id == 5188 {
            return Some(SpruceSign {
                r#waterlogged: true,
                r#rotation: 11,
            });
        }
        if state_id == 5195 {
            return Some(SpruceSign {
                r#rotation: 14,
                r#waterlogged: false,
            });
        }
        if state_id == 5178 {
            return Some(SpruceSign {
                r#waterlogged: true,
                r#rotation: 6,
            });
        }
        if state_id == 5180 {
            return Some(SpruceSign {
                r#rotation: 7,
                r#waterlogged: true,
            });
        }
        if state_id == 5168 {
            return Some(SpruceSign {
                r#rotation: 1,
                r#waterlogged: true,
            });
        }
        if state_id == 5181 {
            return Some(SpruceSign {
                r#waterlogged: false,
                r#rotation: 7,
            });
        }
        if state_id == 5167 {
            return Some(SpruceSign {
                r#rotation: 0,
                r#waterlogged: false,
            });
        }
        if state_id == 5184 {
            return Some(SpruceSign {
                r#waterlogged: true,
                r#rotation: 9,
            });
        }
        if state_id == 5192 {
            return Some(SpruceSign {
                r#rotation: 13,
                r#waterlogged: true,
            });
        }
        if state_id == 5172 {
            return Some(SpruceSign {
                r#rotation: 3,
                r#waterlogged: true,
            });
        }
        if state_id == 5177 {
            return Some(SpruceSign {
                r#rotation: 5,
                r#waterlogged: false,
            });
        }
        if state_id == 5183 {
            return Some(SpruceSign {
                r#waterlogged: false,
                r#rotation: 8,
            });
        }
        if state_id == 5182 {
            return Some(SpruceSign {
                r#waterlogged: true,
                r#rotation: 8,
            });
        }
        if state_id == 5176 {
            return Some(SpruceSign {
                r#rotation: 5,
                r#waterlogged: true,
            });
        }
        if state_id == 5166 {
            return Some(SpruceSign {
                r#waterlogged: true,
                r#rotation: 0,
            });
        }
        if state_id == 5191 {
            return Some(SpruceSign {
                r#rotation: 12,
                r#waterlogged: false,
            });
        }
        if state_id == 5193 {
            return Some(SpruceSign {
                r#waterlogged: false,
                r#rotation: 13,
            });
        }
        if state_id == 5197 {
            return Some(SpruceSign {
                r#waterlogged: false,
                r#rotation: 15,
            });
        }
        if state_id == 5186 {
            return Some(SpruceSign {
                r#rotation: 10,
                r#waterlogged: true,
            });
        }
        if state_id == 5171 {
            return Some(SpruceSign {
                r#waterlogged: false,
                r#rotation: 2,
            });
        }
        if state_id == 5170 {
            return Some(SpruceSign {
                r#waterlogged: true,
                r#rotation: 2,
            });
        }
        if state_id == 5185 {
            return Some(SpruceSign {
                r#waterlogged: false,
                r#rotation: 9,
            });
        }
        if state_id == 5174 {
            return Some(SpruceSign {
                r#rotation: 4,
                r#waterlogged: true,
            });
        }
        if state_id == 5189 {
            return Some(SpruceSign {
                r#waterlogged: false,
                r#rotation: 11,
            });
        }
        if state_id == 5190 {
            return Some(SpruceSign {
                r#rotation: 12,
                r#waterlogged: true,
            });
        }
        if state_id == 5196 {
            return Some(SpruceSign {
                r#rotation: 15,
                r#waterlogged: true,
            });
        }
        if state_id == 5169 {
            return Some(SpruceSign {
                r#rotation: 1,
                r#waterlogged: false,
            });
        }
        if state_id == 5175 {
            return Some(SpruceSign {
                r#waterlogged: false,
                r#rotation: 4,
            });
        }
        if state_id == 5173 {
            return Some(SpruceSign {
                r#waterlogged: false,
                r#rotation: 3,
            });
        }
        if state_id == 5194 {
            return Some(SpruceSign {
                r#rotation: 14,
                r#waterlogged: true,
            });
        }
        if state_id == 5179 {
            return Some(SpruceSign {
                r#waterlogged: false,
                r#rotation: 6,
            });
        }
        return None;
    }
}

