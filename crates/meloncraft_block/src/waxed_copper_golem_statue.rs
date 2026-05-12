use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaxedCopperGolemStatue {
    pub r#copper_golem_pose: CopperGolemPose,
    pub r#facing: Facing,
    pub waterlogged: bool,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CopperGolemPose {
    Standing,
    Sitting,
    Running,
    Star,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for WaxedCopperGolemStatue {
    fn to_id(self) -> i32 {
        if block_state.r#copper_golem_pose == CopperGolemPose::Standing && block_state.r#facing == Facing::North && block_state.r#waterlogged == false { return 27214; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == false && block_state.r#copper_golem_pose == CopperGolemPose::Standing { return 27220; }
        if block_state.r#copper_golem_pose == CopperGolemPose::Running && block_state.r#waterlogged == true && block_state.r#facing == Facing::West { return 27233; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == false && block_state.r#copper_golem_pose == CopperGolemPose::Star { return 27238; }
        if block_state.r#copper_golem_pose == CopperGolemPose::Sitting && block_state.r#facing == Facing::South && block_state.r#waterlogged == false { return 27224; }
        if block_state.r#copper_golem_pose == CopperGolemPose::Star && block_state.r#facing == Facing::West && block_state.r#waterlogged == true { return 27241; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == true && block_state.r#copper_golem_pose == CopperGolemPose::Standing { return 27213; }
        if block_state.r#copper_golem_pose == CopperGolemPose::Star && block_state.r#waterlogged == true && block_state.r#facing == Facing::East { return 27243; }
        if block_state.r#copper_golem_pose == CopperGolemPose::Standing && block_state.r#facing == Facing::West && block_state.r#waterlogged == false { return 27218; }
        if block_state.r#copper_golem_pose == CopperGolemPose::Sitting && block_state.r#facing == Facing::North && block_state.r#waterlogged == false { return 27222; }
        if block_state.r#copper_golem_pose == CopperGolemPose::Star && block_state.r#facing == Facing::North && block_state.r#waterlogged == true { return 27237; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == true && block_state.r#copper_golem_pose == CopperGolemPose::Sitting { return 27225; }
        if block_state.r#facing == Facing::East && block_state.r#copper_golem_pose == CopperGolemPose::Star && block_state.r#waterlogged == false { return 27244; }
        if block_state.r#copper_golem_pose == CopperGolemPose::Sitting && block_state.r#facing == Facing::South && block_state.r#waterlogged == true { return 27223; }
        if block_state.r#copper_golem_pose == CopperGolemPose::Standing && block_state.r#waterlogged == true && block_state.r#facing == Facing::South { return 27215; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == false && block_state.r#copper_golem_pose == CopperGolemPose::Standing { return 27216; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == true && block_state.r#copper_golem_pose == CopperGolemPose::Sitting { return 27227; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == false && block_state.r#copper_golem_pose == CopperGolemPose::Running { return 27234; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == true && block_state.r#copper_golem_pose == CopperGolemPose::Star { return 27239; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::South && block_state.r#copper_golem_pose == CopperGolemPose::Running { return 27232; }
        if block_state.r#copper_golem_pose == CopperGolemPose::Sitting && block_state.r#facing == Facing::West && block_state.r#waterlogged == false { return 27226; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::East && block_state.r#copper_golem_pose == CopperGolemPose::Running { return 27236; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == true && block_state.r#copper_golem_pose == CopperGolemPose::Standing { return 27219; }
        if block_state.r#copper_golem_pose == CopperGolemPose::Star && block_state.r#waterlogged == false && block_state.r#facing == Facing::South { return 27240; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == true && block_state.r#copper_golem_pose == CopperGolemPose::Sitting { return 27221; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == false && block_state.r#copper_golem_pose == CopperGolemPose::Star { return 27242; }
        if block_state.r#copper_golem_pose == CopperGolemPose::Running && block_state.r#waterlogged == true && block_state.r#facing == Facing::North { return 27229; }
        if block_state.r#facing == Facing::North && block_state.r#copper_golem_pose == CopperGolemPose::Running && block_state.r#waterlogged == false { return 27230; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == true && block_state.r#copper_golem_pose == CopperGolemPose::Running { return 27235; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == true && block_state.r#copper_golem_pose == CopperGolemPose::Running { return 27231; }
        if block_state.r#facing == Facing::West && block_state.r#copper_golem_pose == CopperGolemPose::Standing && block_state.r#waterlogged == true { return 27217; }
        if block_state.r#facing == Facing::East && block_state.r#copper_golem_pose == CopperGolemPose::Sitting && block_state.r#waterlogged == false { return 27228; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 27214 {
            return Some(WaxedCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Standing,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 27220 {
            return Some(WaxedCopperGolemStatue {
                r#facing: Facing::East,
                r#waterlogged: false,
                r#copper_golem_pose: CopperGolemPose::Standing,
            });
        }
        if state_id == 27233 {
            return Some(WaxedCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Running,
                r#waterlogged: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 27238 {
            return Some(WaxedCopperGolemStatue {
                r#facing: Facing::North,
                r#waterlogged: false,
                r#copper_golem_pose: CopperGolemPose::Star,
            });
        }
        if state_id == 27224 {
            return Some(WaxedCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Sitting,
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 27241 {
            return Some(WaxedCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Star,
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 27213 {
            return Some(WaxedCopperGolemStatue {
                r#facing: Facing::North,
                r#waterlogged: true,
                r#copper_golem_pose: CopperGolemPose::Standing,
            });
        }
        if state_id == 27243 {
            return Some(WaxedCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Star,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 27218 {
            return Some(WaxedCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Standing,
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 27222 {
            return Some(WaxedCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Sitting,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 27237 {
            return Some(WaxedCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Star,
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 27225 {
            return Some(WaxedCopperGolemStatue {
                r#facing: Facing::West,
                r#waterlogged: true,
                r#copper_golem_pose: CopperGolemPose::Sitting,
            });
        }
        if state_id == 27244 {
            return Some(WaxedCopperGolemStatue {
                r#facing: Facing::East,
                r#copper_golem_pose: CopperGolemPose::Star,
                r#waterlogged: false,
            });
        }
        if state_id == 27223 {
            return Some(WaxedCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Sitting,
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 27215 {
            return Some(WaxedCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Standing,
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 27216 {
            return Some(WaxedCopperGolemStatue {
                r#facing: Facing::South,
                r#waterlogged: false,
                r#copper_golem_pose: CopperGolemPose::Standing,
            });
        }
        if state_id == 27227 {
            return Some(WaxedCopperGolemStatue {
                r#facing: Facing::East,
                r#waterlogged: true,
                r#copper_golem_pose: CopperGolemPose::Sitting,
            });
        }
        if state_id == 27234 {
            return Some(WaxedCopperGolemStatue {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#copper_golem_pose: CopperGolemPose::Running,
            });
        }
        if state_id == 27239 {
            return Some(WaxedCopperGolemStatue {
                r#facing: Facing::South,
                r#waterlogged: true,
                r#copper_golem_pose: CopperGolemPose::Star,
            });
        }
        if state_id == 27232 {
            return Some(WaxedCopperGolemStatue {
                r#waterlogged: false,
                r#facing: Facing::South,
                r#copper_golem_pose: CopperGolemPose::Running,
            });
        }
        if state_id == 27226 {
            return Some(WaxedCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Sitting,
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 27236 {
            return Some(WaxedCopperGolemStatue {
                r#waterlogged: false,
                r#facing: Facing::East,
                r#copper_golem_pose: CopperGolemPose::Running,
            });
        }
        if state_id == 27219 {
            return Some(WaxedCopperGolemStatue {
                r#facing: Facing::East,
                r#waterlogged: true,
                r#copper_golem_pose: CopperGolemPose::Standing,
            });
        }
        if state_id == 27240 {
            return Some(WaxedCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Star,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 27221 {
            return Some(WaxedCopperGolemStatue {
                r#facing: Facing::North,
                r#waterlogged: true,
                r#copper_golem_pose: CopperGolemPose::Sitting,
            });
        }
        if state_id == 27242 {
            return Some(WaxedCopperGolemStatue {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#copper_golem_pose: CopperGolemPose::Star,
            });
        }
        if state_id == 27229 {
            return Some(WaxedCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Running,
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 27230 {
            return Some(WaxedCopperGolemStatue {
                r#facing: Facing::North,
                r#copper_golem_pose: CopperGolemPose::Running,
                r#waterlogged: false,
            });
        }
        if state_id == 27235 {
            return Some(WaxedCopperGolemStatue {
                r#facing: Facing::East,
                r#waterlogged: true,
                r#copper_golem_pose: CopperGolemPose::Running,
            });
        }
        if state_id == 27231 {
            return Some(WaxedCopperGolemStatue {
                r#facing: Facing::South,
                r#waterlogged: true,
                r#copper_golem_pose: CopperGolemPose::Running,
            });
        }
        if state_id == 27217 {
            return Some(WaxedCopperGolemStatue {
                r#facing: Facing::West,
                r#copper_golem_pose: CopperGolemPose::Standing,
                r#waterlogged: true,
            });
        }
        if state_id == 27228 {
            return Some(WaxedCopperGolemStatue {
                r#facing: Facing::East,
                r#copper_golem_pose: CopperGolemPose::Sitting,
                r#waterlogged: false,
            });
        }
        return None;
    }
}

