use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaxedOxidizedCopperTrapdoor {
    pub powered: bool,
    pub r#facing: Facing,
    pub open: bool,
    pub r#half: Half,
    pub waterlogged: bool,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Half {
    Top,
    Bottom,
}

impl BlockState for WaxedOxidizedCopperTrapdoor {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == false && block_state.r#half == Half::Top && block_state.r#facing == Facing::South && block_state.r#open == true && block_state.r#powered == false { return 26736; }
        if block_state.r#facing == Facing::North && block_state.r#open == false && block_state.r#powered == false && block_state.r#half == Half::Top && block_state.r#waterlogged == true { return 26723; }
        if block_state.r#facing == Facing::West && block_state.r#powered == true && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true && block_state.r#open == true { return 26757; }
        if block_state.r#half == Half::Bottom && block_state.r#facing == Facing::South && block_state.r#powered == false && block_state.r#waterlogged == true && block_state.r#open == true { return 26743; }
        if block_state.r#facing == Facing::South && block_state.r#open == false && block_state.r#half == Half::Bottom && block_state.r#waterlogged == false && block_state.r#powered == false { return 26748; }
        if block_state.r#facing == Facing::West && block_state.r#open == false && block_state.r#waterlogged == true && block_state.r#powered == false && block_state.r#half == Half::Bottom { return 26763; }
        if block_state.r#open == false && block_state.r#facing == Facing::West && block_state.r#powered == false && block_state.r#waterlogged == false && block_state.r#half == Half::Top { return 26756; }
        if block_state.r#powered == true && block_state.r#half == Half::Bottom && block_state.r#open == false && block_state.r#waterlogged == false && block_state.r#facing == Facing::North { return 26730; }
        if block_state.r#half == Half::Bottom && block_state.r#open == false && block_state.r#facing == Facing::East && block_state.r#waterlogged == false && block_state.r#powered == false { return 26780; }
        if block_state.r#half == Half::Top && block_state.r#open == false && block_state.r#powered == true && block_state.r#facing == Facing::South && block_state.r#waterlogged == false { return 26738; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::North && block_state.r#open == false && block_state.r#half == Half::Top && block_state.r#powered == true { return 26722; }
        if block_state.r#powered == false && block_state.r#half == Half::Top && block_state.r#waterlogged == true && block_state.r#open == false && block_state.r#facing == Facing::South { return 26739; }
        if block_state.r#open == true && block_state.r#facing == Facing::North && block_state.r#waterlogged == false && block_state.r#half == Half::Bottom && block_state.r#powered == true { return 26726; }
        if block_state.r#half == Half::Bottom && block_state.r#open == true && block_state.r#facing == Facing::East && block_state.r#powered == true && block_state.r#waterlogged == false { return 26774; }
        if block_state.r#open == false && block_state.r#half == Half::Top && block_state.r#facing == Facing::East && block_state.r#powered == false && block_state.r#waterlogged == true { return 26771; }
        if block_state.r#facing == Facing::North && block_state.r#open == false && block_state.r#powered == false && block_state.r#waterlogged == false && block_state.r#half == Half::Top { return 26724; }
        if block_state.r#half == Half::Bottom && block_state.r#facing == Facing::North && block_state.r#open == false && block_state.r#powered == true && block_state.r#waterlogged == true { return 26729; }
        if block_state.r#powered == true && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true && block_state.r#facing == Facing::East && block_state.r#open == false { return 26777; }
        if block_state.r#waterlogged == false && block_state.r#half == Half::Top && block_state.r#powered == true && block_state.r#facing == Facing::North && block_state.r#open == true { return 26718; }
        if block_state.r#open == true && block_state.r#waterlogged == true && block_state.r#half == Half::Bottom && block_state.r#powered == false && block_state.r#facing == Facing::West { return 26759; }
        if block_state.r#powered == false && block_state.r#facing == Facing::South && block_state.r#open == false && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true { return 26747; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == false && block_state.r#powered == true && block_state.r#open == false && block_state.r#half == Half::Bottom { return 26762; }
        if block_state.r#open == false && block_state.r#powered == true && block_state.r#waterlogged == true && block_state.r#facing == Facing::East && block_state.r#half == Half::Top { return 26769; }
        if block_state.r#open == true && block_state.r#waterlogged == true && block_state.r#powered == true && block_state.r#facing == Facing::North && block_state.r#half == Half::Top { return 26717; }
        if block_state.r#facing == Facing::South && block_state.r#half == Half::Top && block_state.r#open == false && block_state.r#powered == true && block_state.r#waterlogged == true { return 26737; }
        if block_state.r#half == Half::Bottom && block_state.r#open == false && block_state.r#powered == false && block_state.r#facing == Facing::North && block_state.r#waterlogged == false { return 26732; }
        if block_state.r#waterlogged == false && block_state.r#open == false && block_state.r#half == Half::Top && block_state.r#facing == Facing::West && block_state.r#powered == true { return 26754; }
        if block_state.r#open == false && block_state.r#waterlogged == false && block_state.r#half == Half::Bottom && block_state.r#powered == true && block_state.r#facing == Facing::South { return 26746; }
        if block_state.r#waterlogged == true && block_state.r#half == Half::Bottom && block_state.r#open == false && block_state.r#powered == false && block_state.r#facing == Facing::East { return 26779; }
        if block_state.r#powered == true && block_state.r#waterlogged == true && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::East && block_state.r#open == true { return 26773; }
        if block_state.r#waterlogged == false && block_state.r#half == Half::Top && block_state.r#facing == Facing::West && block_state.r#powered == true && block_state.r#open == true { return 26750; }
        if block_state.r#open == true && block_state.r#powered == false && block_state.r#half == Half::Bottom && block_state.r#waterlogged == false && block_state.r#facing == Facing::South { return 26744; }
        if block_state.r#waterlogged == true && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::North && block_state.r#open == true && block_state.r#powered == true { return 26725; }
        if block_state.r#powered == false && block_state.r#waterlogged == false && block_state.r#facing == Facing::East && block_state.r#half == Half::Bottom && block_state.r#open == true { return 26776; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::South && block_state.r#powered == false && block_state.r#half == Half::Top && block_state.r#open == false { return 26740; }
        if block_state.r#facing == Facing::East && block_state.r#half == Half::Top && block_state.r#waterlogged == false && block_state.r#open == false && block_state.r#powered == false { return 26772; }
        if block_state.r#powered == true && block_state.r#half == Half::Bottom && block_state.r#open == true && block_state.r#waterlogged == true && block_state.r#facing == Facing::South { return 26741; }
        if block_state.r#half == Half::Top && block_state.r#powered == false && block_state.r#facing == Facing::North && block_state.r#open == true && block_state.r#waterlogged == true { return 26719; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::North && block_state.r#open == false && block_state.r#half == Half::Bottom && block_state.r#powered == false { return 26731; }
        if block_state.r#waterlogged == true && block_state.r#open == false && block_state.r#facing == Facing::South && block_state.r#half == Half::Bottom && block_state.r#powered == true { return 26745; }
        if block_state.r#powered == false && block_state.r#waterlogged == true && block_state.r#half == Half::Top && block_state.r#open == true && block_state.r#facing == Facing::West { return 26751; }
        if block_state.r#waterlogged == false && block_state.r#powered == false && block_state.r#facing == Facing::West && block_state.r#open == true && block_state.r#half == Half::Top { return 26752; }
        if block_state.r#powered == true && block_state.r#waterlogged == true && block_state.r#half == Half::Top && block_state.r#open == true && block_state.r#facing == Facing::East { return 26765; }
        if block_state.r#half == Half::Bottom && block_state.r#facing == Facing::East && block_state.r#waterlogged == false && block_state.r#powered == true && block_state.r#open == false { return 26778; }
        if block_state.r#open == true && block_state.r#waterlogged == false && block_state.r#facing == Facing::South && block_state.r#half == Half::Bottom && block_state.r#powered == true { return 26742; }
        if block_state.r#facing == Facing::North && block_state.r#open == true && block_state.r#waterlogged == true && block_state.r#powered == false && block_state.r#half == Half::Bottom { return 26727; }
        if block_state.r#open == true && block_state.r#waterlogged == false && block_state.r#facing == Facing::North && block_state.r#powered == false && block_state.r#half == Half::Bottom { return 26728; }
        if block_state.r#half == Half::Top && block_state.r#powered == true && block_state.r#open == true && block_state.r#facing == Facing::South && block_state.r#waterlogged == true { return 26733; }
        if block_state.r#waterlogged == true && block_state.r#half == Half::Top && block_state.r#open == true && block_state.r#powered == false && block_state.r#facing == Facing::South { return 26735; }
        if block_state.r#facing == Facing::West && block_state.r#open == true && block_state.r#powered == false && block_state.r#waterlogged == false && block_state.r#half == Half::Bottom { return 26760; }
        if block_state.r#waterlogged == true && block_state.r#powered == true && block_state.r#facing == Facing::West && block_state.r#open == false && block_state.r#half == Half::Top { return 26753; }
        if block_state.r#powered == true && block_state.r#waterlogged == false && block_state.r#facing == Facing::East && block_state.r#half == Half::Top && block_state.r#open == true { return 26766; }
        if block_state.r#powered == false && block_state.r#half == Half::Top && block_state.r#waterlogged == true && block_state.r#facing == Facing::West && block_state.r#open == false { return 26755; }
        if block_state.r#powered == false && block_state.r#facing == Facing::East && block_state.r#open == true && block_state.r#waterlogged == true && block_state.r#half == Half::Top { return 26767; }
        if block_state.r#open == true && block_state.r#facing == Facing::East && block_state.r#powered == false && block_state.r#waterlogged == false && block_state.r#half == Half::Top { return 26768; }
        if block_state.r#open == true && block_state.r#facing == Facing::East && block_state.r#powered == false && block_state.r#waterlogged == true && block_state.r#half == Half::Bottom { return 26775; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::West && block_state.r#half == Half::Bottom && block_state.r#open == false && block_state.r#powered == true { return 26761; }
        if block_state.r#open == true && block_state.r#waterlogged == true && block_state.r#facing == Facing::West && block_state.r#half == Half::Top && block_state.r#powered == true { return 26749; }
        if block_state.r#open == true && block_state.r#half == Half::Top && block_state.r#powered == true && block_state.r#waterlogged == false && block_state.r#facing == Facing::South { return 26734; }
        if block_state.r#powered == false && block_state.r#waterlogged == false && block_state.r#open == true && block_state.r#facing == Facing::North && block_state.r#half == Half::Top { return 26720; }
        if block_state.r#waterlogged == false && block_state.r#open == false && block_state.r#half == Half::Top && block_state.r#facing == Facing::East && block_state.r#powered == true { return 26770; }
        if block_state.r#open == true && block_state.r#half == Half::Bottom && block_state.r#powered == true && block_state.r#waterlogged == false && block_state.r#facing == Facing::West { return 26758; }
        if block_state.r#waterlogged == true && block_state.r#half == Half::Top && block_state.r#facing == Facing::North && block_state.r#open == false && block_state.r#powered == true { return 26721; }
        if block_state.r#half == Half::Bottom && block_state.r#powered == false && block_state.r#waterlogged == false && block_state.r#open == false && block_state.r#facing == Facing::West { return 26764; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 26736 {
            return Some(WaxedOxidizedCopperTrapdoor {
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::South,
                r#open: true,
                r#powered: false,
            });
        }
        if state_id == 26723 {
            return Some(WaxedOxidizedCopperTrapdoor {
                r#facing: Facing::North,
                r#open: false,
                r#powered: false,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 26757 {
            return Some(WaxedOxidizedCopperTrapdoor {
                r#facing: Facing::West,
                r#powered: true,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#open: true,
            });
        }
        if state_id == 26743 {
            return Some(WaxedOxidizedCopperTrapdoor {
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#powered: false,
                r#waterlogged: true,
                r#open: true,
            });
        }
        if state_id == 26748 {
            return Some(WaxedOxidizedCopperTrapdoor {
                r#facing: Facing::South,
                r#open: false,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#powered: false,
            });
        }
        if state_id == 26763 {
            return Some(WaxedOxidizedCopperTrapdoor {
                r#facing: Facing::West,
                r#open: false,
                r#waterlogged: true,
                r#powered: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 26756 {
            return Some(WaxedOxidizedCopperTrapdoor {
                r#open: false,
                r#facing: Facing::West,
                r#powered: false,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 26730 {
            return Some(WaxedOxidizedCopperTrapdoor {
                r#powered: true,
                r#half: Half::Bottom,
                r#open: false,
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 26780 {
            return Some(WaxedOxidizedCopperTrapdoor {
                r#half: Half::Bottom,
                r#open: false,
                r#facing: Facing::East,
                r#waterlogged: false,
                r#powered: false,
            });
        }
        if state_id == 26738 {
            return Some(WaxedOxidizedCopperTrapdoor {
                r#half: Half::Top,
                r#open: false,
                r#powered: true,
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 26722 {
            return Some(WaxedOxidizedCopperTrapdoor {
                r#waterlogged: false,
                r#facing: Facing::North,
                r#open: false,
                r#half: Half::Top,
                r#powered: true,
            });
        }
        if state_id == 26739 {
            return Some(WaxedOxidizedCopperTrapdoor {
                r#powered: false,
                r#half: Half::Top,
                r#waterlogged: true,
                r#open: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 26726 {
            return Some(WaxedOxidizedCopperTrapdoor {
                r#open: true,
                r#facing: Facing::North,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#powered: true,
            });
        }
        if state_id == 26774 {
            return Some(WaxedOxidizedCopperTrapdoor {
                r#half: Half::Bottom,
                r#open: true,
                r#facing: Facing::East,
                r#powered: true,
                r#waterlogged: false,
            });
        }
        if state_id == 26771 {
            return Some(WaxedOxidizedCopperTrapdoor {
                r#open: false,
                r#half: Half::Top,
                r#facing: Facing::East,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 26724 {
            return Some(WaxedOxidizedCopperTrapdoor {
                r#facing: Facing::North,
                r#open: false,
                r#powered: false,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 26729 {
            return Some(WaxedOxidizedCopperTrapdoor {
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#open: false,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 26777 {
            return Some(WaxedOxidizedCopperTrapdoor {
                r#powered: true,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::East,
                r#open: false,
            });
        }
        if state_id == 26718 {
            return Some(WaxedOxidizedCopperTrapdoor {
                r#waterlogged: false,
                r#half: Half::Top,
                r#powered: true,
                r#facing: Facing::North,
                r#open: true,
            });
        }
        if state_id == 26759 {
            return Some(WaxedOxidizedCopperTrapdoor {
                r#open: true,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#powered: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 26747 {
            return Some(WaxedOxidizedCopperTrapdoor {
                r#powered: false,
                r#facing: Facing::South,
                r#open: false,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 26762 {
            return Some(WaxedOxidizedCopperTrapdoor {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#powered: true,
                r#open: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 26769 {
            return Some(WaxedOxidizedCopperTrapdoor {
                r#open: false,
                r#powered: true,
                r#waterlogged: true,
                r#facing: Facing::East,
                r#half: Half::Top,
            });
        }
        if state_id == 26717 {
            return Some(WaxedOxidizedCopperTrapdoor {
                r#open: true,
                r#waterlogged: true,
                r#powered: true,
                r#facing: Facing::North,
                r#half: Half::Top,
            });
        }
        if state_id == 26737 {
            return Some(WaxedOxidizedCopperTrapdoor {
                r#facing: Facing::South,
                r#half: Half::Top,
                r#open: false,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 26732 {
            return Some(WaxedOxidizedCopperTrapdoor {
                r#half: Half::Bottom,
                r#open: false,
                r#powered: false,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 26754 {
            return Some(WaxedOxidizedCopperTrapdoor {
                r#waterlogged: false,
                r#open: false,
                r#half: Half::Top,
                r#facing: Facing::West,
                r#powered: true,
            });
        }
        if state_id == 26746 {
            return Some(WaxedOxidizedCopperTrapdoor {
                r#open: false,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#powered: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 26779 {
            return Some(WaxedOxidizedCopperTrapdoor {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#open: false,
                r#powered: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 26773 {
            return Some(WaxedOxidizedCopperTrapdoor {
                r#powered: true,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#open: true,
            });
        }
        if state_id == 26750 {
            return Some(WaxedOxidizedCopperTrapdoor {
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::West,
                r#powered: true,
                r#open: true,
            });
        }
        if state_id == 26744 {
            return Some(WaxedOxidizedCopperTrapdoor {
                r#open: true,
                r#powered: false,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 26725 {
            return Some(WaxedOxidizedCopperTrapdoor {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 26776 {
            return Some(WaxedOxidizedCopperTrapdoor {
                r#powered: false,
                r#waterlogged: false,
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#open: true,
            });
        }
        if state_id == 26740 {
            return Some(WaxedOxidizedCopperTrapdoor {
                r#waterlogged: false,
                r#facing: Facing::South,
                r#powered: false,
                r#half: Half::Top,
                r#open: false,
            });
        }
        if state_id == 26772 {
            return Some(WaxedOxidizedCopperTrapdoor {
                r#facing: Facing::East,
                r#half: Half::Top,
                r#waterlogged: false,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 26741 {
            return Some(WaxedOxidizedCopperTrapdoor {
                r#powered: true,
                r#half: Half::Bottom,
                r#open: true,
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 26719 {
            return Some(WaxedOxidizedCopperTrapdoor {
                r#half: Half::Top,
                r#powered: false,
                r#facing: Facing::North,
                r#open: true,
                r#waterlogged: true,
            });
        }
        if state_id == 26731 {
            return Some(WaxedOxidizedCopperTrapdoor {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#open: false,
                r#half: Half::Bottom,
                r#powered: false,
            });
        }
        if state_id == 26745 {
            return Some(WaxedOxidizedCopperTrapdoor {
                r#waterlogged: true,
                r#open: false,
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#powered: true,
            });
        }
        if state_id == 26751 {
            return Some(WaxedOxidizedCopperTrapdoor {
                r#powered: false,
                r#waterlogged: true,
                r#half: Half::Top,
                r#open: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 26752 {
            return Some(WaxedOxidizedCopperTrapdoor {
                r#waterlogged: false,
                r#powered: false,
                r#facing: Facing::West,
                r#open: true,
                r#half: Half::Top,
            });
        }
        if state_id == 26765 {
            return Some(WaxedOxidizedCopperTrapdoor {
                r#powered: true,
                r#waterlogged: true,
                r#half: Half::Top,
                r#open: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 26778 {
            return Some(WaxedOxidizedCopperTrapdoor {
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#waterlogged: false,
                r#powered: true,
                r#open: false,
            });
        }
        if state_id == 26742 {
            return Some(WaxedOxidizedCopperTrapdoor {
                r#open: true,
                r#waterlogged: false,
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#powered: true,
            });
        }
        if state_id == 26727 {
            return Some(WaxedOxidizedCopperTrapdoor {
                r#facing: Facing::North,
                r#open: true,
                r#waterlogged: true,
                r#powered: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 26728 {
            return Some(WaxedOxidizedCopperTrapdoor {
                r#open: true,
                r#waterlogged: false,
                r#facing: Facing::North,
                r#powered: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 26733 {
            return Some(WaxedOxidizedCopperTrapdoor {
                r#half: Half::Top,
                r#powered: true,
                r#open: true,
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 26735 {
            return Some(WaxedOxidizedCopperTrapdoor {
                r#waterlogged: true,
                r#half: Half::Top,
                r#open: true,
                r#powered: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 26760 {
            return Some(WaxedOxidizedCopperTrapdoor {
                r#facing: Facing::West,
                r#open: true,
                r#powered: false,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 26753 {
            return Some(WaxedOxidizedCopperTrapdoor {
                r#waterlogged: true,
                r#powered: true,
                r#facing: Facing::West,
                r#open: false,
                r#half: Half::Top,
            });
        }
        if state_id == 26766 {
            return Some(WaxedOxidizedCopperTrapdoor {
                r#powered: true,
                r#waterlogged: false,
                r#facing: Facing::East,
                r#half: Half::Top,
                r#open: true,
            });
        }
        if state_id == 26755 {
            return Some(WaxedOxidizedCopperTrapdoor {
                r#powered: false,
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#open: false,
            });
        }
        if state_id == 26767 {
            return Some(WaxedOxidizedCopperTrapdoor {
                r#powered: false,
                r#facing: Facing::East,
                r#open: true,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 26768 {
            return Some(WaxedOxidizedCopperTrapdoor {
                r#open: true,
                r#facing: Facing::East,
                r#powered: false,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 26775 {
            return Some(WaxedOxidizedCopperTrapdoor {
                r#open: true,
                r#facing: Facing::East,
                r#powered: false,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 26761 {
            return Some(WaxedOxidizedCopperTrapdoor {
                r#waterlogged: true,
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#open: false,
                r#powered: true,
            });
        }
        if state_id == 26749 {
            return Some(WaxedOxidizedCopperTrapdoor {
                r#open: true,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#half: Half::Top,
                r#powered: true,
            });
        }
        if state_id == 26734 {
            return Some(WaxedOxidizedCopperTrapdoor {
                r#open: true,
                r#half: Half::Top,
                r#powered: true,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 26720 {
            return Some(WaxedOxidizedCopperTrapdoor {
                r#powered: false,
                r#waterlogged: false,
                r#open: true,
                r#facing: Facing::North,
                r#half: Half::Top,
            });
        }
        if state_id == 26770 {
            return Some(WaxedOxidizedCopperTrapdoor {
                r#waterlogged: false,
                r#open: false,
                r#half: Half::Top,
                r#facing: Facing::East,
                r#powered: true,
            });
        }
        if state_id == 26758 {
            return Some(WaxedOxidizedCopperTrapdoor {
                r#open: true,
                r#half: Half::Bottom,
                r#powered: true,
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 26721 {
            return Some(WaxedOxidizedCopperTrapdoor {
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::North,
                r#open: false,
                r#powered: true,
            });
        }
        if state_id == 26764 {
            return Some(WaxedOxidizedCopperTrapdoor {
                r#half: Half::Bottom,
                r#powered: false,
                r#waterlogged: false,
                r#open: false,
                r#facing: Facing::West,
            });
        }
        return None;
    }
}

