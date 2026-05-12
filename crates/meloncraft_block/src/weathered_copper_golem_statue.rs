use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WeatheredCopperGolemStatue {
    pub waterlogged: bool,
    pub r#facing: Facing,
    pub r#copper_golem_pose: CopperGolemPose,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CopperGolemPose {
    Standing,
    Sitting,
    Running,
    Star,
}

impl BlockState for WeatheredCopperGolemStatue {
    fn to_id(self) -> i32 {
        if block_state.r#facing == Facing::West && block_state.r#copper_golem_pose == CopperGolemPose::Sitting && block_state.r#waterlogged == false { return 27162; }
        if block_state.r#waterlogged == true && block_state.r#copper_golem_pose == CopperGolemPose::Star && block_state.r#facing == Facing::West { return 27177; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::North && block_state.r#copper_golem_pose == CopperGolemPose::Running { return 27166; }
        if block_state.r#copper_golem_pose == CopperGolemPose::Sitting && block_state.r#facing == Facing::East && block_state.r#waterlogged == false { return 27164; }
        if block_state.r#copper_golem_pose == CopperGolemPose::Star && block_state.r#facing == Facing::East && block_state.r#waterlogged == false { return 27180; }
        if block_state.r#copper_golem_pose == CopperGolemPose::Standing && block_state.r#facing == Facing::East && block_state.r#waterlogged == true { return 27155; }
        if block_state.r#copper_golem_pose == CopperGolemPose::Standing && block_state.r#facing == Facing::South && block_state.r#waterlogged == true { return 27151; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::South && block_state.r#copper_golem_pose == CopperGolemPose::Running { return 27167; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == true && block_state.r#copper_golem_pose == CopperGolemPose::Running { return 27171; }
        if block_state.r#copper_golem_pose == CopperGolemPose::Sitting && block_state.r#facing == Facing::South && block_state.r#waterlogged == true { return 27159; }
        if block_state.r#waterlogged == true && block_state.r#copper_golem_pose == CopperGolemPose::Sitting && block_state.r#facing == Facing::East { return 27163; }
        if block_state.r#facing == Facing::North && block_state.r#copper_golem_pose == CopperGolemPose::Running && block_state.r#waterlogged == true { return 27165; }
        if block_state.r#copper_golem_pose == CopperGolemPose::Star && block_state.r#facing == Facing::North && block_state.r#waterlogged == true { return 27173; }
        if block_state.r#facing == Facing::West && block_state.r#copper_golem_pose == CopperGolemPose::Standing && block_state.r#waterlogged == true { return 27153; }
        if block_state.r#waterlogged == false && block_state.r#copper_golem_pose == CopperGolemPose::Sitting && block_state.r#facing == Facing::North { return 27158; }
        if block_state.r#facing == Facing::North && block_state.r#copper_golem_pose == CopperGolemPose::Standing && block_state.r#waterlogged == true { return 27149; }
        if block_state.r#copper_golem_pose == CopperGolemPose::Star && block_state.r#waterlogged == true && block_state.r#facing == Facing::South { return 27175; }
        if block_state.r#facing == Facing::South && block_state.r#copper_golem_pose == CopperGolemPose::Standing && block_state.r#waterlogged == false { return 27152; }
        if block_state.r#copper_golem_pose == CopperGolemPose::Sitting && block_state.r#waterlogged == true && block_state.r#facing == Facing::West { return 27161; }
        if block_state.r#waterlogged == true && block_state.r#copper_golem_pose == CopperGolemPose::Star && block_state.r#facing == Facing::East { return 27179; }
        if block_state.r#waterlogged == false && block_state.r#copper_golem_pose == CopperGolemPose::Star && block_state.r#facing == Facing::South { return 27176; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::West && block_state.r#copper_golem_pose == CopperGolemPose::Standing { return 27154; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == false && block_state.r#copper_golem_pose == CopperGolemPose::Standing { return 27156; }
        if block_state.r#copper_golem_pose == CopperGolemPose::Star && block_state.r#waterlogged == false && block_state.r#facing == Facing::West { return 27178; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == true && block_state.r#copper_golem_pose == CopperGolemPose::Running { return 27169; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == false && block_state.r#copper_golem_pose == CopperGolemPose::Sitting { return 27160; }
        if block_state.r#copper_golem_pose == CopperGolemPose::Running && block_state.r#facing == Facing::South && block_state.r#waterlogged == false { return 27168; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::West && block_state.r#copper_golem_pose == CopperGolemPose::Running { return 27170; }
        if block_state.r#copper_golem_pose == CopperGolemPose::Sitting && block_state.r#facing == Facing::North && block_state.r#waterlogged == true { return 27157; }
        if block_state.r#facing == Facing::North && block_state.r#copper_golem_pose == CopperGolemPose::Standing && block_state.r#waterlogged == false { return 27150; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::North && block_state.r#copper_golem_pose == CopperGolemPose::Star { return 27174; }
        if block_state.r#copper_golem_pose == CopperGolemPose::Running && block_state.r#facing == Facing::East && block_state.r#waterlogged == false { return 27172; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 27162 {
            return Some(WeatheredCopperGolemStatue {
                r#facing: Facing::West,
                r#copper_golem_pose: CopperGolemPose::Sitting,
                r#waterlogged: false,
            });
        }
        if state_id == 27177 {
            return Some(WeatheredCopperGolemStatue {
                r#waterlogged: true,
                r#copper_golem_pose: CopperGolemPose::Star,
                r#facing: Facing::West,
            });
        }
        if state_id == 27166 {
            return Some(WeatheredCopperGolemStatue {
                r#waterlogged: false,
                r#facing: Facing::North,
                r#copper_golem_pose: CopperGolemPose::Running,
            });
        }
        if state_id == 27164 {
            return Some(WeatheredCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Sitting,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 27180 {
            return Some(WeatheredCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Star,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 27155 {
            return Some(WeatheredCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Standing,
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 27151 {
            return Some(WeatheredCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Standing,
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 27167 {
            return Some(WeatheredCopperGolemStatue {
                r#waterlogged: true,
                r#facing: Facing::South,
                r#copper_golem_pose: CopperGolemPose::Running,
            });
        }
        if state_id == 27171 {
            return Some(WeatheredCopperGolemStatue {
                r#facing: Facing::East,
                r#waterlogged: true,
                r#copper_golem_pose: CopperGolemPose::Running,
            });
        }
        if state_id == 27159 {
            return Some(WeatheredCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Sitting,
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 27163 {
            return Some(WeatheredCopperGolemStatue {
                r#waterlogged: true,
                r#copper_golem_pose: CopperGolemPose::Sitting,
                r#facing: Facing::East,
            });
        }
        if state_id == 27165 {
            return Some(WeatheredCopperGolemStatue {
                r#facing: Facing::North,
                r#copper_golem_pose: CopperGolemPose::Running,
                r#waterlogged: true,
            });
        }
        if state_id == 27173 {
            return Some(WeatheredCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Star,
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 27153 {
            return Some(WeatheredCopperGolemStatue {
                r#facing: Facing::West,
                r#copper_golem_pose: CopperGolemPose::Standing,
                r#waterlogged: true,
            });
        }
        if state_id == 27158 {
            return Some(WeatheredCopperGolemStatue {
                r#waterlogged: false,
                r#copper_golem_pose: CopperGolemPose::Sitting,
                r#facing: Facing::North,
            });
        }
        if state_id == 27149 {
            return Some(WeatheredCopperGolemStatue {
                r#facing: Facing::North,
                r#copper_golem_pose: CopperGolemPose::Standing,
                r#waterlogged: true,
            });
        }
        if state_id == 27175 {
            return Some(WeatheredCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Star,
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 27152 {
            return Some(WeatheredCopperGolemStatue {
                r#facing: Facing::South,
                r#copper_golem_pose: CopperGolemPose::Standing,
                r#waterlogged: false,
            });
        }
        if state_id == 27161 {
            return Some(WeatheredCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Sitting,
                r#waterlogged: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 27179 {
            return Some(WeatheredCopperGolemStatue {
                r#waterlogged: true,
                r#copper_golem_pose: CopperGolemPose::Star,
                r#facing: Facing::East,
            });
        }
        if state_id == 27176 {
            return Some(WeatheredCopperGolemStatue {
                r#waterlogged: false,
                r#copper_golem_pose: CopperGolemPose::Star,
                r#facing: Facing::South,
            });
        }
        if state_id == 27154 {
            return Some(WeatheredCopperGolemStatue {
                r#waterlogged: false,
                r#facing: Facing::West,
                r#copper_golem_pose: CopperGolemPose::Standing,
            });
        }
        if state_id == 27156 {
            return Some(WeatheredCopperGolemStatue {
                r#facing: Facing::East,
                r#waterlogged: false,
                r#copper_golem_pose: CopperGolemPose::Standing,
            });
        }
        if state_id == 27178 {
            return Some(WeatheredCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Star,
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 27169 {
            return Some(WeatheredCopperGolemStatue {
                r#facing: Facing::West,
                r#waterlogged: true,
                r#copper_golem_pose: CopperGolemPose::Running,
            });
        }
        if state_id == 27160 {
            return Some(WeatheredCopperGolemStatue {
                r#facing: Facing::South,
                r#waterlogged: false,
                r#copper_golem_pose: CopperGolemPose::Sitting,
            });
        }
        if state_id == 27168 {
            return Some(WeatheredCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Running,
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 27170 {
            return Some(WeatheredCopperGolemStatue {
                r#waterlogged: false,
                r#facing: Facing::West,
                r#copper_golem_pose: CopperGolemPose::Running,
            });
        }
        if state_id == 27157 {
            return Some(WeatheredCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Sitting,
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 27150 {
            return Some(WeatheredCopperGolemStatue {
                r#facing: Facing::North,
                r#copper_golem_pose: CopperGolemPose::Standing,
                r#waterlogged: false,
            });
        }
        if state_id == 27174 {
            return Some(WeatheredCopperGolemStatue {
                r#waterlogged: false,
                r#facing: Facing::North,
                r#copper_golem_pose: CopperGolemPose::Star,
            });
        }
        if state_id == 27172 {
            return Some(WeatheredCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Running,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        return None;
    }
}

