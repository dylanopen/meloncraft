use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaxedExposedCopperGolemStatue {
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

impl BlockState for WaxedExposedCopperGolemStatue {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::West && block_state.r#copper_golem_pose == CopperGolemPose::Standing { return 27250; }
        if block_state.r#facing == Facing::East && block_state.r#copper_golem_pose == CopperGolemPose::Running && block_state.r#waterlogged == true { return 27267; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == false && block_state.r#copper_golem_pose == CopperGolemPose::Running { return 27264; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == false && block_state.r#copper_golem_pose == CopperGolemPose::Standing { return 27248; }
        if block_state.r#facing == Facing::East && block_state.r#copper_golem_pose == CopperGolemPose::Star && block_state.r#waterlogged == false { return 27276; }
        if block_state.r#copper_golem_pose == CopperGolemPose::Star && block_state.r#waterlogged == true && block_state.r#facing == Facing::South { return 27271; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::South && block_state.r#copper_golem_pose == CopperGolemPose::Sitting { return 27256; }
        if block_state.r#copper_golem_pose == CopperGolemPose::Sitting && block_state.r#facing == Facing::East && block_state.r#waterlogged == false { return 27260; }
        if block_state.r#copper_golem_pose == CopperGolemPose::Running && block_state.r#waterlogged == true && block_state.r#facing == Facing::South { return 27263; }
        if block_state.r#facing == Facing::East && block_state.r#copper_golem_pose == CopperGolemPose::Star && block_state.r#waterlogged == true { return 27275; }
        if block_state.r#copper_golem_pose == CopperGolemPose::Standing && block_state.r#facing == Facing::East && block_state.r#waterlogged == false { return 27252; }
        if block_state.r#facing == Facing::North && block_state.r#copper_golem_pose == CopperGolemPose::Star && block_state.r#waterlogged == true { return 27269; }
        if block_state.r#waterlogged == true && block_state.r#copper_golem_pose == CopperGolemPose::Sitting && block_state.r#facing == Facing::South { return 27255; }
        if block_state.r#waterlogged == false && block_state.r#copper_golem_pose == CopperGolemPose::Star && block_state.r#facing == Facing::West { return 27274; }
        if block_state.r#copper_golem_pose == CopperGolemPose::Standing && block_state.r#waterlogged == true && block_state.r#facing == Facing::West { return 27249; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == true && block_state.r#copper_golem_pose == CopperGolemPose::Sitting { return 27253; }
        if block_state.r#copper_golem_pose == CopperGolemPose::Star && block_state.r#facing == Facing::West && block_state.r#waterlogged == true { return 27273; }
        if block_state.r#facing == Facing::East && block_state.r#copper_golem_pose == CopperGolemPose::Standing && block_state.r#waterlogged == true { return 27251; }
        if block_state.r#copper_golem_pose == CopperGolemPose::Sitting && block_state.r#waterlogged == true && block_state.r#facing == Facing::West { return 27257; }
        if block_state.r#waterlogged == false && block_state.r#copper_golem_pose == CopperGolemPose::Sitting && block_state.r#facing == Facing::West { return 27258; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == true && block_state.r#copper_golem_pose == CopperGolemPose::Running { return 27261; }
        if block_state.r#copper_golem_pose == CopperGolemPose::Running && block_state.r#facing == Facing::West && block_state.r#waterlogged == true { return 27265; }
        if block_state.r#copper_golem_pose == CopperGolemPose::Star && block_state.r#facing == Facing::South && block_state.r#waterlogged == false { return 27272; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::North && block_state.r#copper_golem_pose == CopperGolemPose::Standing { return 27245; }
        if block_state.r#copper_golem_pose == CopperGolemPose::Running && block_state.r#facing == Facing::East && block_state.r#waterlogged == false { return 27268; }
        if block_state.r#facing == Facing::North && block_state.r#copper_golem_pose == CopperGolemPose::Star && block_state.r#waterlogged == false { return 27270; }
        if block_state.r#waterlogged == false && block_state.r#copper_golem_pose == CopperGolemPose::Sitting && block_state.r#facing == Facing::North { return 27254; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::South && block_state.r#copper_golem_pose == CopperGolemPose::Standing { return 27247; }
        if block_state.r#waterlogged == false && block_state.r#copper_golem_pose == CopperGolemPose::Running && block_state.r#facing == Facing::North { return 27262; }
        if block_state.r#copper_golem_pose == CopperGolemPose::Sitting && block_state.r#waterlogged == true && block_state.r#facing == Facing::East { return 27259; }
        if block_state.r#copper_golem_pose == CopperGolemPose::Standing && block_state.r#facing == Facing::North && block_state.r#waterlogged == false { return 27246; }
        if block_state.r#facing == Facing::West && block_state.r#copper_golem_pose == CopperGolemPose::Running && block_state.r#waterlogged == false { return 27266; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 27250 {
            return Some(WaxedExposedCopperGolemStatue {
                r#waterlogged: false,
                r#facing: Facing::West,
                r#copper_golem_pose: CopperGolemPose::Standing,
            });
        }
        if state_id == 27267 {
            return Some(WaxedExposedCopperGolemStatue {
                r#facing: Facing::East,
                r#copper_golem_pose: CopperGolemPose::Running,
                r#waterlogged: true,
            });
        }
        if state_id == 27264 {
            return Some(WaxedExposedCopperGolemStatue {
                r#facing: Facing::South,
                r#waterlogged: false,
                r#copper_golem_pose: CopperGolemPose::Running,
            });
        }
        if state_id == 27248 {
            return Some(WaxedExposedCopperGolemStatue {
                r#facing: Facing::South,
                r#waterlogged: false,
                r#copper_golem_pose: CopperGolemPose::Standing,
            });
        }
        if state_id == 27276 {
            return Some(WaxedExposedCopperGolemStatue {
                r#facing: Facing::East,
                r#copper_golem_pose: CopperGolemPose::Star,
                r#waterlogged: false,
            });
        }
        if state_id == 27271 {
            return Some(WaxedExposedCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Star,
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 27256 {
            return Some(WaxedExposedCopperGolemStatue {
                r#waterlogged: false,
                r#facing: Facing::South,
                r#copper_golem_pose: CopperGolemPose::Sitting,
            });
        }
        if state_id == 27260 {
            return Some(WaxedExposedCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Sitting,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 27263 {
            return Some(WaxedExposedCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Running,
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 27275 {
            return Some(WaxedExposedCopperGolemStatue {
                r#facing: Facing::East,
                r#copper_golem_pose: CopperGolemPose::Star,
                r#waterlogged: true,
            });
        }
        if state_id == 27252 {
            return Some(WaxedExposedCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Standing,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 27269 {
            return Some(WaxedExposedCopperGolemStatue {
                r#facing: Facing::North,
                r#copper_golem_pose: CopperGolemPose::Star,
                r#waterlogged: true,
            });
        }
        if state_id == 27255 {
            return Some(WaxedExposedCopperGolemStatue {
                r#waterlogged: true,
                r#copper_golem_pose: CopperGolemPose::Sitting,
                r#facing: Facing::South,
            });
        }
        if state_id == 27274 {
            return Some(WaxedExposedCopperGolemStatue {
                r#waterlogged: false,
                r#copper_golem_pose: CopperGolemPose::Star,
                r#facing: Facing::West,
            });
        }
        if state_id == 27249 {
            return Some(WaxedExposedCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Standing,
                r#waterlogged: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 27253 {
            return Some(WaxedExposedCopperGolemStatue {
                r#facing: Facing::North,
                r#waterlogged: true,
                r#copper_golem_pose: CopperGolemPose::Sitting,
            });
        }
        if state_id == 27273 {
            return Some(WaxedExposedCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Star,
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 27251 {
            return Some(WaxedExposedCopperGolemStatue {
                r#facing: Facing::East,
                r#copper_golem_pose: CopperGolemPose::Standing,
                r#waterlogged: true,
            });
        }
        if state_id == 27257 {
            return Some(WaxedExposedCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Sitting,
                r#waterlogged: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 27258 {
            return Some(WaxedExposedCopperGolemStatue {
                r#waterlogged: false,
                r#copper_golem_pose: CopperGolemPose::Sitting,
                r#facing: Facing::West,
            });
        }
        if state_id == 27261 {
            return Some(WaxedExposedCopperGolemStatue {
                r#facing: Facing::North,
                r#waterlogged: true,
                r#copper_golem_pose: CopperGolemPose::Running,
            });
        }
        if state_id == 27265 {
            return Some(WaxedExposedCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Running,
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 27272 {
            return Some(WaxedExposedCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Star,
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 27245 {
            return Some(WaxedExposedCopperGolemStatue {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#copper_golem_pose: CopperGolemPose::Standing,
            });
        }
        if state_id == 27268 {
            return Some(WaxedExposedCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Running,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 27270 {
            return Some(WaxedExposedCopperGolemStatue {
                r#facing: Facing::North,
                r#copper_golem_pose: CopperGolemPose::Star,
                r#waterlogged: false,
            });
        }
        if state_id == 27254 {
            return Some(WaxedExposedCopperGolemStatue {
                r#waterlogged: false,
                r#copper_golem_pose: CopperGolemPose::Sitting,
                r#facing: Facing::North,
            });
        }
        if state_id == 27247 {
            return Some(WaxedExposedCopperGolemStatue {
                r#waterlogged: true,
                r#facing: Facing::South,
                r#copper_golem_pose: CopperGolemPose::Standing,
            });
        }
        if state_id == 27262 {
            return Some(WaxedExposedCopperGolemStatue {
                r#waterlogged: false,
                r#copper_golem_pose: CopperGolemPose::Running,
                r#facing: Facing::North,
            });
        }
        if state_id == 27259 {
            return Some(WaxedExposedCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Sitting,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 27246 {
            return Some(WaxedExposedCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Standing,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 27266 {
            return Some(WaxedExposedCopperGolemStatue {
                r#facing: Facing::West,
                r#copper_golem_pose: CopperGolemPose::Running,
                r#waterlogged: false,
            });
        }
        return None;
    }
}

