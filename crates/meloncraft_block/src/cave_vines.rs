use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CaveVines {
    pub berries: bool,
    pub age: i32,
}


impl BlockState for CaveVines {
    fn to_id(self) -> i32 {
        if block_state.r#age == 15 && block_state.r#berries == false { return 27585; }
        if block_state.r#age == 3 && block_state.r#berries == true { return 27560; }
        if block_state.r#age == 2 && block_state.r#berries == true { return 27558; }
        if block_state.r#berries == false && block_state.r#age == 5 { return 27565; }
        if block_state.r#age == 11 && block_state.r#berries == true { return 27576; }
        if block_state.r#age == 1 && block_state.r#berries == false { return 27557; }
        if block_state.r#age == 2 && block_state.r#berries == false { return 27559; }
        if block_state.r#berries == true && block_state.r#age == 14 { return 27582; }
        if block_state.r#berries == true && block_state.r#age == 12 { return 27578; }
        if block_state.r#berries == false && block_state.r#age == 12 { return 27579; }
        if block_state.r#age == 17 && block_state.r#berries == true { return 27588; }
        if block_state.r#age == 19 && block_state.r#berries == false { return 27593; }
        if block_state.r#age == 4 && block_state.r#berries == false { return 27563; }
        if block_state.r#berries == false && block_state.r#age == 20 { return 27595; }
        if block_state.r#berries == true && block_state.r#age == 23 { return 27600; }
        if block_state.r#age == 16 && block_state.r#berries == true { return 27586; }
        if block_state.r#age == 10 && block_state.r#berries == true { return 27574; }
        if block_state.r#age == 23 && block_state.r#berries == false { return 27601; }
        if block_state.r#berries == false && block_state.r#age == 8 { return 27571; }
        if block_state.r#berries == false && block_state.r#age == 25 { return 27605; }
        if block_state.r#age == 22 && block_state.r#berries == true { return 27598; }
        if block_state.r#berries == false && block_state.r#age == 16 { return 27587; }
        if block_state.r#age == 18 && block_state.r#berries == false { return 27591; }
        if block_state.r#age == 9 && block_state.r#berries == false { return 27573; }
        if block_state.r#berries == false && block_state.r#age == 3 { return 27561; }
        if block_state.r#age == 7 && block_state.r#berries == true { return 27568; }
        if block_state.r#age == 6 && block_state.r#berries == true { return 27566; }
        if block_state.r#age == 13 && block_state.r#berries == false { return 27581; }
        if block_state.r#berries == true && block_state.r#age == 8 { return 27570; }
        if block_state.r#berries == false && block_state.r#age == 24 { return 27603; }
        if block_state.r#berries == false && block_state.r#age == 17 { return 27589; }
        if block_state.r#age == 19 && block_state.r#berries == true { return 27592; }
        if block_state.r#berries == false && block_state.r#age == 7 { return 27569; }
        if block_state.r#age == 4 && block_state.r#berries == true { return 27562; }
        if block_state.r#berries == true && block_state.r#age == 18 { return 27590; }
        if block_state.r#age == 0 && block_state.r#berries == false { return 27555; }
        if block_state.r#berries == true && block_state.r#age == 21 { return 27596; }
        if block_state.r#age == 24 && block_state.r#berries == true { return 27602; }
        if block_state.r#berries == true && block_state.r#age == 20 { return 27594; }
        if block_state.r#berries == true && block_state.r#age == 0 { return 27554; }
        if block_state.r#berries == false && block_state.r#age == 14 { return 27583; }
        if block_state.r#age == 15 && block_state.r#berries == true { return 27584; }
        if block_state.r#age == 5 && block_state.r#berries == true { return 27564; }
        if block_state.r#age == 22 && block_state.r#berries == false { return 27599; }
        if block_state.r#age == 25 && block_state.r#berries == true { return 27604; }
        if block_state.r#berries == true && block_state.r#age == 13 { return 27580; }
        if block_state.r#berries == true && block_state.r#age == 9 { return 27572; }
        if block_state.r#age == 6 && block_state.r#berries == false { return 27567; }
        if block_state.r#age == 10 && block_state.r#berries == false { return 27575; }
        if block_state.r#berries == false && block_state.r#age == 11 { return 27577; }
        if block_state.r#age == 1 && block_state.r#berries == true { return 27556; }
        if block_state.r#age == 21 && block_state.r#berries == false { return 27597; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 27585 {
            return Some(CaveVines {
                r#age: 15,
                r#berries: false,
            });
        }
        if state_id == 27560 {
            return Some(CaveVines {
                r#age: 3,
                r#berries: true,
            });
        }
        if state_id == 27558 {
            return Some(CaveVines {
                r#age: 2,
                r#berries: true,
            });
        }
        if state_id == 27565 {
            return Some(CaveVines {
                r#berries: false,
                r#age: 5,
            });
        }
        if state_id == 27576 {
            return Some(CaveVines {
                r#age: 11,
                r#berries: true,
            });
        }
        if state_id == 27557 {
            return Some(CaveVines {
                r#age: 1,
                r#berries: false,
            });
        }
        if state_id == 27559 {
            return Some(CaveVines {
                r#age: 2,
                r#berries: false,
            });
        }
        if state_id == 27582 {
            return Some(CaveVines {
                r#berries: true,
                r#age: 14,
            });
        }
        if state_id == 27578 {
            return Some(CaveVines {
                r#berries: true,
                r#age: 12,
            });
        }
        if state_id == 27579 {
            return Some(CaveVines {
                r#berries: false,
                r#age: 12,
            });
        }
        if state_id == 27588 {
            return Some(CaveVines {
                r#age: 17,
                r#berries: true,
            });
        }
        if state_id == 27593 {
            return Some(CaveVines {
                r#age: 19,
                r#berries: false,
            });
        }
        if state_id == 27563 {
            return Some(CaveVines {
                r#age: 4,
                r#berries: false,
            });
        }
        if state_id == 27595 {
            return Some(CaveVines {
                r#berries: false,
                r#age: 20,
            });
        }
        if state_id == 27600 {
            return Some(CaveVines {
                r#berries: true,
                r#age: 23,
            });
        }
        if state_id == 27586 {
            return Some(CaveVines {
                r#age: 16,
                r#berries: true,
            });
        }
        if state_id == 27574 {
            return Some(CaveVines {
                r#age: 10,
                r#berries: true,
            });
        }
        if state_id == 27601 {
            return Some(CaveVines {
                r#age: 23,
                r#berries: false,
            });
        }
        if state_id == 27571 {
            return Some(CaveVines {
                r#berries: false,
                r#age: 8,
            });
        }
        if state_id == 27605 {
            return Some(CaveVines {
                r#berries: false,
                r#age: 25,
            });
        }
        if state_id == 27598 {
            return Some(CaveVines {
                r#age: 22,
                r#berries: true,
            });
        }
        if state_id == 27587 {
            return Some(CaveVines {
                r#berries: false,
                r#age: 16,
            });
        }
        if state_id == 27591 {
            return Some(CaveVines {
                r#age: 18,
                r#berries: false,
            });
        }
        if state_id == 27573 {
            return Some(CaveVines {
                r#age: 9,
                r#berries: false,
            });
        }
        if state_id == 27561 {
            return Some(CaveVines {
                r#berries: false,
                r#age: 3,
            });
        }
        if state_id == 27568 {
            return Some(CaveVines {
                r#age: 7,
                r#berries: true,
            });
        }
        if state_id == 27566 {
            return Some(CaveVines {
                r#age: 6,
                r#berries: true,
            });
        }
        if state_id == 27581 {
            return Some(CaveVines {
                r#age: 13,
                r#berries: false,
            });
        }
        if state_id == 27570 {
            return Some(CaveVines {
                r#berries: true,
                r#age: 8,
            });
        }
        if state_id == 27603 {
            return Some(CaveVines {
                r#berries: false,
                r#age: 24,
            });
        }
        if state_id == 27589 {
            return Some(CaveVines {
                r#berries: false,
                r#age: 17,
            });
        }
        if state_id == 27592 {
            return Some(CaveVines {
                r#age: 19,
                r#berries: true,
            });
        }
        if state_id == 27569 {
            return Some(CaveVines {
                r#berries: false,
                r#age: 7,
            });
        }
        if state_id == 27562 {
            return Some(CaveVines {
                r#age: 4,
                r#berries: true,
            });
        }
        if state_id == 27590 {
            return Some(CaveVines {
                r#berries: true,
                r#age: 18,
            });
        }
        if state_id == 27555 {
            return Some(CaveVines {
                r#age: 0,
                r#berries: false,
            });
        }
        if state_id == 27596 {
            return Some(CaveVines {
                r#berries: true,
                r#age: 21,
            });
        }
        if state_id == 27602 {
            return Some(CaveVines {
                r#age: 24,
                r#berries: true,
            });
        }
        if state_id == 27594 {
            return Some(CaveVines {
                r#berries: true,
                r#age: 20,
            });
        }
        if state_id == 27554 {
            return Some(CaveVines {
                r#berries: true,
                r#age: 0,
            });
        }
        if state_id == 27583 {
            return Some(CaveVines {
                r#berries: false,
                r#age: 14,
            });
        }
        if state_id == 27584 {
            return Some(CaveVines {
                r#age: 15,
                r#berries: true,
            });
        }
        if state_id == 27564 {
            return Some(CaveVines {
                r#age: 5,
                r#berries: true,
            });
        }
        if state_id == 27599 {
            return Some(CaveVines {
                r#age: 22,
                r#berries: false,
            });
        }
        if state_id == 27604 {
            return Some(CaveVines {
                r#age: 25,
                r#berries: true,
            });
        }
        if state_id == 27580 {
            return Some(CaveVines {
                r#berries: true,
                r#age: 13,
            });
        }
        if state_id == 27572 {
            return Some(CaveVines {
                r#berries: true,
                r#age: 9,
            });
        }
        if state_id == 27567 {
            return Some(CaveVines {
                r#age: 6,
                r#berries: false,
            });
        }
        if state_id == 27575 {
            return Some(CaveVines {
                r#age: 10,
                r#berries: false,
            });
        }
        if state_id == 27577 {
            return Some(CaveVines {
                r#berries: false,
                r#age: 11,
            });
        }
        if state_id == 27556 {
            return Some(CaveVines {
                r#age: 1,
                r#berries: true,
            });
        }
        if state_id == 27597 {
            return Some(CaveVines {
                r#age: 21,
                r#berries: false,
            });
        }
        return None;
    }
}

