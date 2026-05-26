use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CherryShelf {
    pub r#side_chain: SideChain,
    pub powered: bool,
    pub waterlogged: bool,
    pub r#facing: Facing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SideChain {
    Unconnected,
    Right,
    Center,
    Left,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for CherryShelf {
    fn to_id(&self) -> i32 {
        if self.r#powered == false
            && self.r#side_chain == SideChain::Right
            && self.r#waterlogged == false
            && self.r#facing == Facing::East
        {
            return 2650;
        }
        if self.r#waterlogged == false
            && self.r#facing == Facing::East
            && self.r#side_chain == SideChain::Unconnected
            && self.r#powered == false
        {
            return 2648;
        }
        if self.r#facing == Facing::West
            && self.r#waterlogged == true
            && self.r#powered == true
            && self.r#side_chain == SideChain::Right
        {
            return 2625;
        }
        if self.r#side_chain == SideChain::Unconnected
            && self.r#facing == Facing::South
            && self.r#powered == true
            && self.r#waterlogged == true
        {
            return 2607;
        }
        if self.r#powered == false
            && self.r#side_chain == SideChain::Center
            && self.r#waterlogged == false
            && self.r#facing == Facing::South
        {
            return 2620;
        }
        if self.r#waterlogged == false
            && self.r#facing == Facing::North
            && self.r#side_chain == SideChain::Unconnected
            && self.r#powered == false
        {
            return 2600;
        }
        if self.r#waterlogged == false
            && self.r#side_chain == SideChain::Unconnected
            && self.r#powered == true
            && self.r#facing == Facing::West
        {
            return 2624;
        }
        if self.r#waterlogged == false
            && self.r#powered == true
            && self.r#side_chain == SideChain::Right
            && self.r#facing == Facing::North
        {
            return 2594;
        }
        if self.r#side_chain == SideChain::Right
            && self.r#facing == Facing::West
            && self.r#waterlogged == false
            && self.r#powered == false
        {
            return 2634;
        }
        if self.r#facing == Facing::East
            && self.r#side_chain == SideChain::Center
            && self.r#powered == false
            && self.r#waterlogged == true
        {
            return 2651;
        }
        if self.r#facing == Facing::West
            && self.r#powered == false
            && self.r#side_chain == SideChain::Left
            && self.r#waterlogged == false
        {
            return 2638;
        }
        if self.r#facing == Facing::East
            && self.r#side_chain == SideChain::Left
            && self.r#waterlogged == false
            && self.r#powered == false
        {
            return 2654;
        }
        if self.r#side_chain == SideChain::Unconnected
            && self.r#powered == false
            && self.r#facing == Facing::South
            && self.r#waterlogged == false
        {
            return 2616;
        }
        if self.r#waterlogged == true
            && self.r#side_chain == SideChain::Right
            && self.r#powered == false
            && self.r#facing == Facing::East
        {
            return 2649;
        }
        if self.r#facing == Facing::North
            && self.r#waterlogged == false
            && self.r#side_chain == SideChain::Center
            && self.r#powered == false
        {
            return 2604;
        }
        if self.r#facing == Facing::South
            && self.r#side_chain == SideChain::Right
            && self.r#powered == false
            && self.r#waterlogged == true
        {
            return 2617;
        }
        if self.r#waterlogged == false
            && self.r#facing == Facing::West
            && self.r#side_chain == SideChain::Right
            && self.r#powered == true
        {
            return 2626;
        }
        if self.r#facing == Facing::West
            && self.r#side_chain == SideChain::Left
            && self.r#waterlogged == true
            && self.r#powered == true
        {
            return 2629;
        }
        if self.r#facing == Facing::North
            && self.r#side_chain == SideChain::Center
            && self.r#waterlogged == false
            && self.r#powered == true
        {
            return 2596;
        }
        if self.r#powered == true
            && self.r#waterlogged == false
            && self.r#side_chain == SideChain::Center
            && self.r#facing == Facing::West
        {
            return 2628;
        }
        if self.r#facing == Facing::South
            && self.r#waterlogged == true
            && self.r#powered == true
            && self.r#side_chain == SideChain::Left
        {
            return 2613;
        }
        if self.r#powered == false
            && self.r#facing == Facing::North
            && self.r#side_chain == SideChain::Left
            && self.r#waterlogged == true
        {
            return 2605;
        }
        if self.r#waterlogged == false
            && self.r#facing == Facing::West
            && self.r#powered == true
            && self.r#side_chain == SideChain::Left
        {
            return 2630;
        }
        if self.r#powered == true
            && self.r#waterlogged == false
            && self.r#side_chain == SideChain::Center
            && self.r#facing == Facing::South
        {
            return 2612;
        }
        if self.r#side_chain == SideChain::Unconnected
            && self.r#facing == Facing::East
            && self.r#waterlogged == false
            && self.r#powered == true
        {
            return 2640;
        }
        if self.r#side_chain == SideChain::Left
            && self.r#waterlogged == false
            && self.r#facing == Facing::North
            && self.r#powered == true
        {
            return 2598;
        }
        if self.r#powered == false
            && self.r#facing == Facing::North
            && self.r#waterlogged == false
            && self.r#side_chain == SideChain::Left
        {
            return 2606;
        }
        if self.r#facing == Facing::North
            && self.r#waterlogged == false
            && self.r#powered == false
            && self.r#side_chain == SideChain::Right
        {
            return 2602;
        }
        if self.r#powered == true
            && self.r#facing == Facing::East
            && self.r#waterlogged == true
            && self.r#side_chain == SideChain::Unconnected
        {
            return 2639;
        }
        if self.r#powered == false
            && self.r#waterlogged == true
            && self.r#side_chain == SideChain::Unconnected
            && self.r#facing == Facing::North
        {
            return 2599;
        }
        if self.r#side_chain == SideChain::Unconnected
            && self.r#powered == true
            && self.r#waterlogged == false
            && self.r#facing == Facing::South
        {
            return 2608;
        }
        if self.r#facing == Facing::West
            && self.r#waterlogged == true
            && self.r#powered == false
            && self.r#side_chain == SideChain::Left
        {
            return 2637;
        }
        if self.r#waterlogged == false
            && self.r#facing == Facing::East
            && self.r#powered == true
            && self.r#side_chain == SideChain::Right
        {
            return 2642;
        }
        if self.r#side_chain == SideChain::Center
            && self.r#powered == false
            && self.r#facing == Facing::South
            && self.r#waterlogged == true
        {
            return 2619;
        }
        if self.r#facing == Facing::North
            && self.r#powered == true
            && self.r#side_chain == SideChain::Unconnected
            && self.r#waterlogged == true
        {
            return 2591;
        }
        if self.r#facing == Facing::East
            && self.r#powered == false
            && self.r#waterlogged == true
            && self.r#side_chain == SideChain::Left
        {
            return 2653;
        }
        if self.r#facing == Facing::North
            && self.r#waterlogged == true
            && self.r#side_chain == SideChain::Right
            && self.r#powered == false
        {
            return 2601;
        }
        if self.r#waterlogged == true
            && self.r#side_chain == SideChain::Center
            && self.r#facing == Facing::East
            && self.r#powered == true
        {
            return 2643;
        }
        if self.r#facing == Facing::South
            && self.r#waterlogged == false
            && self.r#side_chain == SideChain::Left
            && self.r#powered == true
        {
            return 2614;
        }
        if self.r#waterlogged == true
            && self.r#side_chain == SideChain::Right
            && self.r#facing == Facing::West
            && self.r#powered == false
        {
            return 2633;
        }
        if self.r#facing == Facing::East
            && self.r#side_chain == SideChain::Center
            && self.r#waterlogged == false
            && self.r#powered == false
        {
            return 2652;
        }
        if self.r#powered == true
            && self.r#side_chain == SideChain::Center
            && self.r#facing == Facing::South
            && self.r#waterlogged == true
        {
            return 2611;
        }
        if self.r#powered == true
            && self.r#waterlogged == true
            && self.r#side_chain == SideChain::Unconnected
            && self.r#facing == Facing::West
        {
            return 2623;
        }
        if self.r#powered == false
            && self.r#waterlogged == false
            && self.r#facing == Facing::South
            && self.r#side_chain == SideChain::Right
        {
            return 2618;
        }
        if self.r#powered == false
            && self.r#facing == Facing::East
            && self.r#waterlogged == true
            && self.r#side_chain == SideChain::Unconnected
        {
            return 2647;
        }
        if self.r#powered == true
            && self.r#facing == Facing::North
            && self.r#waterlogged == true
            && self.r#side_chain == SideChain::Center
        {
            return 2595;
        }
        if self.r#facing == Facing::South
            && self.r#powered == true
            && self.r#waterlogged == false
            && self.r#side_chain == SideChain::Right
        {
            return 2610;
        }
        if self.r#powered == false
            && self.r#facing == Facing::South
            && self.r#waterlogged == true
            && self.r#side_chain == SideChain::Unconnected
        {
            return 2615;
        }
        if self.r#facing == Facing::East
            && self.r#powered == true
            && self.r#waterlogged == true
            && self.r#side_chain == SideChain::Right
        {
            return 2641;
        }
        if self.r#waterlogged == false
            && self.r#powered == true
            && self.r#facing == Facing::North
            && self.r#side_chain == SideChain::Unconnected
        {
            return 2592;
        }
        if self.r#waterlogged == true
            && self.r#side_chain == SideChain::Left
            && self.r#powered == false
            && self.r#facing == Facing::South
        {
            return 2621;
        }
        if self.r#facing == Facing::West
            && self.r#powered == false
            && self.r#side_chain == SideChain::Center
            && self.r#waterlogged == false
        {
            return 2636;
        }
        if self.r#facing == Facing::South
            && self.r#waterlogged == false
            && self.r#side_chain == SideChain::Left
            && self.r#powered == false
        {
            return 2622;
        }
        if self.r#side_chain == SideChain::Unconnected
            && self.r#waterlogged == true
            && self.r#facing == Facing::West
            && self.r#powered == false
        {
            return 2631;
        }
        if self.r#facing == Facing::North
            && self.r#powered == true
            && self.r#side_chain == SideChain::Left
            && self.r#waterlogged == true
        {
            return 2597;
        }
        if self.r#facing == Facing::North
            && self.r#waterlogged == true
            && self.r#side_chain == SideChain::Right
            && self.r#powered == true
        {
            return 2593;
        }
        if self.r#powered == false
            && self.r#facing == Facing::West
            && self.r#waterlogged == false
            && self.r#side_chain == SideChain::Unconnected
        {
            return 2632;
        }
        if self.r#side_chain == SideChain::Left
            && self.r#waterlogged == true
            && self.r#powered == true
            && self.r#facing == Facing::East
        {
            return 2645;
        }
        if self.r#facing == Facing::South
            && self.r#waterlogged == true
            && self.r#powered == true
            && self.r#side_chain == SideChain::Right
        {
            return 2609;
        }
        if self.r#side_chain == SideChain::Left
            && self.r#powered == true
            && self.r#facing == Facing::East
            && self.r#waterlogged == false
        {
            return 2646;
        }
        if self.r#powered == true
            && self.r#waterlogged == true
            && self.r#side_chain == SideChain::Center
            && self.r#facing == Facing::West
        {
            return 2627;
        }
        if self.r#powered == false
            && self.r#waterlogged == true
            && self.r#facing == Facing::West
            && self.r#side_chain == SideChain::Center
        {
            return 2635;
        }
        if self.r#side_chain == SideChain::Center
            && self.r#waterlogged == false
            && self.r#powered == true
            && self.r#facing == Facing::East
        {
            return 2644;
        }
        if self.r#powered == false
            && self.r#waterlogged == true
            && self.r#side_chain == SideChain::Center
            && self.r#facing == Facing::North
        {
            return 2603;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 2650 {
            return Some(CherryShelf {
                r#powered: false,
                r#side_chain: SideChain::Right,
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 2648 {
            return Some(CherryShelf {
                r#waterlogged: false,
                r#facing: Facing::East,
                r#side_chain: SideChain::Unconnected,
                r#powered: false,
            });
        }
        if state_id == 2625 {
            return Some(CherryShelf {
                r#facing: Facing::West,
                r#waterlogged: true,
                r#powered: true,
                r#side_chain: SideChain::Right,
            });
        }
        if state_id == 2607 {
            return Some(CherryShelf {
                r#side_chain: SideChain::Unconnected,
                r#facing: Facing::South,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 2620 {
            return Some(CherryShelf {
                r#powered: false,
                r#side_chain: SideChain::Center,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 2600 {
            return Some(CherryShelf {
                r#waterlogged: false,
                r#facing: Facing::North,
                r#side_chain: SideChain::Unconnected,
                r#powered: false,
            });
        }
        if state_id == 2624 {
            return Some(CherryShelf {
                r#waterlogged: false,
                r#side_chain: SideChain::Unconnected,
                r#powered: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 2594 {
            return Some(CherryShelf {
                r#waterlogged: false,
                r#powered: true,
                r#side_chain: SideChain::Right,
                r#facing: Facing::North,
            });
        }
        if state_id == 2634 {
            return Some(CherryShelf {
                r#side_chain: SideChain::Right,
                r#facing: Facing::West,
                r#waterlogged: false,
                r#powered: false,
            });
        }
        if state_id == 2651 {
            return Some(CherryShelf {
                r#facing: Facing::East,
                r#side_chain: SideChain::Center,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 2638 {
            return Some(CherryShelf {
                r#facing: Facing::West,
                r#powered: false,
                r#side_chain: SideChain::Left,
                r#waterlogged: false,
            });
        }
        if state_id == 2654 {
            return Some(CherryShelf {
                r#facing: Facing::East,
                r#side_chain: SideChain::Left,
                r#waterlogged: false,
                r#powered: false,
            });
        }
        if state_id == 2616 {
            return Some(CherryShelf {
                r#side_chain: SideChain::Unconnected,
                r#powered: false,
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 2649 {
            return Some(CherryShelf {
                r#waterlogged: true,
                r#side_chain: SideChain::Right,
                r#powered: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 2604 {
            return Some(CherryShelf {
                r#facing: Facing::North,
                r#waterlogged: false,
                r#side_chain: SideChain::Center,
                r#powered: false,
            });
        }
        if state_id == 2617 {
            return Some(CherryShelf {
                r#facing: Facing::South,
                r#side_chain: SideChain::Right,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 2626 {
            return Some(CherryShelf {
                r#waterlogged: false,
                r#facing: Facing::West,
                r#side_chain: SideChain::Right,
                r#powered: true,
            });
        }
        if state_id == 2629 {
            return Some(CherryShelf {
                r#facing: Facing::West,
                r#side_chain: SideChain::Left,
                r#waterlogged: true,
                r#powered: true,
            });
        }
        if state_id == 2596 {
            return Some(CherryShelf {
                r#facing: Facing::North,
                r#side_chain: SideChain::Center,
                r#waterlogged: false,
                r#powered: true,
            });
        }
        if state_id == 2628 {
            return Some(CherryShelf {
                r#powered: true,
                r#waterlogged: false,
                r#side_chain: SideChain::Center,
                r#facing: Facing::West,
            });
        }
        if state_id == 2613 {
            return Some(CherryShelf {
                r#facing: Facing::South,
                r#waterlogged: true,
                r#powered: true,
                r#side_chain: SideChain::Left,
            });
        }
        if state_id == 2605 {
            return Some(CherryShelf {
                r#powered: false,
                r#facing: Facing::North,
                r#side_chain: SideChain::Left,
                r#waterlogged: true,
            });
        }
        if state_id == 2630 {
            return Some(CherryShelf {
                r#waterlogged: false,
                r#facing: Facing::West,
                r#powered: true,
                r#side_chain: SideChain::Left,
            });
        }
        if state_id == 2612 {
            return Some(CherryShelf {
                r#powered: true,
                r#waterlogged: false,
                r#side_chain: SideChain::Center,
                r#facing: Facing::South,
            });
        }
        if state_id == 2640 {
            return Some(CherryShelf {
                r#side_chain: SideChain::Unconnected,
                r#facing: Facing::East,
                r#waterlogged: false,
                r#powered: true,
            });
        }
        if state_id == 2598 {
            return Some(CherryShelf {
                r#side_chain: SideChain::Left,
                r#waterlogged: false,
                r#facing: Facing::North,
                r#powered: true,
            });
        }
        if state_id == 2606 {
            return Some(CherryShelf {
                r#powered: false,
                r#facing: Facing::North,
                r#waterlogged: false,
                r#side_chain: SideChain::Left,
            });
        }
        if state_id == 2602 {
            return Some(CherryShelf {
                r#facing: Facing::North,
                r#waterlogged: false,
                r#powered: false,
                r#side_chain: SideChain::Right,
            });
        }
        if state_id == 2639 {
            return Some(CherryShelf {
                r#powered: true,
                r#facing: Facing::East,
                r#waterlogged: true,
                r#side_chain: SideChain::Unconnected,
            });
        }
        if state_id == 2599 {
            return Some(CherryShelf {
                r#powered: false,
                r#waterlogged: true,
                r#side_chain: SideChain::Unconnected,
                r#facing: Facing::North,
            });
        }
        if state_id == 2608 {
            return Some(CherryShelf {
                r#side_chain: SideChain::Unconnected,
                r#powered: true,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 2637 {
            return Some(CherryShelf {
                r#facing: Facing::West,
                r#waterlogged: true,
                r#powered: false,
                r#side_chain: SideChain::Left,
            });
        }
        if state_id == 2642 {
            return Some(CherryShelf {
                r#waterlogged: false,
                r#facing: Facing::East,
                r#powered: true,
                r#side_chain: SideChain::Right,
            });
        }
        if state_id == 2619 {
            return Some(CherryShelf {
                r#side_chain: SideChain::Center,
                r#powered: false,
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 2591 {
            return Some(CherryShelf {
                r#facing: Facing::North,
                r#powered: true,
                r#side_chain: SideChain::Unconnected,
                r#waterlogged: true,
            });
        }
        if state_id == 2653 {
            return Some(CherryShelf {
                r#facing: Facing::East,
                r#powered: false,
                r#waterlogged: true,
                r#side_chain: SideChain::Left,
            });
        }
        if state_id == 2601 {
            return Some(CherryShelf {
                r#facing: Facing::North,
                r#waterlogged: true,
                r#side_chain: SideChain::Right,
                r#powered: false,
            });
        }
        if state_id == 2643 {
            return Some(CherryShelf {
                r#waterlogged: true,
                r#side_chain: SideChain::Center,
                r#facing: Facing::East,
                r#powered: true,
            });
        }
        if state_id == 2614 {
            return Some(CherryShelf {
                r#facing: Facing::South,
                r#waterlogged: false,
                r#side_chain: SideChain::Left,
                r#powered: true,
            });
        }
        if state_id == 2633 {
            return Some(CherryShelf {
                r#waterlogged: true,
                r#side_chain: SideChain::Right,
                r#facing: Facing::West,
                r#powered: false,
            });
        }
        if state_id == 2652 {
            return Some(CherryShelf {
                r#facing: Facing::East,
                r#side_chain: SideChain::Center,
                r#waterlogged: false,
                r#powered: false,
            });
        }
        if state_id == 2611 {
            return Some(CherryShelf {
                r#powered: true,
                r#side_chain: SideChain::Center,
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 2623 {
            return Some(CherryShelf {
                r#powered: true,
                r#waterlogged: true,
                r#side_chain: SideChain::Unconnected,
                r#facing: Facing::West,
            });
        }
        if state_id == 2618 {
            return Some(CherryShelf {
                r#powered: false,
                r#waterlogged: false,
                r#facing: Facing::South,
                r#side_chain: SideChain::Right,
            });
        }
        if state_id == 2647 {
            return Some(CherryShelf {
                r#powered: false,
                r#facing: Facing::East,
                r#waterlogged: true,
                r#side_chain: SideChain::Unconnected,
            });
        }
        if state_id == 2595 {
            return Some(CherryShelf {
                r#powered: true,
                r#facing: Facing::North,
                r#waterlogged: true,
                r#side_chain: SideChain::Center,
            });
        }
        if state_id == 2610 {
            return Some(CherryShelf {
                r#facing: Facing::South,
                r#powered: true,
                r#waterlogged: false,
                r#side_chain: SideChain::Right,
            });
        }
        if state_id == 2615 {
            return Some(CherryShelf {
                r#powered: false,
                r#facing: Facing::South,
                r#waterlogged: true,
                r#side_chain: SideChain::Unconnected,
            });
        }
        if state_id == 2641 {
            return Some(CherryShelf {
                r#facing: Facing::East,
                r#powered: true,
                r#waterlogged: true,
                r#side_chain: SideChain::Right,
            });
        }
        if state_id == 2592 {
            return Some(CherryShelf {
                r#waterlogged: false,
                r#powered: true,
                r#facing: Facing::North,
                r#side_chain: SideChain::Unconnected,
            });
        }
        if state_id == 2621 {
            return Some(CherryShelf {
                r#waterlogged: true,
                r#side_chain: SideChain::Left,
                r#powered: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 2636 {
            return Some(CherryShelf {
                r#facing: Facing::West,
                r#powered: false,
                r#side_chain: SideChain::Center,
                r#waterlogged: false,
            });
        }
        if state_id == 2622 {
            return Some(CherryShelf {
                r#facing: Facing::South,
                r#waterlogged: false,
                r#side_chain: SideChain::Left,
                r#powered: false,
            });
        }
        if state_id == 2631 {
            return Some(CherryShelf {
                r#side_chain: SideChain::Unconnected,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#powered: false,
            });
        }
        if state_id == 2597 {
            return Some(CherryShelf {
                r#facing: Facing::North,
                r#powered: true,
                r#side_chain: SideChain::Left,
                r#waterlogged: true,
            });
        }
        if state_id == 2593 {
            return Some(CherryShelf {
                r#facing: Facing::North,
                r#waterlogged: true,
                r#side_chain: SideChain::Right,
                r#powered: true,
            });
        }
        if state_id == 2632 {
            return Some(CherryShelf {
                r#powered: false,
                r#facing: Facing::West,
                r#waterlogged: false,
                r#side_chain: SideChain::Unconnected,
            });
        }
        if state_id == 2645 {
            return Some(CherryShelf {
                r#side_chain: SideChain::Left,
                r#waterlogged: true,
                r#powered: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 2609 {
            return Some(CherryShelf {
                r#facing: Facing::South,
                r#waterlogged: true,
                r#powered: true,
                r#side_chain: SideChain::Right,
            });
        }
        if state_id == 2646 {
            return Some(CherryShelf {
                r#side_chain: SideChain::Left,
                r#powered: true,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 2627 {
            return Some(CherryShelf {
                r#powered: true,
                r#waterlogged: true,
                r#side_chain: SideChain::Center,
                r#facing: Facing::West,
            });
        }
        if state_id == 2635 {
            return Some(CherryShelf {
                r#powered: false,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#side_chain: SideChain::Center,
            });
        }
        if state_id == 2644 {
            return Some(CherryShelf {
                r#side_chain: SideChain::Center,
                r#waterlogged: false,
                r#powered: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 2603 {
            return Some(CherryShelf {
                r#powered: false,
                r#waterlogged: true,
                r#side_chain: SideChain::Center,
                r#facing: Facing::North,
            });
        }
        return None;
    }
}
