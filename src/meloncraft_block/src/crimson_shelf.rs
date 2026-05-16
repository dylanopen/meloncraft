use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrimsonShelf {
    pub r#facing: Facing,
    pub r#side_chain: SideChain,
    pub powered: bool,
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
pub enum SideChain {
    Unconnected,
    Right,
    Center,
    Left,
}

impl BlockState for CrimsonShelf {
    fn to_id(&self) -> i32 {
        if self.r#facing == Facing::East && self.r#powered == true && self.r#side_chain == SideChain::Center && self.r#waterlogged == true { return 2707; }
        if self.r#facing == Facing::East && self.r#waterlogged == true && self.r#powered == false && self.r#side_chain == SideChain::Unconnected { return 2711; }
        if self.r#powered == false && self.r#side_chain == SideChain::Unconnected && self.r#facing == Facing::East && self.r#waterlogged == false { return 2712; }
        if self.r#side_chain == SideChain::Right && self.r#waterlogged == false && self.r#facing == Facing::South && self.r#powered == false { return 2682; }
        if self.r#powered == false && self.r#waterlogged == false && self.r#side_chain == SideChain::Right && self.r#facing == Facing::North { return 2666; }
        if self.r#powered == false && self.r#facing == Facing::East && self.r#side_chain == SideChain::Center && self.r#waterlogged == false { return 2716; }
        if self.r#side_chain == SideChain::Right && self.r#waterlogged == true && self.r#facing == Facing::South && self.r#powered == false { return 2681; }
        if self.r#facing == Facing::North && self.r#side_chain == SideChain::Left && self.r#powered == false && self.r#waterlogged == false { return 2670; }
        if self.r#waterlogged == true && self.r#facing == Facing::East && self.r#powered == true && self.r#side_chain == SideChain::Left { return 2709; }
        if self.r#facing == Facing::East && self.r#waterlogged == false && self.r#powered == true && self.r#side_chain == SideChain::Left { return 2710; }
        if self.r#facing == Facing::East && self.r#powered == false && self.r#side_chain == SideChain::Left && self.r#waterlogged == false { return 2718; }
        if self.r#waterlogged == false && self.r#facing == Facing::North && self.r#powered == true && self.r#side_chain == SideChain::Right { return 2658; }
        if self.r#side_chain == SideChain::Unconnected && self.r#facing == Facing::South && self.r#powered == false && self.r#waterlogged == true { return 2679; }
        if self.r#waterlogged == true && self.r#facing == Facing::North && self.r#side_chain == SideChain::Unconnected && self.r#powered == true { return 2655; }
        if self.r#waterlogged == true && self.r#powered == true && self.r#side_chain == SideChain::Right && self.r#facing == Facing::South { return 2673; }
        if self.r#facing == Facing::West && self.r#side_chain == SideChain::Unconnected && self.r#waterlogged == true && self.r#powered == false { return 2695; }
        if self.r#powered == false && self.r#facing == Facing::West && self.r#side_chain == SideChain::Left && self.r#waterlogged == true { return 2701; }
        if self.r#waterlogged == false && self.r#facing == Facing::West && self.r#side_chain == SideChain::Left && self.r#powered == false { return 2702; }
        if self.r#side_chain == SideChain::Right && self.r#waterlogged == false && self.r#facing == Facing::East && self.r#powered == true { return 2706; }
        if self.r#side_chain == SideChain::Unconnected && self.r#waterlogged == false && self.r#facing == Facing::South && self.r#powered == true { return 2672; }
        if self.r#facing == Facing::South && self.r#side_chain == SideChain::Center && self.r#powered == true && self.r#waterlogged == false { return 2676; }
        if self.r#side_chain == SideChain::Center && self.r#waterlogged == false && self.r#powered == false && self.r#facing == Facing::South { return 2684; }
        if self.r#side_chain == SideChain::Center && self.r#facing == Facing::East && self.r#powered == false && self.r#waterlogged == true { return 2715; }
        if self.r#facing == Facing::South && self.r#waterlogged == false && self.r#side_chain == SideChain::Left && self.r#powered == true { return 2678; }
        if self.r#facing == Facing::North && self.r#powered == true && self.r#side_chain == SideChain::Center && self.r#waterlogged == false { return 2660; }
        if self.r#facing == Facing::South && self.r#powered == true && self.r#side_chain == SideChain::Right && self.r#waterlogged == false { return 2674; }
        if self.r#facing == Facing::South && self.r#powered == false && self.r#side_chain == SideChain::Unconnected && self.r#waterlogged == false { return 2680; }
        if self.r#side_chain == SideChain::Unconnected && self.r#powered == false && self.r#facing == Facing::North && self.r#waterlogged == false { return 2664; }
        if self.r#powered == false && self.r#facing == Facing::South && self.r#side_chain == SideChain::Center && self.r#waterlogged == true { return 2683; }
        if self.r#waterlogged == false && self.r#facing == Facing::West && self.r#side_chain == SideChain::Left && self.r#powered == true { return 2694; }
        if self.r#powered == true && self.r#side_chain == SideChain::Left && self.r#facing == Facing::South && self.r#waterlogged == true { return 2677; }
        if self.r#facing == Facing::West && self.r#side_chain == SideChain::Right && self.r#powered == false && self.r#waterlogged == false { return 2698; }
        if self.r#facing == Facing::West && self.r#waterlogged == false && self.r#powered == false && self.r#side_chain == SideChain::Unconnected { return 2696; }
        if self.r#waterlogged == false && self.r#facing == Facing::West && self.r#powered == true && self.r#side_chain == SideChain::Unconnected { return 2688; }
        if self.r#facing == Facing::West && self.r#side_chain == SideChain::Right && self.r#waterlogged == false && self.r#powered == true { return 2690; }
        if self.r#waterlogged == true && self.r#facing == Facing::North && self.r#powered == true && self.r#side_chain == SideChain::Center { return 2659; }
        if self.r#powered == true && self.r#side_chain == SideChain::Right && self.r#facing == Facing::East && self.r#waterlogged == true { return 2705; }
        if self.r#facing == Facing::South && self.r#powered == true && self.r#side_chain == SideChain::Center && self.r#waterlogged == true { return 2675; }
        if self.r#powered == true && self.r#waterlogged == false && self.r#facing == Facing::East && self.r#side_chain == SideChain::Center { return 2708; }
        if self.r#side_chain == SideChain::Right && self.r#facing == Facing::West && self.r#waterlogged == true && self.r#powered == true { return 2689; }
        if self.r#powered == false && self.r#facing == Facing::East && self.r#side_chain == SideChain::Right && self.r#waterlogged == false { return 2714; }
        if self.r#waterlogged == true && self.r#facing == Facing::North && self.r#side_chain == SideChain::Right && self.r#powered == true { return 2657; }
        if self.r#powered == false && self.r#facing == Facing::East && self.r#side_chain == SideChain::Left && self.r#waterlogged == true { return 2717; }
        if self.r#side_chain == SideChain::Left && self.r#waterlogged == true && self.r#facing == Facing::North && self.r#powered == true { return 2661; }
        if self.r#facing == Facing::North && self.r#waterlogged == true && self.r#side_chain == SideChain::Left && self.r#powered == false { return 2669; }
        if self.r#side_chain == SideChain::Right && self.r#waterlogged == true && self.r#powered == false && self.r#facing == Facing::East { return 2713; }
        if self.r#powered == true && self.r#waterlogged == false && self.r#facing == Facing::North && self.r#side_chain == SideChain::Left { return 2662; }
        if self.r#side_chain == SideChain::Unconnected && self.r#powered == true && self.r#facing == Facing::East && self.r#waterlogged == true { return 2703; }
        if self.r#powered == false && self.r#facing == Facing::North && self.r#side_chain == SideChain::Right && self.r#waterlogged == true { return 2665; }
        if self.r#facing == Facing::North && self.r#powered == false && self.r#waterlogged == true && self.r#side_chain == SideChain::Unconnected { return 2663; }
        if self.r#facing == Facing::South && self.r#powered == false && self.r#side_chain == SideChain::Left && self.r#waterlogged == false { return 2686; }
        if self.r#powered == true && self.r#side_chain == SideChain::Left && self.r#facing == Facing::West && self.r#waterlogged == true { return 2693; }
        if self.r#facing == Facing::South && self.r#waterlogged == true && self.r#side_chain == SideChain::Unconnected && self.r#powered == true { return 2671; }
        if self.r#facing == Facing::North && self.r#powered == false && self.r#side_chain == SideChain::Center && self.r#waterlogged == true { return 2667; }
        if self.r#side_chain == SideChain::Center && self.r#powered == false && self.r#waterlogged == true && self.r#facing == Facing::West { return 2699; }
        if self.r#facing == Facing::South && self.r#waterlogged == true && self.r#powered == false && self.r#side_chain == SideChain::Left { return 2685; }
        if self.r#waterlogged == true && self.r#powered == true && self.r#side_chain == SideChain::Center && self.r#facing == Facing::West { return 2691; }
        if self.r#side_chain == SideChain::Center && self.r#powered == false && self.r#waterlogged == false && self.r#facing == Facing::West { return 2700; }
        if self.r#side_chain == SideChain::Right && self.r#facing == Facing::West && self.r#powered == false && self.r#waterlogged == true { return 2697; }
        if self.r#waterlogged == false && self.r#powered == true && self.r#facing == Facing::North && self.r#side_chain == SideChain::Unconnected { return 2656; }
        if self.r#side_chain == SideChain::Center && self.r#powered == true && self.r#waterlogged == false && self.r#facing == Facing::West { return 2692; }
        if self.r#powered == true && self.r#waterlogged == false && self.r#facing == Facing::East && self.r#side_chain == SideChain::Unconnected { return 2704; }
        if self.r#facing == Facing::North && self.r#waterlogged == false && self.r#powered == false && self.r#side_chain == SideChain::Center { return 2668; }
        if self.r#facing == Facing::West && self.r#powered == true && self.r#side_chain == SideChain::Unconnected && self.r#waterlogged == true { return 2687; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 2707 {
            return Some(CrimsonShelf {
                r#facing: Facing::East,
                r#powered: true,
                r#side_chain: SideChain::Center,
                r#waterlogged: true,
            });
        }
        if state_id == 2711 {
            return Some(CrimsonShelf {
                r#facing: Facing::East,
                r#waterlogged: true,
                r#powered: false,
                r#side_chain: SideChain::Unconnected,
            });
        }
        if state_id == 2712 {
            return Some(CrimsonShelf {
                r#powered: false,
                r#side_chain: SideChain::Unconnected,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 2682 {
            return Some(CrimsonShelf {
                r#side_chain: SideChain::Right,
                r#waterlogged: false,
                r#facing: Facing::South,
                r#powered: false,
            });
        }
        if state_id == 2666 {
            return Some(CrimsonShelf {
                r#powered: false,
                r#waterlogged: false,
                r#side_chain: SideChain::Right,
                r#facing: Facing::North,
            });
        }
        if state_id == 2716 {
            return Some(CrimsonShelf {
                r#powered: false,
                r#facing: Facing::East,
                r#side_chain: SideChain::Center,
                r#waterlogged: false,
            });
        }
        if state_id == 2681 {
            return Some(CrimsonShelf {
                r#side_chain: SideChain::Right,
                r#waterlogged: true,
                r#facing: Facing::South,
                r#powered: false,
            });
        }
        if state_id == 2670 {
            return Some(CrimsonShelf {
                r#facing: Facing::North,
                r#side_chain: SideChain::Left,
                r#powered: false,
                r#waterlogged: false,
            });
        }
        if state_id == 2709 {
            return Some(CrimsonShelf {
                r#waterlogged: true,
                r#facing: Facing::East,
                r#powered: true,
                r#side_chain: SideChain::Left,
            });
        }
        if state_id == 2710 {
            return Some(CrimsonShelf {
                r#facing: Facing::East,
                r#waterlogged: false,
                r#powered: true,
                r#side_chain: SideChain::Left,
            });
        }
        if state_id == 2718 {
            return Some(CrimsonShelf {
                r#facing: Facing::East,
                r#powered: false,
                r#side_chain: SideChain::Left,
                r#waterlogged: false,
            });
        }
        if state_id == 2658 {
            return Some(CrimsonShelf {
                r#waterlogged: false,
                r#facing: Facing::North,
                r#powered: true,
                r#side_chain: SideChain::Right,
            });
        }
        if state_id == 2679 {
            return Some(CrimsonShelf {
                r#side_chain: SideChain::Unconnected,
                r#facing: Facing::South,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 2655 {
            return Some(CrimsonShelf {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#side_chain: SideChain::Unconnected,
                r#powered: true,
            });
        }
        if state_id == 2673 {
            return Some(CrimsonShelf {
                r#waterlogged: true,
                r#powered: true,
                r#side_chain: SideChain::Right,
                r#facing: Facing::South,
            });
        }
        if state_id == 2695 {
            return Some(CrimsonShelf {
                r#facing: Facing::West,
                r#side_chain: SideChain::Unconnected,
                r#waterlogged: true,
                r#powered: false,
            });
        }
        if state_id == 2701 {
            return Some(CrimsonShelf {
                r#powered: false,
                r#facing: Facing::West,
                r#side_chain: SideChain::Left,
                r#waterlogged: true,
            });
        }
        if state_id == 2702 {
            return Some(CrimsonShelf {
                r#waterlogged: false,
                r#facing: Facing::West,
                r#side_chain: SideChain::Left,
                r#powered: false,
            });
        }
        if state_id == 2706 {
            return Some(CrimsonShelf {
                r#side_chain: SideChain::Right,
                r#waterlogged: false,
                r#facing: Facing::East,
                r#powered: true,
            });
        }
        if state_id == 2672 {
            return Some(CrimsonShelf {
                r#side_chain: SideChain::Unconnected,
                r#waterlogged: false,
                r#facing: Facing::South,
                r#powered: true,
            });
        }
        if state_id == 2676 {
            return Some(CrimsonShelf {
                r#facing: Facing::South,
                r#side_chain: SideChain::Center,
                r#powered: true,
                r#waterlogged: false,
            });
        }
        if state_id == 2684 {
            return Some(CrimsonShelf {
                r#side_chain: SideChain::Center,
                r#waterlogged: false,
                r#powered: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 2715 {
            return Some(CrimsonShelf {
                r#side_chain: SideChain::Center,
                r#facing: Facing::East,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 2678 {
            return Some(CrimsonShelf {
                r#facing: Facing::South,
                r#waterlogged: false,
                r#side_chain: SideChain::Left,
                r#powered: true,
            });
        }
        if state_id == 2660 {
            return Some(CrimsonShelf {
                r#facing: Facing::North,
                r#powered: true,
                r#side_chain: SideChain::Center,
                r#waterlogged: false,
            });
        }
        if state_id == 2674 {
            return Some(CrimsonShelf {
                r#facing: Facing::South,
                r#powered: true,
                r#side_chain: SideChain::Right,
                r#waterlogged: false,
            });
        }
        if state_id == 2680 {
            return Some(CrimsonShelf {
                r#facing: Facing::South,
                r#powered: false,
                r#side_chain: SideChain::Unconnected,
                r#waterlogged: false,
            });
        }
        if state_id == 2664 {
            return Some(CrimsonShelf {
                r#side_chain: SideChain::Unconnected,
                r#powered: false,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 2683 {
            return Some(CrimsonShelf {
                r#powered: false,
                r#facing: Facing::South,
                r#side_chain: SideChain::Center,
                r#waterlogged: true,
            });
        }
        if state_id == 2694 {
            return Some(CrimsonShelf {
                r#waterlogged: false,
                r#facing: Facing::West,
                r#side_chain: SideChain::Left,
                r#powered: true,
            });
        }
        if state_id == 2677 {
            return Some(CrimsonShelf {
                r#powered: true,
                r#side_chain: SideChain::Left,
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 2698 {
            return Some(CrimsonShelf {
                r#facing: Facing::West,
                r#side_chain: SideChain::Right,
                r#powered: false,
                r#waterlogged: false,
            });
        }
        if state_id == 2696 {
            return Some(CrimsonShelf {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#powered: false,
                r#side_chain: SideChain::Unconnected,
            });
        }
        if state_id == 2688 {
            return Some(CrimsonShelf {
                r#waterlogged: false,
                r#facing: Facing::West,
                r#powered: true,
                r#side_chain: SideChain::Unconnected,
            });
        }
        if state_id == 2690 {
            return Some(CrimsonShelf {
                r#facing: Facing::West,
                r#side_chain: SideChain::Right,
                r#waterlogged: false,
                r#powered: true,
            });
        }
        if state_id == 2659 {
            return Some(CrimsonShelf {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#powered: true,
                r#side_chain: SideChain::Center,
            });
        }
        if state_id == 2705 {
            return Some(CrimsonShelf {
                r#powered: true,
                r#side_chain: SideChain::Right,
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 2675 {
            return Some(CrimsonShelf {
                r#facing: Facing::South,
                r#powered: true,
                r#side_chain: SideChain::Center,
                r#waterlogged: true,
            });
        }
        if state_id == 2708 {
            return Some(CrimsonShelf {
                r#powered: true,
                r#waterlogged: false,
                r#facing: Facing::East,
                r#side_chain: SideChain::Center,
            });
        }
        if state_id == 2689 {
            return Some(CrimsonShelf {
                r#side_chain: SideChain::Right,
                r#facing: Facing::West,
                r#waterlogged: true,
                r#powered: true,
            });
        }
        if state_id == 2714 {
            return Some(CrimsonShelf {
                r#powered: false,
                r#facing: Facing::East,
                r#side_chain: SideChain::Right,
                r#waterlogged: false,
            });
        }
        if state_id == 2657 {
            return Some(CrimsonShelf {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#side_chain: SideChain::Right,
                r#powered: true,
            });
        }
        if state_id == 2717 {
            return Some(CrimsonShelf {
                r#powered: false,
                r#facing: Facing::East,
                r#side_chain: SideChain::Left,
                r#waterlogged: true,
            });
        }
        if state_id == 2661 {
            return Some(CrimsonShelf {
                r#side_chain: SideChain::Left,
                r#waterlogged: true,
                r#facing: Facing::North,
                r#powered: true,
            });
        }
        if state_id == 2669 {
            return Some(CrimsonShelf {
                r#facing: Facing::North,
                r#waterlogged: true,
                r#side_chain: SideChain::Left,
                r#powered: false,
            });
        }
        if state_id == 2713 {
            return Some(CrimsonShelf {
                r#side_chain: SideChain::Right,
                r#waterlogged: true,
                r#powered: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 2662 {
            return Some(CrimsonShelf {
                r#powered: true,
                r#waterlogged: false,
                r#facing: Facing::North,
                r#side_chain: SideChain::Left,
            });
        }
        if state_id == 2703 {
            return Some(CrimsonShelf {
                r#side_chain: SideChain::Unconnected,
                r#powered: true,
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 2665 {
            return Some(CrimsonShelf {
                r#powered: false,
                r#facing: Facing::North,
                r#side_chain: SideChain::Right,
                r#waterlogged: true,
            });
        }
        if state_id == 2663 {
            return Some(CrimsonShelf {
                r#facing: Facing::North,
                r#powered: false,
                r#waterlogged: true,
                r#side_chain: SideChain::Unconnected,
            });
        }
        if state_id == 2686 {
            return Some(CrimsonShelf {
                r#facing: Facing::South,
                r#powered: false,
                r#side_chain: SideChain::Left,
                r#waterlogged: false,
            });
        }
        if state_id == 2693 {
            return Some(CrimsonShelf {
                r#powered: true,
                r#side_chain: SideChain::Left,
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 2671 {
            return Some(CrimsonShelf {
                r#facing: Facing::South,
                r#waterlogged: true,
                r#side_chain: SideChain::Unconnected,
                r#powered: true,
            });
        }
        if state_id == 2667 {
            return Some(CrimsonShelf {
                r#facing: Facing::North,
                r#powered: false,
                r#side_chain: SideChain::Center,
                r#waterlogged: true,
            });
        }
        if state_id == 2699 {
            return Some(CrimsonShelf {
                r#side_chain: SideChain::Center,
                r#powered: false,
                r#waterlogged: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 2685 {
            return Some(CrimsonShelf {
                r#facing: Facing::South,
                r#waterlogged: true,
                r#powered: false,
                r#side_chain: SideChain::Left,
            });
        }
        if state_id == 2691 {
            return Some(CrimsonShelf {
                r#waterlogged: true,
                r#powered: true,
                r#side_chain: SideChain::Center,
                r#facing: Facing::West,
            });
        }
        if state_id == 2700 {
            return Some(CrimsonShelf {
                r#side_chain: SideChain::Center,
                r#powered: false,
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 2697 {
            return Some(CrimsonShelf {
                r#side_chain: SideChain::Right,
                r#facing: Facing::West,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 2656 {
            return Some(CrimsonShelf {
                r#waterlogged: false,
                r#powered: true,
                r#facing: Facing::North,
                r#side_chain: SideChain::Unconnected,
            });
        }
        if state_id == 2692 {
            return Some(CrimsonShelf {
                r#side_chain: SideChain::Center,
                r#powered: true,
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 2704 {
            return Some(CrimsonShelf {
                r#powered: true,
                r#waterlogged: false,
                r#facing: Facing::East,
                r#side_chain: SideChain::Unconnected,
            });
        }
        if state_id == 2668 {
            return Some(CrimsonShelf {
                r#facing: Facing::North,
                r#waterlogged: false,
                r#powered: false,
                r#side_chain: SideChain::Center,
            });
        }
        if state_id == 2687 {
            return Some(CrimsonShelf {
                r#facing: Facing::West,
                r#powered: true,
                r#side_chain: SideChain::Unconnected,
                r#waterlogged: true,
            });
        }
        return None;
    }
}

