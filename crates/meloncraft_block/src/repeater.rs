use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Repeater {
    pub delay: i32,
    pub r#facing: Facing,
    pub powered: bool,
    pub locked: bool,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for Repeater {
    fn to_id(self) -> i32 {
        if block_state.r#locked == true && block_state.r#delay == 1 && block_state.r#powered == false && block_state.r#facing == Facing::West { return 6842; }
        if block_state.r#powered == true && block_state.r#delay == 4 && block_state.r#facing == Facing::East && block_state.r#locked == false { return 6895; }
        if block_state.r#delay == 2 && block_state.r#facing == Facing::East && block_state.r#locked == false && block_state.r#powered == false { return 6864; }
        if block_state.r#locked == false && block_state.r#facing == Facing::East && block_state.r#delay == 1 && block_state.r#powered == false { return 6848; }
        if block_state.r#powered == false && block_state.r#locked == true && block_state.r#facing == Facing::West && block_state.r#delay == 4 { return 6890; }
        if block_state.r#facing == Facing::East && block_state.r#locked == true && block_state.r#delay == 4 && block_state.r#powered == false { return 6894; }
        if block_state.r#powered == false && block_state.r#delay == 4 && block_state.r#locked == false && block_state.r#facing == Facing::East { return 6896; }
        if block_state.r#facing == Facing::East && block_state.r#locked == true && block_state.r#powered == false && block_state.r#delay == 1 { return 6846; }
        if block_state.r#delay == 1 && block_state.r#powered == false && block_state.r#locked == false && block_state.r#facing == Facing::West { return 6844; }
        if block_state.r#powered == false && block_state.r#locked == true && block_state.r#facing == Facing::South && block_state.r#delay == 1 { return 6838; }
        if block_state.r#powered == true && block_state.r#delay == 3 && block_state.r#facing == Facing::North && block_state.r#locked == false { return 6867; }
        if block_state.r#facing == Facing::West && block_state.r#delay == 4 && block_state.r#locked == true && block_state.r#powered == true { return 6889; }
        if block_state.r#delay == 4 && block_state.r#powered == false && block_state.r#locked == false && block_state.r#facing == Facing::North { return 6884; }
        if block_state.r#locked == true && block_state.r#powered == true && block_state.r#delay == 2 && block_state.r#facing == Facing::West { return 6857; }
        if block_state.r#delay == 3 && block_state.r#powered == true && block_state.r#locked == true && block_state.r#facing == Facing::North { return 6865; }
        if block_state.r#locked == false && block_state.r#powered == true && block_state.r#facing == Facing::South && block_state.r#delay == 3 { return 6871; }
        if block_state.r#delay == 4 && block_state.r#locked == false && block_state.r#facing == Facing::West && block_state.r#powered == false { return 6892; }
        if block_state.r#powered == true && block_state.r#delay == 1 && block_state.r#facing == Facing::West && block_state.r#locked == true { return 6841; }
        if block_state.r#locked == false && block_state.r#facing == Facing::West && block_state.r#powered == true && block_state.r#delay == 3 { return 6875; }
        if block_state.r#locked == false && block_state.r#powered == true && block_state.r#facing == Facing::North && block_state.r#delay == 4 { return 6883; }
        if block_state.r#powered == true && block_state.r#locked == true && block_state.r#facing == Facing::South && block_state.r#delay == 4 { return 6885; }
        if block_state.r#locked == true && block_state.r#delay == 4 && block_state.r#powered == false && block_state.r#facing == Facing::South { return 6886; }
        if block_state.r#facing == Facing::South && block_state.r#powered == true && block_state.r#delay == 1 && block_state.r#locked == true { return 6837; }
        if block_state.r#powered == false && block_state.r#locked == false && block_state.r#delay == 3 && block_state.r#facing == Facing::North { return 6868; }
        if block_state.r#facing == Facing::East && block_state.r#powered == false && block_state.r#locked == false && block_state.r#delay == 3 { return 6880; }
        if block_state.r#locked == true && block_state.r#facing == Facing::South && block_state.r#powered == true && block_state.r#delay == 2 { return 6853; }
        if block_state.r#locked == false && block_state.r#powered == true && block_state.r#delay == 2 && block_state.r#facing == Facing::East { return 6863; }
        if block_state.r#facing == Facing::South && block_state.r#delay == 2 && block_state.r#locked == true && block_state.r#powered == false { return 6854; }
        if block_state.r#facing == Facing::East && block_state.r#delay == 1 && block_state.r#locked == true && block_state.r#powered == true { return 6845; }
        if block_state.r#delay == 2 && block_state.r#locked == true && block_state.r#facing == Facing::East && block_state.r#powered == true { return 6861; }
        if block_state.r#powered == false && block_state.r#delay == 2 && block_state.r#facing == Facing::South && block_state.r#locked == false { return 6856; }
        if block_state.r#delay == 3 && block_state.r#facing == Facing::South && block_state.r#locked == false && block_state.r#powered == false { return 6872; }
        if block_state.r#delay == 1 && block_state.r#facing == Facing::West && block_state.r#locked == false && block_state.r#powered == true { return 6843; }
        if block_state.r#powered == false && block_state.r#delay == 3 && block_state.r#locked == true && block_state.r#facing == Facing::North { return 6866; }
        if block_state.r#locked == true && block_state.r#delay == 3 && block_state.r#powered == false && block_state.r#facing == Facing::East { return 6878; }
        if block_state.r#facing == Facing::West && block_state.r#powered == true && block_state.r#delay == 4 && block_state.r#locked == false { return 6891; }
        if block_state.r#facing == Facing::North && block_state.r#locked == false && block_state.r#powered == true && block_state.r#delay == 2 { return 6851; }
        if block_state.r#powered == false && block_state.r#facing == Facing::West && block_state.r#delay == 3 && block_state.r#locked == true { return 6874; }
        if block_state.r#facing == Facing::North && block_state.r#locked == true && block_state.r#delay == 2 && block_state.r#powered == true { return 6849; }
        if block_state.r#facing == Facing::North && block_state.r#locked == true && block_state.r#powered == false && block_state.r#delay == 1 { return 6834; }
        if block_state.r#delay == 1 && block_state.r#powered == true && block_state.r#facing == Facing::North && block_state.r#locked == false { return 6835; }
        if block_state.r#powered == false && block_state.r#locked == true && block_state.r#delay == 2 && block_state.r#facing == Facing::West { return 6858; }
        if block_state.r#powered == false && block_state.r#facing == Facing::South && block_state.r#locked == true && block_state.r#delay == 3 { return 6870; }
        if block_state.r#locked == false && block_state.r#powered == true && block_state.r#delay == 3 && block_state.r#facing == Facing::East { return 6879; }
        if block_state.r#delay == 1 && block_state.r#powered == true && block_state.r#facing == Facing::South && block_state.r#locked == false { return 6839; }
        if block_state.r#powered == true && block_state.r#facing == Facing::East && block_state.r#locked == true && block_state.r#delay == 4 { return 6893; }
        if block_state.r#locked == true && block_state.r#facing == Facing::East && block_state.r#delay == 2 && block_state.r#powered == false { return 6862; }
        if block_state.r#locked == false && block_state.r#powered == true && block_state.r#delay == 4 && block_state.r#facing == Facing::South { return 6887; }
        if block_state.r#facing == Facing::South && block_state.r#powered == false && block_state.r#locked == false && block_state.r#delay == 1 { return 6840; }
        if block_state.r#delay == 2 && block_state.r#powered == false && block_state.r#locked == true && block_state.r#facing == Facing::North { return 6850; }
        if block_state.r#facing == Facing::North && block_state.r#locked == false && block_state.r#powered == false && block_state.r#delay == 2 { return 6852; }
        if block_state.r#facing == Facing::West && block_state.r#locked == false && block_state.r#delay == 3 && block_state.r#powered == false { return 6876; }
        if block_state.r#powered == true && block_state.r#locked == true && block_state.r#delay == 1 && block_state.r#facing == Facing::North { return 6833; }
        if block_state.r#locked == false && block_state.r#powered == true && block_state.r#facing == Facing::South && block_state.r#delay == 2 { return 6855; }
        if block_state.r#delay == 1 && block_state.r#powered == false && block_state.r#locked == false && block_state.r#facing == Facing::North { return 6836; }
        if block_state.r#facing == Facing::East && block_state.r#locked == false && block_state.r#powered == true && block_state.r#delay == 1 { return 6847; }
        if block_state.r#facing == Facing::East && block_state.r#locked == true && block_state.r#delay == 3 && block_state.r#powered == true { return 6877; }
        if block_state.r#powered == false && block_state.r#delay == 2 && block_state.r#locked == false && block_state.r#facing == Facing::West { return 6860; }
        if block_state.r#locked == true && block_state.r#powered == true && block_state.r#delay == 4 && block_state.r#facing == Facing::North { return 6881; }
        if block_state.r#locked == true && block_state.r#powered == true && block_state.r#delay == 3 && block_state.r#facing == Facing::South { return 6869; }
        if block_state.r#locked == true && block_state.r#delay == 3 && block_state.r#facing == Facing::West && block_state.r#powered == true { return 6873; }
        if block_state.r#delay == 4 && block_state.r#powered == false && block_state.r#locked == true && block_state.r#facing == Facing::North { return 6882; }
        if block_state.r#locked == false && block_state.r#powered == false && block_state.r#facing == Facing::South && block_state.r#delay == 4 { return 6888; }
        if block_state.r#facing == Facing::West && block_state.r#powered == true && block_state.r#delay == 2 && block_state.r#locked == false { return 6859; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 6842 {
            return Some(Repeater {
                r#locked: true,
                r#delay: 1,
                r#powered: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 6895 {
            return Some(Repeater {
                r#powered: true,
                r#delay: 4,
                r#facing: Facing::East,
                r#locked: false,
            });
        }
        if state_id == 6864 {
            return Some(Repeater {
                r#delay: 2,
                r#facing: Facing::East,
                r#locked: false,
                r#powered: false,
            });
        }
        if state_id == 6848 {
            return Some(Repeater {
                r#locked: false,
                r#facing: Facing::East,
                r#delay: 1,
                r#powered: false,
            });
        }
        if state_id == 6890 {
            return Some(Repeater {
                r#powered: false,
                r#locked: true,
                r#facing: Facing::West,
                r#delay: 4,
            });
        }
        if state_id == 6894 {
            return Some(Repeater {
                r#facing: Facing::East,
                r#locked: true,
                r#delay: 4,
                r#powered: false,
            });
        }
        if state_id == 6896 {
            return Some(Repeater {
                r#powered: false,
                r#delay: 4,
                r#locked: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 6846 {
            return Some(Repeater {
                r#facing: Facing::East,
                r#locked: true,
                r#powered: false,
                r#delay: 1,
            });
        }
        if state_id == 6844 {
            return Some(Repeater {
                r#delay: 1,
                r#powered: false,
                r#locked: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 6838 {
            return Some(Repeater {
                r#powered: false,
                r#locked: true,
                r#facing: Facing::South,
                r#delay: 1,
            });
        }
        if state_id == 6867 {
            return Some(Repeater {
                r#powered: true,
                r#delay: 3,
                r#facing: Facing::North,
                r#locked: false,
            });
        }
        if state_id == 6889 {
            return Some(Repeater {
                r#facing: Facing::West,
                r#delay: 4,
                r#locked: true,
                r#powered: true,
            });
        }
        if state_id == 6884 {
            return Some(Repeater {
                r#delay: 4,
                r#powered: false,
                r#locked: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 6857 {
            return Some(Repeater {
                r#locked: true,
                r#powered: true,
                r#delay: 2,
                r#facing: Facing::West,
            });
        }
        if state_id == 6865 {
            return Some(Repeater {
                r#delay: 3,
                r#powered: true,
                r#locked: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 6871 {
            return Some(Repeater {
                r#locked: false,
                r#powered: true,
                r#facing: Facing::South,
                r#delay: 3,
            });
        }
        if state_id == 6892 {
            return Some(Repeater {
                r#delay: 4,
                r#locked: false,
                r#facing: Facing::West,
                r#powered: false,
            });
        }
        if state_id == 6841 {
            return Some(Repeater {
                r#powered: true,
                r#delay: 1,
                r#facing: Facing::West,
                r#locked: true,
            });
        }
        if state_id == 6875 {
            return Some(Repeater {
                r#locked: false,
                r#facing: Facing::West,
                r#powered: true,
                r#delay: 3,
            });
        }
        if state_id == 6883 {
            return Some(Repeater {
                r#locked: false,
                r#powered: true,
                r#facing: Facing::North,
                r#delay: 4,
            });
        }
        if state_id == 6885 {
            return Some(Repeater {
                r#powered: true,
                r#locked: true,
                r#facing: Facing::South,
                r#delay: 4,
            });
        }
        if state_id == 6886 {
            return Some(Repeater {
                r#locked: true,
                r#delay: 4,
                r#powered: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 6837 {
            return Some(Repeater {
                r#facing: Facing::South,
                r#powered: true,
                r#delay: 1,
                r#locked: true,
            });
        }
        if state_id == 6868 {
            return Some(Repeater {
                r#powered: false,
                r#locked: false,
                r#delay: 3,
                r#facing: Facing::North,
            });
        }
        if state_id == 6880 {
            return Some(Repeater {
                r#facing: Facing::East,
                r#powered: false,
                r#locked: false,
                r#delay: 3,
            });
        }
        if state_id == 6853 {
            return Some(Repeater {
                r#locked: true,
                r#facing: Facing::South,
                r#powered: true,
                r#delay: 2,
            });
        }
        if state_id == 6863 {
            return Some(Repeater {
                r#locked: false,
                r#powered: true,
                r#delay: 2,
                r#facing: Facing::East,
            });
        }
        if state_id == 6854 {
            return Some(Repeater {
                r#facing: Facing::South,
                r#delay: 2,
                r#locked: true,
                r#powered: false,
            });
        }
        if state_id == 6845 {
            return Some(Repeater {
                r#facing: Facing::East,
                r#delay: 1,
                r#locked: true,
                r#powered: true,
            });
        }
        if state_id == 6861 {
            return Some(Repeater {
                r#delay: 2,
                r#locked: true,
                r#facing: Facing::East,
                r#powered: true,
            });
        }
        if state_id == 6856 {
            return Some(Repeater {
                r#powered: false,
                r#delay: 2,
                r#facing: Facing::South,
                r#locked: false,
            });
        }
        if state_id == 6872 {
            return Some(Repeater {
                r#delay: 3,
                r#facing: Facing::South,
                r#locked: false,
                r#powered: false,
            });
        }
        if state_id == 6843 {
            return Some(Repeater {
                r#delay: 1,
                r#facing: Facing::West,
                r#locked: false,
                r#powered: true,
            });
        }
        if state_id == 6866 {
            return Some(Repeater {
                r#powered: false,
                r#delay: 3,
                r#locked: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 6878 {
            return Some(Repeater {
                r#locked: true,
                r#delay: 3,
                r#powered: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 6891 {
            return Some(Repeater {
                r#facing: Facing::West,
                r#powered: true,
                r#delay: 4,
                r#locked: false,
            });
        }
        if state_id == 6851 {
            return Some(Repeater {
                r#facing: Facing::North,
                r#locked: false,
                r#powered: true,
                r#delay: 2,
            });
        }
        if state_id == 6874 {
            return Some(Repeater {
                r#powered: false,
                r#facing: Facing::West,
                r#delay: 3,
                r#locked: true,
            });
        }
        if state_id == 6849 {
            return Some(Repeater {
                r#facing: Facing::North,
                r#locked: true,
                r#delay: 2,
                r#powered: true,
            });
        }
        if state_id == 6834 {
            return Some(Repeater {
                r#facing: Facing::North,
                r#locked: true,
                r#powered: false,
                r#delay: 1,
            });
        }
        if state_id == 6835 {
            return Some(Repeater {
                r#delay: 1,
                r#powered: true,
                r#facing: Facing::North,
                r#locked: false,
            });
        }
        if state_id == 6858 {
            return Some(Repeater {
                r#powered: false,
                r#locked: true,
                r#delay: 2,
                r#facing: Facing::West,
            });
        }
        if state_id == 6870 {
            return Some(Repeater {
                r#powered: false,
                r#facing: Facing::South,
                r#locked: true,
                r#delay: 3,
            });
        }
        if state_id == 6879 {
            return Some(Repeater {
                r#locked: false,
                r#powered: true,
                r#delay: 3,
                r#facing: Facing::East,
            });
        }
        if state_id == 6839 {
            return Some(Repeater {
                r#delay: 1,
                r#powered: true,
                r#facing: Facing::South,
                r#locked: false,
            });
        }
        if state_id == 6893 {
            return Some(Repeater {
                r#powered: true,
                r#facing: Facing::East,
                r#locked: true,
                r#delay: 4,
            });
        }
        if state_id == 6862 {
            return Some(Repeater {
                r#locked: true,
                r#facing: Facing::East,
                r#delay: 2,
                r#powered: false,
            });
        }
        if state_id == 6887 {
            return Some(Repeater {
                r#locked: false,
                r#powered: true,
                r#delay: 4,
                r#facing: Facing::South,
            });
        }
        if state_id == 6840 {
            return Some(Repeater {
                r#facing: Facing::South,
                r#powered: false,
                r#locked: false,
                r#delay: 1,
            });
        }
        if state_id == 6850 {
            return Some(Repeater {
                r#delay: 2,
                r#powered: false,
                r#locked: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 6852 {
            return Some(Repeater {
                r#facing: Facing::North,
                r#locked: false,
                r#powered: false,
                r#delay: 2,
            });
        }
        if state_id == 6876 {
            return Some(Repeater {
                r#facing: Facing::West,
                r#locked: false,
                r#delay: 3,
                r#powered: false,
            });
        }
        if state_id == 6833 {
            return Some(Repeater {
                r#powered: true,
                r#locked: true,
                r#delay: 1,
                r#facing: Facing::North,
            });
        }
        if state_id == 6855 {
            return Some(Repeater {
                r#locked: false,
                r#powered: true,
                r#facing: Facing::South,
                r#delay: 2,
            });
        }
        if state_id == 6836 {
            return Some(Repeater {
                r#delay: 1,
                r#powered: false,
                r#locked: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 6847 {
            return Some(Repeater {
                r#facing: Facing::East,
                r#locked: false,
                r#powered: true,
                r#delay: 1,
            });
        }
        if state_id == 6877 {
            return Some(Repeater {
                r#facing: Facing::East,
                r#locked: true,
                r#delay: 3,
                r#powered: true,
            });
        }
        if state_id == 6860 {
            return Some(Repeater {
                r#powered: false,
                r#delay: 2,
                r#locked: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 6881 {
            return Some(Repeater {
                r#locked: true,
                r#powered: true,
                r#delay: 4,
                r#facing: Facing::North,
            });
        }
        if state_id == 6869 {
            return Some(Repeater {
                r#locked: true,
                r#powered: true,
                r#delay: 3,
                r#facing: Facing::South,
            });
        }
        if state_id == 6873 {
            return Some(Repeater {
                r#locked: true,
                r#delay: 3,
                r#facing: Facing::West,
                r#powered: true,
            });
        }
        if state_id == 6882 {
            return Some(Repeater {
                r#delay: 4,
                r#powered: false,
                r#locked: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 6888 {
            return Some(Repeater {
                r#locked: false,
                r#powered: false,
                r#facing: Facing::South,
                r#delay: 4,
            });
        }
        if state_id == 6859 {
            return Some(Repeater {
                r#facing: Facing::West,
                r#powered: true,
                r#delay: 2,
                r#locked: false,
            });
        }
        return None;
    }
}

