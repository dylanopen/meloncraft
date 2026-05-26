use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JungleShelf {
    pub powered: bool,
    pub waterlogged: bool,
    pub r#side_chain: SideChain,
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

impl BlockState for JungleShelf {
    fn to_id(&self) -> i32 {
        if self.r#side_chain == SideChain::Left
            && self.r#waterlogged == true
            && self.r#facing == Facing::West
            && self.r#powered == false
        {
            return 2829;
        }
        if self.r#powered == false
            && self.r#waterlogged == true
            && self.r#facing == Facing::North
            && self.r#side_chain == SideChain::Center
        {
            return 2795;
        }
        if self.r#facing == Facing::South
            && self.r#waterlogged == false
            && self.r#powered == false
            && self.r#side_chain == SideChain::Unconnected
        {
            return 2808;
        }
        if self.r#facing == Facing::West
            && self.r#side_chain == SideChain::Right
            && self.r#powered == false
            && self.r#waterlogged == true
        {
            return 2825;
        }
        if self.r#side_chain == SideChain::Center
            && self.r#powered == false
            && self.r#waterlogged == false
            && self.r#facing == Facing::West
        {
            return 2828;
        }
        if self.r#powered == false
            && self.r#facing == Facing::East
            && self.r#side_chain == SideChain::Unconnected
            && self.r#waterlogged == true
        {
            return 2839;
        }
        if self.r#powered == true
            && self.r#facing == Facing::North
            && self.r#side_chain == SideChain::Unconnected
            && self.r#waterlogged == false
        {
            return 2784;
        }
        if self.r#facing == Facing::West
            && self.r#powered == true
            && self.r#side_chain == SideChain::Right
            && self.r#waterlogged == false
        {
            return 2818;
        }
        if self.r#waterlogged == true
            && self.r#facing == Facing::West
            && self.r#powered == false
            && self.r#side_chain == SideChain::Center
        {
            return 2827;
        }
        if self.r#side_chain == SideChain::Left
            && self.r#waterlogged == false
            && self.r#powered == true
            && self.r#facing == Facing::South
        {
            return 2806;
        }
        if self.r#powered == true
            && self.r#waterlogged == false
            && self.r#facing == Facing::West
            && self.r#side_chain == SideChain::Unconnected
        {
            return 2816;
        }
        if self.r#powered == false
            && self.r#facing == Facing::North
            && self.r#side_chain == SideChain::Left
            && self.r#waterlogged == false
        {
            return 2798;
        }
        if self.r#powered == false
            && self.r#side_chain == SideChain::Unconnected
            && self.r#facing == Facing::North
            && self.r#waterlogged == true
        {
            return 2791;
        }
        if self.r#side_chain == SideChain::Center
            && self.r#facing == Facing::South
            && self.r#powered == false
            && self.r#waterlogged == false
        {
            return 2812;
        }
        if self.r#facing == Facing::East
            && self.r#waterlogged == true
            && self.r#side_chain == SideChain::Left
            && self.r#powered == false
        {
            return 2845;
        }
        if self.r#facing == Facing::East
            && self.r#powered == false
            && self.r#side_chain == SideChain::Unconnected
            && self.r#waterlogged == false
        {
            return 2840;
        }
        if self.r#waterlogged == true
            && self.r#powered == true
            && self.r#side_chain == SideChain::Left
            && self.r#facing == Facing::North
        {
            return 2789;
        }
        if self.r#facing == Facing::East
            && self.r#waterlogged == false
            && self.r#powered == true
            && self.r#side_chain == SideChain::Unconnected
        {
            return 2832;
        }
        if self.r#side_chain == SideChain::Unconnected
            && self.r#facing == Facing::South
            && self.r#waterlogged == true
            && self.r#powered == true
        {
            return 2799;
        }
        if self.r#side_chain == SideChain::Right
            && self.r#waterlogged == false
            && self.r#facing == Facing::North
            && self.r#powered == true
        {
            return 2786;
        }
        if self.r#side_chain == SideChain::Left
            && self.r#powered == true
            && self.r#waterlogged == false
            && self.r#facing == Facing::East
        {
            return 2838;
        }
        if self.r#side_chain == SideChain::Left
            && self.r#powered == false
            && self.r#waterlogged == false
            && self.r#facing == Facing::South
        {
            return 2814;
        }
        if self.r#side_chain == SideChain::Unconnected
            && self.r#powered == false
            && self.r#waterlogged == false
            && self.r#facing == Facing::West
        {
            return 2824;
        }
        if self.r#side_chain == SideChain::Right
            && self.r#facing == Facing::West
            && self.r#waterlogged == true
            && self.r#powered == true
        {
            return 2817;
        }
        if self.r#powered == true
            && self.r#side_chain == SideChain::Left
            && self.r#facing == Facing::West
            && self.r#waterlogged == false
        {
            return 2822;
        }
        if self.r#facing == Facing::East
            && self.r#side_chain == SideChain::Left
            && self.r#powered == false
            && self.r#waterlogged == false
        {
            return 2846;
        }
        if self.r#waterlogged == false
            && self.r#powered == true
            && self.r#facing == Facing::West
            && self.r#side_chain == SideChain::Center
        {
            return 2820;
        }
        if self.r#side_chain == SideChain::Center
            && self.r#powered == true
            && self.r#waterlogged == true
            && self.r#facing == Facing::South
        {
            return 2803;
        }
        if self.r#facing == Facing::East
            && self.r#waterlogged == false
            && self.r#powered == false
            && self.r#side_chain == SideChain::Right
        {
            return 2842;
        }
        if self.r#powered == false
            && self.r#side_chain == SideChain::Left
            && self.r#facing == Facing::West
            && self.r#waterlogged == false
        {
            return 2830;
        }
        if self.r#powered == true
            && self.r#side_chain == SideChain::Right
            && self.r#facing == Facing::North
            && self.r#waterlogged == true
        {
            return 2785;
        }
        if self.r#waterlogged == false
            && self.r#powered == true
            && self.r#side_chain == SideChain::Center
            && self.r#facing == Facing::South
        {
            return 2804;
        }
        if self.r#facing == Facing::South
            && self.r#waterlogged == true
            && self.r#side_chain == SideChain::Right
            && self.r#powered == true
        {
            return 2801;
        }
        if self.r#facing == Facing::North
            && self.r#side_chain == SideChain::Left
            && self.r#powered == true
            && self.r#waterlogged == false
        {
            return 2790;
        }
        if self.r#side_chain == SideChain::Center
            && self.r#powered == false
            && self.r#waterlogged == true
            && self.r#facing == Facing::South
        {
            return 2811;
        }
        if self.r#side_chain == SideChain::Right
            && self.r#powered == true
            && self.r#facing == Facing::East
            && self.r#waterlogged == true
        {
            return 2833;
        }
        if self.r#waterlogged == true
            && self.r#facing == Facing::East
            && self.r#side_chain == SideChain::Left
            && self.r#powered == true
        {
            return 2837;
        }
        if self.r#waterlogged == true
            && self.r#powered == false
            && self.r#facing == Facing::East
            && self.r#side_chain == SideChain::Right
        {
            return 2841;
        }
        if self.r#side_chain == SideChain::Center
            && self.r#waterlogged == true
            && self.r#powered == true
            && self.r#facing == Facing::North
        {
            return 2787;
        }
        if self.r#facing == Facing::South
            && self.r#waterlogged == true
            && self.r#side_chain == SideChain::Right
            && self.r#powered == false
        {
            return 2809;
        }
        if self.r#facing == Facing::North
            && self.r#powered == true
            && self.r#side_chain == SideChain::Unconnected
            && self.r#waterlogged == true
        {
            return 2783;
        }
        if self.r#powered == false
            && self.r#side_chain == SideChain::Center
            && self.r#waterlogged == false
            && self.r#facing == Facing::North
        {
            return 2796;
        }
        if self.r#facing == Facing::East
            && self.r#powered == true
            && self.r#waterlogged == false
            && self.r#side_chain == SideChain::Right
        {
            return 2834;
        }
        if self.r#side_chain == SideChain::Center
            && self.r#waterlogged == false
            && self.r#facing == Facing::East
            && self.r#powered == true
        {
            return 2836;
        }
        if self.r#facing == Facing::North
            && self.r#side_chain == SideChain::Right
            && self.r#powered == false
            && self.r#waterlogged == false
        {
            return 2794;
        }
        if self.r#side_chain == SideChain::Left
            && self.r#facing == Facing::South
            && self.r#waterlogged == true
            && self.r#powered == false
        {
            return 2813;
        }
        if self.r#waterlogged == false
            && self.r#facing == Facing::West
            && self.r#powered == false
            && self.r#side_chain == SideChain::Right
        {
            return 2826;
        }
        if self.r#facing == Facing::North
            && self.r#powered == true
            && self.r#side_chain == SideChain::Center
            && self.r#waterlogged == false
        {
            return 2788;
        }
        if self.r#powered == true
            && self.r#side_chain == SideChain::Left
            && self.r#waterlogged == true
            && self.r#facing == Facing::West
        {
            return 2821;
        }
        if self.r#side_chain == SideChain::Unconnected
            && self.r#facing == Facing::West
            && self.r#powered == false
            && self.r#waterlogged == true
        {
            return 2823;
        }
        if self.r#side_chain == SideChain::Right
            && self.r#waterlogged == false
            && self.r#powered == true
            && self.r#facing == Facing::South
        {
            return 2802;
        }
        if self.r#powered == false
            && self.r#side_chain == SideChain::Right
            && self.r#facing == Facing::South
            && self.r#waterlogged == false
        {
            return 2810;
        }
        if self.r#facing == Facing::East
            && self.r#side_chain == SideChain::Center
            && self.r#waterlogged == true
            && self.r#powered == false
        {
            return 2843;
        }
        if self.r#powered == true
            && self.r#facing == Facing::East
            && self.r#side_chain == SideChain::Unconnected
            && self.r#waterlogged == true
        {
            return 2831;
        }
        if self.r#powered == false
            && self.r#side_chain == SideChain::Right
            && self.r#waterlogged == true
            && self.r#facing == Facing::North
        {
            return 2793;
        }
        if self.r#waterlogged == false
            && self.r#powered == false
            && self.r#facing == Facing::North
            && self.r#side_chain == SideChain::Unconnected
        {
            return 2792;
        }
        if self.r#waterlogged == true
            && self.r#side_chain == SideChain::Left
            && self.r#powered == true
            && self.r#facing == Facing::South
        {
            return 2805;
        }
        if self.r#side_chain == SideChain::Center
            && self.r#powered == true
            && self.r#facing == Facing::East
            && self.r#waterlogged == true
        {
            return 2835;
        }
        if self.r#facing == Facing::West
            && self.r#powered == true
            && self.r#waterlogged == true
            && self.r#side_chain == SideChain::Unconnected
        {
            return 2815;
        }
        if self.r#powered == true
            && self.r#facing == Facing::West
            && self.r#side_chain == SideChain::Center
            && self.r#waterlogged == true
        {
            return 2819;
        }
        if self.r#facing == Facing::South
            && self.r#waterlogged == false
            && self.r#side_chain == SideChain::Unconnected
            && self.r#powered == true
        {
            return 2800;
        }
        if self.r#powered == false
            && self.r#facing == Facing::North
            && self.r#side_chain == SideChain::Left
            && self.r#waterlogged == true
        {
            return 2797;
        }
        if self.r#side_chain == SideChain::Unconnected
            && self.r#waterlogged == true
            && self.r#facing == Facing::South
            && self.r#powered == false
        {
            return 2807;
        }
        if self.r#facing == Facing::East
            && self.r#waterlogged == false
            && self.r#powered == false
            && self.r#side_chain == SideChain::Center
        {
            return 2844;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 2829 {
            return Some(JungleShelf {
                r#side_chain: SideChain::Left,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#powered: false,
            });
        }
        if state_id == 2795 {
            return Some(JungleShelf {
                r#powered: false,
                r#waterlogged: true,
                r#facing: Facing::North,
                r#side_chain: SideChain::Center,
            });
        }
        if state_id == 2808 {
            return Some(JungleShelf {
                r#facing: Facing::South,
                r#waterlogged: false,
                r#powered: false,
                r#side_chain: SideChain::Unconnected,
            });
        }
        if state_id == 2825 {
            return Some(JungleShelf {
                r#facing: Facing::West,
                r#side_chain: SideChain::Right,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 2828 {
            return Some(JungleShelf {
                r#side_chain: SideChain::Center,
                r#powered: false,
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 2839 {
            return Some(JungleShelf {
                r#powered: false,
                r#facing: Facing::East,
                r#side_chain: SideChain::Unconnected,
                r#waterlogged: true,
            });
        }
        if state_id == 2784 {
            return Some(JungleShelf {
                r#powered: true,
                r#facing: Facing::North,
                r#side_chain: SideChain::Unconnected,
                r#waterlogged: false,
            });
        }
        if state_id == 2818 {
            return Some(JungleShelf {
                r#facing: Facing::West,
                r#powered: true,
                r#side_chain: SideChain::Right,
                r#waterlogged: false,
            });
        }
        if state_id == 2827 {
            return Some(JungleShelf {
                r#waterlogged: true,
                r#facing: Facing::West,
                r#powered: false,
                r#side_chain: SideChain::Center,
            });
        }
        if state_id == 2806 {
            return Some(JungleShelf {
                r#side_chain: SideChain::Left,
                r#waterlogged: false,
                r#powered: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 2816 {
            return Some(JungleShelf {
                r#powered: true,
                r#waterlogged: false,
                r#facing: Facing::West,
                r#side_chain: SideChain::Unconnected,
            });
        }
        if state_id == 2798 {
            return Some(JungleShelf {
                r#powered: false,
                r#facing: Facing::North,
                r#side_chain: SideChain::Left,
                r#waterlogged: false,
            });
        }
        if state_id == 2791 {
            return Some(JungleShelf {
                r#powered: false,
                r#side_chain: SideChain::Unconnected,
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 2812 {
            return Some(JungleShelf {
                r#side_chain: SideChain::Center,
                r#facing: Facing::South,
                r#powered: false,
                r#waterlogged: false,
            });
        }
        if state_id == 2845 {
            return Some(JungleShelf {
                r#facing: Facing::East,
                r#waterlogged: true,
                r#side_chain: SideChain::Left,
                r#powered: false,
            });
        }
        if state_id == 2840 {
            return Some(JungleShelf {
                r#facing: Facing::East,
                r#powered: false,
                r#side_chain: SideChain::Unconnected,
                r#waterlogged: false,
            });
        }
        if state_id == 2789 {
            return Some(JungleShelf {
                r#waterlogged: true,
                r#powered: true,
                r#side_chain: SideChain::Left,
                r#facing: Facing::North,
            });
        }
        if state_id == 2832 {
            return Some(JungleShelf {
                r#facing: Facing::East,
                r#waterlogged: false,
                r#powered: true,
                r#side_chain: SideChain::Unconnected,
            });
        }
        if state_id == 2799 {
            return Some(JungleShelf {
                r#side_chain: SideChain::Unconnected,
                r#facing: Facing::South,
                r#waterlogged: true,
                r#powered: true,
            });
        }
        if state_id == 2786 {
            return Some(JungleShelf {
                r#side_chain: SideChain::Right,
                r#waterlogged: false,
                r#facing: Facing::North,
                r#powered: true,
            });
        }
        if state_id == 2838 {
            return Some(JungleShelf {
                r#side_chain: SideChain::Left,
                r#powered: true,
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 2814 {
            return Some(JungleShelf {
                r#side_chain: SideChain::Left,
                r#powered: false,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 2824 {
            return Some(JungleShelf {
                r#side_chain: SideChain::Unconnected,
                r#powered: false,
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 2817 {
            return Some(JungleShelf {
                r#side_chain: SideChain::Right,
                r#facing: Facing::West,
                r#waterlogged: true,
                r#powered: true,
            });
        }
        if state_id == 2822 {
            return Some(JungleShelf {
                r#powered: true,
                r#side_chain: SideChain::Left,
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 2846 {
            return Some(JungleShelf {
                r#facing: Facing::East,
                r#side_chain: SideChain::Left,
                r#powered: false,
                r#waterlogged: false,
            });
        }
        if state_id == 2820 {
            return Some(JungleShelf {
                r#waterlogged: false,
                r#powered: true,
                r#facing: Facing::West,
                r#side_chain: SideChain::Center,
            });
        }
        if state_id == 2803 {
            return Some(JungleShelf {
                r#side_chain: SideChain::Center,
                r#powered: true,
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 2842 {
            return Some(JungleShelf {
                r#facing: Facing::East,
                r#waterlogged: false,
                r#powered: false,
                r#side_chain: SideChain::Right,
            });
        }
        if state_id == 2830 {
            return Some(JungleShelf {
                r#powered: false,
                r#side_chain: SideChain::Left,
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 2785 {
            return Some(JungleShelf {
                r#powered: true,
                r#side_chain: SideChain::Right,
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 2804 {
            return Some(JungleShelf {
                r#waterlogged: false,
                r#powered: true,
                r#side_chain: SideChain::Center,
                r#facing: Facing::South,
            });
        }
        if state_id == 2801 {
            return Some(JungleShelf {
                r#facing: Facing::South,
                r#waterlogged: true,
                r#side_chain: SideChain::Right,
                r#powered: true,
            });
        }
        if state_id == 2790 {
            return Some(JungleShelf {
                r#facing: Facing::North,
                r#side_chain: SideChain::Left,
                r#powered: true,
                r#waterlogged: false,
            });
        }
        if state_id == 2811 {
            return Some(JungleShelf {
                r#side_chain: SideChain::Center,
                r#powered: false,
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 2833 {
            return Some(JungleShelf {
                r#side_chain: SideChain::Right,
                r#powered: true,
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 2837 {
            return Some(JungleShelf {
                r#waterlogged: true,
                r#facing: Facing::East,
                r#side_chain: SideChain::Left,
                r#powered: true,
            });
        }
        if state_id == 2841 {
            return Some(JungleShelf {
                r#waterlogged: true,
                r#powered: false,
                r#facing: Facing::East,
                r#side_chain: SideChain::Right,
            });
        }
        if state_id == 2787 {
            return Some(JungleShelf {
                r#side_chain: SideChain::Center,
                r#waterlogged: true,
                r#powered: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 2809 {
            return Some(JungleShelf {
                r#facing: Facing::South,
                r#waterlogged: true,
                r#side_chain: SideChain::Right,
                r#powered: false,
            });
        }
        if state_id == 2783 {
            return Some(JungleShelf {
                r#facing: Facing::North,
                r#powered: true,
                r#side_chain: SideChain::Unconnected,
                r#waterlogged: true,
            });
        }
        if state_id == 2796 {
            return Some(JungleShelf {
                r#powered: false,
                r#side_chain: SideChain::Center,
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 2834 {
            return Some(JungleShelf {
                r#facing: Facing::East,
                r#powered: true,
                r#waterlogged: false,
                r#side_chain: SideChain::Right,
            });
        }
        if state_id == 2836 {
            return Some(JungleShelf {
                r#side_chain: SideChain::Center,
                r#waterlogged: false,
                r#facing: Facing::East,
                r#powered: true,
            });
        }
        if state_id == 2794 {
            return Some(JungleShelf {
                r#facing: Facing::North,
                r#side_chain: SideChain::Right,
                r#powered: false,
                r#waterlogged: false,
            });
        }
        if state_id == 2813 {
            return Some(JungleShelf {
                r#side_chain: SideChain::Left,
                r#facing: Facing::South,
                r#waterlogged: true,
                r#powered: false,
            });
        }
        if state_id == 2826 {
            return Some(JungleShelf {
                r#waterlogged: false,
                r#facing: Facing::West,
                r#powered: false,
                r#side_chain: SideChain::Right,
            });
        }
        if state_id == 2788 {
            return Some(JungleShelf {
                r#facing: Facing::North,
                r#powered: true,
                r#side_chain: SideChain::Center,
                r#waterlogged: false,
            });
        }
        if state_id == 2821 {
            return Some(JungleShelf {
                r#powered: true,
                r#side_chain: SideChain::Left,
                r#waterlogged: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 2823 {
            return Some(JungleShelf {
                r#side_chain: SideChain::Unconnected,
                r#facing: Facing::West,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 2802 {
            return Some(JungleShelf {
                r#side_chain: SideChain::Right,
                r#waterlogged: false,
                r#powered: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 2810 {
            return Some(JungleShelf {
                r#powered: false,
                r#side_chain: SideChain::Right,
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 2843 {
            return Some(JungleShelf {
                r#facing: Facing::East,
                r#side_chain: SideChain::Center,
                r#waterlogged: true,
                r#powered: false,
            });
        }
        if state_id == 2831 {
            return Some(JungleShelf {
                r#powered: true,
                r#facing: Facing::East,
                r#side_chain: SideChain::Unconnected,
                r#waterlogged: true,
            });
        }
        if state_id == 2793 {
            return Some(JungleShelf {
                r#powered: false,
                r#side_chain: SideChain::Right,
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 2792 {
            return Some(JungleShelf {
                r#waterlogged: false,
                r#powered: false,
                r#facing: Facing::North,
                r#side_chain: SideChain::Unconnected,
            });
        }
        if state_id == 2805 {
            return Some(JungleShelf {
                r#waterlogged: true,
                r#side_chain: SideChain::Left,
                r#powered: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 2835 {
            return Some(JungleShelf {
                r#side_chain: SideChain::Center,
                r#powered: true,
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 2815 {
            return Some(JungleShelf {
                r#facing: Facing::West,
                r#powered: true,
                r#waterlogged: true,
                r#side_chain: SideChain::Unconnected,
            });
        }
        if state_id == 2819 {
            return Some(JungleShelf {
                r#powered: true,
                r#facing: Facing::West,
                r#side_chain: SideChain::Center,
                r#waterlogged: true,
            });
        }
        if state_id == 2800 {
            return Some(JungleShelf {
                r#facing: Facing::South,
                r#waterlogged: false,
                r#side_chain: SideChain::Unconnected,
                r#powered: true,
            });
        }
        if state_id == 2797 {
            return Some(JungleShelf {
                r#powered: false,
                r#facing: Facing::North,
                r#side_chain: SideChain::Left,
                r#waterlogged: true,
            });
        }
        if state_id == 2807 {
            return Some(JungleShelf {
                r#side_chain: SideChain::Unconnected,
                r#waterlogged: true,
                r#facing: Facing::South,
                r#powered: false,
            });
        }
        if state_id == 2844 {
            return Some(JungleShelf {
                r#facing: Facing::East,
                r#waterlogged: false,
                r#powered: false,
                r#side_chain: SideChain::Center,
            });
        }
        return None;
    }
}
