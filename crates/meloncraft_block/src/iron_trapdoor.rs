use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IronTrapdoor {
    pub open: bool,
    pub r#facing: Facing,
    pub waterlogged: bool,
    pub r#half: Half,
    pub powered: bool,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Half {
    Top,
    Bottom,
}

impl BlockState for IronTrapdoor {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::West && block_state.r#half == Half::Bottom && block_state.r#open == false && block_state.r#powered == false { return 12412; }
        if block_state.r#open == true && block_state.r#powered == false && block_state.r#facing == Facing::West && block_state.r#waterlogged == true && block_state.r#half == Half::Bottom { return 12407; }
        if block_state.r#facing == Facing::North && block_state.r#open == true && block_state.r#waterlogged == false && block_state.r#powered == false && block_state.r#half == Half::Top { return 12368; }
        if block_state.r#half == Half::Bottom && block_state.r#powered == true && block_state.r#open == false && block_state.r#waterlogged == false && block_state.r#facing == Facing::North { return 12378; }
        if block_state.r#facing == Facing::East && block_state.r#open == false && block_state.r#powered == true && block_state.r#half == Half::Bottom && block_state.r#waterlogged == false { return 12426; }
        if block_state.r#half == Half::Top && block_state.r#powered == true && block_state.r#waterlogged == false && block_state.r#facing == Facing::North && block_state.r#open == false { return 12370; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == true && block_state.r#half == Half::Top && block_state.r#powered == false && block_state.r#open == false { return 12371; }
        if block_state.r#facing == Facing::West && block_state.r#open == true && block_state.r#powered == false && block_state.r#half == Half::Bottom && block_state.r#waterlogged == false { return 12408; }
        if block_state.r#powered == false && block_state.r#waterlogged == false && block_state.r#open == false && block_state.r#facing == Facing::East && block_state.r#half == Half::Bottom { return 12428; }
        if block_state.r#facing == Facing::North && block_state.r#open == true && block_state.r#half == Half::Bottom && block_state.r#powered == false && block_state.r#waterlogged == false { return 12376; }
        if block_state.r#half == Half::Bottom && block_state.r#open == false && block_state.r#waterlogged == true && block_state.r#facing == Facing::East && block_state.r#powered == true { return 12425; }
        if block_state.r#open == false && block_state.r#waterlogged == false && block_state.r#facing == Facing::South && block_state.r#half == Half::Top && block_state.r#powered == false { return 12388; }
        if block_state.r#powered == true && block_state.r#facing == Facing::West && block_state.r#open == false && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true { return 12409; }
        if block_state.r#facing == Facing::North && block_state.r#open == true && block_state.r#waterlogged == true && block_state.r#powered == true && block_state.r#half == Half::Bottom { return 12373; }
        if block_state.r#half == Half::Top && block_state.r#powered == true && block_state.r#facing == Facing::South && block_state.r#waterlogged == true && block_state.r#open == true { return 12381; }
        if block_state.r#powered == true && block_state.r#facing == Facing::West && block_state.r#half == Half::Bottom && block_state.r#waterlogged == false && block_state.r#open == true { return 12406; }
        if block_state.r#waterlogged == true && block_state.r#powered == true && block_state.r#open == true && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::West { return 12405; }
        if block_state.r#open == false && block_state.r#waterlogged == false && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::North && block_state.r#powered == false { return 12380; }
        if block_state.r#half == Half::Bottom && block_state.r#powered == false && block_state.r#open == true && block_state.r#waterlogged == true && block_state.r#facing == Facing::East { return 12423; }
        if block_state.r#half == Half::Bottom && block_state.r#facing == Facing::North && block_state.r#powered == true && block_state.r#open == false && block_state.r#waterlogged == true { return 12377; }
        if block_state.r#open == true && block_state.r#powered == false && block_state.r#waterlogged == false && block_state.r#half == Half::Top && block_state.r#facing == Facing::East { return 12416; }
        if block_state.r#facing == Facing::West && block_state.r#half == Half::Top && block_state.r#open == true && block_state.r#powered == false && block_state.r#waterlogged == false { return 12400; }
        if block_state.r#powered == false && block_state.r#waterlogged == false && block_state.r#facing == Facing::South && block_state.r#half == Half::Bottom && block_state.r#open == true { return 12392; }
        if block_state.r#powered == false && block_state.r#open == true && block_state.r#facing == Facing::North && block_state.r#waterlogged == true && block_state.r#half == Half::Bottom { return 12375; }
        if block_state.r#waterlogged == false && block_state.r#half == Half::Top && block_state.r#open == true && block_state.r#facing == Facing::East && block_state.r#powered == true { return 12414; }
        if block_state.r#facing == Facing::East && block_state.r#open == true && block_state.r#powered == true && block_state.r#half == Half::Bottom && block_state.r#waterlogged == false { return 12422; }
        if block_state.r#waterlogged == false && block_state.r#open == false && block_state.r#half == Half::Top && block_state.r#powered == true && block_state.r#facing == Facing::East { return 12418; }
        if block_state.r#facing == Facing::North && block_state.r#open == true && block_state.r#half == Half::Top && block_state.r#waterlogged == true && block_state.r#powered == false { return 12367; }
        if block_state.r#open == true && block_state.r#facing == Facing::South && block_state.r#powered == false && block_state.r#half == Half::Top && block_state.r#waterlogged == false { return 12384; }
        if block_state.r#facing == Facing::East && block_state.r#open == true && block_state.r#powered == false && block_state.r#waterlogged == true && block_state.r#half == Half::Top { return 12415; }
        if block_state.r#half == Half::Bottom && block_state.r#open == false && block_state.r#powered == false && block_state.r#waterlogged == false && block_state.r#facing == Facing::South { return 12396; }
        if block_state.r#facing == Facing::East && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true && block_state.r#powered == true && block_state.r#open == true { return 12421; }
        if block_state.r#powered == false && block_state.r#facing == Facing::West && block_state.r#half == Half::Top && block_state.r#waterlogged == true && block_state.r#open == false { return 12403; }
        if block_state.r#facing == Facing::South && block_state.r#powered == true && block_state.r#open == false && block_state.r#waterlogged == true && block_state.r#half == Half::Top { return 12385; }
        if block_state.r#waterlogged == false && block_state.r#open == false && block_state.r#powered == true && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::West { return 12410; }
        if block_state.r#facing == Facing::West && block_state.r#half == Half::Top && block_state.r#powered == true && block_state.r#open == true && block_state.r#waterlogged == false { return 12398; }
        if block_state.r#powered == true && block_state.r#waterlogged == false && block_state.r#open == true && block_state.r#facing == Facing::North && block_state.r#half == Half::Bottom { return 12374; }
        if block_state.r#half == Half::Top && block_state.r#open == true && block_state.r#facing == Facing::South && block_state.r#powered == true && block_state.r#waterlogged == false { return 12382; }
        if block_state.r#facing == Facing::South && block_state.r#half == Half::Top && block_state.r#open == false && block_state.r#powered == false && block_state.r#waterlogged == true { return 12387; }
        if block_state.r#open == false && block_state.r#powered == true && block_state.r#waterlogged == false && block_state.r#half == Half::Top && block_state.r#facing == Facing::West { return 12402; }
        if block_state.r#half == Half::Top && block_state.r#facing == Facing::North && block_state.r#open == false && block_state.r#waterlogged == true && block_state.r#powered == true { return 12369; }
        if block_state.r#open == false && block_state.r#half == Half::Top && block_state.r#facing == Facing::East && block_state.r#waterlogged == false && block_state.r#powered == false { return 12420; }
        if block_state.r#half == Half::Bottom && block_state.r#facing == Facing::South && block_state.r#open == false && block_state.r#powered == true && block_state.r#waterlogged == true { return 12393; }
        if block_state.r#waterlogged == false && block_state.r#powered == false && block_state.r#open == false && block_state.r#facing == Facing::West && block_state.r#half == Half::Top { return 12404; }
        if block_state.r#waterlogged == true && block_state.r#powered == true && block_state.r#facing == Facing::South && block_state.r#open == true && block_state.r#half == Half::Bottom { return 12389; }
        if block_state.r#open == true && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::South && block_state.r#powered == true && block_state.r#waterlogged == false { return 12390; }
        if block_state.r#facing == Facing::West && block_state.r#open == true && block_state.r#waterlogged == true && block_state.r#half == Half::Top && block_state.r#powered == true { return 12397; }
        if block_state.r#facing == Facing::South && block_state.r#half == Half::Bottom && block_state.r#powered == false && block_state.r#open == false && block_state.r#waterlogged == true { return 12395; }
        if block_state.r#facing == Facing::South && block_state.r#open == false && block_state.r#waterlogged == false && block_state.r#powered == true && block_state.r#half == Half::Top { return 12386; }
        if block_state.r#powered == false && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::West && block_state.r#open == false && block_state.r#waterlogged == true { return 12411; }
        if block_state.r#half == Half::Top && block_state.r#open == true && block_state.r#facing == Facing::East && block_state.r#powered == true && block_state.r#waterlogged == true { return 12413; }
        if block_state.r#powered == false && block_state.r#half == Half::Top && block_state.r#facing == Facing::South && block_state.r#waterlogged == true && block_state.r#open == true { return 12383; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::East && block_state.r#open == false && block_state.r#half == Half::Top && block_state.r#powered == true { return 12417; }
        if block_state.r#waterlogged == true && block_state.r#open == false && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::East && block_state.r#powered == false { return 12427; }
        if block_state.r#open == false && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::South && block_state.r#powered == true && block_state.r#waterlogged == false { return 12394; }
        if block_state.r#facing == Facing::South && block_state.r#powered == false && block_state.r#half == Half::Bottom && block_state.r#open == true && block_state.r#waterlogged == true { return 12391; }
        if block_state.r#open == true && block_state.r#waterlogged == true && block_state.r#half == Half::Top && block_state.r#powered == false && block_state.r#facing == Facing::West { return 12399; }
        if block_state.r#half == Half::Top && block_state.r#powered == true && block_state.r#open == true && block_state.r#facing == Facing::North && block_state.r#waterlogged == false { return 12366; }
        if block_state.r#open == false && block_state.r#half == Half::Top && block_state.r#powered == false && block_state.r#waterlogged == false && block_state.r#facing == Facing::North { return 12372; }
        if block_state.r#half == Half::Top && block_state.r#open == false && block_state.r#waterlogged == true && block_state.r#facing == Facing::East && block_state.r#powered == false { return 12419; }
        if block_state.r#powered == true && block_state.r#open == true && block_state.r#waterlogged == true && block_state.r#half == Half::Top && block_state.r#facing == Facing::North { return 12365; }
        if block_state.r#waterlogged == false && block_state.r#open == true && block_state.r#powered == false && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::East { return 12424; }
        if block_state.r#open == false && block_state.r#facing == Facing::North && block_state.r#waterlogged == true && block_state.r#half == Half::Bottom && block_state.r#powered == false { return 12379; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == true && block_state.r#half == Half::Top && block_state.r#open == false && block_state.r#powered == true { return 12401; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 12412 {
            return Some(IronTrapdoor {
                r#waterlogged: false,
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 12407 {
            return Some(IronTrapdoor {
                r#open: true,
                r#powered: false,
                r#facing: Facing::West,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 12368 {
            return Some(IronTrapdoor {
                r#facing: Facing::North,
                r#open: true,
                r#waterlogged: false,
                r#powered: false,
                r#half: Half::Top,
            });
        }
        if state_id == 12378 {
            return Some(IronTrapdoor {
                r#half: Half::Bottom,
                r#powered: true,
                r#open: false,
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 12426 {
            return Some(IronTrapdoor {
                r#facing: Facing::East,
                r#open: false,
                r#powered: true,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 12370 {
            return Some(IronTrapdoor {
                r#half: Half::Top,
                r#powered: true,
                r#waterlogged: false,
                r#facing: Facing::North,
                r#open: false,
            });
        }
        if state_id == 12371 {
            return Some(IronTrapdoor {
                r#facing: Facing::North,
                r#waterlogged: true,
                r#half: Half::Top,
                r#powered: false,
                r#open: false,
            });
        }
        if state_id == 12408 {
            return Some(IronTrapdoor {
                r#facing: Facing::West,
                r#open: true,
                r#powered: false,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 12428 {
            return Some(IronTrapdoor {
                r#powered: false,
                r#waterlogged: false,
                r#open: false,
                r#facing: Facing::East,
                r#half: Half::Bottom,
            });
        }
        if state_id == 12376 {
            return Some(IronTrapdoor {
                r#facing: Facing::North,
                r#open: true,
                r#half: Half::Bottom,
                r#powered: false,
                r#waterlogged: false,
            });
        }
        if state_id == 12425 {
            return Some(IronTrapdoor {
                r#half: Half::Bottom,
                r#open: false,
                r#waterlogged: true,
                r#facing: Facing::East,
                r#powered: true,
            });
        }
        if state_id == 12388 {
            return Some(IronTrapdoor {
                r#open: false,
                r#waterlogged: false,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#powered: false,
            });
        }
        if state_id == 12409 {
            return Some(IronTrapdoor {
                r#powered: true,
                r#facing: Facing::West,
                r#open: false,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 12373 {
            return Some(IronTrapdoor {
                r#facing: Facing::North,
                r#open: true,
                r#waterlogged: true,
                r#powered: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 12381 {
            return Some(IronTrapdoor {
                r#half: Half::Top,
                r#powered: true,
                r#facing: Facing::South,
                r#waterlogged: true,
                r#open: true,
            });
        }
        if state_id == 12406 {
            return Some(IronTrapdoor {
                r#powered: true,
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#open: true,
            });
        }
        if state_id == 12405 {
            return Some(IronTrapdoor {
                r#waterlogged: true,
                r#powered: true,
                r#open: true,
                r#half: Half::Bottom,
                r#facing: Facing::West,
            });
        }
        if state_id == 12380 {
            return Some(IronTrapdoor {
                r#open: false,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#powered: false,
            });
        }
        if state_id == 12423 {
            return Some(IronTrapdoor {
                r#half: Half::Bottom,
                r#powered: false,
                r#open: true,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 12377 {
            return Some(IronTrapdoor {
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#powered: true,
                r#open: false,
                r#waterlogged: true,
            });
        }
        if state_id == 12416 {
            return Some(IronTrapdoor {
                r#open: true,
                r#powered: false,
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::East,
            });
        }
        if state_id == 12400 {
            return Some(IronTrapdoor {
                r#facing: Facing::West,
                r#half: Half::Top,
                r#open: true,
                r#powered: false,
                r#waterlogged: false,
            });
        }
        if state_id == 12392 {
            return Some(IronTrapdoor {
                r#powered: false,
                r#waterlogged: false,
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#open: true,
            });
        }
        if state_id == 12375 {
            return Some(IronTrapdoor {
                r#powered: false,
                r#open: true,
                r#facing: Facing::North,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 12414 {
            return Some(IronTrapdoor {
                r#waterlogged: false,
                r#half: Half::Top,
                r#open: true,
                r#facing: Facing::East,
                r#powered: true,
            });
        }
        if state_id == 12422 {
            return Some(IronTrapdoor {
                r#facing: Facing::East,
                r#open: true,
                r#powered: true,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 12418 {
            return Some(IronTrapdoor {
                r#waterlogged: false,
                r#open: false,
                r#half: Half::Top,
                r#powered: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 12367 {
            return Some(IronTrapdoor {
                r#facing: Facing::North,
                r#open: true,
                r#half: Half::Top,
                r#waterlogged: true,
                r#powered: false,
            });
        }
        if state_id == 12384 {
            return Some(IronTrapdoor {
                r#open: true,
                r#facing: Facing::South,
                r#powered: false,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 12415 {
            return Some(IronTrapdoor {
                r#facing: Facing::East,
                r#open: true,
                r#powered: false,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 12396 {
            return Some(IronTrapdoor {
                r#half: Half::Bottom,
                r#open: false,
                r#powered: false,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 12421 {
            return Some(IronTrapdoor {
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#powered: true,
                r#open: true,
            });
        }
        if state_id == 12403 {
            return Some(IronTrapdoor {
                r#powered: false,
                r#facing: Facing::West,
                r#half: Half::Top,
                r#waterlogged: true,
                r#open: false,
            });
        }
        if state_id == 12385 {
            return Some(IronTrapdoor {
                r#facing: Facing::South,
                r#powered: true,
                r#open: false,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 12410 {
            return Some(IronTrapdoor {
                r#waterlogged: false,
                r#open: false,
                r#powered: true,
                r#half: Half::Bottom,
                r#facing: Facing::West,
            });
        }
        if state_id == 12398 {
            return Some(IronTrapdoor {
                r#facing: Facing::West,
                r#half: Half::Top,
                r#powered: true,
                r#open: true,
                r#waterlogged: false,
            });
        }
        if state_id == 12374 {
            return Some(IronTrapdoor {
                r#powered: true,
                r#waterlogged: false,
                r#open: true,
                r#facing: Facing::North,
                r#half: Half::Bottom,
            });
        }
        if state_id == 12382 {
            return Some(IronTrapdoor {
                r#half: Half::Top,
                r#open: true,
                r#facing: Facing::South,
                r#powered: true,
                r#waterlogged: false,
            });
        }
        if state_id == 12387 {
            return Some(IronTrapdoor {
                r#facing: Facing::South,
                r#half: Half::Top,
                r#open: false,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 12402 {
            return Some(IronTrapdoor {
                r#open: false,
                r#powered: true,
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::West,
            });
        }
        if state_id == 12369 {
            return Some(IronTrapdoor {
                r#half: Half::Top,
                r#facing: Facing::North,
                r#open: false,
                r#waterlogged: true,
                r#powered: true,
            });
        }
        if state_id == 12420 {
            return Some(IronTrapdoor {
                r#open: false,
                r#half: Half::Top,
                r#facing: Facing::East,
                r#waterlogged: false,
                r#powered: false,
            });
        }
        if state_id == 12393 {
            return Some(IronTrapdoor {
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#open: false,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 12404 {
            return Some(IronTrapdoor {
                r#waterlogged: false,
                r#powered: false,
                r#open: false,
                r#facing: Facing::West,
                r#half: Half::Top,
            });
        }
        if state_id == 12389 {
            return Some(IronTrapdoor {
                r#waterlogged: true,
                r#powered: true,
                r#facing: Facing::South,
                r#open: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 12390 {
            return Some(IronTrapdoor {
                r#open: true,
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#powered: true,
                r#waterlogged: false,
            });
        }
        if state_id == 12397 {
            return Some(IronTrapdoor {
                r#facing: Facing::West,
                r#open: true,
                r#waterlogged: true,
                r#half: Half::Top,
                r#powered: true,
            });
        }
        if state_id == 12395 {
            return Some(IronTrapdoor {
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#powered: false,
                r#open: false,
                r#waterlogged: true,
            });
        }
        if state_id == 12386 {
            return Some(IronTrapdoor {
                r#facing: Facing::South,
                r#open: false,
                r#waterlogged: false,
                r#powered: true,
                r#half: Half::Top,
            });
        }
        if state_id == 12411 {
            return Some(IronTrapdoor {
                r#powered: false,
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#open: false,
                r#waterlogged: true,
            });
        }
        if state_id == 12413 {
            return Some(IronTrapdoor {
                r#half: Half::Top,
                r#open: true,
                r#facing: Facing::East,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 12383 {
            return Some(IronTrapdoor {
                r#powered: false,
                r#half: Half::Top,
                r#facing: Facing::South,
                r#waterlogged: true,
                r#open: true,
            });
        }
        if state_id == 12417 {
            return Some(IronTrapdoor {
                r#waterlogged: true,
                r#facing: Facing::East,
                r#open: false,
                r#half: Half::Top,
                r#powered: true,
            });
        }
        if state_id == 12427 {
            return Some(IronTrapdoor {
                r#waterlogged: true,
                r#open: false,
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#powered: false,
            });
        }
        if state_id == 12394 {
            return Some(IronTrapdoor {
                r#open: false,
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#powered: true,
                r#waterlogged: false,
            });
        }
        if state_id == 12391 {
            return Some(IronTrapdoor {
                r#facing: Facing::South,
                r#powered: false,
                r#half: Half::Bottom,
                r#open: true,
                r#waterlogged: true,
            });
        }
        if state_id == 12399 {
            return Some(IronTrapdoor {
                r#open: true,
                r#waterlogged: true,
                r#half: Half::Top,
                r#powered: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 12366 {
            return Some(IronTrapdoor {
                r#half: Half::Top,
                r#powered: true,
                r#open: true,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 12372 {
            return Some(IronTrapdoor {
                r#open: false,
                r#half: Half::Top,
                r#powered: false,
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 12419 {
            return Some(IronTrapdoor {
                r#half: Half::Top,
                r#open: false,
                r#waterlogged: true,
                r#facing: Facing::East,
                r#powered: false,
            });
        }
        if state_id == 12365 {
            return Some(IronTrapdoor {
                r#powered: true,
                r#open: true,
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::North,
            });
        }
        if state_id == 12424 {
            return Some(IronTrapdoor {
                r#waterlogged: false,
                r#open: true,
                r#powered: false,
                r#half: Half::Bottom,
                r#facing: Facing::East,
            });
        }
        if state_id == 12379 {
            return Some(IronTrapdoor {
                r#open: false,
                r#facing: Facing::North,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#powered: false,
            });
        }
        if state_id == 12401 {
            return Some(IronTrapdoor {
                r#facing: Facing::West,
                r#waterlogged: true,
                r#half: Half::Top,
                r#open: false,
                r#powered: true,
            });
        }
        return None;
    }
}

