use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DragonHead {
    pub powered: bool,
    pub rotation: i32,
}


impl BlockState for DragonHead {
    fn to_id(self) -> i32 {
        if block_state.r#powered == true && block_state.r#rotation == 15 { return 10928; }
        if block_state.r#rotation == 11 && block_state.r#powered == true { return 10924; }
        if block_state.r#rotation == 0 && block_state.r#powered == false { return 10929; }
        if block_state.r#powered == true && block_state.r#rotation == 7 { return 10920; }
        if block_state.r#powered == true && block_state.r#rotation == 13 { return 10926; }
        if block_state.r#powered == false && block_state.r#rotation == 9 { return 10938; }
        if block_state.r#rotation == 10 && block_state.r#powered == false { return 10939; }
        if block_state.r#rotation == 9 && block_state.r#powered == true { return 10922; }
        if block_state.r#rotation == 4 && block_state.r#powered == false { return 10933; }
        if block_state.r#powered == true && block_state.r#rotation == 12 { return 10925; }
        if block_state.r#powered == true && block_state.r#rotation == 6 { return 10919; }
        if block_state.r#powered == false && block_state.r#rotation == 13 { return 10942; }
        if block_state.r#rotation == 0 && block_state.r#powered == true { return 10913; }
        if block_state.r#powered == true && block_state.r#rotation == 8 { return 10921; }
        if block_state.r#rotation == 15 && block_state.r#powered == false { return 10944; }
        if block_state.r#rotation == 12 && block_state.r#powered == false { return 10941; }
        if block_state.r#rotation == 14 && block_state.r#powered == false { return 10943; }
        if block_state.r#powered == false && block_state.r#rotation == 8 { return 10937; }
        if block_state.r#rotation == 4 && block_state.r#powered == true { return 10917; }
        if block_state.r#rotation == 6 && block_state.r#powered == false { return 10935; }
        if block_state.r#powered == true && block_state.r#rotation == 5 { return 10918; }
        if block_state.r#powered == false && block_state.r#rotation == 2 { return 10931; }
        if block_state.r#powered == false && block_state.r#rotation == 5 { return 10934; }
        if block_state.r#rotation == 11 && block_state.r#powered == false { return 10940; }
        if block_state.r#rotation == 3 && block_state.r#powered == true { return 10916; }
        if block_state.r#powered == true && block_state.r#rotation == 14 { return 10927; }
        if block_state.r#powered == false && block_state.r#rotation == 1 { return 10930; }
        if block_state.r#powered == true && block_state.r#rotation == 2 { return 10915; }
        if block_state.r#powered == false && block_state.r#rotation == 7 { return 10936; }
        if block_state.r#rotation == 3 && block_state.r#powered == false { return 10932; }
        if block_state.r#rotation == 10 && block_state.r#powered == true { return 10923; }
        if block_state.r#powered == true && block_state.r#rotation == 1 { return 10914; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 10928 {
            return Some(DragonHead {
                r#powered: true,
                r#rotation: 15,
            });
        }
        if state_id == 10924 {
            return Some(DragonHead {
                r#rotation: 11,
                r#powered: true,
            });
        }
        if state_id == 10929 {
            return Some(DragonHead {
                r#rotation: 0,
                r#powered: false,
            });
        }
        if state_id == 10920 {
            return Some(DragonHead {
                r#powered: true,
                r#rotation: 7,
            });
        }
        if state_id == 10926 {
            return Some(DragonHead {
                r#powered: true,
                r#rotation: 13,
            });
        }
        if state_id == 10938 {
            return Some(DragonHead {
                r#powered: false,
                r#rotation: 9,
            });
        }
        if state_id == 10939 {
            return Some(DragonHead {
                r#rotation: 10,
                r#powered: false,
            });
        }
        if state_id == 10922 {
            return Some(DragonHead {
                r#rotation: 9,
                r#powered: true,
            });
        }
        if state_id == 10933 {
            return Some(DragonHead {
                r#rotation: 4,
                r#powered: false,
            });
        }
        if state_id == 10925 {
            return Some(DragonHead {
                r#powered: true,
                r#rotation: 12,
            });
        }
        if state_id == 10919 {
            return Some(DragonHead {
                r#powered: true,
                r#rotation: 6,
            });
        }
        if state_id == 10942 {
            return Some(DragonHead {
                r#powered: false,
                r#rotation: 13,
            });
        }
        if state_id == 10913 {
            return Some(DragonHead {
                r#rotation: 0,
                r#powered: true,
            });
        }
        if state_id == 10921 {
            return Some(DragonHead {
                r#powered: true,
                r#rotation: 8,
            });
        }
        if state_id == 10944 {
            return Some(DragonHead {
                r#rotation: 15,
                r#powered: false,
            });
        }
        if state_id == 10941 {
            return Some(DragonHead {
                r#rotation: 12,
                r#powered: false,
            });
        }
        if state_id == 10943 {
            return Some(DragonHead {
                r#rotation: 14,
                r#powered: false,
            });
        }
        if state_id == 10937 {
            return Some(DragonHead {
                r#powered: false,
                r#rotation: 8,
            });
        }
        if state_id == 10917 {
            return Some(DragonHead {
                r#rotation: 4,
                r#powered: true,
            });
        }
        if state_id == 10935 {
            return Some(DragonHead {
                r#rotation: 6,
                r#powered: false,
            });
        }
        if state_id == 10918 {
            return Some(DragonHead {
                r#powered: true,
                r#rotation: 5,
            });
        }
        if state_id == 10931 {
            return Some(DragonHead {
                r#powered: false,
                r#rotation: 2,
            });
        }
        if state_id == 10934 {
            return Some(DragonHead {
                r#powered: false,
                r#rotation: 5,
            });
        }
        if state_id == 10940 {
            return Some(DragonHead {
                r#rotation: 11,
                r#powered: false,
            });
        }
        if state_id == 10916 {
            return Some(DragonHead {
                r#rotation: 3,
                r#powered: true,
            });
        }
        if state_id == 10927 {
            return Some(DragonHead {
                r#powered: true,
                r#rotation: 14,
            });
        }
        if state_id == 10930 {
            return Some(DragonHead {
                r#powered: false,
                r#rotation: 1,
            });
        }
        if state_id == 10915 {
            return Some(DragonHead {
                r#powered: true,
                r#rotation: 2,
            });
        }
        if state_id == 10936 {
            return Some(DragonHead {
                r#powered: false,
                r#rotation: 7,
            });
        }
        if state_id == 10932 {
            return Some(DragonHead {
                r#rotation: 3,
                r#powered: false,
            });
        }
        if state_id == 10923 {
            return Some(DragonHead {
                r#rotation: 10,
                r#powered: true,
            });
        }
        if state_id == 10914 {
            return Some(DragonHead {
                r#powered: true,
                r#rotation: 1,
            });
        }
        return None;
    }
}

