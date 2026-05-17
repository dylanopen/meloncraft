use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaxedOxidizedCopperGolemStatue {
    pub r#facing: Facing,
    pub waterlogged: bool,
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

impl BlockState for WaxedOxidizedCopperGolemStatue {
    fn to_id(&self) -> i32 {
        if self.r#copper_golem_pose == CopperGolemPose::Sitting && self.r#waterlogged == false && self.r#facing == Facing::South { return 27320; }
        if self.r#facing == Facing::East && self.r#copper_golem_pose == CopperGolemPose::Sitting && self.r#waterlogged == true { return 27323; }
        if self.r#waterlogged == false && self.r#copper_golem_pose == CopperGolemPose::Running && self.r#facing == Facing::South { return 27328; }
        if self.r#copper_golem_pose == CopperGolemPose::Standing && self.r#facing == Facing::South && self.r#waterlogged == false { return 27312; }
        if self.r#facing == Facing::West && self.r#waterlogged == false && self.r#copper_golem_pose == CopperGolemPose::Star { return 27338; }
        if self.r#copper_golem_pose == CopperGolemPose::Standing && self.r#facing == Facing::North && self.r#waterlogged == false { return 27310; }
        if self.r#copper_golem_pose == CopperGolemPose::Running && self.r#waterlogged == true && self.r#facing == Facing::West { return 27329; }
        if self.r#waterlogged == true && self.r#facing == Facing::West && self.r#copper_golem_pose == CopperGolemPose::Standing { return 27313; }
        if self.r#waterlogged == false && self.r#facing == Facing::North && self.r#copper_golem_pose == CopperGolemPose::Running { return 27326; }
        if self.r#waterlogged == false && self.r#copper_golem_pose == CopperGolemPose::Standing && self.r#facing == Facing::East { return 27316; }
        if self.r#copper_golem_pose == CopperGolemPose::Star && self.r#facing == Facing::East && self.r#waterlogged == false { return 27340; }
        if self.r#waterlogged == true && self.r#facing == Facing::South && self.r#copper_golem_pose == CopperGolemPose::Star { return 27335; }
        if self.r#waterlogged == true && self.r#facing == Facing::South && self.r#copper_golem_pose == CopperGolemPose::Running { return 27327; }
        if self.r#waterlogged == false && self.r#copper_golem_pose == CopperGolemPose::Running && self.r#facing == Facing::West { return 27330; }
        if self.r#copper_golem_pose == CopperGolemPose::Sitting && self.r#facing == Facing::West && self.r#waterlogged == true { return 27321; }
        if self.r#waterlogged == true && self.r#facing == Facing::East && self.r#copper_golem_pose == CopperGolemPose::Standing { return 27315; }
        if self.r#facing == Facing::West && self.r#waterlogged == false && self.r#copper_golem_pose == CopperGolemPose::Sitting { return 27322; }
        if self.r#copper_golem_pose == CopperGolemPose::Sitting && self.r#waterlogged == false && self.r#facing == Facing::East { return 27324; }
        if self.r#facing == Facing::North && self.r#waterlogged == true && self.r#copper_golem_pose == CopperGolemPose::Running { return 27325; }
        if self.r#copper_golem_pose == CopperGolemPose::Star && self.r#facing == Facing::East && self.r#waterlogged == true { return 27339; }
        if self.r#waterlogged == true && self.r#copper_golem_pose == CopperGolemPose::Standing && self.r#facing == Facing::South { return 27311; }
        if self.r#copper_golem_pose == CopperGolemPose::Running && self.r#waterlogged == true && self.r#facing == Facing::East { return 27331; }
        if self.r#waterlogged == false && self.r#facing == Facing::West && self.r#copper_golem_pose == CopperGolemPose::Standing { return 27314; }
        if self.r#waterlogged == false && self.r#facing == Facing::East && self.r#copper_golem_pose == CopperGolemPose::Running { return 27332; }
        if self.r#waterlogged == false && self.r#copper_golem_pose == CopperGolemPose::Star && self.r#facing == Facing::South { return 27336; }
        if self.r#waterlogged == false && self.r#copper_golem_pose == CopperGolemPose::Sitting && self.r#facing == Facing::North { return 27318; }
        if self.r#waterlogged == true && self.r#facing == Facing::West && self.r#copper_golem_pose == CopperGolemPose::Star { return 27337; }
        if self.r#facing == Facing::North && self.r#waterlogged == true && self.r#copper_golem_pose == CopperGolemPose::Star { return 27333; }
        if self.r#copper_golem_pose == CopperGolemPose::Star && self.r#waterlogged == false && self.r#facing == Facing::North { return 27334; }
        if self.r#copper_golem_pose == CopperGolemPose::Standing && self.r#waterlogged == true && self.r#facing == Facing::North { return 27309; }
        if self.r#copper_golem_pose == CopperGolemPose::Sitting && self.r#facing == Facing::South && self.r#waterlogged == true { return 27319; }
        if self.r#copper_golem_pose == CopperGolemPose::Sitting && self.r#facing == Facing::North && self.r#waterlogged == true { return 27317; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 27320 {
            return Some(WaxedOxidizedCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Sitting,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 27323 {
            return Some(WaxedOxidizedCopperGolemStatue {
                r#facing: Facing::East,
                r#copper_golem_pose: CopperGolemPose::Sitting,
                r#waterlogged: true,
            });
        }
        if state_id == 27328 {
            return Some(WaxedOxidizedCopperGolemStatue {
                r#waterlogged: false,
                r#copper_golem_pose: CopperGolemPose::Running,
                r#facing: Facing::South,
            });
        }
        if state_id == 27312 {
            return Some(WaxedOxidizedCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Standing,
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 27338 {
            return Some(WaxedOxidizedCopperGolemStatue {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#copper_golem_pose: CopperGolemPose::Star,
            });
        }
        if state_id == 27310 {
            return Some(WaxedOxidizedCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Standing,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 27329 {
            return Some(WaxedOxidizedCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Running,
                r#waterlogged: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 27313 {
            return Some(WaxedOxidizedCopperGolemStatue {
                r#waterlogged: true,
                r#facing: Facing::West,
                r#copper_golem_pose: CopperGolemPose::Standing,
            });
        }
        if state_id == 27326 {
            return Some(WaxedOxidizedCopperGolemStatue {
                r#waterlogged: false,
                r#facing: Facing::North,
                r#copper_golem_pose: CopperGolemPose::Running,
            });
        }
        if state_id == 27316 {
            return Some(WaxedOxidizedCopperGolemStatue {
                r#waterlogged: false,
                r#copper_golem_pose: CopperGolemPose::Standing,
                r#facing: Facing::East,
            });
        }
        if state_id == 27340 {
            return Some(WaxedOxidizedCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Star,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 27335 {
            return Some(WaxedOxidizedCopperGolemStatue {
                r#waterlogged: true,
                r#facing: Facing::South,
                r#copper_golem_pose: CopperGolemPose::Star,
            });
        }
        if state_id == 27327 {
            return Some(WaxedOxidizedCopperGolemStatue {
                r#waterlogged: true,
                r#facing: Facing::South,
                r#copper_golem_pose: CopperGolemPose::Running,
            });
        }
        if state_id == 27330 {
            return Some(WaxedOxidizedCopperGolemStatue {
                r#waterlogged: false,
                r#copper_golem_pose: CopperGolemPose::Running,
                r#facing: Facing::West,
            });
        }
        if state_id == 27321 {
            return Some(WaxedOxidizedCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Sitting,
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 27315 {
            return Some(WaxedOxidizedCopperGolemStatue {
                r#waterlogged: true,
                r#facing: Facing::East,
                r#copper_golem_pose: CopperGolemPose::Standing,
            });
        }
        if state_id == 27322 {
            return Some(WaxedOxidizedCopperGolemStatue {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#copper_golem_pose: CopperGolemPose::Sitting,
            });
        }
        if state_id == 27324 {
            return Some(WaxedOxidizedCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Sitting,
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 27325 {
            return Some(WaxedOxidizedCopperGolemStatue {
                r#facing: Facing::North,
                r#waterlogged: true,
                r#copper_golem_pose: CopperGolemPose::Running,
            });
        }
        if state_id == 27339 {
            return Some(WaxedOxidizedCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Star,
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 27311 {
            return Some(WaxedOxidizedCopperGolemStatue {
                r#waterlogged: true,
                r#copper_golem_pose: CopperGolemPose::Standing,
                r#facing: Facing::South,
            });
        }
        if state_id == 27331 {
            return Some(WaxedOxidizedCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Running,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 27314 {
            return Some(WaxedOxidizedCopperGolemStatue {
                r#waterlogged: false,
                r#facing: Facing::West,
                r#copper_golem_pose: CopperGolemPose::Standing,
            });
        }
        if state_id == 27332 {
            return Some(WaxedOxidizedCopperGolemStatue {
                r#waterlogged: false,
                r#facing: Facing::East,
                r#copper_golem_pose: CopperGolemPose::Running,
            });
        }
        if state_id == 27336 {
            return Some(WaxedOxidizedCopperGolemStatue {
                r#waterlogged: false,
                r#copper_golem_pose: CopperGolemPose::Star,
                r#facing: Facing::South,
            });
        }
        if state_id == 27318 {
            return Some(WaxedOxidizedCopperGolemStatue {
                r#waterlogged: false,
                r#copper_golem_pose: CopperGolemPose::Sitting,
                r#facing: Facing::North,
            });
        }
        if state_id == 27337 {
            return Some(WaxedOxidizedCopperGolemStatue {
                r#waterlogged: true,
                r#facing: Facing::West,
                r#copper_golem_pose: CopperGolemPose::Star,
            });
        }
        if state_id == 27333 {
            return Some(WaxedOxidizedCopperGolemStatue {
                r#facing: Facing::North,
                r#waterlogged: true,
                r#copper_golem_pose: CopperGolemPose::Star,
            });
        }
        if state_id == 27334 {
            return Some(WaxedOxidizedCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Star,
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 27309 {
            return Some(WaxedOxidizedCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Standing,
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 27319 {
            return Some(WaxedOxidizedCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Sitting,
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 27317 {
            return Some(WaxedOxidizedCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Sitting,
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        return None;
    }
}

