use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WeatheredCopperChest {
    pub waterlogged: bool,
    pub r#type: Type,
    pub r#facing: Facing,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Single,
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

impl BlockState for WeatheredCopperChest {
    fn to_id(self) -> i32 {
        if block_state.r#type == Type::Left && block_state.r#waterlogged == false && block_state.r#facing == Facing::West { return 26956; }
        if block_state.r#type == Type::Left && block_state.r#facing == Facing::North && block_state.r#waterlogged == false { return 26944; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::South && block_state.r#type == Type::Right { return 26951; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == true && block_state.r#type == Type::Right { return 26957; }
        if block_state.r#type == Type::Left && block_state.r#waterlogged == true && block_state.r#facing == Facing::West { return 26955; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == false && block_state.r#type == Type::Right { return 26958; }
        if block_state.r#waterlogged == true && block_state.r#type == Type::Single && block_state.r#facing == Facing::East { return 26959; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::East && block_state.r#type == Type::Right { return 26963; }
        if block_state.r#waterlogged == true && block_state.r#type == Type::Single && block_state.r#facing == Facing::South { return 26947; }
        if block_state.r#waterlogged == true && block_state.r#type == Type::Single && block_state.r#facing == Facing::West { return 26953; }
        if block_state.r#waterlogged == false && block_state.r#type == Type::Right && block_state.r#facing == Facing::North { return 26946; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == true && block_state.r#type == Type::Left { return 26943; }
        if block_state.r#waterlogged == false && block_state.r#type == Type::Single && block_state.r#facing == Facing::North { return 26942; }
        if block_state.r#waterlogged == true && block_state.r#type == Type::Left && block_state.r#facing == Facing::South { return 26949; }
        if block_state.r#type == Type::Single && block_state.r#facing == Facing::South && block_state.r#waterlogged == false { return 26948; }
        if block_state.r#waterlogged == false && block_state.r#type == Type::Left && block_state.r#facing == Facing::South { return 26950; }
        if block_state.r#waterlogged == false && block_state.r#type == Type::Right && block_state.r#facing == Facing::East { return 26964; }
        if block_state.r#type == Type::Single && block_state.r#waterlogged == true && block_state.r#facing == Facing::North { return 26941; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::East && block_state.r#type == Type::Single { return 26960; }
        if block_state.r#waterlogged == true && block_state.r#type == Type::Left && block_state.r#facing == Facing::East { return 26961; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == false && block_state.r#type == Type::Left { return 26962; }
        if block_state.r#type == Type::Right && block_state.r#waterlogged == false && block_state.r#facing == Facing::South { return 26952; }
        if block_state.r#facing == Facing::West && block_state.r#type == Type::Single && block_state.r#waterlogged == false { return 26954; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::North && block_state.r#type == Type::Right { return 26945; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 26956 {
            return Some(WeatheredCopperChest {
                r#type: Type::Left,
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 26944 {
            return Some(WeatheredCopperChest {
                r#type: Type::Left,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 26951 {
            return Some(WeatheredCopperChest {
                r#waterlogged: true,
                r#facing: Facing::South,
                r#type: Type::Right,
            });
        }
        if state_id == 26957 {
            return Some(WeatheredCopperChest {
                r#facing: Facing::West,
                r#waterlogged: true,
                r#type: Type::Right,
            });
        }
        if state_id == 26955 {
            return Some(WeatheredCopperChest {
                r#type: Type::Left,
                r#waterlogged: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 26958 {
            return Some(WeatheredCopperChest {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#type: Type::Right,
            });
        }
        if state_id == 26959 {
            return Some(WeatheredCopperChest {
                r#waterlogged: true,
                r#type: Type::Single,
                r#facing: Facing::East,
            });
        }
        if state_id == 26963 {
            return Some(WeatheredCopperChest {
                r#waterlogged: true,
                r#facing: Facing::East,
                r#type: Type::Right,
            });
        }
        if state_id == 26947 {
            return Some(WeatheredCopperChest {
                r#waterlogged: true,
                r#type: Type::Single,
                r#facing: Facing::South,
            });
        }
        if state_id == 26953 {
            return Some(WeatheredCopperChest {
                r#waterlogged: true,
                r#type: Type::Single,
                r#facing: Facing::West,
            });
        }
        if state_id == 26946 {
            return Some(WeatheredCopperChest {
                r#waterlogged: false,
                r#type: Type::Right,
                r#facing: Facing::North,
            });
        }
        if state_id == 26943 {
            return Some(WeatheredCopperChest {
                r#facing: Facing::North,
                r#waterlogged: true,
                r#type: Type::Left,
            });
        }
        if state_id == 26942 {
            return Some(WeatheredCopperChest {
                r#waterlogged: false,
                r#type: Type::Single,
                r#facing: Facing::North,
            });
        }
        if state_id == 26949 {
            return Some(WeatheredCopperChest {
                r#waterlogged: true,
                r#type: Type::Left,
                r#facing: Facing::South,
            });
        }
        if state_id == 26948 {
            return Some(WeatheredCopperChest {
                r#type: Type::Single,
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 26950 {
            return Some(WeatheredCopperChest {
                r#waterlogged: false,
                r#type: Type::Left,
                r#facing: Facing::South,
            });
        }
        if state_id == 26964 {
            return Some(WeatheredCopperChest {
                r#waterlogged: false,
                r#type: Type::Right,
                r#facing: Facing::East,
            });
        }
        if state_id == 26941 {
            return Some(WeatheredCopperChest {
                r#type: Type::Single,
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 26960 {
            return Some(WeatheredCopperChest {
                r#waterlogged: false,
                r#facing: Facing::East,
                r#type: Type::Single,
            });
        }
        if state_id == 26961 {
            return Some(WeatheredCopperChest {
                r#waterlogged: true,
                r#type: Type::Left,
                r#facing: Facing::East,
            });
        }
        if state_id == 26962 {
            return Some(WeatheredCopperChest {
                r#facing: Facing::East,
                r#waterlogged: false,
                r#type: Type::Left,
            });
        }
        if state_id == 26952 {
            return Some(WeatheredCopperChest {
                r#type: Type::Right,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 26954 {
            return Some(WeatheredCopperChest {
                r#facing: Facing::West,
                r#type: Type::Single,
                r#waterlogged: false,
            });
        }
        if state_id == 26945 {
            return Some(WeatheredCopperChest {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#type: Type::Right,
            });
        }
        return None;
    }
}

