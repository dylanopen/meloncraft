use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CaveVines {
    pub berries: bool,
    pub age: i32,
}


impl BlockState for CaveVines {
    fn to_id(&self) -> i32 {
        if self.r#berries == false && self.r#age == 21 { return 27597; }
        if self.r#age == 22 && self.r#berries == true { return 27598; }
        if self.r#age == 22 && self.r#berries == false { return 27599; }
        if self.r#berries == true && self.r#age == 3 { return 27560; }
        if self.r#berries == false && self.r#age == 15 { return 27585; }
        if self.r#age == 19 && self.r#berries == true { return 27592; }
        if self.r#berries == false && self.r#age == 2 { return 27559; }
        if self.r#age == 21 && self.r#berries == true { return 27596; }
        if self.r#age == 18 && self.r#berries == false { return 27591; }
        if self.r#berries == false && self.r#age == 0 { return 27555; }
        if self.r#berries == true && self.r#age == 2 { return 27558; }
        if self.r#berries == true && self.r#age == 9 { return 27572; }
        if self.r#age == 0 && self.r#berries == true { return 27554; }
        if self.r#berries == true && self.r#age == 23 { return 27600; }
        if self.r#berries == true && self.r#age == 15 { return 27584; }
        if self.r#berries == false && self.r#age == 25 { return 27605; }
        if self.r#berries == true && self.r#age == 25 { return 27604; }
        if self.r#age == 8 && self.r#berries == false { return 27571; }
        if self.r#berries == false && self.r#age == 24 { return 27603; }
        if self.r#age == 18 && self.r#berries == true { return 27590; }
        if self.r#age == 16 && self.r#berries == false { return 27587; }
        if self.r#berries == false && self.r#age == 7 { return 27569; }
        if self.r#age == 23 && self.r#berries == false { return 27601; }
        if self.r#age == 24 && self.r#berries == true { return 27602; }
        if self.r#age == 10 && self.r#berries == false { return 27575; }
        if self.r#age == 14 && self.r#berries == true { return 27582; }
        if self.r#berries == true && self.r#age == 12 { return 27578; }
        if self.r#berries == true && self.r#age == 1 { return 27556; }
        if self.r#berries == false && self.r#age == 1 { return 27557; }
        if self.r#berries == false && self.r#age == 3 { return 27561; }
        if self.r#age == 8 && self.r#berries == true { return 27570; }
        if self.r#age == 9 && self.r#berries == false { return 27573; }
        if self.r#berries == false && self.r#age == 11 { return 27577; }
        if self.r#berries == false && self.r#age == 14 { return 27583; }
        if self.r#age == 12 && self.r#berries == false { return 27579; }
        if self.r#age == 17 && self.r#berries == true { return 27588; }
        if self.r#age == 13 && self.r#berries == false { return 27581; }
        if self.r#berries == true && self.r#age == 4 { return 27562; }
        if self.r#berries == false && self.r#age == 4 { return 27563; }
        if self.r#berries == true && self.r#age == 11 { return 27576; }
        if self.r#age == 5 && self.r#berries == true { return 27564; }
        if self.r#age == 5 && self.r#berries == false { return 27565; }
        if self.r#age == 6 && self.r#berries == true { return 27566; }
        if self.r#age == 7 && self.r#berries == true { return 27568; }
        if self.r#age == 17 && self.r#berries == false { return 27589; }
        if self.r#berries == false && self.r#age == 19 { return 27593; }
        if self.r#age == 20 && self.r#berries == true { return 27594; }
        if self.r#age == 6 && self.r#berries == false { return 27567; }
        if self.r#berries == true && self.r#age == 13 { return 27580; }
        if self.r#age == 10 && self.r#berries == true { return 27574; }
        if self.r#age == 16 && self.r#berries == true { return 27586; }
        if self.r#berries == false && self.r#age == 20 { return 27595; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 27597 {
            return Some(CaveVines {
                r#berries: false,
                r#age: 21,
            });
        }
        if state_id == 27598 {
            return Some(CaveVines {
                r#age: 22,
                r#berries: true,
            });
        }
        if state_id == 27599 {
            return Some(CaveVines {
                r#age: 22,
                r#berries: false,
            });
        }
        if state_id == 27560 {
            return Some(CaveVines {
                r#berries: true,
                r#age: 3,
            });
        }
        if state_id == 27585 {
            return Some(CaveVines {
                r#berries: false,
                r#age: 15,
            });
        }
        if state_id == 27592 {
            return Some(CaveVines {
                r#age: 19,
                r#berries: true,
            });
        }
        if state_id == 27559 {
            return Some(CaveVines {
                r#berries: false,
                r#age: 2,
            });
        }
        if state_id == 27596 {
            return Some(CaveVines {
                r#age: 21,
                r#berries: true,
            });
        }
        if state_id == 27591 {
            return Some(CaveVines {
                r#age: 18,
                r#berries: false,
            });
        }
        if state_id == 27555 {
            return Some(CaveVines {
                r#berries: false,
                r#age: 0,
            });
        }
        if state_id == 27558 {
            return Some(CaveVines {
                r#berries: true,
                r#age: 2,
            });
        }
        if state_id == 27572 {
            return Some(CaveVines {
                r#berries: true,
                r#age: 9,
            });
        }
        if state_id == 27554 {
            return Some(CaveVines {
                r#age: 0,
                r#berries: true,
            });
        }
        if state_id == 27600 {
            return Some(CaveVines {
                r#berries: true,
                r#age: 23,
            });
        }
        if state_id == 27584 {
            return Some(CaveVines {
                r#berries: true,
                r#age: 15,
            });
        }
        if state_id == 27605 {
            return Some(CaveVines {
                r#berries: false,
                r#age: 25,
            });
        }
        if state_id == 27604 {
            return Some(CaveVines {
                r#berries: true,
                r#age: 25,
            });
        }
        if state_id == 27571 {
            return Some(CaveVines {
                r#age: 8,
                r#berries: false,
            });
        }
        if state_id == 27603 {
            return Some(CaveVines {
                r#berries: false,
                r#age: 24,
            });
        }
        if state_id == 27590 {
            return Some(CaveVines {
                r#age: 18,
                r#berries: true,
            });
        }
        if state_id == 27587 {
            return Some(CaveVines {
                r#age: 16,
                r#berries: false,
            });
        }
        if state_id == 27569 {
            return Some(CaveVines {
                r#berries: false,
                r#age: 7,
            });
        }
        if state_id == 27601 {
            return Some(CaveVines {
                r#age: 23,
                r#berries: false,
            });
        }
        if state_id == 27602 {
            return Some(CaveVines {
                r#age: 24,
                r#berries: true,
            });
        }
        if state_id == 27575 {
            return Some(CaveVines {
                r#age: 10,
                r#berries: false,
            });
        }
        if state_id == 27582 {
            return Some(CaveVines {
                r#age: 14,
                r#berries: true,
            });
        }
        if state_id == 27578 {
            return Some(CaveVines {
                r#berries: true,
                r#age: 12,
            });
        }
        if state_id == 27556 {
            return Some(CaveVines {
                r#berries: true,
                r#age: 1,
            });
        }
        if state_id == 27557 {
            return Some(CaveVines {
                r#berries: false,
                r#age: 1,
            });
        }
        if state_id == 27561 {
            return Some(CaveVines {
                r#berries: false,
                r#age: 3,
            });
        }
        if state_id == 27570 {
            return Some(CaveVines {
                r#age: 8,
                r#berries: true,
            });
        }
        if state_id == 27573 {
            return Some(CaveVines {
                r#age: 9,
                r#berries: false,
            });
        }
        if state_id == 27577 {
            return Some(CaveVines {
                r#berries: false,
                r#age: 11,
            });
        }
        if state_id == 27583 {
            return Some(CaveVines {
                r#berries: false,
                r#age: 14,
            });
        }
        if state_id == 27579 {
            return Some(CaveVines {
                r#age: 12,
                r#berries: false,
            });
        }
        if state_id == 27588 {
            return Some(CaveVines {
                r#age: 17,
                r#berries: true,
            });
        }
        if state_id == 27581 {
            return Some(CaveVines {
                r#age: 13,
                r#berries: false,
            });
        }
        if state_id == 27562 {
            return Some(CaveVines {
                r#berries: true,
                r#age: 4,
            });
        }
        if state_id == 27563 {
            return Some(CaveVines {
                r#berries: false,
                r#age: 4,
            });
        }
        if state_id == 27576 {
            return Some(CaveVines {
                r#berries: true,
                r#age: 11,
            });
        }
        if state_id == 27564 {
            return Some(CaveVines {
                r#age: 5,
                r#berries: true,
            });
        }
        if state_id == 27565 {
            return Some(CaveVines {
                r#age: 5,
                r#berries: false,
            });
        }
        if state_id == 27566 {
            return Some(CaveVines {
                r#age: 6,
                r#berries: true,
            });
        }
        if state_id == 27568 {
            return Some(CaveVines {
                r#age: 7,
                r#berries: true,
            });
        }
        if state_id == 27589 {
            return Some(CaveVines {
                r#age: 17,
                r#berries: false,
            });
        }
        if state_id == 27593 {
            return Some(CaveVines {
                r#berries: false,
                r#age: 19,
            });
        }
        if state_id == 27594 {
            return Some(CaveVines {
                r#age: 20,
                r#berries: true,
            });
        }
        if state_id == 27567 {
            return Some(CaveVines {
                r#age: 6,
                r#berries: false,
            });
        }
        if state_id == 27580 {
            return Some(CaveVines {
                r#berries: true,
                r#age: 13,
            });
        }
        if state_id == 27574 {
            return Some(CaveVines {
                r#age: 10,
                r#berries: true,
            });
        }
        if state_id == 27586 {
            return Some(CaveVines {
                r#age: 16,
                r#berries: true,
            });
        }
        if state_id == 27595 {
            return Some(CaveVines {
                r#berries: false,
                r#age: 20,
            });
        }
        return None;
    }
}

