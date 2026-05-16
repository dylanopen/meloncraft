use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlayerHead {
    pub rotation: i32,
    pub powered: bool,
}


impl BlockState for PlayerHead {
    fn to_id(&self) -> i32 {
        if self.r#rotation == 7 && self.r#powered == true { return 10840; }
        if self.r#powered == false && self.r#rotation == 3 { return 10852; }
        if self.r#rotation == 12 && self.r#powered == true { return 10845; }
        if self.r#powered == false && self.r#rotation == 1 { return 10850; }
        if self.r#rotation == 15 && self.r#powered == true { return 10848; }
        if self.r#powered == false && self.r#rotation == 13 { return 10862; }
        if self.r#powered == false && self.r#rotation == 14 { return 10863; }
        if self.r#rotation == 5 && self.r#powered == false { return 10854; }
        if self.r#powered == false && self.r#rotation == 6 { return 10855; }
        if self.r#powered == false && self.r#rotation == 11 { return 10860; }
        if self.r#rotation == 3 && self.r#powered == true { return 10836; }
        if self.r#powered == true && self.r#rotation == 14 { return 10847; }
        if self.r#rotation == 9 && self.r#powered == false { return 10858; }
        if self.r#rotation == 2 && self.r#powered == true { return 10835; }
        if self.r#rotation == 2 && self.r#powered == false { return 10851; }
        if self.r#powered == true && self.r#rotation == 8 { return 10841; }
        if self.r#powered == true && self.r#rotation == 5 { return 10838; }
        if self.r#rotation == 6 && self.r#powered == true { return 10839; }
        if self.r#powered == false && self.r#rotation == 12 { return 10861; }
        if self.r#powered == true && self.r#rotation == 9 { return 10842; }
        if self.r#powered == false && self.r#rotation == 4 { return 10853; }
        if self.r#rotation == 13 && self.r#powered == true { return 10846; }
        if self.r#powered == false && self.r#rotation == 8 { return 10857; }
        if self.r#rotation == 10 && self.r#powered == false { return 10859; }
        if self.r#rotation == 7 && self.r#powered == false { return 10856; }
        if self.r#rotation == 11 && self.r#powered == true { return 10844; }
        if self.r#powered == true && self.r#rotation == 10 { return 10843; }
        if self.r#powered == false && self.r#rotation == 0 { return 10849; }
        if self.r#rotation == 15 && self.r#powered == false { return 10864; }
        if self.r#rotation == 0 && self.r#powered == true { return 10833; }
        if self.r#powered == true && self.r#rotation == 1 { return 10834; }
        if self.r#powered == true && self.r#rotation == 4 { return 10837; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 10840 {
            return Some(PlayerHead {
                r#rotation: 7,
                r#powered: true,
            });
        }
        if state_id == 10852 {
            return Some(PlayerHead {
                r#powered: false,
                r#rotation: 3,
            });
        }
        if state_id == 10845 {
            return Some(PlayerHead {
                r#rotation: 12,
                r#powered: true,
            });
        }
        if state_id == 10850 {
            return Some(PlayerHead {
                r#powered: false,
                r#rotation: 1,
            });
        }
        if state_id == 10848 {
            return Some(PlayerHead {
                r#rotation: 15,
                r#powered: true,
            });
        }
        if state_id == 10862 {
            return Some(PlayerHead {
                r#powered: false,
                r#rotation: 13,
            });
        }
        if state_id == 10863 {
            return Some(PlayerHead {
                r#powered: false,
                r#rotation: 14,
            });
        }
        if state_id == 10854 {
            return Some(PlayerHead {
                r#rotation: 5,
                r#powered: false,
            });
        }
        if state_id == 10855 {
            return Some(PlayerHead {
                r#powered: false,
                r#rotation: 6,
            });
        }
        if state_id == 10860 {
            return Some(PlayerHead {
                r#powered: false,
                r#rotation: 11,
            });
        }
        if state_id == 10836 {
            return Some(PlayerHead {
                r#rotation: 3,
                r#powered: true,
            });
        }
        if state_id == 10847 {
            return Some(PlayerHead {
                r#powered: true,
                r#rotation: 14,
            });
        }
        if state_id == 10858 {
            return Some(PlayerHead {
                r#rotation: 9,
                r#powered: false,
            });
        }
        if state_id == 10835 {
            return Some(PlayerHead {
                r#rotation: 2,
                r#powered: true,
            });
        }
        if state_id == 10851 {
            return Some(PlayerHead {
                r#rotation: 2,
                r#powered: false,
            });
        }
        if state_id == 10841 {
            return Some(PlayerHead {
                r#powered: true,
                r#rotation: 8,
            });
        }
        if state_id == 10838 {
            return Some(PlayerHead {
                r#powered: true,
                r#rotation: 5,
            });
        }
        if state_id == 10839 {
            return Some(PlayerHead {
                r#rotation: 6,
                r#powered: true,
            });
        }
        if state_id == 10861 {
            return Some(PlayerHead {
                r#powered: false,
                r#rotation: 12,
            });
        }
        if state_id == 10842 {
            return Some(PlayerHead {
                r#powered: true,
                r#rotation: 9,
            });
        }
        if state_id == 10853 {
            return Some(PlayerHead {
                r#powered: false,
                r#rotation: 4,
            });
        }
        if state_id == 10846 {
            return Some(PlayerHead {
                r#rotation: 13,
                r#powered: true,
            });
        }
        if state_id == 10857 {
            return Some(PlayerHead {
                r#powered: false,
                r#rotation: 8,
            });
        }
        if state_id == 10859 {
            return Some(PlayerHead {
                r#rotation: 10,
                r#powered: false,
            });
        }
        if state_id == 10856 {
            return Some(PlayerHead {
                r#rotation: 7,
                r#powered: false,
            });
        }
        if state_id == 10844 {
            return Some(PlayerHead {
                r#rotation: 11,
                r#powered: true,
            });
        }
        if state_id == 10843 {
            return Some(PlayerHead {
                r#powered: true,
                r#rotation: 10,
            });
        }
        if state_id == 10849 {
            return Some(PlayerHead {
                r#powered: false,
                r#rotation: 0,
            });
        }
        if state_id == 10864 {
            return Some(PlayerHead {
                r#rotation: 15,
                r#powered: false,
            });
        }
        if state_id == 10833 {
            return Some(PlayerHead {
                r#rotation: 0,
                r#powered: true,
            });
        }
        if state_id == 10834 {
            return Some(PlayerHead {
                r#powered: true,
                r#rotation: 1,
            });
        }
        if state_id == 10837 {
            return Some(PlayerHead {
                r#powered: true,
                r#rotation: 4,
            });
        }
        return None;
    }
}

