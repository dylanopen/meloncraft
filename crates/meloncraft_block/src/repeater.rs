use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Repeater {
    pub delay: i32,
    pub locked: bool,
    pub r#facing: Facing,
    pub powered: bool,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for Repeater {
    fn to_id(&self) -> i32 {
        if self.r#facing == Facing::West && self.r#locked == false && self.r#delay == 4 && self.r#powered == false { return 6892; }
        if self.r#delay == 3 && self.r#facing == Facing::North && self.r#powered == true && self.r#locked == false { return 6867; }
        if self.r#powered == true && self.r#locked == true && self.r#delay == 3 && self.r#facing == Facing::South { return 6869; }
        if self.r#locked == false && self.r#powered == false && self.r#facing == Facing::North && self.r#delay == 2 { return 6852; }
        if self.r#powered == true && self.r#locked == false && self.r#delay == 4 && self.r#facing == Facing::East { return 6895; }
        if self.r#facing == Facing::East && self.r#locked == false && self.r#powered == false && self.r#delay == 3 { return 6880; }
        if self.r#delay == 2 && self.r#powered == false && self.r#locked == true && self.r#facing == Facing::West { return 6858; }
        if self.r#delay == 2 && self.r#facing == Facing::North && self.r#locked == true && self.r#powered == false { return 6850; }
        if self.r#locked == false && self.r#powered == false && self.r#delay == 3 && self.r#facing == Facing::South { return 6872; }
        if self.r#delay == 4 && self.r#powered == false && self.r#locked == false && self.r#facing == Facing::North { return 6884; }
        if self.r#powered == false && self.r#delay == 4 && self.r#locked == true && self.r#facing == Facing::East { return 6894; }
        if self.r#facing == Facing::North && self.r#delay == 4 && self.r#powered == true && self.r#locked == true { return 6881; }
        if self.r#powered == false && self.r#delay == 3 && self.r#facing == Facing::South && self.r#locked == true { return 6870; }
        if self.r#powered == true && self.r#delay == 4 && self.r#facing == Facing::South && self.r#locked == true { return 6885; }
        if self.r#locked == true && self.r#delay == 2 && self.r#facing == Facing::East && self.r#powered == false { return 6862; }
        if self.r#locked == false && self.r#facing == Facing::East && self.r#delay == 2 && self.r#powered == false { return 6864; }
        if self.r#facing == Facing::West && self.r#powered == true && self.r#delay == 1 && self.r#locked == true { return 6841; }
        if self.r#facing == Facing::South && self.r#delay == 4 && self.r#locked == true && self.r#powered == false { return 6886; }
        if self.r#facing == Facing::South && self.r#locked == false && self.r#powered == false && self.r#delay == 2 { return 6856; }
        if self.r#locked == false && self.r#facing == Facing::North && self.r#powered == false && self.r#delay == 1 { return 6836; }
        if self.r#facing == Facing::West && self.r#delay == 2 && self.r#locked == true && self.r#powered == true { return 6857; }
        if self.r#facing == Facing::East && self.r#locked == true && self.r#powered == true && self.r#delay == 2 { return 6861; }
        if self.r#facing == Facing::West && self.r#powered == false && self.r#delay == 3 && self.r#locked == true { return 6874; }
        if self.r#delay == 4 && self.r#locked == false && self.r#facing == Facing::South && self.r#powered == false { return 6888; }
        if self.r#facing == Facing::West && self.r#powered == true && self.r#delay == 3 && self.r#locked == true { return 6873; }
        if self.r#delay == 2 && self.r#powered == true && self.r#facing == Facing::West && self.r#locked == false { return 6859; }
        if self.r#locked == true && self.r#facing == Facing::South && self.r#powered == true && self.r#delay == 1 { return 6837; }
        if self.r#facing == Facing::North && self.r#delay == 2 && self.r#locked == false && self.r#powered == true { return 6851; }
        if self.r#delay == 4 && self.r#powered == true && self.r#facing == Facing::West && self.r#locked == true { return 6889; }
        if self.r#locked == false && self.r#delay == 1 && self.r#facing == Facing::East && self.r#powered == false { return 6848; }
        if self.r#delay == 4 && self.r#facing == Facing::West && self.r#powered == true && self.r#locked == false { return 6891; }
        if self.r#facing == Facing::North && self.r#locked == true && self.r#delay == 3 && self.r#powered == false { return 6866; }
        if self.r#delay == 3 && self.r#facing == Facing::East && self.r#locked == false && self.r#powered == true { return 6879; }
        if self.r#delay == 4 && self.r#facing == Facing::East && self.r#powered == true && self.r#locked == true { return 6893; }
        if self.r#powered == true && self.r#delay == 3 && self.r#locked == true && self.r#facing == Facing::North { return 6865; }
        if self.r#delay == 2 && self.r#locked == false && self.r#powered == true && self.r#facing == Facing::East { return 6863; }
        if self.r#locked == false && self.r#powered == true && self.r#facing == Facing::West && self.r#delay == 3 { return 6875; }
        if self.r#locked == false && self.r#delay == 1 && self.r#powered == false && self.r#facing == Facing::South { return 6840; }
        if self.r#powered == true && self.r#locked == true && self.r#delay == 2 && self.r#facing == Facing::South { return 6853; }
        if self.r#locked == false && self.r#powered == true && self.r#delay == 1 && self.r#facing == Facing::North { return 6835; }
        if self.r#delay == 2 && self.r#locked == false && self.r#powered == false && self.r#facing == Facing::West { return 6860; }
        if self.r#facing == Facing::South && self.r#powered == true && self.r#delay == 4 && self.r#locked == false { return 6887; }
        if self.r#facing == Facing::East && self.r#powered == true && self.r#delay == 1 && self.r#locked == false { return 6847; }
        if self.r#powered == false && self.r#facing == Facing::West && self.r#locked == true && self.r#delay == 4 { return 6890; }
        if self.r#delay == 3 && self.r#facing == Facing::East && self.r#powered == false && self.r#locked == true { return 6878; }
        if self.r#locked == true && self.r#delay == 2 && self.r#facing == Facing::North && self.r#powered == true { return 6849; }
        if self.r#locked == false && self.r#delay == 3 && self.r#powered == false && self.r#facing == Facing::North { return 6868; }
        if self.r#locked == false && self.r#delay == 3 && self.r#facing == Facing::South && self.r#powered == true { return 6871; }
        if self.r#powered == true && self.r#facing == Facing::South && self.r#locked == false && self.r#delay == 1 { return 6839; }
        if self.r#facing == Facing::South && self.r#powered == true && self.r#delay == 2 && self.r#locked == false { return 6855; }
        if self.r#locked == true && self.r#delay == 4 && self.r#powered == false && self.r#facing == Facing::North { return 6882; }
        if self.r#powered == false && self.r#locked == false && self.r#delay == 3 && self.r#facing == Facing::West { return 6876; }
        if self.r#delay == 2 && self.r#facing == Facing::South && self.r#powered == false && self.r#locked == true { return 6854; }
        if self.r#locked == false && self.r#facing == Facing::North && self.r#delay == 4 && self.r#powered == true { return 6883; }
        if self.r#facing == Facing::East && self.r#delay == 4 && self.r#locked == false && self.r#powered == false { return 6896; }
        if self.r#locked == true && self.r#powered == false && self.r#delay == 1 && self.r#facing == Facing::South { return 6838; }
        if self.r#delay == 1 && self.r#locked == true && self.r#facing == Facing::East && self.r#powered == true { return 6845; }
        if self.r#delay == 1 && self.r#facing == Facing::North && self.r#powered == true && self.r#locked == true { return 6833; }
        if self.r#locked == true && self.r#powered == false && self.r#delay == 1 && self.r#facing == Facing::North { return 6834; }
        if self.r#delay == 1 && self.r#facing == Facing::West && self.r#locked == false && self.r#powered == false { return 6844; }
        if self.r#facing == Facing::West && self.r#powered == true && self.r#delay == 1 && self.r#locked == false { return 6843; }
        if self.r#delay == 1 && self.r#locked == true && self.r#facing == Facing::West && self.r#powered == false { return 6842; }
        if self.r#delay == 1 && self.r#locked == true && self.r#facing == Facing::East && self.r#powered == false { return 6846; }
        if self.r#delay == 3 && self.r#facing == Facing::East && self.r#powered == true && self.r#locked == true { return 6877; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 6892 {
            return Some(Repeater {
                r#facing: Facing::West,
                r#locked: false,
                r#delay: 4,
                r#powered: false,
            });
        }
        if state_id == 6867 {
            return Some(Repeater {
                r#delay: 3,
                r#facing: Facing::North,
                r#powered: true,
                r#locked: false,
            });
        }
        if state_id == 6869 {
            return Some(Repeater {
                r#powered: true,
                r#locked: true,
                r#delay: 3,
                r#facing: Facing::South,
            });
        }
        if state_id == 6852 {
            return Some(Repeater {
                r#locked: false,
                r#powered: false,
                r#facing: Facing::North,
                r#delay: 2,
            });
        }
        if state_id == 6895 {
            return Some(Repeater {
                r#powered: true,
                r#locked: false,
                r#delay: 4,
                r#facing: Facing::East,
            });
        }
        if state_id == 6880 {
            return Some(Repeater {
                r#facing: Facing::East,
                r#locked: false,
                r#powered: false,
                r#delay: 3,
            });
        }
        if state_id == 6858 {
            return Some(Repeater {
                r#delay: 2,
                r#powered: false,
                r#locked: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 6850 {
            return Some(Repeater {
                r#delay: 2,
                r#facing: Facing::North,
                r#locked: true,
                r#powered: false,
            });
        }
        if state_id == 6872 {
            return Some(Repeater {
                r#locked: false,
                r#powered: false,
                r#delay: 3,
                r#facing: Facing::South,
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
        if state_id == 6894 {
            return Some(Repeater {
                r#powered: false,
                r#delay: 4,
                r#locked: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 6881 {
            return Some(Repeater {
                r#facing: Facing::North,
                r#delay: 4,
                r#powered: true,
                r#locked: true,
            });
        }
        if state_id == 6870 {
            return Some(Repeater {
                r#powered: false,
                r#delay: 3,
                r#facing: Facing::South,
                r#locked: true,
            });
        }
        if state_id == 6885 {
            return Some(Repeater {
                r#powered: true,
                r#delay: 4,
                r#facing: Facing::South,
                r#locked: true,
            });
        }
        if state_id == 6862 {
            return Some(Repeater {
                r#locked: true,
                r#delay: 2,
                r#facing: Facing::East,
                r#powered: false,
            });
        }
        if state_id == 6864 {
            return Some(Repeater {
                r#locked: false,
                r#facing: Facing::East,
                r#delay: 2,
                r#powered: false,
            });
        }
        if state_id == 6841 {
            return Some(Repeater {
                r#facing: Facing::West,
                r#powered: true,
                r#delay: 1,
                r#locked: true,
            });
        }
        if state_id == 6886 {
            return Some(Repeater {
                r#facing: Facing::South,
                r#delay: 4,
                r#locked: true,
                r#powered: false,
            });
        }
        if state_id == 6856 {
            return Some(Repeater {
                r#facing: Facing::South,
                r#locked: false,
                r#powered: false,
                r#delay: 2,
            });
        }
        if state_id == 6836 {
            return Some(Repeater {
                r#locked: false,
                r#facing: Facing::North,
                r#powered: false,
                r#delay: 1,
            });
        }
        if state_id == 6857 {
            return Some(Repeater {
                r#facing: Facing::West,
                r#delay: 2,
                r#locked: true,
                r#powered: true,
            });
        }
        if state_id == 6861 {
            return Some(Repeater {
                r#facing: Facing::East,
                r#locked: true,
                r#powered: true,
                r#delay: 2,
            });
        }
        if state_id == 6874 {
            return Some(Repeater {
                r#facing: Facing::West,
                r#powered: false,
                r#delay: 3,
                r#locked: true,
            });
        }
        if state_id == 6888 {
            return Some(Repeater {
                r#delay: 4,
                r#locked: false,
                r#facing: Facing::South,
                r#powered: false,
            });
        }
        if state_id == 6873 {
            return Some(Repeater {
                r#facing: Facing::West,
                r#powered: true,
                r#delay: 3,
                r#locked: true,
            });
        }
        if state_id == 6859 {
            return Some(Repeater {
                r#delay: 2,
                r#powered: true,
                r#facing: Facing::West,
                r#locked: false,
            });
        }
        if state_id == 6837 {
            return Some(Repeater {
                r#locked: true,
                r#facing: Facing::South,
                r#powered: true,
                r#delay: 1,
            });
        }
        if state_id == 6851 {
            return Some(Repeater {
                r#facing: Facing::North,
                r#delay: 2,
                r#locked: false,
                r#powered: true,
            });
        }
        if state_id == 6889 {
            return Some(Repeater {
                r#delay: 4,
                r#powered: true,
                r#facing: Facing::West,
                r#locked: true,
            });
        }
        if state_id == 6848 {
            return Some(Repeater {
                r#locked: false,
                r#delay: 1,
                r#facing: Facing::East,
                r#powered: false,
            });
        }
        if state_id == 6891 {
            return Some(Repeater {
                r#delay: 4,
                r#facing: Facing::West,
                r#powered: true,
                r#locked: false,
            });
        }
        if state_id == 6866 {
            return Some(Repeater {
                r#facing: Facing::North,
                r#locked: true,
                r#delay: 3,
                r#powered: false,
            });
        }
        if state_id == 6879 {
            return Some(Repeater {
                r#delay: 3,
                r#facing: Facing::East,
                r#locked: false,
                r#powered: true,
            });
        }
        if state_id == 6893 {
            return Some(Repeater {
                r#delay: 4,
                r#facing: Facing::East,
                r#powered: true,
                r#locked: true,
            });
        }
        if state_id == 6865 {
            return Some(Repeater {
                r#powered: true,
                r#delay: 3,
                r#locked: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 6863 {
            return Some(Repeater {
                r#delay: 2,
                r#locked: false,
                r#powered: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 6875 {
            return Some(Repeater {
                r#locked: false,
                r#powered: true,
                r#facing: Facing::West,
                r#delay: 3,
            });
        }
        if state_id == 6840 {
            return Some(Repeater {
                r#locked: false,
                r#delay: 1,
                r#powered: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 6853 {
            return Some(Repeater {
                r#powered: true,
                r#locked: true,
                r#delay: 2,
                r#facing: Facing::South,
            });
        }
        if state_id == 6835 {
            return Some(Repeater {
                r#locked: false,
                r#powered: true,
                r#delay: 1,
                r#facing: Facing::North,
            });
        }
        if state_id == 6860 {
            return Some(Repeater {
                r#delay: 2,
                r#locked: false,
                r#powered: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 6887 {
            return Some(Repeater {
                r#facing: Facing::South,
                r#powered: true,
                r#delay: 4,
                r#locked: false,
            });
        }
        if state_id == 6847 {
            return Some(Repeater {
                r#facing: Facing::East,
                r#powered: true,
                r#delay: 1,
                r#locked: false,
            });
        }
        if state_id == 6890 {
            return Some(Repeater {
                r#powered: false,
                r#facing: Facing::West,
                r#locked: true,
                r#delay: 4,
            });
        }
        if state_id == 6878 {
            return Some(Repeater {
                r#delay: 3,
                r#facing: Facing::East,
                r#powered: false,
                r#locked: true,
            });
        }
        if state_id == 6849 {
            return Some(Repeater {
                r#locked: true,
                r#delay: 2,
                r#facing: Facing::North,
                r#powered: true,
            });
        }
        if state_id == 6868 {
            return Some(Repeater {
                r#locked: false,
                r#delay: 3,
                r#powered: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 6871 {
            return Some(Repeater {
                r#locked: false,
                r#delay: 3,
                r#facing: Facing::South,
                r#powered: true,
            });
        }
        if state_id == 6839 {
            return Some(Repeater {
                r#powered: true,
                r#facing: Facing::South,
                r#locked: false,
                r#delay: 1,
            });
        }
        if state_id == 6855 {
            return Some(Repeater {
                r#facing: Facing::South,
                r#powered: true,
                r#delay: 2,
                r#locked: false,
            });
        }
        if state_id == 6882 {
            return Some(Repeater {
                r#locked: true,
                r#delay: 4,
                r#powered: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 6876 {
            return Some(Repeater {
                r#powered: false,
                r#locked: false,
                r#delay: 3,
                r#facing: Facing::West,
            });
        }
        if state_id == 6854 {
            return Some(Repeater {
                r#delay: 2,
                r#facing: Facing::South,
                r#powered: false,
                r#locked: true,
            });
        }
        if state_id == 6883 {
            return Some(Repeater {
                r#locked: false,
                r#facing: Facing::North,
                r#delay: 4,
                r#powered: true,
            });
        }
        if state_id == 6896 {
            return Some(Repeater {
                r#facing: Facing::East,
                r#delay: 4,
                r#locked: false,
                r#powered: false,
            });
        }
        if state_id == 6838 {
            return Some(Repeater {
                r#locked: true,
                r#powered: false,
                r#delay: 1,
                r#facing: Facing::South,
            });
        }
        if state_id == 6845 {
            return Some(Repeater {
                r#delay: 1,
                r#locked: true,
                r#facing: Facing::East,
                r#powered: true,
            });
        }
        if state_id == 6833 {
            return Some(Repeater {
                r#delay: 1,
                r#facing: Facing::North,
                r#powered: true,
                r#locked: true,
            });
        }
        if state_id == 6834 {
            return Some(Repeater {
                r#locked: true,
                r#powered: false,
                r#delay: 1,
                r#facing: Facing::North,
            });
        }
        if state_id == 6844 {
            return Some(Repeater {
                r#delay: 1,
                r#facing: Facing::West,
                r#locked: false,
                r#powered: false,
            });
        }
        if state_id == 6843 {
            return Some(Repeater {
                r#facing: Facing::West,
                r#powered: true,
                r#delay: 1,
                r#locked: false,
            });
        }
        if state_id == 6842 {
            return Some(Repeater {
                r#delay: 1,
                r#locked: true,
                r#facing: Facing::West,
                r#powered: false,
            });
        }
        if state_id == 6846 {
            return Some(Repeater {
                r#delay: 1,
                r#locked: true,
                r#facing: Facing::East,
                r#powered: false,
            });
        }
        if state_id == 6877 {
            return Some(Repeater {
                r#delay: 3,
                r#facing: Facing::East,
                r#powered: true,
                r#locked: true,
            });
        }
        return None;
    }
}

