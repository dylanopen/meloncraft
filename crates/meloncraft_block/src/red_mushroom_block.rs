use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedMushroomBlock {
    pub down: bool,
    pub east: bool,
    pub north: bool,
    pub south: bool,
    pub up: bool,
    pub west: bool,
}


impl BlockState for RedMushroomBlock {
    fn to_id(&self) -> i32 {
        if self.r#north == false && self.r#south == false && self.r#up == true && self.r#east == false && self.r#down == true && self.r#west == false { return 7658; }
        if self.r#east == true && self.r#west == true && self.r#north == true && self.r#up == false && self.r#down == false && self.r#south == false { return 7667; }
        if self.r#down == false && self.r#north == false && self.r#east == true && self.r#up == false && self.r#south == true && self.r#west == false { return 7672; }
        if self.r#south == true && self.r#east == false && self.r#up == true && self.r#down == true && self.r#north == true && self.r#west == true { return 7645; }
        if self.r#south == true && self.r#north == false && self.r#west == true && self.r#up == true && self.r#down == true && self.r#east == false { return 7653; }
        if self.r#down == true && self.r#east == true && self.r#north == true && self.r#south == true && self.r#up == true && self.r#west == false { return 7630; }
        if self.r#north == true && self.r#down == true && self.r#east == true && self.r#up == true && self.r#south == false && self.r#west == false { return 7634; }
        if self.r#down == true && self.r#south == true && self.r#up == true && self.r#west == false && self.r#east == false && self.r#north == true { return 7646; }
        if self.r#down == false && self.r#east == false && self.r#south == false && self.r#north == false && self.r#up == true && self.r#west == true { return 7689; }
        if self.r#down == true && self.r#up == true && self.r#north == false && self.r#west == true && self.r#south == false && self.r#east == false { return 7657; }
        if self.r#west == true && self.r#south == false && self.r#north == true && self.r#down == false && self.r#up == true && self.r#east == false { return 7681; }
        if self.r#up == false && self.r#down == false && self.r#south == false && self.r#west == true && self.r#north == false && self.r#east == true { return 7675; }
        if self.r#down == true && self.r#east == true && self.r#north == true && self.r#up == true && self.r#west == true && self.r#south == true { return 7629; }
        if self.r#north == false && self.r#down == false && self.r#east == false && self.r#west == false && self.r#south == true && self.r#up == false { return 7688; }
        if self.r#down == false && self.r#east == true && self.r#west == false && self.r#south == false && self.r#up == true && self.r#north == true { return 7666; }
        if self.r#east == false && self.r#west == true && self.r#north == true && self.r#up == false && self.r#down == false && self.r#south == false { return 7683; }
        if self.r#up == true && self.r#down == false && self.r#north == false && self.r#south == true && self.r#west == true && self.r#east == true { return 7669; }
        if self.r#up == false && self.r#west == false && self.r#south == true && self.r#east == false && self.r#down == true && self.r#north == false { return 7656; }
        if self.r#up == false && self.r#down == false && self.r#east == true && self.r#south == true && self.r#west == true && self.r#north == true { return 7663; }
        if self.r#down == true && self.r#south == false && self.r#up == true && self.r#north == false && self.r#west == false && self.r#east == true { return 7642; }
        if self.r#south == false && self.r#up == false && self.r#west == true && self.r#north == true && self.r#down == true && self.r#east == false { return 7651; }
        if self.r#south == false && self.r#north == true && self.r#west == false && self.r#up == false && self.r#down == false && self.r#east == false { return 7684; }
        if self.r#south == true && self.r#down == false && self.r#east == false && self.r#up == false && self.r#north == true && self.r#west == false { return 7680; }
        if self.r#north == false && self.r#down == false && self.r#east == false && self.r#south == false && self.r#up == false && self.r#west == false { return 7692; }
        if self.r#west == true && self.r#east == true && self.r#down == true && self.r#north == true && self.r#south == true && self.r#up == false { return 7631; }
        if self.r#south == false && self.r#north == false && self.r#down == true && self.r#up == false && self.r#east == false && self.r#west == true { return 7659; }
        if self.r#west == true && self.r#east == false && self.r#north == false && self.r#down == false && self.r#south == true && self.r#up == false { return 7687; }
        if self.r#south == false && self.r#north == true && self.r#up == true && self.r#down == true && self.r#east == true && self.r#west == true { return 7633; }
        if self.r#east == true && self.r#south == false && self.r#west == true && self.r#down == false && self.r#north == false && self.r#up == true { return 7673; }
        if self.r#south == true && self.r#down == false && self.r#up == false && self.r#east == false && self.r#north == true && self.r#west == true { return 7679; }
        if self.r#south == true && self.r#up == true && self.r#west == true && self.r#down == true && self.r#east == true && self.r#north == false { return 7637; }
        if self.r#down == true && self.r#west == false && self.r#north == false && self.r#south == true && self.r#east == false && self.r#up == true { return 7654; }
        if self.r#down == false && self.r#east == true && self.r#north == true && self.r#west == false && self.r#up == false && self.r#south == true { return 7664; }
        if self.r#down == false && self.r#east == false && self.r#up == true && self.r#west == false && self.r#north == false && self.r#south == true { return 7686; }
        if self.r#east == true && self.r#down == true && self.r#north == true && self.r#south == false && self.r#up == false && self.r#west == true { return 7635; }
        if self.r#east == true && self.r#down == false && self.r#west == false && self.r#south == false && self.r#up == false && self.r#north == true { return 7668; }
        if self.r#south == true && self.r#west == true && self.r#up == false && self.r#down == true && self.r#east == true && self.r#north == false { return 7639; }
        if self.r#down == true && self.r#north == true && self.r#east == true && self.r#up == false && self.r#west == false && self.r#south == false { return 7636; }
        if self.r#north == false && self.r#east == true && self.r#up == false && self.r#down == false && self.r#south == true && self.r#west == true { return 7671; }
        if self.r#up == true && self.r#west == false && self.r#down == false && self.r#north == false && self.r#east == true && self.r#south == false { return 7674; }
        if self.r#south == false && self.r#up == false && self.r#west == false && self.r#east == false && self.r#down == true && self.r#north == true { return 7652; }
        if self.r#south == false && self.r#down == false && self.r#north == false && self.r#east == true && self.r#up == false && self.r#west == false { return 7676; }
        if self.r#south == false && self.r#north == true && self.r#up == true && self.r#east == false && self.r#down == true && self.r#west == false { return 7650; }
        if self.r#east == true && self.r#north == true && self.r#up == true && self.r#west == true && self.r#south == true && self.r#down == false { return 7661; }
        if self.r#north == false && self.r#up == false && self.r#west == true && self.r#south == false && self.r#down == false && self.r#east == false { return 7691; }
        if self.r#north == false && self.r#east == true && self.r#up == true && self.r#west == false && self.r#south == true && self.r#down == true { return 7638; }
        if self.r#north == true && self.r#up == true && self.r#west == false && self.r#east == true && self.r#south == true && self.r#down == false { return 7662; }
        if self.r#north == false && self.r#up == false && self.r#down == true && self.r#south == false && self.r#east == false && self.r#west == false { return 7660; }
        if self.r#north == true && self.r#down == true && self.r#west == false && self.r#south == true && self.r#east == true && self.r#up == false { return 7632; }
        if self.r#down == false && self.r#north == false && self.r#east == true && self.r#south == true && self.r#west == false && self.r#up == true { return 7670; }
        if self.r#south == false && self.r#north == true && self.r#east == false && self.r#up == true && self.r#west == false && self.r#down == false { return 7682; }
        if self.r#south == true && self.r#north == false && self.r#west == true && self.r#up == true && self.r#down == false && self.r#east == false { return 7685; }
        if self.r#west == true && self.r#east == false && self.r#up == false && self.r#down == true && self.r#north == false && self.r#south == true { return 7655; }
        if self.r#east == false && self.r#north == true && self.r#down == true && self.r#south == true && self.r#west == false && self.r#up == false { return 7648; }
        if self.r#north == false && self.r#west == false && self.r#east == true && self.r#down == true && self.r#up == false && self.r#south == false { return 7644; }
        if self.r#east == false && self.r#south == true && self.r#west == false && self.r#north == true && self.r#down == false && self.r#up == true { return 7678; }
        if self.r#north == false && self.r#east == true && self.r#up == false && self.r#west == false && self.r#down == true && self.r#south == true { return 7640; }
        if self.r#east == true && self.r#south == false && self.r#up == true && self.r#west == true && self.r#down == true && self.r#north == false { return 7641; }
        if self.r#north == true && self.r#up == false && self.r#down == true && self.r#east == false && self.r#south == true && self.r#west == true { return 7647; }
        if self.r#south == false && self.r#up == true && self.r#down == false && self.r#east == true && self.r#west == true && self.r#north == true { return 7665; }
        if self.r#east == false && self.r#up == true && self.r#south == true && self.r#west == true && self.r#north == true && self.r#down == false { return 7677; }
        if self.r#south == false && self.r#west == true && self.r#north == true && self.r#east == false && self.r#up == true && self.r#down == true { return 7649; }
        if self.r#east == true && self.r#up == false && self.r#down == true && self.r#west == true && self.r#south == false && self.r#north == false { return 7643; }
        if self.r#south == false && self.r#up == true && self.r#down == false && self.r#north == false && self.r#west == false && self.r#east == false { return 7690; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 7658 {
            return Some(RedMushroomBlock {
                r#north: false,
                r#south: false,
                r#up: true,
                r#east: false,
                r#down: true,
                r#west: false,
            });
        }
        if state_id == 7667 {
            return Some(RedMushroomBlock {
                r#east: true,
                r#west: true,
                r#north: true,
                r#up: false,
                r#down: false,
                r#south: false,
            });
        }
        if state_id == 7672 {
            return Some(RedMushroomBlock {
                r#down: false,
                r#north: false,
                r#east: true,
                r#up: false,
                r#south: true,
                r#west: false,
            });
        }
        if state_id == 7645 {
            return Some(RedMushroomBlock {
                r#south: true,
                r#east: false,
                r#up: true,
                r#down: true,
                r#north: true,
                r#west: true,
            });
        }
        if state_id == 7653 {
            return Some(RedMushroomBlock {
                r#south: true,
                r#north: false,
                r#west: true,
                r#up: true,
                r#down: true,
                r#east: false,
            });
        }
        if state_id == 7630 {
            return Some(RedMushroomBlock {
                r#down: true,
                r#east: true,
                r#north: true,
                r#south: true,
                r#up: true,
                r#west: false,
            });
        }
        if state_id == 7634 {
            return Some(RedMushroomBlock {
                r#north: true,
                r#down: true,
                r#east: true,
                r#up: true,
                r#south: false,
                r#west: false,
            });
        }
        if state_id == 7646 {
            return Some(RedMushroomBlock {
                r#down: true,
                r#south: true,
                r#up: true,
                r#west: false,
                r#east: false,
                r#north: true,
            });
        }
        if state_id == 7689 {
            return Some(RedMushroomBlock {
                r#down: false,
                r#east: false,
                r#south: false,
                r#north: false,
                r#up: true,
                r#west: true,
            });
        }
        if state_id == 7657 {
            return Some(RedMushroomBlock {
                r#down: true,
                r#up: true,
                r#north: false,
                r#west: true,
                r#south: false,
                r#east: false,
            });
        }
        if state_id == 7681 {
            return Some(RedMushroomBlock {
                r#west: true,
                r#south: false,
                r#north: true,
                r#down: false,
                r#up: true,
                r#east: false,
            });
        }
        if state_id == 7675 {
            return Some(RedMushroomBlock {
                r#up: false,
                r#down: false,
                r#south: false,
                r#west: true,
                r#north: false,
                r#east: true,
            });
        }
        if state_id == 7629 {
            return Some(RedMushroomBlock {
                r#down: true,
                r#east: true,
                r#north: true,
                r#up: true,
                r#west: true,
                r#south: true,
            });
        }
        if state_id == 7688 {
            return Some(RedMushroomBlock {
                r#north: false,
                r#down: false,
                r#east: false,
                r#west: false,
                r#south: true,
                r#up: false,
            });
        }
        if state_id == 7666 {
            return Some(RedMushroomBlock {
                r#down: false,
                r#east: true,
                r#west: false,
                r#south: false,
                r#up: true,
                r#north: true,
            });
        }
        if state_id == 7683 {
            return Some(RedMushroomBlock {
                r#east: false,
                r#west: true,
                r#north: true,
                r#up: false,
                r#down: false,
                r#south: false,
            });
        }
        if state_id == 7669 {
            return Some(RedMushroomBlock {
                r#up: true,
                r#down: false,
                r#north: false,
                r#south: true,
                r#west: true,
                r#east: true,
            });
        }
        if state_id == 7656 {
            return Some(RedMushroomBlock {
                r#up: false,
                r#west: false,
                r#south: true,
                r#east: false,
                r#down: true,
                r#north: false,
            });
        }
        if state_id == 7663 {
            return Some(RedMushroomBlock {
                r#up: false,
                r#down: false,
                r#east: true,
                r#south: true,
                r#west: true,
                r#north: true,
            });
        }
        if state_id == 7642 {
            return Some(RedMushroomBlock {
                r#down: true,
                r#south: false,
                r#up: true,
                r#north: false,
                r#west: false,
                r#east: true,
            });
        }
        if state_id == 7651 {
            return Some(RedMushroomBlock {
                r#south: false,
                r#up: false,
                r#west: true,
                r#north: true,
                r#down: true,
                r#east: false,
            });
        }
        if state_id == 7684 {
            return Some(RedMushroomBlock {
                r#south: false,
                r#north: true,
                r#west: false,
                r#up: false,
                r#down: false,
                r#east: false,
            });
        }
        if state_id == 7680 {
            return Some(RedMushroomBlock {
                r#south: true,
                r#down: false,
                r#east: false,
                r#up: false,
                r#north: true,
                r#west: false,
            });
        }
        if state_id == 7692 {
            return Some(RedMushroomBlock {
                r#north: false,
                r#down: false,
                r#east: false,
                r#south: false,
                r#up: false,
                r#west: false,
            });
        }
        if state_id == 7631 {
            return Some(RedMushroomBlock {
                r#west: true,
                r#east: true,
                r#down: true,
                r#north: true,
                r#south: true,
                r#up: false,
            });
        }
        if state_id == 7659 {
            return Some(RedMushroomBlock {
                r#south: false,
                r#north: false,
                r#down: true,
                r#up: false,
                r#east: false,
                r#west: true,
            });
        }
        if state_id == 7687 {
            return Some(RedMushroomBlock {
                r#west: true,
                r#east: false,
                r#north: false,
                r#down: false,
                r#south: true,
                r#up: false,
            });
        }
        if state_id == 7633 {
            return Some(RedMushroomBlock {
                r#south: false,
                r#north: true,
                r#up: true,
                r#down: true,
                r#east: true,
                r#west: true,
            });
        }
        if state_id == 7673 {
            return Some(RedMushroomBlock {
                r#east: true,
                r#south: false,
                r#west: true,
                r#down: false,
                r#north: false,
                r#up: true,
            });
        }
        if state_id == 7679 {
            return Some(RedMushroomBlock {
                r#south: true,
                r#down: false,
                r#up: false,
                r#east: false,
                r#north: true,
                r#west: true,
            });
        }
        if state_id == 7637 {
            return Some(RedMushroomBlock {
                r#south: true,
                r#up: true,
                r#west: true,
                r#down: true,
                r#east: true,
                r#north: false,
            });
        }
        if state_id == 7654 {
            return Some(RedMushroomBlock {
                r#down: true,
                r#west: false,
                r#north: false,
                r#south: true,
                r#east: false,
                r#up: true,
            });
        }
        if state_id == 7664 {
            return Some(RedMushroomBlock {
                r#down: false,
                r#east: true,
                r#north: true,
                r#west: false,
                r#up: false,
                r#south: true,
            });
        }
        if state_id == 7686 {
            return Some(RedMushroomBlock {
                r#down: false,
                r#east: false,
                r#up: true,
                r#west: false,
                r#north: false,
                r#south: true,
            });
        }
        if state_id == 7635 {
            return Some(RedMushroomBlock {
                r#east: true,
                r#down: true,
                r#north: true,
                r#south: false,
                r#up: false,
                r#west: true,
            });
        }
        if state_id == 7668 {
            return Some(RedMushroomBlock {
                r#east: true,
                r#down: false,
                r#west: false,
                r#south: false,
                r#up: false,
                r#north: true,
            });
        }
        if state_id == 7639 {
            return Some(RedMushroomBlock {
                r#south: true,
                r#west: true,
                r#up: false,
                r#down: true,
                r#east: true,
                r#north: false,
            });
        }
        if state_id == 7636 {
            return Some(RedMushroomBlock {
                r#down: true,
                r#north: true,
                r#east: true,
                r#up: false,
                r#west: false,
                r#south: false,
            });
        }
        if state_id == 7671 {
            return Some(RedMushroomBlock {
                r#north: false,
                r#east: true,
                r#up: false,
                r#down: false,
                r#south: true,
                r#west: true,
            });
        }
        if state_id == 7674 {
            return Some(RedMushroomBlock {
                r#up: true,
                r#west: false,
                r#down: false,
                r#north: false,
                r#east: true,
                r#south: false,
            });
        }
        if state_id == 7652 {
            return Some(RedMushroomBlock {
                r#south: false,
                r#up: false,
                r#west: false,
                r#east: false,
                r#down: true,
                r#north: true,
            });
        }
        if state_id == 7676 {
            return Some(RedMushroomBlock {
                r#south: false,
                r#down: false,
                r#north: false,
                r#east: true,
                r#up: false,
                r#west: false,
            });
        }
        if state_id == 7650 {
            return Some(RedMushroomBlock {
                r#south: false,
                r#north: true,
                r#up: true,
                r#east: false,
                r#down: true,
                r#west: false,
            });
        }
        if state_id == 7661 {
            return Some(RedMushroomBlock {
                r#east: true,
                r#north: true,
                r#up: true,
                r#west: true,
                r#south: true,
                r#down: false,
            });
        }
        if state_id == 7691 {
            return Some(RedMushroomBlock {
                r#north: false,
                r#up: false,
                r#west: true,
                r#south: false,
                r#down: false,
                r#east: false,
            });
        }
        if state_id == 7638 {
            return Some(RedMushroomBlock {
                r#north: false,
                r#east: true,
                r#up: true,
                r#west: false,
                r#south: true,
                r#down: true,
            });
        }
        if state_id == 7662 {
            return Some(RedMushroomBlock {
                r#north: true,
                r#up: true,
                r#west: false,
                r#east: true,
                r#south: true,
                r#down: false,
            });
        }
        if state_id == 7660 {
            return Some(RedMushroomBlock {
                r#north: false,
                r#up: false,
                r#down: true,
                r#south: false,
                r#east: false,
                r#west: false,
            });
        }
        if state_id == 7632 {
            return Some(RedMushroomBlock {
                r#north: true,
                r#down: true,
                r#west: false,
                r#south: true,
                r#east: true,
                r#up: false,
            });
        }
        if state_id == 7670 {
            return Some(RedMushroomBlock {
                r#down: false,
                r#north: false,
                r#east: true,
                r#south: true,
                r#west: false,
                r#up: true,
            });
        }
        if state_id == 7682 {
            return Some(RedMushroomBlock {
                r#south: false,
                r#north: true,
                r#east: false,
                r#up: true,
                r#west: false,
                r#down: false,
            });
        }
        if state_id == 7685 {
            return Some(RedMushroomBlock {
                r#south: true,
                r#north: false,
                r#west: true,
                r#up: true,
                r#down: false,
                r#east: false,
            });
        }
        if state_id == 7655 {
            return Some(RedMushroomBlock {
                r#west: true,
                r#east: false,
                r#up: false,
                r#down: true,
                r#north: false,
                r#south: true,
            });
        }
        if state_id == 7648 {
            return Some(RedMushroomBlock {
                r#east: false,
                r#north: true,
                r#down: true,
                r#south: true,
                r#west: false,
                r#up: false,
            });
        }
        if state_id == 7644 {
            return Some(RedMushroomBlock {
                r#north: false,
                r#west: false,
                r#east: true,
                r#down: true,
                r#up: false,
                r#south: false,
            });
        }
        if state_id == 7678 {
            return Some(RedMushroomBlock {
                r#east: false,
                r#south: true,
                r#west: false,
                r#north: true,
                r#down: false,
                r#up: true,
            });
        }
        if state_id == 7640 {
            return Some(RedMushroomBlock {
                r#north: false,
                r#east: true,
                r#up: false,
                r#west: false,
                r#down: true,
                r#south: true,
            });
        }
        if state_id == 7641 {
            return Some(RedMushroomBlock {
                r#east: true,
                r#south: false,
                r#up: true,
                r#west: true,
                r#down: true,
                r#north: false,
            });
        }
        if state_id == 7647 {
            return Some(RedMushroomBlock {
                r#north: true,
                r#up: false,
                r#down: true,
                r#east: false,
                r#south: true,
                r#west: true,
            });
        }
        if state_id == 7665 {
            return Some(RedMushroomBlock {
                r#south: false,
                r#up: true,
                r#down: false,
                r#east: true,
                r#west: true,
                r#north: true,
            });
        }
        if state_id == 7677 {
            return Some(RedMushroomBlock {
                r#east: false,
                r#up: true,
                r#south: true,
                r#west: true,
                r#north: true,
                r#down: false,
            });
        }
        if state_id == 7649 {
            return Some(RedMushroomBlock {
                r#south: false,
                r#west: true,
                r#north: true,
                r#east: false,
                r#up: true,
                r#down: true,
            });
        }
        if state_id == 7643 {
            return Some(RedMushroomBlock {
                r#east: true,
                r#up: false,
                r#down: true,
                r#west: true,
                r#south: false,
                r#north: false,
            });
        }
        if state_id == 7690 {
            return Some(RedMushroomBlock {
                r#south: false,
                r#up: true,
                r#down: false,
                r#north: false,
                r#west: false,
                r#east: false,
            });
        }
        return None;
    }
}

