use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CopperGolemStatue {
    pub r#copper_golem_pose: CopperGolemPose,
    pub waterlogged: bool,
    pub r#facing: Facing,
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

impl BlockState for CopperGolemStatue {
    fn to_id(&self) -> i32 {
        if self.r#facing == Facing::West
            && self.r#waterlogged == true
            && self.r#copper_golem_pose == CopperGolemPose::Standing
        {
            return 27089;
        }
        if self.r#copper_golem_pose == CopperGolemPose::Standing
            && self.r#facing == Facing::West
            && self.r#waterlogged == false
        {
            return 27090;
        }
        if self.r#facing == Facing::South
            && self.r#copper_golem_pose == CopperGolemPose::Sitting
            && self.r#waterlogged == true
        {
            return 27095;
        }
        if self.r#facing == Facing::East
            && self.r#waterlogged == false
            && self.r#copper_golem_pose == CopperGolemPose::Standing
        {
            return 27092;
        }
        if self.r#waterlogged == true
            && self.r#copper_golem_pose == CopperGolemPose::Running
            && self.r#facing == Facing::East
        {
            return 27107;
        }
        if self.r#copper_golem_pose == CopperGolemPose::Sitting
            && self.r#facing == Facing::East
            && self.r#waterlogged == false
        {
            return 27100;
        }
        if self.r#waterlogged == true
            && self.r#copper_golem_pose == CopperGolemPose::Sitting
            && self.r#facing == Facing::North
        {
            return 27093;
        }
        if self.r#copper_golem_pose == CopperGolemPose::Standing
            && self.r#waterlogged == true
            && self.r#facing == Facing::East
        {
            return 27091;
        }
        if self.r#facing == Facing::South
            && self.r#waterlogged == false
            && self.r#copper_golem_pose == CopperGolemPose::Standing
        {
            return 27088;
        }
        if self.r#facing == Facing::West
            && self.r#copper_golem_pose == CopperGolemPose::Star
            && self.r#waterlogged == true
        {
            return 27113;
        }
        if self.r#copper_golem_pose == CopperGolemPose::Star
            && self.r#waterlogged == false
            && self.r#facing == Facing::West
        {
            return 27114;
        }
        if self.r#waterlogged == true
            && self.r#facing == Facing::South
            && self.r#copper_golem_pose == CopperGolemPose::Running
        {
            return 27103;
        }
        if self.r#waterlogged == true
            && self.r#facing == Facing::South
            && self.r#copper_golem_pose == CopperGolemPose::Standing
        {
            return 27087;
        }
        if self.r#waterlogged == false
            && self.r#copper_golem_pose == CopperGolemPose::Sitting
            && self.r#facing == Facing::West
        {
            return 27098;
        }
        if self.r#facing == Facing::East
            && self.r#waterlogged == true
            && self.r#copper_golem_pose == CopperGolemPose::Star
        {
            return 27115;
        }
        if self.r#waterlogged == true
            && self.r#copper_golem_pose == CopperGolemPose::Star
            && self.r#facing == Facing::North
        {
            return 27109;
        }
        if self.r#facing == Facing::West
            && self.r#copper_golem_pose == CopperGolemPose::Sitting
            && self.r#waterlogged == true
        {
            return 27097;
        }
        if self.r#copper_golem_pose == CopperGolemPose::Running
            && self.r#facing == Facing::North
            && self.r#waterlogged == true
        {
            return 27101;
        }
        if self.r#waterlogged == false
            && self.r#copper_golem_pose == CopperGolemPose::Star
            && self.r#facing == Facing::South
        {
            return 27112;
        }
        if self.r#waterlogged == true
            && self.r#copper_golem_pose == CopperGolemPose::Sitting
            && self.r#facing == Facing::East
        {
            return 27099;
        }
        if self.r#copper_golem_pose == CopperGolemPose::Running
            && self.r#facing == Facing::South
            && self.r#waterlogged == false
        {
            return 27104;
        }
        if self.r#copper_golem_pose == CopperGolemPose::Running
            && self.r#waterlogged == true
            && self.r#facing == Facing::West
        {
            return 27105;
        }
        if self.r#facing == Facing::South
            && self.r#waterlogged == true
            && self.r#copper_golem_pose == CopperGolemPose::Star
        {
            return 27111;
        }
        if self.r#waterlogged == false
            && self.r#copper_golem_pose == CopperGolemPose::Standing
            && self.r#facing == Facing::North
        {
            return 27086;
        }
        if self.r#copper_golem_pose == CopperGolemPose::Sitting
            && self.r#facing == Facing::South
            && self.r#waterlogged == false
        {
            return 27096;
        }
        if self.r#waterlogged == true
            && self.r#facing == Facing::North
            && self.r#copper_golem_pose == CopperGolemPose::Standing
        {
            return 27085;
        }
        if self.r#facing == Facing::North
            && self.r#waterlogged == false
            && self.r#copper_golem_pose == CopperGolemPose::Star
        {
            return 27110;
        }
        if self.r#copper_golem_pose == CopperGolemPose::Star
            && self.r#facing == Facing::East
            && self.r#waterlogged == false
        {
            return 27116;
        }
        if self.r#facing == Facing::East
            && self.r#copper_golem_pose == CopperGolemPose::Running
            && self.r#waterlogged == false
        {
            return 27108;
        }
        if self.r#facing == Facing::West
            && self.r#copper_golem_pose == CopperGolemPose::Running
            && self.r#waterlogged == false
        {
            return 27106;
        }
        if self.r#facing == Facing::North
            && self.r#copper_golem_pose == CopperGolemPose::Sitting
            && self.r#waterlogged == false
        {
            return 27094;
        }
        if self.r#waterlogged == false
            && self.r#copper_golem_pose == CopperGolemPose::Running
            && self.r#facing == Facing::North
        {
            return 27102;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 27089 {
            return Some(CopperGolemStatue {
                r#facing: Facing::West,
                r#waterlogged: true,
                r#copper_golem_pose: CopperGolemPose::Standing,
            });
        }
        if state_id == 27090 {
            return Some(CopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Standing,
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 27095 {
            return Some(CopperGolemStatue {
                r#facing: Facing::South,
                r#copper_golem_pose: CopperGolemPose::Sitting,
                r#waterlogged: true,
            });
        }
        if state_id == 27092 {
            return Some(CopperGolemStatue {
                r#facing: Facing::East,
                r#waterlogged: false,
                r#copper_golem_pose: CopperGolemPose::Standing,
            });
        }
        if state_id == 27107 {
            return Some(CopperGolemStatue {
                r#waterlogged: true,
                r#copper_golem_pose: CopperGolemPose::Running,
                r#facing: Facing::East,
            });
        }
        if state_id == 27100 {
            return Some(CopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Sitting,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 27093 {
            return Some(CopperGolemStatue {
                r#waterlogged: true,
                r#copper_golem_pose: CopperGolemPose::Sitting,
                r#facing: Facing::North,
            });
        }
        if state_id == 27091 {
            return Some(CopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Standing,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 27088 {
            return Some(CopperGolemStatue {
                r#facing: Facing::South,
                r#waterlogged: false,
                r#copper_golem_pose: CopperGolemPose::Standing,
            });
        }
        if state_id == 27113 {
            return Some(CopperGolemStatue {
                r#facing: Facing::West,
                r#copper_golem_pose: CopperGolemPose::Star,
                r#waterlogged: true,
            });
        }
        if state_id == 27114 {
            return Some(CopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Star,
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 27103 {
            return Some(CopperGolemStatue {
                r#waterlogged: true,
                r#facing: Facing::South,
                r#copper_golem_pose: CopperGolemPose::Running,
            });
        }
        if state_id == 27087 {
            return Some(CopperGolemStatue {
                r#waterlogged: true,
                r#facing: Facing::South,
                r#copper_golem_pose: CopperGolemPose::Standing,
            });
        }
        if state_id == 27098 {
            return Some(CopperGolemStatue {
                r#waterlogged: false,
                r#copper_golem_pose: CopperGolemPose::Sitting,
                r#facing: Facing::West,
            });
        }
        if state_id == 27115 {
            return Some(CopperGolemStatue {
                r#facing: Facing::East,
                r#waterlogged: true,
                r#copper_golem_pose: CopperGolemPose::Star,
            });
        }
        if state_id == 27109 {
            return Some(CopperGolemStatue {
                r#waterlogged: true,
                r#copper_golem_pose: CopperGolemPose::Star,
                r#facing: Facing::North,
            });
        }
        if state_id == 27097 {
            return Some(CopperGolemStatue {
                r#facing: Facing::West,
                r#copper_golem_pose: CopperGolemPose::Sitting,
                r#waterlogged: true,
            });
        }
        if state_id == 27101 {
            return Some(CopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Running,
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 27112 {
            return Some(CopperGolemStatue {
                r#waterlogged: false,
                r#copper_golem_pose: CopperGolemPose::Star,
                r#facing: Facing::South,
            });
        }
        if state_id == 27099 {
            return Some(CopperGolemStatue {
                r#waterlogged: true,
                r#copper_golem_pose: CopperGolemPose::Sitting,
                r#facing: Facing::East,
            });
        }
        if state_id == 27104 {
            return Some(CopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Running,
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 27105 {
            return Some(CopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Running,
                r#waterlogged: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 27111 {
            return Some(CopperGolemStatue {
                r#facing: Facing::South,
                r#waterlogged: true,
                r#copper_golem_pose: CopperGolemPose::Star,
            });
        }
        if state_id == 27086 {
            return Some(CopperGolemStatue {
                r#waterlogged: false,
                r#copper_golem_pose: CopperGolemPose::Standing,
                r#facing: Facing::North,
            });
        }
        if state_id == 27096 {
            return Some(CopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Sitting,
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 27085 {
            return Some(CopperGolemStatue {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#copper_golem_pose: CopperGolemPose::Standing,
            });
        }
        if state_id == 27110 {
            return Some(CopperGolemStatue {
                r#facing: Facing::North,
                r#waterlogged: false,
                r#copper_golem_pose: CopperGolemPose::Star,
            });
        }
        if state_id == 27116 {
            return Some(CopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Star,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 27108 {
            return Some(CopperGolemStatue {
                r#facing: Facing::East,
                r#copper_golem_pose: CopperGolemPose::Running,
                r#waterlogged: false,
            });
        }
        if state_id == 27106 {
            return Some(CopperGolemStatue {
                r#facing: Facing::West,
                r#copper_golem_pose: CopperGolemPose::Running,
                r#waterlogged: false,
            });
        }
        if state_id == 27094 {
            return Some(CopperGolemStatue {
                r#facing: Facing::North,
                r#copper_golem_pose: CopperGolemPose::Sitting,
                r#waterlogged: false,
            });
        }
        if state_id == 27102 {
            return Some(CopperGolemStatue {
                r#waterlogged: false,
                r#copper_golem_pose: CopperGolemPose::Running,
                r#facing: Facing::North,
            });
        }
        return None;
    }
}
