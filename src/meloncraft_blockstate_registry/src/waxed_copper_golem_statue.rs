use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaxedCopperGolemStatue {
    pub waterlogged: bool,
    pub r#copper_golem_pose: CopperGolemPose,
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

impl BlockState for WaxedCopperGolemStatue {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == true
            && self.r#facing == Facing::South
            && self.r#copper_golem_pose == CopperGolemPose::Standing
        {
            return 27215;
        }
        if self.r#facing == Facing::East
            && self.r#copper_golem_pose == CopperGolemPose::Sitting
            && self.r#waterlogged == true
        {
            return 27227;
        }
        if self.r#facing == Facing::West
            && self.r#waterlogged == false
            && self.r#copper_golem_pose == CopperGolemPose::Sitting
        {
            return 27226;
        }
        if self.r#copper_golem_pose == CopperGolemPose::Sitting
            && self.r#facing == Facing::East
            && self.r#waterlogged == false
        {
            return 27228;
        }
        if self.r#facing == Facing::East
            && self.r#waterlogged == false
            && self.r#copper_golem_pose == CopperGolemPose::Running
        {
            return 27236;
        }
        if self.r#waterlogged == false
            && self.r#copper_golem_pose == CopperGolemPose::Standing
            && self.r#facing == Facing::West
        {
            return 27218;
        }
        if self.r#facing == Facing::North
            && self.r#waterlogged == true
            && self.r#copper_golem_pose == CopperGolemPose::Sitting
        {
            return 27221;
        }
        if self.r#copper_golem_pose == CopperGolemPose::Standing
            && self.r#facing == Facing::East
            && self.r#waterlogged == true
        {
            return 27219;
        }
        if self.r#waterlogged == true
            && self.r#copper_golem_pose == CopperGolemPose::Sitting
            && self.r#facing == Facing::West
        {
            return 27225;
        }
        if self.r#waterlogged == false
            && self.r#facing == Facing::North
            && self.r#copper_golem_pose == CopperGolemPose::Running
        {
            return 27230;
        }
        if self.r#waterlogged == true
            && self.r#copper_golem_pose == CopperGolemPose::Running
            && self.r#facing == Facing::East
        {
            return 27235;
        }
        if self.r#copper_golem_pose == CopperGolemPose::Sitting
            && self.r#waterlogged == false
            && self.r#facing == Facing::South
        {
            return 27224;
        }
        if self.r#facing == Facing::North
            && self.r#waterlogged == false
            && self.r#copper_golem_pose == CopperGolemPose::Sitting
        {
            return 27222;
        }
        if self.r#copper_golem_pose == CopperGolemPose::Star
            && self.r#waterlogged == true
            && self.r#facing == Facing::North
        {
            return 27237;
        }
        if self.r#facing == Facing::South
            && self.r#copper_golem_pose == CopperGolemPose::Star
            && self.r#waterlogged == false
        {
            return 27240;
        }
        if self.r#facing == Facing::West
            && self.r#waterlogged == false
            && self.r#copper_golem_pose == CopperGolemPose::Running
        {
            return 27234;
        }
        if self.r#copper_golem_pose == CopperGolemPose::Star
            && self.r#facing == Facing::North
            && self.r#waterlogged == false
        {
            return 27238;
        }
        if self.r#facing == Facing::West
            && self.r#waterlogged == true
            && self.r#copper_golem_pose == CopperGolemPose::Star
        {
            return 27241;
        }
        if self.r#facing == Facing::West
            && self.r#waterlogged == false
            && self.r#copper_golem_pose == CopperGolemPose::Star
        {
            return 27242;
        }
        if self.r#waterlogged == false
            && self.r#copper_golem_pose == CopperGolemPose::Running
            && self.r#facing == Facing::South
        {
            return 27232;
        }
        if self.r#facing == Facing::East
            && self.r#copper_golem_pose == CopperGolemPose::Star
            && self.r#waterlogged == true
        {
            return 27243;
        }
        if self.r#facing == Facing::North
            && self.r#waterlogged == true
            && self.r#copper_golem_pose == CopperGolemPose::Running
        {
            return 27229;
        }
        if self.r#facing == Facing::South
            && self.r#waterlogged == true
            && self.r#copper_golem_pose == CopperGolemPose::Star
        {
            return 27239;
        }
        if self.r#facing == Facing::North
            && self.r#waterlogged == false
            && self.r#copper_golem_pose == CopperGolemPose::Standing
        {
            return 27214;
        }
        if self.r#facing == Facing::South
            && self.r#waterlogged == true
            && self.r#copper_golem_pose == CopperGolemPose::Sitting
        {
            return 27223;
        }
        if self.r#waterlogged == false
            && self.r#copper_golem_pose == CopperGolemPose::Standing
            && self.r#facing == Facing::South
        {
            return 27216;
        }
        if self.r#facing == Facing::West
            && self.r#waterlogged == true
            && self.r#copper_golem_pose == CopperGolemPose::Standing
        {
            return 27217;
        }
        if self.r#copper_golem_pose == CopperGolemPose::Running
            && self.r#waterlogged == true
            && self.r#facing == Facing::South
        {
            return 27231;
        }
        if self.r#copper_golem_pose == CopperGolemPose::Running
            && self.r#waterlogged == true
            && self.r#facing == Facing::West
        {
            return 27233;
        }
        if self.r#facing == Facing::North
            && self.r#copper_golem_pose == CopperGolemPose::Standing
            && self.r#waterlogged == true
        {
            return 27213;
        }
        if self.r#waterlogged == false
            && self.r#copper_golem_pose == CopperGolemPose::Standing
            && self.r#facing == Facing::East
        {
            return 27220;
        }
        if self.r#waterlogged == false
            && self.r#copper_golem_pose == CopperGolemPose::Star
            && self.r#facing == Facing::East
        {
            return 27244;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 27215 {
            return Some(WaxedCopperGolemStatue {
                r#waterlogged: true,
                r#facing: Facing::South,
                r#copper_golem_pose: CopperGolemPose::Standing,
            });
        }
        if state_id == 27227 {
            return Some(WaxedCopperGolemStatue {
                r#facing: Facing::East,
                r#copper_golem_pose: CopperGolemPose::Sitting,
                r#waterlogged: true,
            });
        }
        if state_id == 27226 {
            return Some(WaxedCopperGolemStatue {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#copper_golem_pose: CopperGolemPose::Sitting,
            });
        }
        if state_id == 27228 {
            return Some(WaxedCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Sitting,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 27236 {
            return Some(WaxedCopperGolemStatue {
                r#facing: Facing::East,
                r#waterlogged: false,
                r#copper_golem_pose: CopperGolemPose::Running,
            });
        }
        if state_id == 27218 {
            return Some(WaxedCopperGolemStatue {
                r#waterlogged: false,
                r#copper_golem_pose: CopperGolemPose::Standing,
                r#facing: Facing::West,
            });
        }
        if state_id == 27221 {
            return Some(WaxedCopperGolemStatue {
                r#facing: Facing::North,
                r#waterlogged: true,
                r#copper_golem_pose: CopperGolemPose::Sitting,
            });
        }
        if state_id == 27219 {
            return Some(WaxedCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Standing,
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 27225 {
            return Some(WaxedCopperGolemStatue {
                r#waterlogged: true,
                r#copper_golem_pose: CopperGolemPose::Sitting,
                r#facing: Facing::West,
            });
        }
        if state_id == 27230 {
            return Some(WaxedCopperGolemStatue {
                r#waterlogged: false,
                r#facing: Facing::North,
                r#copper_golem_pose: CopperGolemPose::Running,
            });
        }
        if state_id == 27235 {
            return Some(WaxedCopperGolemStatue {
                r#waterlogged: true,
                r#copper_golem_pose: CopperGolemPose::Running,
                r#facing: Facing::East,
            });
        }
        if state_id == 27224 {
            return Some(WaxedCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Sitting,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 27222 {
            return Some(WaxedCopperGolemStatue {
                r#facing: Facing::North,
                r#waterlogged: false,
                r#copper_golem_pose: CopperGolemPose::Sitting,
            });
        }
        if state_id == 27237 {
            return Some(WaxedCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Star,
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 27240 {
            return Some(WaxedCopperGolemStatue {
                r#facing: Facing::South,
                r#copper_golem_pose: CopperGolemPose::Star,
                r#waterlogged: false,
            });
        }
        if state_id == 27234 {
            return Some(WaxedCopperGolemStatue {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#copper_golem_pose: CopperGolemPose::Running,
            });
        }
        if state_id == 27238 {
            return Some(WaxedCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Star,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 27241 {
            return Some(WaxedCopperGolemStatue {
                r#facing: Facing::West,
                r#waterlogged: true,
                r#copper_golem_pose: CopperGolemPose::Star,
            });
        }
        if state_id == 27242 {
            return Some(WaxedCopperGolemStatue {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#copper_golem_pose: CopperGolemPose::Star,
            });
        }
        if state_id == 27232 {
            return Some(WaxedCopperGolemStatue {
                r#waterlogged: false,
                r#copper_golem_pose: CopperGolemPose::Running,
                r#facing: Facing::South,
            });
        }
        if state_id == 27243 {
            return Some(WaxedCopperGolemStatue {
                r#facing: Facing::East,
                r#copper_golem_pose: CopperGolemPose::Star,
                r#waterlogged: true,
            });
        }
        if state_id == 27229 {
            return Some(WaxedCopperGolemStatue {
                r#facing: Facing::North,
                r#waterlogged: true,
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
        if state_id == 27214 {
            return Some(WaxedCopperGolemStatue {
                r#facing: Facing::North,
                r#waterlogged: false,
                r#copper_golem_pose: CopperGolemPose::Standing,
            });
        }
        if state_id == 27223 {
            return Some(WaxedCopperGolemStatue {
                r#facing: Facing::South,
                r#waterlogged: true,
                r#copper_golem_pose: CopperGolemPose::Sitting,
            });
        }
        if state_id == 27216 {
            return Some(WaxedCopperGolemStatue {
                r#waterlogged: false,
                r#copper_golem_pose: CopperGolemPose::Standing,
                r#facing: Facing::South,
            });
        }
        if state_id == 27217 {
            return Some(WaxedCopperGolemStatue {
                r#facing: Facing::West,
                r#waterlogged: true,
                r#copper_golem_pose: CopperGolemPose::Standing,
            });
        }
        if state_id == 27231 {
            return Some(WaxedCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Running,
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 27233 {
            return Some(WaxedCopperGolemStatue {
                r#copper_golem_pose: CopperGolemPose::Running,
                r#waterlogged: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 27213 {
            return Some(WaxedCopperGolemStatue {
                r#facing: Facing::North,
                r#copper_golem_pose: CopperGolemPose::Standing,
                r#waterlogged: true,
            });
        }
        if state_id == 27220 {
            return Some(WaxedCopperGolemStatue {
                r#waterlogged: false,
                r#copper_golem_pose: CopperGolemPose::Standing,
                r#facing: Facing::East,
            });
        }
        if state_id == 27244 {
            return Some(WaxedCopperGolemStatue {
                r#waterlogged: false,
                r#copper_golem_pose: CopperGolemPose::Star,
                r#facing: Facing::East,
            });
        }
        return None;
    }
}
