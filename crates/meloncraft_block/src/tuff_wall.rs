use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TuffWall {
    pub up: bool,
    pub waterlogged: bool,
    pub r#north: North,
    pub r#west: West,
    pub r#south: South,
    pub r#east: East,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum North {
    None,
    Low,
    Tall,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum West {
    None,
    Low,
    Tall,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum South {
    None,
    Low,
    Tall,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum East {
    None,
    Low,
    Tall,
}

impl BlockState for TuffWall {
    fn to_id(&self) -> i32 {
        if self.r#east == East::Tall && self.r#waterlogged == false && self.r#north == North::Tall && self.r#south == South::Low && self.r#up == true && self.r#west == West::Tall { return 23642; }
        if self.r#north == North::Low && self.r#up == false && self.r#west == West::None && self.r#south == South::None && self.r#east == East::None && self.r#waterlogged == true { return 23379; }
        if self.r#south == South::None && self.r#east == East::None && self.r#waterlogged == false && self.r#up == false && self.r#west == West::Tall && self.r#north == North::Tall { return 23420; }
        if self.r#waterlogged == true && self.r#west == West::None && self.r#up == false && self.r#south == South::Tall && self.r#east == East::Low && self.r#north == North::Low { return 23511; }
        if self.r#waterlogged == true && self.r#east == East::None && self.r#west == West::Low && self.r#up == false && self.r#south == South::Tall && self.r#north == North::Tall { return 23440; }
        if self.r#east == East::Low && self.r#south == South::Low && self.r#up == true && self.r#north == North::None && self.r#waterlogged == false && self.r#west == West::Tall { return 23462; }
        if self.r#north == North::Low && self.r#waterlogged == true && self.r#east == East::None && self.r#south == South::Low && self.r#up == false && self.r#west == West::Tall { return 23393; }
        if self.r#north == North::Low && self.r#east == East::Low && self.r#up == true && self.r#west == West::Low && self.r#waterlogged == false && self.r#south == South::Low { return 23497; }
        if self.r#west == West::Tall && self.r#waterlogged == false && self.r#up == true && self.r#east == East::Tall && self.r#south == South::Low && self.r#north == North::Low { return 23606; }
        if self.r#south == South::Low && self.r#up == false && self.r#west == West::Tall && self.r#east == East::Low && self.r#north == North::Tall && self.r#waterlogged == false { return 23540; }
        if self.r#south == South::Tall && self.r#up == true && self.r#north == North::None && self.r#waterlogged == false && self.r#west == West::Low && self.r#east == East::Low { return 23473; }
        if self.r#waterlogged == true && self.r#up == true && self.r#east == East::Low && self.r#west == West::Tall && self.r#north == North::None && self.r#south == South::None { return 23447; }
        if self.r#east == East::None && self.r#up == true && self.r#west == West::None && self.r#south == South::Tall && self.r#waterlogged == false && self.r#north == North::Low { return 23400; }
        if self.r#south == South::Low && self.r#up == false && self.r#west == West::None && self.r#waterlogged == false && self.r#east == East::Low && self.r#north == North::Low { return 23502; }
        if self.r#north == North::None && self.r#south == South::Tall && self.r#west == West::Low && self.r#east == East::Tall && self.r#up == true && self.r#waterlogged == false { return 23581; }
        if self.r#waterlogged == true && self.r#south == South::None && self.r#west == West::Tall && self.r#north == North::Low && self.r#east == East::None && self.r#up == true { return 23375; }
        if self.r#south == South::Tall && self.r#up == true && self.r#north == North::None && self.r#east == East::Low && self.r#waterlogged == true && self.r#west == West::None { return 23469; }
        if self.r#south == South::None && self.r#waterlogged == false && self.r#up == false && self.r#east == East::Tall && self.r#west == West::Low && self.r#north == North::Low { return 23599; }
        if self.r#west == West::None && self.r#waterlogged == false && self.r#north == North::None && self.r#south == South::None && self.r#up == false && self.r#east == East::Tall { return 23562; }
        if self.r#waterlogged == true && self.r#up == false && self.r#west == West::None && self.r#south == South::Tall && self.r#east == East::None && self.r#north == North::None { return 23367; }
        if self.r#south == South::Tall && self.r#north == North::Tall && self.r#waterlogged == true && self.r#east == East::None && self.r#up == false && self.r#west == West::None { return 23439; }
        if self.r#south == South::Tall && self.r#waterlogged == true && self.r#up == true && self.r#west == West::Low && self.r#north == North::Tall && self.r#east == East::Low { return 23542; }
        if self.r#south == South::Tall && self.r#west == West::Low && self.r#waterlogged == false && self.r#north == North::Low && self.r#up == false && self.r#east == East::Tall { return 23623; }
        if self.r#west == West::Tall && self.r#waterlogged == false && self.r#south == South::None && self.r#up == true && self.r#east == East::Tall && self.r#north == North::None { return 23558; }
        if self.r#up == false && self.r#north == North::None && self.r#east == East::Low && self.r#waterlogged == true && self.r#west == West::Tall && self.r#south == South::None { return 23453; }
        if self.r#south == South::Tall && self.r#waterlogged == false && self.r#up == false && self.r#west == West::Low && self.r#east == East::Low && self.r#north == North::Low { return 23515; }
        if self.r#up == false && self.r#north == North::Tall && self.r#south == South::None && self.r#waterlogged == false && self.r#east == East::None && self.r#west == West::None { return 23418; }
        if self.r#north == North::Tall && self.r#up == true && self.r#east == East::Low && self.r#south == South::None && self.r#waterlogged == false && self.r#west == West::None { return 23520; }
        if self.r#south == South::None && self.r#west == West::Low && self.r#east == East::Low && self.r#waterlogged == false && self.r#north == North::Tall && self.r#up == false { return 23527; }
        if self.r#up == false && self.r#east == East::Low && self.r#west == West::None && self.r#waterlogged == true && self.r#south == South::Low && self.r#north == North::Low { return 23499; }
        if self.r#east == East::Tall && self.r#waterlogged == true && self.r#west == West::Tall && self.r#up == false && self.r#north == North::Low && self.r#south == South::Tall { return 23621; }
        if self.r#north == North::None && self.r#east == East::None && self.r#up == true && self.r#waterlogged == true && self.r#south == South::Tall && self.r#west == West::None { return 23361; }
        if self.r#up == true && self.r#waterlogged == true && self.r#north == North::None && self.r#west == West::Low && self.r#south == South::Tall && self.r#east == East::Tall { return 23578; }
        if self.r#east == East::None && self.r#north == North::Tall && self.r#south == South::Tall && self.r#up == true && self.r#waterlogged == false && self.r#west == West::Tall { return 23438; }
        if self.r#north == North::Low && self.r#west == West::Low && self.r#east == East::None && self.r#waterlogged == true && self.r#south == South::Tall && self.r#up == false { return 23404; }
        if self.r#up == true && self.r#east == East::None && self.r#north == North::None && self.r#waterlogged == false && self.r#west == West::Tall && self.r#south == South::None { return 23342; }
        if self.r#north == North::Tall && self.r#east == East::Low && self.r#south == South::None && self.r#waterlogged == true && self.r#up == true && self.r#west == West::Tall { return 23519; }
        if self.r#west == West::Low && self.r#north == North::Tall && self.r#up == true && self.r#east == East::Low && self.r#south == South::Low && self.r#waterlogged == false { return 23533; }
        if self.r#north == North::Low && self.r#south == South::Low && self.r#waterlogged == true && self.r#up == true && self.r#west == West::Low && self.r#east == East::Low { return 23494; }
        if self.r#east == East::Low && self.r#south == South::Tall && self.r#waterlogged == false && self.r#west == West::None && self.r#north == North::None && self.r#up == false { return 23478; }
        if self.r#up == true && self.r#east == East::Low && self.r#south == South::Tall && self.r#north == North::Low && self.r#waterlogged == false && self.r#west == West::None { return 23508; }
        if self.r#up == false && self.r#east == East::Tall && self.r#north == North::Low && self.r#west == West::Tall && self.r#south == South::None && self.r#waterlogged == true { return 23597; }
        if self.r#east == East::Low && self.r#north == North::Low && self.r#west == West::Tall && self.r#waterlogged == false && self.r#south == South::Low && self.r#up == true { return 23498; }
        if self.r#west == West::Low && self.r#waterlogged == true && self.r#south == South::Low && self.r#north == North::Tall && self.r#up == true && self.r#east == East::None { return 23422; }
        if self.r#waterlogged == true && self.r#east == East::None && self.r#west == West::Low && self.r#up == true && self.r#south == South::Tall && self.r#north == North::Tall { return 23434; }
        if self.r#east == East::Low && self.r#north == North::Low && self.r#south == South::None && self.r#waterlogged == true && self.r#west == West::Low && self.r#up == false { return 23488; }
        if self.r#up == false && self.r#west == West::Low && self.r#east == East::None && self.r#north == North::Tall && self.r#south == South::Low && self.r#waterlogged == false { return 23431; }
        if self.r#west == West::None && self.r#north == North::None && self.r#east == East::Low && self.r#south == South::None && self.r#up == true && self.r#waterlogged == true { return 23445; }
        if self.r#up == false && self.r#north == North::Tall && self.r#waterlogged == true && self.r#west == West::Low && self.r#south == South::Tall && self.r#east == East::Low { return 23548; }
        if self.r#east == East::None && self.r#up == false && self.r#waterlogged == true && self.r#west == West::None && self.r#north == North::Tall && self.r#south == South::None { return 23415; }
        if self.r#up == true && self.r#north == North::None && self.r#east == East::Low && self.r#waterlogged == false && self.r#west == West::Low && self.r#south == South::None { return 23449; }
        if self.r#south == South::Low && self.r#west == West::None && self.r#east == East::Low && self.r#north == North::None && self.r#up == true && self.r#waterlogged == false { return 23460; }
        if self.r#west == West::None && self.r#south == South::None && self.r#north == North::None && self.r#east == East::Low && self.r#up == false && self.r#waterlogged == true { return 23451; }
        if self.r#south == South::None && self.r#up == false && self.r#waterlogged == false && self.r#north == North::None && self.r#east == East::None && self.r#west == West::None { return 23346; }
        if self.r#east == East::None && self.r#up == false && self.r#waterlogged == false && self.r#south == South::Tall && self.r#north == North::None && self.r#west == West::Tall { return 23372; }
        if self.r#up == true && self.r#waterlogged == false && self.r#south == South::None && self.r#west == West::Low && self.r#north == North::Low && self.r#east == East::None { return 23377; }
        if self.r#west == West::None && self.r#east == East::None && self.r#south == South::None && self.r#up == true && self.r#north == North::Tall && self.r#waterlogged == true { return 23409; }
        if self.r#north == North::Tall && self.r#west == West::None && self.r#east == East::Low && self.r#up == true && self.r#waterlogged == true && self.r#south == South::None { return 23517; }
        if self.r#north == North::Tall && self.r#up == false && self.r#waterlogged == true && self.r#east == East::Tall && self.r#south == South::Tall && self.r#west == West::None { return 23655; }
        if self.r#waterlogged == false && self.r#east == East::Low && self.r#north == North::Tall && self.r#south == South::Low && self.r#up == false && self.r#west == West::Low { return 23539; }
        if self.r#up == false && self.r#waterlogged == false && self.r#north == North::Tall && self.r#west == West::Tall && self.r#south == South::Tall && self.r#east == East::Low { return 23552; }
        if self.r#up == true && self.r#waterlogged == false && self.r#north == North::Tall && self.r#west == West::None && self.r#east == East::Tall && self.r#south == South::None { return 23628; }
        if self.r#east == East::Low && self.r#up == false && self.r#west == West::None && self.r#north == North::Tall && self.r#south == South::Tall && self.r#waterlogged == false { return 23550; }
        if self.r#waterlogged == false && self.r#west == West::None && self.r#south == South::Low && self.r#north == North::Tall && self.r#east == East::Low && self.r#up == true { return 23532; }
        if self.r#south == South::None && self.r#up == false && self.r#waterlogged == true && self.r#west == West::Tall && self.r#north == North::Tall && self.r#east == East::Tall { return 23633; }
        if self.r#up == true && self.r#north == North::Low && self.r#east == East::Tall && self.r#south == South::None && self.r#waterlogged == true && self.r#west == West::Tall { return 23591; }
        if self.r#south == South::Tall && self.r#north == North::Low && self.r#up == false && self.r#waterlogged == false && self.r#east == East::Tall && self.r#west == West::Tall { return 23624; }
        if self.r#north == North::Tall && self.r#waterlogged == true && self.r#south == South::None && self.r#up == true && self.r#west == West::None && self.r#east == East::Tall { return 23625; }
        if self.r#waterlogged == false && self.r#east == East::None && self.r#west == West::Low && self.r#south == South::Tall && self.r#north == North::Low && self.r#up == false { return 23407; }
        if self.r#west == West::Tall && self.r#waterlogged == false && self.r#south == South::Tall && self.r#north == North::None && self.r#east == East::None && self.r#up == true { return 23366; }
        if self.r#north == North::Low && self.r#east == East::Low && self.r#waterlogged == false && self.r#up == true && self.r#west == West::None && self.r#south == South::None { return 23484; }
        if self.r#east == East::Tall && self.r#north == North::None && self.r#south == South::Low && self.r#up == true && self.r#waterlogged == false && self.r#west == West::None { return 23568; }
        if self.r#waterlogged == true && self.r#west == West::Tall && self.r#up == false && self.r#south == South::Tall && self.r#north == North::None && self.r#east == East::Tall { return 23585; }
        if self.r#waterlogged == true && self.r#west == West::Tall && self.r#south == South::Tall && self.r#north == North::Low && self.r#east == East::Tall && self.r#up == true { return 23615; }
        if self.r#east == East::Tall && self.r#north == North::Tall && self.r#up == true && self.r#west == West::Tall && self.r#waterlogged == true && self.r#south == South::Tall { return 23651; }
        if self.r#north == North::Tall && self.r#east == East::Tall && self.r#south == South::Low && self.r#waterlogged == false && self.r#up == true && self.r#west == West::Low { return 23641; }
        if self.r#west == West::None && self.r#waterlogged == true && self.r#north == North::Low && self.r#up == true && self.r#east == East::None && self.r#south == South::Low { return 23385; }
        if self.r#north == North::None && self.r#west == West::Low && self.r#south == South::None && self.r#waterlogged == true && self.r#up == false && self.r#east == East::Low { return 23452; }
        if self.r#north == North::None && self.r#east == East::Tall && self.r#west == West::Low && self.r#south == South::None && self.r#waterlogged == false && self.r#up == false { return 23563; }
        if self.r#waterlogged == true && self.r#south == South::None && self.r#up == true && self.r#west == West::None && self.r#north == North::Low && self.r#east == East::Tall { return 23589; }
        if self.r#waterlogged == true && self.r#south == South::Low && self.r#west == West::Low && self.r#north == North::None && self.r#up == true && self.r#east == East::Tall { return 23566; }
        if self.r#east == East::None && self.r#south == South::Tall && self.r#up == true && self.r#north == North::Low && self.r#waterlogged == true && self.r#west == West::Tall { return 23399; }
        if self.r#north == North::Tall && self.r#waterlogged == false && self.r#west == West::None && self.r#up == true && self.r#east == East::Tall && self.r#south == South::Tall { return 23652; }
        if self.r#up == false && self.r#waterlogged == false && self.r#east == East::None && self.r#north == North::None && self.r#west == West::Low && self.r#south == South::Tall { return 23371; }
        if self.r#waterlogged == false && self.r#west == West::Tall && self.r#north == North::None && self.r#east == East::Tall && self.r#south == South::Low && self.r#up == true { return 23570; }
        if self.r#up == false && self.r#east == East::Low && self.r#south == South::Tall && self.r#waterlogged == false && self.r#west == West::Tall && self.r#north == North::Low { return 23516; }
        if self.r#west == West::Tall && self.r#up == true && self.r#east == East::Low && self.r#north == North::None && self.r#south == South::Low && self.r#waterlogged == true { return 23459; }
        if self.r#north == North::Low && self.r#waterlogged == false && self.r#east == East::None && self.r#south == South::Low && self.r#up == true && self.r#west == West::Tall { return 23390; }
        if self.r#waterlogged == true && self.r#east == East::None && self.r#south == South::None && self.r#west == West::Tall && self.r#north == North::None && self.r#up == false { return 23345; }
        if self.r#east == East::Tall && self.r#up == true && self.r#waterlogged == false && self.r#west == West::None && self.r#north == North::None && self.r#south == South::Tall { return 23580; }
        if self.r#up == true && self.r#waterlogged == true && self.r#west == West::Low && self.r#south == South::Tall && self.r#east == East::Tall && self.r#north == North::Tall { return 23650; }
        if self.r#south == South::None && self.r#waterlogged == false && self.r#west == West::Tall && self.r#north == North::Low && self.r#east == East::Low && self.r#up == true { return 23486; }
        if self.r#south == South::Low && self.r#waterlogged == false && self.r#east == East::Tall && self.r#north == North::None && self.r#up == false && self.r#west == West::Low { return 23575; }
        if self.r#north == North::Tall && self.r#up == false && self.r#waterlogged == false && self.r#east == East::Tall && self.r#west == West::Tall && self.r#south == South::Low { return 23648; }
        if self.r#south == South::Low && self.r#waterlogged == false && self.r#east == East::None && self.r#west == West::Tall && self.r#up == false && self.r#north == North::Low { return 23396; }
        if self.r#south == South::None && self.r#north == North::Low && self.r#east == East::None && self.r#up == false && self.r#waterlogged == false && self.r#west == West::None { return 23382; }
        if self.r#east == East::None && self.r#north == North::Tall && self.r#south == South::Low && self.r#up == true && self.r#west == West::None && self.r#waterlogged == false { return 23424; }
        if self.r#south == South::Tall && self.r#west == West::Low && self.r#north == North::None && self.r#east == East::Low && self.r#up == true && self.r#waterlogged == true { return 23470; }
        if self.r#south == South::None && self.r#east == East::Low && self.r#waterlogged == true && self.r#north == North::Low && self.r#west == West::Tall && self.r#up == false { return 23489; }
        if self.r#waterlogged == false && self.r#east == East::None && self.r#south == South::Tall && self.r#up == true && self.r#west == West::Low && self.r#north == North::Tall { return 23437; }
        if self.r#south == South::None && self.r#north == North::None && self.r#west == West::Tall && self.r#up == false && self.r#waterlogged == false && self.r#east == East::Low { return 23456; }
        if self.r#north == North::None && self.r#west == West::None && self.r#up == true && self.r#waterlogged == true && self.r#east == East::Tall && self.r#south == South::Tall { return 23577; }
        if self.r#waterlogged == true && self.r#north == North::Low && self.r#south == South::Low && self.r#up == true && self.r#east == East::Tall && self.r#west == West::Low { return 23602; }
        if self.r#south == South::Low && self.r#up == false && self.r#west == West::None && self.r#east == East::Tall && self.r#north == North::Low && self.r#waterlogged == false { return 23610; }
        if self.r#waterlogged == false && self.r#west == West::Low && self.r#up == false && self.r#north == North::None && self.r#south == South::None && self.r#east == East::None { return 23347; }
        if self.r#up == false && self.r#west == West::Tall && self.r#south == South::Low && self.r#north == North::None && self.r#waterlogged == true && self.r#east == East::Tall { return 23573; }
        if self.r#east == East::None && self.r#north == North::None && self.r#south == South::Low && self.r#up == false && self.r#waterlogged == true && self.r#west == West::Tall { return 23357; }
        if self.r#west == West::Tall && self.r#east == East::None && self.r#up == true && self.r#north == North::None && self.r#south == South::None && self.r#waterlogged == true { return 23339; }
        if self.r#up == true && self.r#north == North::None && self.r#west == West::Low && self.r#south == South::Low && self.r#waterlogged == true && self.r#east == East::Low { return 23458; }
        if self.r#south == South::Low && self.r#north == North::None && self.r#up == true && self.r#waterlogged == false && self.r#east == East::Low && self.r#west == West::Low { return 23461; }
        if self.r#up == false && self.r#west == West::None && self.r#south == South::Low && self.r#north == North::None && self.r#waterlogged == true && self.r#east == East::Low { return 23463; }
        if self.r#east == East::Tall && self.r#south == South::Low && self.r#north == North::Low && self.r#up == false && self.r#waterlogged == true && self.r#west == West::None { return 23607; }
        if self.r#west == West::None && self.r#east == East::Low && self.r#north == North::Low && self.r#up == true && self.r#waterlogged == true && self.r#south == South::Tall { return 23505; }
        if self.r#east == East::Low && self.r#north == North::Tall && self.r#south == South::None && self.r#west == West::Tall && self.r#up == false && self.r#waterlogged == false { return 23528; }
        if self.r#south == South::Tall && self.r#up == false && self.r#north == North::None && self.r#waterlogged == false && self.r#west == West::None && self.r#east == East::None { return 23370; }
        if self.r#north == North::Tall && self.r#south == South::None && self.r#waterlogged == true && self.r#up == true && self.r#east == East::None && self.r#west == West::Tall { return 23411; }
        if self.r#west == West::Low && self.r#up == true && self.r#east == East::None && self.r#south == South::None && self.r#waterlogged == false && self.r#north == North::None { return 23341; }
        if self.r#north == North::Tall && self.r#east == East::Low && self.r#south == South::None && self.r#west == West::None && self.r#up == false && self.r#waterlogged == true { return 23523; }
        if self.r#north == North::Low && self.r#up == true && self.r#waterlogged == false && self.r#west == West::None && self.r#east == East::None && self.r#south == South::None { return 23376; }
        if self.r#up == true && self.r#south == South::Low && self.r#east == East::None && self.r#waterlogged == false && self.r#west == West::Low && self.r#north == North::None { return 23353; }
        if self.r#south == South::Low && self.r#east == East::None && self.r#up == false && self.r#waterlogged == true && self.r#north == North::Tall && self.r#west == West::Tall { return 23429; }
        if self.r#east == East::Tall && self.r#north == North::None && self.r#up == true && self.r#west == West::Low && self.r#waterlogged == false && self.r#south == South::Low { return 23569; }
        if self.r#east == East::None && self.r#up == true && self.r#west == West::None && self.r#south == South::Tall && self.r#waterlogged == true && self.r#north == North::Low { return 23397; }
        if self.r#north == North::Low && self.r#waterlogged == true && self.r#south == South::None && self.r#east == East::Tall && self.r#west == West::Low && self.r#up == true { return 23590; }
        if self.r#east == East::Tall && self.r#south == South::Tall && self.r#waterlogged == true && self.r#west == West::None && self.r#north == North::Low && self.r#up == true { return 23613; }
        if self.r#up == true && self.r#north == North::Tall && self.r#south == South::Low && self.r#east == East::Tall && self.r#waterlogged == true && self.r#west == West::None { return 23637; }
        if self.r#up == true && self.r#west == West::Tall && self.r#north == North::Low && self.r#east == East::Tall && self.r#waterlogged == false && self.r#south == South::Tall { return 23618; }
        if self.r#east == East::Low && self.r#north == North::Low && self.r#south == South::Low && self.r#waterlogged == true && self.r#up == false && self.r#west == West::Tall { return 23501; }
        if self.r#south == South::Tall && self.r#west == West::None && self.r#up == false && self.r#waterlogged == true && self.r#north == North::Tall && self.r#east == East::Low { return 23547; }
        if self.r#south == South::Tall && self.r#east == East::None && self.r#north == North::Tall && self.r#waterlogged == true && self.r#up == true && self.r#west == West::Tall { return 23435; }
        if self.r#north == North::Tall && self.r#east == East::None && self.r#west == West::Tall && self.r#south == South::Tall && self.r#waterlogged == true && self.r#up == false { return 23441; }
        if self.r#east == East::Low && self.r#west == West::Low && self.r#waterlogged == false && self.r#north == North::Low && self.r#south == South::Tall && self.r#up == true { return 23509; }
        if self.r#south == South::Tall && self.r#up == true && self.r#north == North::Tall && self.r#east == East::Low && self.r#waterlogged == true && self.r#west == West::Tall { return 23543; }
        if self.r#south == South::Tall && self.r#north == North::None && self.r#up == false && self.r#waterlogged == false && self.r#east == East::Tall && self.r#west == West::Low { return 23587; }
        if self.r#south == South::None && self.r#north == North::Low && self.r#east == East::Tall && self.r#up == true && self.r#waterlogged == false && self.r#west == West::Low { return 23593; }
        if self.r#east == East::Tall && self.r#west == West::None && self.r#up == true && self.r#south == South::Low && self.r#north == North::Tall && self.r#waterlogged == false { return 23640; }
        if self.r#waterlogged == false && self.r#east == East::Tall && self.r#south == South::None && self.r#west == West::None && self.r#up == false && self.r#north == North::Low { return 23598; }
        if self.r#waterlogged == false && self.r#east == East::Low && self.r#up == false && self.r#south == South::Tall && self.r#west == West::Low && self.r#north == North::Tall { return 23551; }
        if self.r#west == West::Low && self.r#up == false && self.r#south == South::Low && self.r#north == North::None && self.r#waterlogged == false && self.r#east == East::Low { return 23467; }
        if self.r#up == true && self.r#waterlogged == false && self.r#west == West::None && self.r#east == East::Low && self.r#north == North::Tall && self.r#south == South::Tall { return 23544; }
        if self.r#east == East::None && self.r#north == North::Low && self.r#south == South::Low && self.r#waterlogged == false && self.r#up == true && self.r#west == West::Low { return 23389; }
        if self.r#up == true && self.r#west == West::Tall && self.r#waterlogged == true && self.r#east == East::None && self.r#south == South::Low && self.r#north == North::None { return 23351; }
        if self.r#up == true && self.r#waterlogged == true && self.r#west == West::Tall && self.r#east == East::Low && self.r#south == South::Low && self.r#north == North::Low { return 23495; }
        if self.r#up == true && self.r#west == West::None && self.r#east == East::Low && self.r#waterlogged == false && self.r#north == North::Low && self.r#south == South::Low { return 23496; }
        if self.r#west == West::Low && self.r#waterlogged == true && self.r#north == North::Low && self.r#east == East::Low && self.r#south == South::Low && self.r#up == false { return 23500; }
        if self.r#up == false && self.r#south == South::None && self.r#waterlogged == true && self.r#west == West::Tall && self.r#east == East::Tall && self.r#north == North::None { return 23561; }
        if self.r#waterlogged == true && self.r#east == East::None && self.r#up == true && self.r#west == West::None && self.r#south == South::Low && self.r#north == North::Tall { return 23421; }
        if self.r#north == North::None && self.r#east == East::Low && self.r#south == South::Tall && self.r#up == false && self.r#waterlogged == true && self.r#west == West::None { return 23475; }
        if self.r#west == West::Tall && self.r#south == South::Low && self.r#waterlogged == false && self.r#up == true && self.r#east == East::Low && self.r#north == North::Tall { return 23534; }
        if self.r#up == true && self.r#waterlogged == true && self.r#west == West::Tall && self.r#north == North::Low && self.r#east == East::None && self.r#south == South::Low { return 23387; }
        if self.r#up == true && self.r#waterlogged == true && self.r#south == South::None && self.r#west == West::Tall && self.r#east == East::Low && self.r#north == North::Low { return 23483; }
        if self.r#south == South::Low && self.r#up == true && self.r#waterlogged == false && self.r#east == East::None && self.r#west == West::None && self.r#north == North::Low { return 23388; }
        if self.r#north == North::None && self.r#waterlogged == true && self.r#east == East::Tall && self.r#south == South::None && self.r#up == true && self.r#west == West::None { return 23553; }
        if self.r#west == West::Tall && self.r#north == North::Tall && self.r#east == East::Tall && self.r#waterlogged == false && self.r#south == South::Tall && self.r#up == true { return 23654; }
        if self.r#up == true && self.r#waterlogged == true && self.r#east == East::None && self.r#south == South::Tall && self.r#north == North::None && self.r#west == West::Tall { return 23363; }
        if self.r#west == West::None && self.r#up == true && self.r#north == North::Low && self.r#south == South::None && self.r#waterlogged == true && self.r#east == East::Low { return 23481; }
        if self.r#waterlogged == false && self.r#north == North::Tall && self.r#south == South::None && self.r#east == East::Low && self.r#up == false && self.r#west == West::None { return 23526; }
        if self.r#north == North::Low && self.r#east == East::Tall && self.r#south == South::Low && self.r#waterlogged == false && self.r#west == West::Tall && self.r#up == false { return 23612; }
        if self.r#south == South::Tall && self.r#west == West::Tall && self.r#east == East::None && self.r#north == North::Low && self.r#waterlogged == false && self.r#up == false { return 23408; }
        if self.r#up == true && self.r#west == West::Low && self.r#north == North::Tall && self.r#waterlogged == false && self.r#south == South::None && self.r#east == East::Low { return 23521; }
        if self.r#north == North::None && self.r#waterlogged == false && self.r#south == South::None && self.r#up == false && self.r#west == West::Tall && self.r#east == East::Tall { return 23564; }
        if self.r#east == East::Tall && self.r#waterlogged == true && self.r#north == North::Low && self.r#west == West::Low && self.r#south == South::Tall && self.r#up == true { return 23614; }
        if self.r#east == East::None && self.r#south == South::None && self.r#west == West::Low && self.r#waterlogged == false && self.r#north == North::Tall && self.r#up == false { return 23419; }
        if self.r#up == false && self.r#west == West::Low && self.r#south == South::Low && self.r#north == North::Low && self.r#waterlogged == false && self.r#east == East::Low { return 23503; }
        if self.r#west == West::None && self.r#east == East::None && self.r#south == South::Tall && self.r#north == North::Tall && self.r#waterlogged == true && self.r#up == true { return 23433; }
        if self.r#east == East::Tall && self.r#up == true && self.r#south == South::None && self.r#west == West::None && self.r#waterlogged == false && self.r#north == North::None { return 23556; }
        if self.r#west == West::Tall && self.r#north == North::Tall && self.r#waterlogged == true && self.r#east == East::Low && self.r#south == South::Tall && self.r#up == false { return 23549; }
        if self.r#west == West::Low && self.r#east == East::Low && self.r#north == North::None && self.r#south == South::Tall && self.r#up == false && self.r#waterlogged == true { return 23476; }
        if self.r#north == North::None && self.r#east == East::None && self.r#up == false && self.r#west == West::Low && self.r#south == South::Tall && self.r#waterlogged == true { return 23368; }
        if self.r#west == West::None && self.r#east == East::Low && self.r#waterlogged == true && self.r#up == true && self.r#north == North::None && self.r#south == South::Low { return 23457; }
        if self.r#west == West::Tall && self.r#north == North::None && self.r#waterlogged == false && self.r#east == East::Tall && self.r#south == South::Tall && self.r#up == false { return 23588; }
        if self.r#east == East::Tall && self.r#up == false && self.r#west == West::Low && self.r#south == South::Tall && self.r#waterlogged == true && self.r#north == North::None { return 23584; }
        if self.r#south == South::Low && self.r#waterlogged == false && self.r#west == West::None && self.r#north == North::None && self.r#up == true && self.r#east == East::None { return 23352; }
        if self.r#east == East::Tall && self.r#south == South::None && self.r#up == false && self.r#west == West::Tall && self.r#waterlogged == false && self.r#north == North::Tall { return 23636; }
        if self.r#south == South::Low && self.r#up == true && self.r#north == North::Tall && self.r#waterlogged == true && self.r#west == West::Tall && self.r#east == East::Low { return 23531; }
        if self.r#waterlogged == true && self.r#west == West::None && self.r#up == false && self.r#east == East::None && self.r#south == South::Low && self.r#north == North::Tall { return 23427; }
        if self.r#waterlogged == false && self.r#east == East::Low && self.r#south == South::Low && self.r#north == North::Tall && self.r#up == false && self.r#west == West::None { return 23538; }
        if self.r#south == South::Low && self.r#east == East::None && self.r#up == true && self.r#waterlogged == true && self.r#west == West::Low && self.r#north == North::None { return 23350; }
        if self.r#waterlogged == true && self.r#west == West::Low && self.r#south == South::None && self.r#up == true && self.r#east == East::None && self.r#north == North::Low { return 23374; }
        if self.r#up == true && self.r#waterlogged == false && self.r#south == South::Tall && self.r#east == East::Low && self.r#west == West::None && self.r#north == North::None { return 23472; }
        if self.r#up == false && self.r#north == North::Low && self.r#east == East::Low && self.r#waterlogged == false && self.r#south == South::None && self.r#west == West::Low { return 23491; }
        if self.r#east == East::Tall && self.r#north == North::None && self.r#south == South::None && self.r#waterlogged == true && self.r#west == West::Low && self.r#up == false { return 23560; }
        if self.r#south == South::Tall && self.r#north == North::Tall && self.r#west == West::None && self.r#waterlogged == true && self.r#up == true && self.r#east == East::Low { return 23541; }
        if self.r#south == South::Tall && self.r#up == true && self.r#north == North::Low && self.r#west == West::Low && self.r#waterlogged == true && self.r#east == East::Low { return 23506; }
        if self.r#up == true && self.r#waterlogged == true && self.r#west == West::Low && self.r#north == North::None && self.r#south == South::None && self.r#east == East::Tall { return 23554; }
        if self.r#west == West::Low && self.r#east == East::Low && self.r#up == true && self.r#south == South::None && self.r#north == North::Low && self.r#waterlogged == true { return 23482; }
        if self.r#west == West::None && self.r#east == East::Tall && self.r#north == North::Low && self.r#south == South::None && self.r#up == false && self.r#waterlogged == true { return 23595; }
        if self.r#west == West::Tall && self.r#up == false && self.r#south == South::None && self.r#north == North::Low && self.r#waterlogged == false && self.r#east == East::Tall { return 23600; }
        if self.r#south == South::Low && self.r#west == West::Tall && self.r#waterlogged == false && self.r#up == false && self.r#east == East::None && self.r#north == North::Tall { return 23432; }
        if self.r#north == North::Tall && self.r#waterlogged == true && self.r#west == West::Low && self.r#up == false && self.r#south == South::None && self.r#east == East::Low { return 23524; }
        if self.r#south == South::Low && self.r#east == East::Tall && self.r#up == true && self.r#waterlogged == true && self.r#west == West::Low && self.r#north == North::Tall { return 23638; }
        if self.r#west == West::Tall && self.r#waterlogged == true && self.r#north == North::Tall && self.r#east == East::Low && self.r#south == South::None && self.r#up == false { return 23525; }
        if self.r#up == false && self.r#east == East::None && self.r#waterlogged == false && self.r#south == South::Low && self.r#west == West::Tall && self.r#north == North::None { return 23360; }
        if self.r#west == West::Tall && self.r#east == East::Low && self.r#waterlogged == true && self.r#north == North::Tall && self.r#south == South::Low && self.r#up == false { return 23537; }
        if self.r#north == North::None && self.r#south == South::None && self.r#west == West::Low && self.r#east == East::Tall && self.r#waterlogged == false && self.r#up == true { return 23557; }
        if self.r#east == East::Tall && self.r#north == North::Tall && self.r#up == false && self.r#waterlogged == true && self.r#west == West::Low && self.r#south == South::None { return 23632; }
        if self.r#east == East::Low && self.r#waterlogged == true && self.r#west == West::Tall && self.r#up == true && self.r#north == North::Low && self.r#south == South::Tall { return 23507; }
        if self.r#up == true && self.r#north == North::Tall && self.r#south == South::None && self.r#west == West::Low && self.r#waterlogged == false && self.r#east == East::None { return 23413; }
        if self.r#waterlogged == false && self.r#east == East::None && self.r#up == true && self.r#west == West::Tall && self.r#south == South::Low && self.r#north == North::Tall { return 23426; }
        if self.r#up == true && self.r#south == South::Low && self.r#north == North::None && self.r#waterlogged == true && self.r#west == West::None && self.r#east == East::None { return 23349; }
        if self.r#east == East::None && self.r#north == North::Low && self.r#waterlogged == true && self.r#west == West::Low && self.r#south == South::None && self.r#up == false { return 23380; }
        if self.r#up == true && self.r#waterlogged == false && self.r#south == South::Tall && self.r#west == West::Low && self.r#north == North::None && self.r#east == East::None { return 23365; }
        if self.r#up == true && self.r#waterlogged == false && self.r#west == West::Low && self.r#east == East::Low && self.r#north == North::Low && self.r#south == South::None { return 23485; }
        if self.r#up == false && self.r#south == South::Low && self.r#north == North::None && self.r#west == West::None && self.r#waterlogged == true && self.r#east == East::Tall { return 23571; }
        if self.r#up == true && self.r#north == North::Tall && self.r#south == South::None && self.r#west == West::Tall && self.r#east == East::Tall && self.r#waterlogged == false { return 23630; }
        if self.r#north == North::Low && self.r#up == true && self.r#west == West::Low && self.r#waterlogged == true && self.r#east == East::None && self.r#south == South::Tall { return 23398; }
        if self.r#south == South::None && self.r#east == East::Tall && self.r#up == true && self.r#waterlogged == false && self.r#north == North::Tall && self.r#west == West::Low { return 23629; }
        if self.r#west == West::Tall && self.r#south == South::Tall && self.r#north == North::Tall && self.r#east == East::Tall && self.r#up == false && self.r#waterlogged == true { return 23657; }
        if self.r#south == South::Low && self.r#up == true && self.r#waterlogged == true && self.r#west == West::None && self.r#north == North::Low && self.r#east == East::Tall { return 23601; }
        if self.r#east == East::None && self.r#west == West::Low && self.r#up == false && self.r#south == South::None && self.r#waterlogged == true && self.r#north == North::None { return 23344; }
        if self.r#up == true && self.r#north == North::None && self.r#south == South::Low && self.r#waterlogged == false && self.r#east == East::None && self.r#west == West::Tall { return 23354; }
        if self.r#up == false && self.r#waterlogged == true && self.r#north == North::Tall && self.r#south == South::Low && self.r#west == West::Low && self.r#east == East::None { return 23428; }
        if self.r#up == false && self.r#north == North::Tall && self.r#south == South::Low && self.r#waterlogged == true && self.r#east == East::Tall && self.r#west == West::Low { return 23644; }
        if self.r#up == false && self.r#waterlogged == false && self.r#south == South::Low && self.r#west == West::Low && self.r#north == North::Low && self.r#east == East::None { return 23395; }
        if self.r#north == North::None && self.r#east == East::Tall && self.r#south == South::Low && self.r#up == false && self.r#waterlogged == true && self.r#west == West::Low { return 23572; }
        if self.r#south == South::Tall && self.r#east == East::None && self.r#west == West::None && self.r#waterlogged == true && self.r#north == North::Low && self.r#up == false { return 23403; }
        if self.r#east == East::None && self.r#south == South::None && self.r#north == North::Tall && self.r#up == true && self.r#waterlogged == true && self.r#west == West::Low { return 23410; }
        if self.r#south == South::Tall && self.r#east == East::Tall && self.r#up == false && self.r#west == West::Low && self.r#north == North::Tall && self.r#waterlogged == false { return 23659; }
        if self.r#east == East::None && self.r#up == false && self.r#west == West::Low && self.r#north == North::Tall && self.r#south == South::Tall && self.r#waterlogged == false { return 23443; }
        if self.r#south == South::None && self.r#waterlogged == false && self.r#east == East::None && self.r#up == false && self.r#west == West::Tall && self.r#north == North::Low { return 23384; }
        if self.r#east == East::Tall && self.r#up == true && self.r#north == North::Tall && self.r#waterlogged == true && self.r#south == South::None && self.r#west == West::Low { return 23626; }
        if self.r#north == North::Tall && self.r#up == true && self.r#east == East::Low && self.r#south == South::None && self.r#waterlogged == false && self.r#west == West::Tall { return 23522; }
        if self.r#north == North::Tall && self.r#east == East::Low && self.r#waterlogged == false && self.r#west == West::Low && self.r#up == true && self.r#south == South::Tall { return 23545; }
        if self.r#up == false && self.r#waterlogged == true && self.r#south == South::Low && self.r#east == East::Low && self.r#north == North::Tall && self.r#west == West::Low { return 23536; }
        if self.r#east == East::None && self.r#waterlogged == true && self.r#west == West::Tall && self.r#south == South::None && self.r#north == North::Low && self.r#up == false { return 23381; }
        if self.r#west == West::None && self.r#north == North::None && self.r#south == South::None && self.r#waterlogged == false && self.r#east == East::Low && self.r#up == true { return 23448; }
        if self.r#west == West::Tall && self.r#up == false && self.r#north == North::None && self.r#east == East::Low && self.r#south == South::Low && self.r#waterlogged == false { return 23468; }
        if self.r#north == North::None && self.r#east == East::Low && self.r#waterlogged == true && self.r#up == false && self.r#west == West::Tall && self.r#south == South::Tall { return 23477; }
        if self.r#south == South::Low && self.r#waterlogged == true && self.r#up == true && self.r#west == West::Tall && self.r#north == North::Tall && self.r#east == East::Tall { return 23639; }
        if self.r#north == North::Tall && self.r#up == true && self.r#south == South::None && self.r#west == West::Tall && self.r#east == East::None && self.r#waterlogged == false { return 23414; }
        if self.r#up == false && self.r#north == North::None && self.r#waterlogged == true && self.r#west == West::Tall && self.r#south == South::Low && self.r#east == East::Low { return 23465; }
        if self.r#waterlogged == false && self.r#east == East::Low && self.r#south == South::Tall && self.r#west == West::Tall && self.r#north == North::Low && self.r#up == true { return 23510; }
        if self.r#west == West::Low && self.r#south == South::Tall && self.r#east == East::Tall && self.r#north == North::Tall && self.r#up == false && self.r#waterlogged == true { return 23656; }
        if self.r#north == North::None && self.r#east == East::None && self.r#south == South::Tall && self.r#west == West::None && self.r#up == true && self.r#waterlogged == false { return 23364; }
        if self.r#east == East::Tall && self.r#up == false && self.r#north == North::Low && self.r#south == South::Low && self.r#waterlogged == true && self.r#west == West::Tall { return 23609; }
        if self.r#east == East::Low && self.r#waterlogged == false && self.r#up == true && self.r#north == North::None && self.r#west == West::Tall && self.r#south == South::None { return 23450; }
        if self.r#east == East::Tall && self.r#waterlogged == false && self.r#north == North::Low && self.r#south == South::Tall && self.r#up == true && self.r#west == West::None { return 23616; }
        if self.r#west == West::None && self.r#up == false && self.r#east == East::Tall && self.r#north == North::None && self.r#waterlogged == false && self.r#south == South::Low { return 23574; }
        if self.r#north == North::Tall && self.r#west == West::Low && self.r#up == false && self.r#east == East::Tall && self.r#south == South::Low && self.r#waterlogged == false { return 23647; }
        if self.r#west == West::None && self.r#up == false && self.r#north == North::Tall && self.r#east == East::None && self.r#south == South::Low && self.r#waterlogged == false { return 23430; }
        if self.r#up == false && self.r#waterlogged == true && self.r#east == East::Tall && self.r#west == West::None && self.r#north == North::Low && self.r#south == South::Tall { return 23619; }
        if self.r#waterlogged == false && self.r#west == West::Tall && self.r#north == North::Low && self.r#south == South::None && self.r#east == East::Tall && self.r#up == true { return 23594; }
        if self.r#waterlogged == true && self.r#east == East::None && self.r#north == North::None && self.r#west == West::Low && self.r#south == South::Low && self.r#up == false { return 23356; }
        if self.r#up == false && self.r#waterlogged == false && self.r#north == North::None && self.r#south == South::Tall && self.r#east == East::Low && self.r#west == West::Tall { return 23480; }
        if self.r#east == East::None && self.r#north == North::None && self.r#south == South::Low && self.r#waterlogged == true && self.r#up == false && self.r#west == West::None { return 23355; }
        if self.r#east == East::Tall && self.r#west == West::Low && self.r#north == North::Low && self.r#south == South::Low && self.r#up == false && self.r#waterlogged == true { return 23608; }
        if self.r#waterlogged == false && self.r#west == West::Low && self.r#east == East::None && self.r#north == North::Low && self.r#south == South::Tall && self.r#up == true { return 23401; }
        if self.r#up == false && self.r#south == South::Low && self.r#west == West::Tall && self.r#north == North::None && self.r#east == East::Tall && self.r#waterlogged == false { return 23576; }
        if self.r#up == false && self.r#east == East::Tall && self.r#waterlogged == true && self.r#west == West::None && self.r#south == South::Tall && self.r#north == North::None { return 23583; }
        if self.r#south == South::None && self.r#east == East::None && self.r#up == true && self.r#waterlogged == false && self.r#north == North::Low && self.r#west == West::Tall { return 23378; }
        if self.r#east == East::None && self.r#north == North::Low && self.r#waterlogged == true && self.r#south == South::Low && self.r#west == West::Low && self.r#up == true { return 23386; }
        if self.r#north == North::None && self.r#up == true && self.r#west == West::Tall && self.r#east == East::Low && self.r#south == South::Tall && self.r#waterlogged == true { return 23471; }
        if self.r#east == East::Low && self.r#up == true && self.r#north == North::None && self.r#south == South::Tall && self.r#waterlogged == false && self.r#west == West::Tall { return 23474; }
        if self.r#waterlogged == true && self.r#south == South::None && self.r#up == true && self.r#west == West::Tall && self.r#north == North::Tall && self.r#east == East::Tall { return 23627; }
        if self.r#north == North::Low && self.r#up == true && self.r#waterlogged == false && self.r#west == West::None && self.r#east == East::Tall && self.r#south == South::None { return 23592; }
        if self.r#south == South::None && self.r#east == East::Low && self.r#north == North::Low && self.r#up == false && self.r#waterlogged == false && self.r#west == West::None { return 23490; }
        if self.r#west == West::Low && self.r#north == North::Low && self.r#up == true && self.r#east == East::Tall && self.r#waterlogged == false && self.r#south == South::Tall { return 23617; }
        if self.r#west == West::None && self.r#south == South::Tall && self.r#east == East::Low && self.r#north == North::Low && self.r#waterlogged == false && self.r#up == false { return 23514; }
        if self.r#east == East::Tall && self.r#waterlogged == false && self.r#north == North::Tall && self.r#west == West::None && self.r#south == South::None && self.r#up == false { return 23634; }
        if self.r#north == North::None && self.r#waterlogged == true && self.r#south == South::Tall && self.r#up == true && self.r#east == East::None && self.r#west == West::Low { return 23362; }
        if self.r#up == false && self.r#west == West::Low && self.r#waterlogged == false && self.r#north == North::Tall && self.r#south == South::None && self.r#east == East::Tall { return 23635; }
        if self.r#east == East::None && self.r#north == North::None && self.r#south == South::None && self.r#waterlogged == false && self.r#up == true && self.r#west == West::None { return 23340; }
        if self.r#east == East::Low && self.r#south == South::Low && self.r#west == West::Low && self.r#up == false && self.r#north == North::None && self.r#waterlogged == true { return 23464; }
        if self.r#up == true && self.r#west == West::None && self.r#waterlogged == true && self.r#east == East::Tall && self.r#north == North::None && self.r#south == South::Low { return 23565; }
        if self.r#east == East::Tall && self.r#north == North::Low && self.r#up == false && self.r#waterlogged == true && self.r#south == South::Tall && self.r#west == West::Low { return 23620; }
        if self.r#up == false && self.r#waterlogged == true && self.r#west == West::Tall && self.r#north == North::None && self.r#south == South::Tall && self.r#east == East::None { return 23369; }
        if self.r#west == West::None && self.r#south == South::Tall && self.r#north == North::None && self.r#east == East::Tall && self.r#up == false && self.r#waterlogged == false { return 23586; }
        if self.r#east == East::Tall && self.r#north == North::Tall && self.r#waterlogged == false && self.r#south == South::Low && self.r#up == false && self.r#west == West::None { return 23646; }
        if self.r#east == East::Low && self.r#north == North::None && self.r#south == South::Low && self.r#waterlogged == false && self.r#west == West::None && self.r#up == false { return 23466; }
        if self.r#south == South::None && self.r#east == East::Tall && self.r#up == false && self.r#waterlogged == true && self.r#west == West::None && self.r#north == North::None { return 23559; }
        if self.r#west == West::None && self.r#north == North::Tall && self.r#south == South::Tall && self.r#waterlogged == false && self.r#up == true && self.r#east == East::None { return 23436; }
        if self.r#north == North::Tall && self.r#east == East::Low && self.r#south == South::Tall && self.r#up == true && self.r#waterlogged == false && self.r#west == West::Tall { return 23546; }
        if self.r#north == North::Tall && self.r#up == false && self.r#west == West::None && self.r#waterlogged == true && self.r#east == East::Tall && self.r#south == South::None { return 23631; }
        if self.r#north == North::Tall && self.r#up == true && self.r#west == West::Low && self.r#south == South::None && self.r#waterlogged == true && self.r#east == East::Low { return 23518; }
        if self.r#north == North::None && self.r#south == South::None && self.r#up == true && self.r#east == East::Tall && self.r#waterlogged == true && self.r#west == West::Tall { return 23555; }
        if self.r#east == East::Low && self.r#south == South::None && self.r#up == true && self.r#waterlogged == true && self.r#west == West::Low && self.r#north == North::None { return 23446; }
        if self.r#south == South::Tall && self.r#north == North::Tall && self.r#west == West::Tall && self.r#east == East::None && self.r#up == false && self.r#waterlogged == false { return 23444; }
        if self.r#north == North::Tall && self.r#up == true && self.r#waterlogged == true && self.r#west == West::None && self.r#east == East::Low && self.r#south == South::Low { return 23529; }
        if self.r#west == West::Tall && self.r#north == North::Low && self.r#south == South::Low && self.r#east == East::Low && self.r#up == false && self.r#waterlogged == false { return 23504; }
        if self.r#west == West::Tall && self.r#up == true && self.r#east == East::Tall && self.r#south == South::Tall && self.r#waterlogged == false && self.r#north == North::None { return 23582; }
        if self.r#north == North::Tall && self.r#east == East::Tall && self.r#south == South::Tall && self.r#up == true && self.r#waterlogged == true && self.r#west == West::None { return 23649; }
        if self.r#east == East::Tall && self.r#south == South::Low && self.r#waterlogged == false && self.r#west == West::None && self.r#north == North::Low && self.r#up == true { return 23604; }
        if self.r#north == North::None && self.r#south == South::Low && self.r#east == East::Tall && self.r#waterlogged == true && self.r#up == true && self.r#west == West::Tall { return 23567; }
        if self.r#waterlogged == false && self.r#north == North::Low && self.r#up == false && self.r#east == East::Tall && self.r#south == South::Tall && self.r#west == West::None { return 23622; }
        if self.r#waterlogged == true && self.r#east == East::None && self.r#north == North::Tall && self.r#south == South::Low && self.r#up == true && self.r#west == West::Tall { return 23423; }
        if self.r#up == false && self.r#waterlogged == false && self.r#west == West::None && self.r#east == East::None && self.r#north == North::Tall && self.r#south == South::Tall { return 23442; }
        if self.r#east == East::Tall && self.r#west == West::Low && self.r#south == South::Low && self.r#waterlogged == false && self.r#north == North::Low && self.r#up == false { return 23611; }
        if self.r#east == East::None && self.r#waterlogged == true && self.r#north == North::Low && self.r#west == West::Low && self.r#up == false && self.r#south == South::Low { return 23392; }
        if self.r#north == North::Low && self.r#up == false && self.r#east == East::Low && self.r#west == West::None && self.r#south == South::None && self.r#waterlogged == true { return 23487; }
        if self.r#south == South::Low && self.r#north == North::Tall && self.r#waterlogged == true && self.r#west == West::None && self.r#east == East::Low && self.r#up == false { return 23535; }
        if self.r#west == West::Tall && self.r#up == true && self.r#south == South::Tall && self.r#east == East::Tall && self.r#north == North::None && self.r#waterlogged == true { return 23579; }
        if self.r#north == North::Tall && self.r#east == East::None && self.r#waterlogged == false && self.r#south == South::None && self.r#up == true && self.r#west == West::None { return 23412; }
        if self.r#north == North::Tall && self.r#up == true && self.r#waterlogged == false && self.r#west == West::Low && self.r#east == East::None && self.r#south == South::Low { return 23425; }
        if self.r#east == East::Low && self.r#south == South::Low && self.r#up == true && self.r#waterlogged == true && self.r#north == North::Tall && self.r#west == West::Low { return 23530; }
        if self.r#north == North::None && self.r#south == South::None && self.r#east == East::Low && self.r#up == false && self.r#west == West::Low && self.r#waterlogged == false { return 23455; }
        if self.r#east == East::Tall && self.r#west == West::None && self.r#waterlogged == true && self.r#south == South::Low && self.r#up == false && self.r#north == North::Tall { return 23643; }
        if self.r#south == South::None && self.r#up == false && self.r#waterlogged == true && self.r#east == East::Tall && self.r#north == North::Low && self.r#west == West::Low { return 23596; }
        if self.r#east == East::None && self.r#up == true && self.r#waterlogged == true && self.r#west == West::None && self.r#south == South::None && self.r#north == North::None { return 23337; }
        if self.r#north == North::Low && self.r#east == East::None && self.r#up == false && self.r#south == South::Low && self.r#waterlogged == false && self.r#west == West::None { return 23394; }
        if self.r#west == West::Tall && self.r#south == South::Tall && self.r#waterlogged == true && self.r#east == East::Low && self.r#north == North::Low && self.r#up == false { return 23513; }
        if self.r#east == East::None && self.r#north == North::None && self.r#up == false && self.r#south == South::None && self.r#waterlogged == false && self.r#west == West::Tall { return 23348; }
        if self.r#east == East::None && self.r#south == South::None && self.r#north == North::Low && self.r#up == true && self.r#waterlogged == true && self.r#west == West::None { return 23373; }
        if self.r#west == West::Tall && self.r#south == South::Low && self.r#north == North::Low && self.r#east == East::Tall && self.r#up == true && self.r#waterlogged == true { return 23603; }
        if self.r#waterlogged == false && self.r#west == West::Low && self.r#south == South::Low && self.r#east == East::None && self.r#north == North::None && self.r#up == false { return 23359; }
        if self.r#north == North::Tall && self.r#south == South::None && self.r#east == East::None && self.r#west == West::Low && self.r#up == false && self.r#waterlogged == true { return 23416; }
        if self.r#waterlogged == true && self.r#north == North::None && self.r#up == false && self.r#south == South::None && self.r#west == West::None && self.r#east == East::None { return 23343; }
        if self.r#up == false && self.r#waterlogged == true && self.r#west == West::None && self.r#south == South::Low && self.r#north == North::Low && self.r#east == East::None { return 23391; }
        if self.r#east == East::Tall && self.r#south == South::Low && self.r#up == true && self.r#west == West::Low && self.r#north == North::Low && self.r#waterlogged == false { return 23605; }
        if self.r#south == South::None && self.r#north == North::Low && self.r#waterlogged == false && self.r#west == West::Tall && self.r#east == East::Low && self.r#up == false { return 23492; }
        if self.r#west == West::Tall && self.r#up == false && self.r#east == East::Tall && self.r#waterlogged == true && self.r#north == North::Tall && self.r#south == South::Low { return 23645; }
        if self.r#south == South::None && self.r#north == North::Tall && self.r#waterlogged == true && self.r#east == East::None && self.r#up == false && self.r#west == West::Tall { return 23417; }
        if self.r#west == West::Tall && self.r#north == North::Low && self.r#waterlogged == true && self.r#east == East::None && self.r#south == South::Tall && self.r#up == false { return 23405; }
        if self.r#up == false && self.r#waterlogged == false && self.r#east == East::Low && self.r#north == North::None && self.r#south == South::None && self.r#west == West::None { return 23454; }
        if self.r#up == true && self.r#west == West::Low && self.r#waterlogged == false && self.r#north == North::Tall && self.r#east == East::Tall && self.r#south == South::Tall { return 23653; }
        if self.r#east == East::Tall && self.r#waterlogged == false && self.r#north == North::Tall && self.r#south == South::Tall && self.r#up == false && self.r#west == West::Tall { return 23660; }
        if self.r#up == false && self.r#waterlogged == false && self.r#north == North::None && self.r#west == West::Low && self.r#south == South::Tall && self.r#east == East::Low { return 23479; }
        if self.r#up == false && self.r#west == West::Low && self.r#east == East::Low && self.r#south == South::Tall && self.r#waterlogged == true && self.r#north == North::Low { return 23512; }
        if self.r#waterlogged == false && self.r#south == South::Tall && self.r#east == East::None && self.r#north == North::Low && self.r#up == false && self.r#west == West::None { return 23406; }
        if self.r#south == South::Tall && self.r#waterlogged == false && self.r#north == North::Tall && self.r#up == false && self.r#east == East::Tall && self.r#west == West::None { return 23658; }
        if self.r#west == West::None && self.r#up == false && self.r#south == South::Low && self.r#east == East::None && self.r#north == North::None && self.r#waterlogged == false { return 23358; }
        if self.r#east == East::None && self.r#north == North::Low && self.r#up == false && self.r#waterlogged == false && self.r#west == West::Low && self.r#south == South::None { return 23383; }
        if self.r#east == East::None && self.r#south == South::None && self.r#north == North::None && self.r#up == true && self.r#waterlogged == true && self.r#west == West::Low { return 23338; }
        if self.r#east == East::None && self.r#up == true && self.r#north == North::Low && self.r#waterlogged == false && self.r#west == West::Tall && self.r#south == South::Tall { return 23402; }
        if self.r#up == true && self.r#waterlogged == true && self.r#west == West::None && self.r#north == North::Low && self.r#east == East::Low && self.r#south == South::Low { return 23493; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 23642 {
            return Some(TuffWall {
                r#east: East::Tall,
                r#waterlogged: false,
                r#north: North::Tall,
                r#south: South::Low,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 23379 {
            return Some(TuffWall {
                r#north: North::Low,
                r#up: false,
                r#west: West::None,
                r#south: South::None,
                r#east: East::None,
                r#waterlogged: true,
            });
        }
        if state_id == 23420 {
            return Some(TuffWall {
                r#south: South::None,
                r#east: East::None,
                r#waterlogged: false,
                r#up: false,
                r#west: West::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 23511 {
            return Some(TuffWall {
                r#waterlogged: true,
                r#west: West::None,
                r#up: false,
                r#south: South::Tall,
                r#east: East::Low,
                r#north: North::Low,
            });
        }
        if state_id == 23440 {
            return Some(TuffWall {
                r#waterlogged: true,
                r#east: East::None,
                r#west: West::Low,
                r#up: false,
                r#south: South::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 23462 {
            return Some(TuffWall {
                r#east: East::Low,
                r#south: South::Low,
                r#up: true,
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 23393 {
            return Some(TuffWall {
                r#north: North::Low,
                r#waterlogged: true,
                r#east: East::None,
                r#south: South::Low,
                r#up: false,
                r#west: West::Tall,
            });
        }
        if state_id == 23497 {
            return Some(TuffWall {
                r#north: North::Low,
                r#east: East::Low,
                r#up: true,
                r#west: West::Low,
                r#waterlogged: false,
                r#south: South::Low,
            });
        }
        if state_id == 23606 {
            return Some(TuffWall {
                r#west: West::Tall,
                r#waterlogged: false,
                r#up: true,
                r#east: East::Tall,
                r#south: South::Low,
                r#north: North::Low,
            });
        }
        if state_id == 23540 {
            return Some(TuffWall {
                r#south: South::Low,
                r#up: false,
                r#west: West::Tall,
                r#east: East::Low,
                r#north: North::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 23473 {
            return Some(TuffWall {
                r#south: South::Tall,
                r#up: true,
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::Low,
            });
        }
        if state_id == 23447 {
            return Some(TuffWall {
                r#waterlogged: true,
                r#up: true,
                r#east: East::Low,
                r#west: West::Tall,
                r#north: North::None,
                r#south: South::None,
            });
        }
        if state_id == 23400 {
            return Some(TuffWall {
                r#east: East::None,
                r#up: true,
                r#west: West::None,
                r#south: South::Tall,
                r#waterlogged: false,
                r#north: North::Low,
            });
        }
        if state_id == 23502 {
            return Some(TuffWall {
                r#south: South::Low,
                r#up: false,
                r#west: West::None,
                r#waterlogged: false,
                r#east: East::Low,
                r#north: North::Low,
            });
        }
        if state_id == 23581 {
            return Some(TuffWall {
                r#north: North::None,
                r#south: South::Tall,
                r#west: West::Low,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 23375 {
            return Some(TuffWall {
                r#waterlogged: true,
                r#south: South::None,
                r#west: West::Tall,
                r#north: North::Low,
                r#east: East::None,
                r#up: true,
            });
        }
        if state_id == 23469 {
            return Some(TuffWall {
                r#south: South::Tall,
                r#up: true,
                r#north: North::None,
                r#east: East::Low,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 23599 {
            return Some(TuffWall {
                r#south: South::None,
                r#waterlogged: false,
                r#up: false,
                r#east: East::Tall,
                r#west: West::Low,
                r#north: North::Low,
            });
        }
        if state_id == 23562 {
            return Some(TuffWall {
                r#west: West::None,
                r#waterlogged: false,
                r#north: North::None,
                r#south: South::None,
                r#up: false,
                r#east: East::Tall,
            });
        }
        if state_id == 23367 {
            return Some(TuffWall {
                r#waterlogged: true,
                r#up: false,
                r#west: West::None,
                r#south: South::Tall,
                r#east: East::None,
                r#north: North::None,
            });
        }
        if state_id == 23439 {
            return Some(TuffWall {
                r#south: South::Tall,
                r#north: North::Tall,
                r#waterlogged: true,
                r#east: East::None,
                r#up: false,
                r#west: West::None,
            });
        }
        if state_id == 23542 {
            return Some(TuffWall {
                r#south: South::Tall,
                r#waterlogged: true,
                r#up: true,
                r#west: West::Low,
                r#north: North::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 23623 {
            return Some(TuffWall {
                r#south: South::Tall,
                r#west: West::Low,
                r#waterlogged: false,
                r#north: North::Low,
                r#up: false,
                r#east: East::Tall,
            });
        }
        if state_id == 23558 {
            return Some(TuffWall {
                r#west: West::Tall,
                r#waterlogged: false,
                r#south: South::None,
                r#up: true,
                r#east: East::Tall,
                r#north: North::None,
            });
        }
        if state_id == 23453 {
            return Some(TuffWall {
                r#up: false,
                r#north: North::None,
                r#east: East::Low,
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::None,
            });
        }
        if state_id == 23515 {
            return Some(TuffWall {
                r#south: South::Tall,
                r#waterlogged: false,
                r#up: false,
                r#west: West::Low,
                r#east: East::Low,
                r#north: North::Low,
            });
        }
        if state_id == 23418 {
            return Some(TuffWall {
                r#up: false,
                r#north: North::Tall,
                r#south: South::None,
                r#waterlogged: false,
                r#east: East::None,
                r#west: West::None,
            });
        }
        if state_id == 23520 {
            return Some(TuffWall {
                r#north: North::Tall,
                r#up: true,
                r#east: East::Low,
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 23527 {
            return Some(TuffWall {
                r#south: South::None,
                r#west: West::Low,
                r#east: East::Low,
                r#waterlogged: false,
                r#north: North::Tall,
                r#up: false,
            });
        }
        if state_id == 23499 {
            return Some(TuffWall {
                r#up: false,
                r#east: East::Low,
                r#west: West::None,
                r#waterlogged: true,
                r#south: South::Low,
                r#north: North::Low,
            });
        }
        if state_id == 23621 {
            return Some(TuffWall {
                r#east: East::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
                r#up: false,
                r#north: North::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 23361 {
            return Some(TuffWall {
                r#north: North::None,
                r#east: East::None,
                r#up: true,
                r#waterlogged: true,
                r#south: South::Tall,
                r#west: West::None,
            });
        }
        if state_id == 23578 {
            return Some(TuffWall {
                r#up: true,
                r#waterlogged: true,
                r#north: North::None,
                r#west: West::Low,
                r#south: South::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 23438 {
            return Some(TuffWall {
                r#east: East::None,
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 23404 {
            return Some(TuffWall {
                r#north: North::Low,
                r#west: West::Low,
                r#east: East::None,
                r#waterlogged: true,
                r#south: South::Tall,
                r#up: false,
            });
        }
        if state_id == 23342 {
            return Some(TuffWall {
                r#up: true,
                r#east: East::None,
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::None,
            });
        }
        if state_id == 23519 {
            return Some(TuffWall {
                r#north: North::Tall,
                r#east: East::Low,
                r#south: South::None,
                r#waterlogged: true,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 23533 {
            return Some(TuffWall {
                r#west: West::Low,
                r#north: North::Tall,
                r#up: true,
                r#east: East::Low,
                r#south: South::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 23494 {
            return Some(TuffWall {
                r#north: North::Low,
                r#south: South::Low,
                r#waterlogged: true,
                r#up: true,
                r#west: West::Low,
                r#east: East::Low,
            });
        }
        if state_id == 23478 {
            return Some(TuffWall {
                r#east: East::Low,
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::None,
                r#up: false,
            });
        }
        if state_id == 23508 {
            return Some(TuffWall {
                r#up: true,
                r#east: East::Low,
                r#south: South::Tall,
                r#north: North::Low,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 23597 {
            return Some(TuffWall {
                r#up: false,
                r#east: East::Tall,
                r#north: North::Low,
                r#west: West::Tall,
                r#south: South::None,
                r#waterlogged: true,
            });
        }
        if state_id == 23498 {
            return Some(TuffWall {
                r#east: East::Low,
                r#north: North::Low,
                r#west: West::Tall,
                r#waterlogged: false,
                r#south: South::Low,
                r#up: true,
            });
        }
        if state_id == 23422 {
            return Some(TuffWall {
                r#west: West::Low,
                r#waterlogged: true,
                r#south: South::Low,
                r#north: North::Tall,
                r#up: true,
                r#east: East::None,
            });
        }
        if state_id == 23434 {
            return Some(TuffWall {
                r#waterlogged: true,
                r#east: East::None,
                r#west: West::Low,
                r#up: true,
                r#south: South::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 23488 {
            return Some(TuffWall {
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::Low,
                r#up: false,
            });
        }
        if state_id == 23431 {
            return Some(TuffWall {
                r#up: false,
                r#west: West::Low,
                r#east: East::None,
                r#north: North::Tall,
                r#south: South::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 23445 {
            return Some(TuffWall {
                r#west: West::None,
                r#north: North::None,
                r#east: East::Low,
                r#south: South::None,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 23548 {
            return Some(TuffWall {
                r#up: false,
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 23415 {
            return Some(TuffWall {
                r#east: East::None,
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::Tall,
                r#south: South::None,
            });
        }
        if state_id == 23449 {
            return Some(TuffWall {
                r#up: true,
                r#north: North::None,
                r#east: East::Low,
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::None,
            });
        }
        if state_id == 23460 {
            return Some(TuffWall {
                r#south: South::Low,
                r#west: West::None,
                r#east: East::Low,
                r#north: North::None,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 23451 {
            return Some(TuffWall {
                r#west: West::None,
                r#south: South::None,
                r#north: North::None,
                r#east: East::Low,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 23346 {
            return Some(TuffWall {
                r#south: South::None,
                r#up: false,
                r#waterlogged: false,
                r#north: North::None,
                r#east: East::None,
                r#west: West::None,
            });
        }
        if state_id == 23372 {
            return Some(TuffWall {
                r#east: East::None,
                r#up: false,
                r#waterlogged: false,
                r#south: South::Tall,
                r#north: North::None,
                r#west: West::Tall,
            });
        }
        if state_id == 23377 {
            return Some(TuffWall {
                r#up: true,
                r#waterlogged: false,
                r#south: South::None,
                r#west: West::Low,
                r#north: North::Low,
                r#east: East::None,
            });
        }
        if state_id == 23409 {
            return Some(TuffWall {
                r#west: West::None,
                r#east: East::None,
                r#south: South::None,
                r#up: true,
                r#north: North::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 23517 {
            return Some(TuffWall {
                r#north: North::Tall,
                r#west: West::None,
                r#east: East::Low,
                r#up: true,
                r#waterlogged: true,
                r#south: South::None,
            });
        }
        if state_id == 23655 {
            return Some(TuffWall {
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: true,
                r#east: East::Tall,
                r#south: South::Tall,
                r#west: West::None,
            });
        }
        if state_id == 23539 {
            return Some(TuffWall {
                r#waterlogged: false,
                r#east: East::Low,
                r#north: North::Tall,
                r#south: South::Low,
                r#up: false,
                r#west: West::Low,
            });
        }
        if state_id == 23552 {
            return Some(TuffWall {
                r#up: false,
                r#waterlogged: false,
                r#north: North::Tall,
                r#west: West::Tall,
                r#south: South::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 23628 {
            return Some(TuffWall {
                r#up: true,
                r#waterlogged: false,
                r#north: North::Tall,
                r#west: West::None,
                r#east: East::Tall,
                r#south: South::None,
            });
        }
        if state_id == 23550 {
            return Some(TuffWall {
                r#east: East::Low,
                r#up: false,
                r#west: West::None,
                r#north: North::Tall,
                r#south: South::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 23532 {
            return Some(TuffWall {
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::Low,
                r#north: North::Tall,
                r#east: East::Low,
                r#up: true,
            });
        }
        if state_id == 23633 {
            return Some(TuffWall {
                r#south: South::None,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 23591 {
            return Some(TuffWall {
                r#up: true,
                r#north: North::Low,
                r#east: East::Tall,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 23624 {
            return Some(TuffWall {
                r#south: South::Tall,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: false,
                r#east: East::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 23625 {
            return Some(TuffWall {
                r#north: North::Tall,
                r#waterlogged: true,
                r#south: South::None,
                r#up: true,
                r#west: West::None,
                r#east: East::Tall,
            });
        }
        if state_id == 23407 {
            return Some(TuffWall {
                r#waterlogged: false,
                r#east: East::None,
                r#west: West::Low,
                r#south: South::Tall,
                r#north: North::Low,
                r#up: false,
            });
        }
        if state_id == 23366 {
            return Some(TuffWall {
                r#west: West::Tall,
                r#waterlogged: false,
                r#south: South::Tall,
                r#north: North::None,
                r#east: East::None,
                r#up: true,
            });
        }
        if state_id == 23484 {
            return Some(TuffWall {
                r#north: North::Low,
                r#east: East::Low,
                r#waterlogged: false,
                r#up: true,
                r#west: West::None,
                r#south: South::None,
            });
        }
        if state_id == 23568 {
            return Some(TuffWall {
                r#east: East::Tall,
                r#north: North::None,
                r#south: South::Low,
                r#up: true,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 23585 {
            return Some(TuffWall {
                r#waterlogged: true,
                r#west: West::Tall,
                r#up: false,
                r#south: South::Tall,
                r#north: North::None,
                r#east: East::Tall,
            });
        }
        if state_id == 23615 {
            return Some(TuffWall {
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::Tall,
                r#north: North::Low,
                r#east: East::Tall,
                r#up: true,
            });
        }
        if state_id == 23651 {
            return Some(TuffWall {
                r#east: East::Tall,
                r#north: North::Tall,
                r#up: true,
                r#west: West::Tall,
                r#waterlogged: true,
                r#south: South::Tall,
            });
        }
        if state_id == 23641 {
            return Some(TuffWall {
                r#north: North::Tall,
                r#east: East::Tall,
                r#south: South::Low,
                r#waterlogged: false,
                r#up: true,
                r#west: West::Low,
            });
        }
        if state_id == 23385 {
            return Some(TuffWall {
                r#west: West::None,
                r#waterlogged: true,
                r#north: North::Low,
                r#up: true,
                r#east: East::None,
                r#south: South::Low,
            });
        }
        if state_id == 23452 {
            return Some(TuffWall {
                r#north: North::None,
                r#west: West::Low,
                r#south: South::None,
                r#waterlogged: true,
                r#up: false,
                r#east: East::Low,
            });
        }
        if state_id == 23563 {
            return Some(TuffWall {
                r#north: North::None,
                r#east: East::Tall,
                r#west: West::Low,
                r#south: South::None,
                r#waterlogged: false,
                r#up: false,
            });
        }
        if state_id == 23589 {
            return Some(TuffWall {
                r#waterlogged: true,
                r#south: South::None,
                r#up: true,
                r#west: West::None,
                r#north: North::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 23566 {
            return Some(TuffWall {
                r#waterlogged: true,
                r#south: South::Low,
                r#west: West::Low,
                r#north: North::None,
                r#up: true,
                r#east: East::Tall,
            });
        }
        if state_id == 23399 {
            return Some(TuffWall {
                r#east: East::None,
                r#south: South::Tall,
                r#up: true,
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 23652 {
            return Some(TuffWall {
                r#north: North::Tall,
                r#waterlogged: false,
                r#west: West::None,
                r#up: true,
                r#east: East::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 23371 {
            return Some(TuffWall {
                r#up: false,
                r#waterlogged: false,
                r#east: East::None,
                r#north: North::None,
                r#west: West::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 23570 {
            return Some(TuffWall {
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::None,
                r#east: East::Tall,
                r#south: South::Low,
                r#up: true,
            });
        }
        if state_id == 23516 {
            return Some(TuffWall {
                r#up: false,
                r#east: East::Low,
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 23459 {
            return Some(TuffWall {
                r#west: West::Tall,
                r#up: true,
                r#east: East::Low,
                r#north: North::None,
                r#south: South::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 23390 {
            return Some(TuffWall {
                r#north: North::Low,
                r#waterlogged: false,
                r#east: East::None,
                r#south: South::Low,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 23345 {
            return Some(TuffWall {
                r#waterlogged: true,
                r#east: East::None,
                r#south: South::None,
                r#west: West::Tall,
                r#north: North::None,
                r#up: false,
            });
        }
        if state_id == 23580 {
            return Some(TuffWall {
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::None,
                r#south: South::Tall,
            });
        }
        if state_id == 23650 {
            return Some(TuffWall {
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::Tall,
                r#east: East::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 23486 {
            return Some(TuffWall {
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::Low,
                r#east: East::Low,
                r#up: true,
            });
        }
        if state_id == 23575 {
            return Some(TuffWall {
                r#south: South::Low,
                r#waterlogged: false,
                r#east: East::Tall,
                r#north: North::None,
                r#up: false,
                r#west: West::Low,
            });
        }
        if state_id == 23648 {
            return Some(TuffWall {
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: false,
                r#east: East::Tall,
                r#west: West::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 23396 {
            return Some(TuffWall {
                r#south: South::Low,
                r#waterlogged: false,
                r#east: East::None,
                r#west: West::Tall,
                r#up: false,
                r#north: North::Low,
            });
        }
        if state_id == 23382 {
            return Some(TuffWall {
                r#south: South::None,
                r#north: North::Low,
                r#east: East::None,
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 23424 {
            return Some(TuffWall {
                r#east: East::None,
                r#north: North::Tall,
                r#south: South::Low,
                r#up: true,
                r#west: West::None,
                r#waterlogged: false,
            });
        }
        if state_id == 23470 {
            return Some(TuffWall {
                r#south: South::Tall,
                r#west: West::Low,
                r#north: North::None,
                r#east: East::Low,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 23489 {
            return Some(TuffWall {
                r#south: South::None,
                r#east: East::Low,
                r#waterlogged: true,
                r#north: North::Low,
                r#west: West::Tall,
                r#up: false,
            });
        }
        if state_id == 23437 {
            return Some(TuffWall {
                r#waterlogged: false,
                r#east: East::None,
                r#south: South::Tall,
                r#up: true,
                r#west: West::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 23456 {
            return Some(TuffWall {
                r#south: South::None,
                r#north: North::None,
                r#west: West::Tall,
                r#up: false,
                r#waterlogged: false,
                r#east: East::Low,
            });
        }
        if state_id == 23577 {
            return Some(TuffWall {
                r#north: North::None,
                r#west: West::None,
                r#up: true,
                r#waterlogged: true,
                r#east: East::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 23602 {
            return Some(TuffWall {
                r#waterlogged: true,
                r#north: North::Low,
                r#south: South::Low,
                r#up: true,
                r#east: East::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 23610 {
            return Some(TuffWall {
                r#south: South::Low,
                r#up: false,
                r#west: West::None,
                r#east: East::Tall,
                r#north: North::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 23347 {
            return Some(TuffWall {
                r#waterlogged: false,
                r#west: West::Low,
                r#up: false,
                r#north: North::None,
                r#south: South::None,
                r#east: East::None,
            });
        }
        if state_id == 23573 {
            return Some(TuffWall {
                r#up: false,
                r#west: West::Tall,
                r#south: South::Low,
                r#north: North::None,
                r#waterlogged: true,
                r#east: East::Tall,
            });
        }
        if state_id == 23357 {
            return Some(TuffWall {
                r#east: East::None,
                r#north: North::None,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 23339 {
            return Some(TuffWall {
                r#west: West::Tall,
                r#east: East::None,
                r#up: true,
                r#north: North::None,
                r#south: South::None,
                r#waterlogged: true,
            });
        }
        if state_id == 23458 {
            return Some(TuffWall {
                r#up: true,
                r#north: North::None,
                r#west: West::Low,
                r#south: South::Low,
                r#waterlogged: true,
                r#east: East::Low,
            });
        }
        if state_id == 23461 {
            return Some(TuffWall {
                r#south: South::Low,
                r#north: North::None,
                r#up: true,
                r#waterlogged: false,
                r#east: East::Low,
                r#west: West::Low,
            });
        }
        if state_id == 23463 {
            return Some(TuffWall {
                r#up: false,
                r#west: West::None,
                r#south: South::Low,
                r#north: North::None,
                r#waterlogged: true,
                r#east: East::Low,
            });
        }
        if state_id == 23607 {
            return Some(TuffWall {
                r#east: East::Tall,
                r#south: South::Low,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 23505 {
            return Some(TuffWall {
                r#west: West::None,
                r#east: East::Low,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: true,
                r#south: South::Tall,
            });
        }
        if state_id == 23528 {
            return Some(TuffWall {
                r#east: East::Low,
                r#north: North::Tall,
                r#south: South::None,
                r#west: West::Tall,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 23370 {
            return Some(TuffWall {
                r#south: South::Tall,
                r#up: false,
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::None,
            });
        }
        if state_id == 23411 {
            return Some(TuffWall {
                r#north: North::Tall,
                r#south: South::None,
                r#waterlogged: true,
                r#up: true,
                r#east: East::None,
                r#west: West::Tall,
            });
        }
        if state_id == 23341 {
            return Some(TuffWall {
                r#west: West::Low,
                r#up: true,
                r#east: East::None,
                r#south: South::None,
                r#waterlogged: false,
                r#north: North::None,
            });
        }
        if state_id == 23523 {
            return Some(TuffWall {
                r#north: North::Tall,
                r#east: East::Low,
                r#south: South::None,
                r#west: West::None,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 23376 {
            return Some(TuffWall {
                r#north: North::Low,
                r#up: true,
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::None,
                r#south: South::None,
            });
        }
        if state_id == 23353 {
            return Some(TuffWall {
                r#up: true,
                r#south: South::Low,
                r#east: East::None,
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::None,
            });
        }
        if state_id == 23429 {
            return Some(TuffWall {
                r#south: South::Low,
                r#east: East::None,
                r#up: false,
                r#waterlogged: true,
                r#north: North::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 23569 {
            return Some(TuffWall {
                r#east: East::Tall,
                r#north: North::None,
                r#up: true,
                r#west: West::Low,
                r#waterlogged: false,
                r#south: South::Low,
            });
        }
        if state_id == 23397 {
            return Some(TuffWall {
                r#east: East::None,
                r#up: true,
                r#west: West::None,
                r#south: South::Tall,
                r#waterlogged: true,
                r#north: North::Low,
            });
        }
        if state_id == 23590 {
            return Some(TuffWall {
                r#north: North::Low,
                r#waterlogged: true,
                r#south: South::None,
                r#east: East::Tall,
                r#west: West::Low,
                r#up: true,
            });
        }
        if state_id == 23613 {
            return Some(TuffWall {
                r#east: East::Tall,
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::Low,
                r#up: true,
            });
        }
        if state_id == 23637 {
            return Some(TuffWall {
                r#up: true,
                r#north: North::Tall,
                r#south: South::Low,
                r#east: East::Tall,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 23618 {
            return Some(TuffWall {
                r#up: true,
                r#west: West::Tall,
                r#north: North::Low,
                r#east: East::Tall,
                r#waterlogged: false,
                r#south: South::Tall,
            });
        }
        if state_id == 23501 {
            return Some(TuffWall {
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::Low,
                r#waterlogged: true,
                r#up: false,
                r#west: West::Tall,
            });
        }
        if state_id == 23547 {
            return Some(TuffWall {
                r#south: South::Tall,
                r#west: West::None,
                r#up: false,
                r#waterlogged: true,
                r#north: North::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 23435 {
            return Some(TuffWall {
                r#south: South::Tall,
                r#east: East::None,
                r#north: North::Tall,
                r#waterlogged: true,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 23441 {
            return Some(TuffWall {
                r#north: North::Tall,
                r#east: East::None,
                r#west: West::Tall,
                r#south: South::Tall,
                r#waterlogged: true,
                r#up: false,
            });
        }
        if state_id == 23509 {
            return Some(TuffWall {
                r#east: East::Low,
                r#west: West::Low,
                r#waterlogged: false,
                r#north: North::Low,
                r#south: South::Tall,
                r#up: true,
            });
        }
        if state_id == 23543 {
            return Some(TuffWall {
                r#south: South::Tall,
                r#up: true,
                r#north: North::Tall,
                r#east: East::Low,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 23587 {
            return Some(TuffWall {
                r#south: South::Tall,
                r#north: North::None,
                r#up: false,
                r#waterlogged: false,
                r#east: East::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 23593 {
            return Some(TuffWall {
                r#south: South::None,
                r#north: North::Low,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 23640 {
            return Some(TuffWall {
                r#east: East::Tall,
                r#west: West::None,
                r#up: true,
                r#south: South::Low,
                r#north: North::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 23598 {
            return Some(TuffWall {
                r#waterlogged: false,
                r#east: East::Tall,
                r#south: South::None,
                r#west: West::None,
                r#up: false,
                r#north: North::Low,
            });
        }
        if state_id == 23551 {
            return Some(TuffWall {
                r#waterlogged: false,
                r#east: East::Low,
                r#up: false,
                r#south: South::Tall,
                r#west: West::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 23467 {
            return Some(TuffWall {
                r#west: West::Low,
                r#up: false,
                r#south: South::Low,
                r#north: North::None,
                r#waterlogged: false,
                r#east: East::Low,
            });
        }
        if state_id == 23544 {
            return Some(TuffWall {
                r#up: true,
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::Low,
                r#north: North::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 23389 {
            return Some(TuffWall {
                r#east: East::None,
                r#north: North::Low,
                r#south: South::Low,
                r#waterlogged: false,
                r#up: true,
                r#west: West::Low,
            });
        }
        if state_id == 23351 {
            return Some(TuffWall {
                r#up: true,
                r#west: West::Tall,
                r#waterlogged: true,
                r#east: East::None,
                r#south: South::Low,
                r#north: North::None,
            });
        }
        if state_id == 23495 {
            return Some(TuffWall {
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::Low,
                r#south: South::Low,
                r#north: North::Low,
            });
        }
        if state_id == 23496 {
            return Some(TuffWall {
                r#up: true,
                r#west: West::None,
                r#east: East::Low,
                r#waterlogged: false,
                r#north: North::Low,
                r#south: South::Low,
            });
        }
        if state_id == 23500 {
            return Some(TuffWall {
                r#west: West::Low,
                r#waterlogged: true,
                r#north: North::Low,
                r#east: East::Low,
                r#south: South::Low,
                r#up: false,
            });
        }
        if state_id == 23561 {
            return Some(TuffWall {
                r#up: false,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::Tall,
                r#north: North::None,
            });
        }
        if state_id == 23421 {
            return Some(TuffWall {
                r#waterlogged: true,
                r#east: East::None,
                r#up: true,
                r#west: West::None,
                r#south: South::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 23475 {
            return Some(TuffWall {
                r#north: North::None,
                r#east: East::Low,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 23534 {
            return Some(TuffWall {
                r#west: West::Tall,
                r#south: South::Low,
                r#waterlogged: false,
                r#up: true,
                r#east: East::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 23387 {
            return Some(TuffWall {
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::Low,
                r#east: East::None,
                r#south: South::Low,
            });
        }
        if state_id == 23483 {
            return Some(TuffWall {
                r#up: true,
                r#waterlogged: true,
                r#south: South::None,
                r#west: West::Tall,
                r#east: East::Low,
                r#north: North::Low,
            });
        }
        if state_id == 23388 {
            return Some(TuffWall {
                r#south: South::Low,
                r#up: true,
                r#waterlogged: false,
                r#east: East::None,
                r#west: West::None,
                r#north: North::Low,
            });
        }
        if state_id == 23553 {
            return Some(TuffWall {
                r#north: North::None,
                r#waterlogged: true,
                r#east: East::Tall,
                r#south: South::None,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 23654 {
            return Some(TuffWall {
                r#west: West::Tall,
                r#north: North::Tall,
                r#east: East::Tall,
                r#waterlogged: false,
                r#south: South::Tall,
                r#up: true,
            });
        }
        if state_id == 23363 {
            return Some(TuffWall {
                r#up: true,
                r#waterlogged: true,
                r#east: East::None,
                r#south: South::Tall,
                r#north: North::None,
                r#west: West::Tall,
            });
        }
        if state_id == 23481 {
            return Some(TuffWall {
                r#west: West::None,
                r#up: true,
                r#north: North::Low,
                r#south: South::None,
                r#waterlogged: true,
                r#east: East::Low,
            });
        }
        if state_id == 23526 {
            return Some(TuffWall {
                r#waterlogged: false,
                r#north: North::Tall,
                r#south: South::None,
                r#east: East::Low,
                r#up: false,
                r#west: West::None,
            });
        }
        if state_id == 23612 {
            return Some(TuffWall {
                r#north: North::Low,
                r#east: East::Tall,
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::Tall,
                r#up: false,
            });
        }
        if state_id == 23408 {
            return Some(TuffWall {
                r#south: South::Tall,
                r#west: West::Tall,
                r#east: East::None,
                r#north: North::Low,
                r#waterlogged: false,
                r#up: false,
            });
        }
        if state_id == 23521 {
            return Some(TuffWall {
                r#up: true,
                r#west: West::Low,
                r#north: North::Tall,
                r#waterlogged: false,
                r#south: South::None,
                r#east: East::Low,
            });
        }
        if state_id == 23564 {
            return Some(TuffWall {
                r#north: North::None,
                r#waterlogged: false,
                r#south: South::None,
                r#up: false,
                r#west: West::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 23614 {
            return Some(TuffWall {
                r#east: East::Tall,
                r#waterlogged: true,
                r#north: North::Low,
                r#west: West::Low,
                r#south: South::Tall,
                r#up: true,
            });
        }
        if state_id == 23419 {
            return Some(TuffWall {
                r#east: East::None,
                r#south: South::None,
                r#west: West::Low,
                r#waterlogged: false,
                r#north: North::Tall,
                r#up: false,
            });
        }
        if state_id == 23503 {
            return Some(TuffWall {
                r#up: false,
                r#west: West::Low,
                r#south: South::Low,
                r#north: North::Low,
                r#waterlogged: false,
                r#east: East::Low,
            });
        }
        if state_id == 23433 {
            return Some(TuffWall {
                r#west: West::None,
                r#east: East::None,
                r#south: South::Tall,
                r#north: North::Tall,
                r#waterlogged: true,
                r#up: true,
            });
        }
        if state_id == 23556 {
            return Some(TuffWall {
                r#east: East::Tall,
                r#up: true,
                r#south: South::None,
                r#west: West::None,
                r#waterlogged: false,
                r#north: North::None,
            });
        }
        if state_id == 23549 {
            return Some(TuffWall {
                r#west: West::Tall,
                r#north: North::Tall,
                r#waterlogged: true,
                r#east: East::Low,
                r#south: South::Tall,
                r#up: false,
            });
        }
        if state_id == 23476 {
            return Some(TuffWall {
                r#west: West::Low,
                r#east: East::Low,
                r#north: North::None,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 23368 {
            return Some(TuffWall {
                r#north: North::None,
                r#east: East::None,
                r#up: false,
                r#west: West::Low,
                r#south: South::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 23457 {
            return Some(TuffWall {
                r#west: West::None,
                r#east: East::Low,
                r#waterlogged: true,
                r#up: true,
                r#north: North::None,
                r#south: South::Low,
            });
        }
        if state_id == 23588 {
            return Some(TuffWall {
                r#west: West::Tall,
                r#north: North::None,
                r#waterlogged: false,
                r#east: East::Tall,
                r#south: South::Tall,
                r#up: false,
            });
        }
        if state_id == 23584 {
            return Some(TuffWall {
                r#east: East::Tall,
                r#up: false,
                r#west: West::Low,
                r#south: South::Tall,
                r#waterlogged: true,
                r#north: North::None,
            });
        }
        if state_id == 23352 {
            return Some(TuffWall {
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::None,
                r#up: true,
                r#east: East::None,
            });
        }
        if state_id == 23636 {
            return Some(TuffWall {
                r#east: East::Tall,
                r#south: South::None,
                r#up: false,
                r#west: West::Tall,
                r#waterlogged: false,
                r#north: North::Tall,
            });
        }
        if state_id == 23531 {
            return Some(TuffWall {
                r#south: South::Low,
                r#up: true,
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 23427 {
            return Some(TuffWall {
                r#waterlogged: true,
                r#west: West::None,
                r#up: false,
                r#east: East::None,
                r#south: South::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 23538 {
            return Some(TuffWall {
                r#waterlogged: false,
                r#east: East::Low,
                r#south: South::Low,
                r#north: North::Tall,
                r#up: false,
                r#west: West::None,
            });
        }
        if state_id == 23350 {
            return Some(TuffWall {
                r#south: South::Low,
                r#east: East::None,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::None,
            });
        }
        if state_id == 23374 {
            return Some(TuffWall {
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::None,
                r#up: true,
                r#east: East::None,
                r#north: North::Low,
            });
        }
        if state_id == 23472 {
            return Some(TuffWall {
                r#up: true,
                r#waterlogged: false,
                r#south: South::Tall,
                r#east: East::Low,
                r#west: West::None,
                r#north: North::None,
            });
        }
        if state_id == 23491 {
            return Some(TuffWall {
                r#up: false,
                r#north: North::Low,
                r#east: East::Low,
                r#waterlogged: false,
                r#south: South::None,
                r#west: West::Low,
            });
        }
        if state_id == 23560 {
            return Some(TuffWall {
                r#east: East::Tall,
                r#north: North::None,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::Low,
                r#up: false,
            });
        }
        if state_id == 23541 {
            return Some(TuffWall {
                r#south: South::Tall,
                r#north: North::Tall,
                r#west: West::None,
                r#waterlogged: true,
                r#up: true,
                r#east: East::Low,
            });
        }
        if state_id == 23506 {
            return Some(TuffWall {
                r#south: South::Tall,
                r#up: true,
                r#north: North::Low,
                r#west: West::Low,
                r#waterlogged: true,
                r#east: East::Low,
            });
        }
        if state_id == 23554 {
            return Some(TuffWall {
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::None,
                r#south: South::None,
                r#east: East::Tall,
            });
        }
        if state_id == 23482 {
            return Some(TuffWall {
                r#west: West::Low,
                r#east: East::Low,
                r#up: true,
                r#south: South::None,
                r#north: North::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 23595 {
            return Some(TuffWall {
                r#west: West::None,
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::None,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 23600 {
            return Some(TuffWall {
                r#west: West::Tall,
                r#up: false,
                r#south: South::None,
                r#north: North::Low,
                r#waterlogged: false,
                r#east: East::Tall,
            });
        }
        if state_id == 23432 {
            return Some(TuffWall {
                r#south: South::Low,
                r#west: West::Tall,
                r#waterlogged: false,
                r#up: false,
                r#east: East::None,
                r#north: North::Tall,
            });
        }
        if state_id == 23524 {
            return Some(TuffWall {
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::Low,
                r#up: false,
                r#south: South::None,
                r#east: East::Low,
            });
        }
        if state_id == 23638 {
            return Some(TuffWall {
                r#south: South::Low,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 23525 {
            return Some(TuffWall {
                r#west: West::Tall,
                r#waterlogged: true,
                r#north: North::Tall,
                r#east: East::Low,
                r#south: South::None,
                r#up: false,
            });
        }
        if state_id == 23360 {
            return Some(TuffWall {
                r#up: false,
                r#east: East::None,
                r#waterlogged: false,
                r#south: South::Low,
                r#west: West::Tall,
                r#north: North::None,
            });
        }
        if state_id == 23537 {
            return Some(TuffWall {
                r#west: West::Tall,
                r#east: East::Low,
                r#waterlogged: true,
                r#north: North::Tall,
                r#south: South::Low,
                r#up: false,
            });
        }
        if state_id == 23557 {
            return Some(TuffWall {
                r#north: North::None,
                r#south: South::None,
                r#west: West::Low,
                r#east: East::Tall,
                r#waterlogged: false,
                r#up: true,
            });
        }
        if state_id == 23632 {
            return Some(TuffWall {
                r#east: East::Tall,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::None,
            });
        }
        if state_id == 23507 {
            return Some(TuffWall {
                r#east: East::Low,
                r#waterlogged: true,
                r#west: West::Tall,
                r#up: true,
                r#north: North::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 23413 {
            return Some(TuffWall {
                r#up: true,
                r#north: North::Tall,
                r#south: South::None,
                r#west: West::Low,
                r#waterlogged: false,
                r#east: East::None,
            });
        }
        if state_id == 23426 {
            return Some(TuffWall {
                r#waterlogged: false,
                r#east: East::None,
                r#up: true,
                r#west: West::Tall,
                r#south: South::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 23349 {
            return Some(TuffWall {
                r#up: true,
                r#south: South::Low,
                r#north: North::None,
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::None,
            });
        }
        if state_id == 23380 {
            return Some(TuffWall {
                r#east: East::None,
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::None,
                r#up: false,
            });
        }
        if state_id == 23365 {
            return Some(TuffWall {
                r#up: true,
                r#waterlogged: false,
                r#south: South::Tall,
                r#west: West::Low,
                r#north: North::None,
                r#east: East::None,
            });
        }
        if state_id == 23485 {
            return Some(TuffWall {
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::None,
            });
        }
        if state_id == 23571 {
            return Some(TuffWall {
                r#up: false,
                r#south: South::Low,
                r#north: North::None,
                r#west: West::None,
                r#waterlogged: true,
                r#east: East::Tall,
            });
        }
        if state_id == 23630 {
            return Some(TuffWall {
                r#up: true,
                r#north: North::Tall,
                r#south: South::None,
                r#west: West::Tall,
                r#east: East::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 23398 {
            return Some(TuffWall {
                r#north: North::Low,
                r#up: true,
                r#west: West::Low,
                r#waterlogged: true,
                r#east: East::None,
                r#south: South::Tall,
            });
        }
        if state_id == 23629 {
            return Some(TuffWall {
                r#south: South::None,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: false,
                r#north: North::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 23657 {
            return Some(TuffWall {
                r#west: West::Tall,
                r#south: South::Tall,
                r#north: North::Tall,
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 23601 {
            return Some(TuffWall {
                r#south: South::Low,
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 23344 {
            return Some(TuffWall {
                r#east: East::None,
                r#west: West::Low,
                r#up: false,
                r#south: South::None,
                r#waterlogged: true,
                r#north: North::None,
            });
        }
        if state_id == 23354 {
            return Some(TuffWall {
                r#up: true,
                r#north: North::None,
                r#south: South::Low,
                r#waterlogged: false,
                r#east: East::None,
                r#west: West::Tall,
            });
        }
        if state_id == 23428 {
            return Some(TuffWall {
                r#up: false,
                r#waterlogged: true,
                r#north: North::Tall,
                r#south: South::Low,
                r#west: West::Low,
                r#east: East::None,
            });
        }
        if state_id == 23644 {
            return Some(TuffWall {
                r#up: false,
                r#north: North::Tall,
                r#south: South::Low,
                r#waterlogged: true,
                r#east: East::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 23395 {
            return Some(TuffWall {
                r#up: false,
                r#waterlogged: false,
                r#south: South::Low,
                r#west: West::Low,
                r#north: North::Low,
                r#east: East::None,
            });
        }
        if state_id == 23572 {
            return Some(TuffWall {
                r#north: North::None,
                r#east: East::Tall,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 23403 {
            return Some(TuffWall {
                r#south: South::Tall,
                r#east: East::None,
                r#west: West::None,
                r#waterlogged: true,
                r#north: North::Low,
                r#up: false,
            });
        }
        if state_id == 23410 {
            return Some(TuffWall {
                r#east: East::None,
                r#south: South::None,
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 23659 {
            return Some(TuffWall {
                r#south: South::Tall,
                r#east: East::Tall,
                r#up: false,
                r#west: West::Low,
                r#north: North::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 23443 {
            return Some(TuffWall {
                r#east: East::None,
                r#up: false,
                r#west: West::Low,
                r#north: North::Tall,
                r#south: South::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 23384 {
            return Some(TuffWall {
                r#south: South::None,
                r#waterlogged: false,
                r#east: East::None,
                r#up: false,
                r#west: West::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 23626 {
            return Some(TuffWall {
                r#east: East::Tall,
                r#up: true,
                r#north: North::Tall,
                r#waterlogged: true,
                r#south: South::None,
                r#west: West::Low,
            });
        }
        if state_id == 23522 {
            return Some(TuffWall {
                r#north: North::Tall,
                r#up: true,
                r#east: East::Low,
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 23545 {
            return Some(TuffWall {
                r#north: North::Tall,
                r#east: East::Low,
                r#waterlogged: false,
                r#west: West::Low,
                r#up: true,
                r#south: South::Tall,
            });
        }
        if state_id == 23536 {
            return Some(TuffWall {
                r#up: false,
                r#waterlogged: true,
                r#south: South::Low,
                r#east: East::Low,
                r#north: North::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 23381 {
            return Some(TuffWall {
                r#east: East::None,
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::None,
                r#north: North::Low,
                r#up: false,
            });
        }
        if state_id == 23448 {
            return Some(TuffWall {
                r#west: West::None,
                r#north: North::None,
                r#south: South::None,
                r#waterlogged: false,
                r#east: East::Low,
                r#up: true,
            });
        }
        if state_id == 23468 {
            return Some(TuffWall {
                r#west: West::Tall,
                r#up: false,
                r#north: North::None,
                r#east: East::Low,
                r#south: South::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 23477 {
            return Some(TuffWall {
                r#north: North::None,
                r#east: East::Low,
                r#waterlogged: true,
                r#up: false,
                r#west: West::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 23639 {
            return Some(TuffWall {
                r#south: South::Low,
                r#waterlogged: true,
                r#up: true,
                r#west: West::Tall,
                r#north: North::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 23414 {
            return Some(TuffWall {
                r#north: North::Tall,
                r#up: true,
                r#south: South::None,
                r#west: West::Tall,
                r#east: East::None,
                r#waterlogged: false,
            });
        }
        if state_id == 23465 {
            return Some(TuffWall {
                r#up: false,
                r#north: North::None,
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::Low,
                r#east: East::Low,
            });
        }
        if state_id == 23510 {
            return Some(TuffWall {
                r#waterlogged: false,
                r#east: East::Low,
                r#south: South::Tall,
                r#west: West::Tall,
                r#north: North::Low,
                r#up: true,
            });
        }
        if state_id == 23656 {
            return Some(TuffWall {
                r#west: West::Low,
                r#south: South::Tall,
                r#east: East::Tall,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 23364 {
            return Some(TuffWall {
                r#north: North::None,
                r#east: East::None,
                r#south: South::Tall,
                r#west: West::None,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 23609 {
            return Some(TuffWall {
                r#east: East::Tall,
                r#up: false,
                r#north: North::Low,
                r#south: South::Low,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 23450 {
            return Some(TuffWall {
                r#east: East::Low,
                r#waterlogged: false,
                r#up: true,
                r#north: North::None,
                r#west: West::Tall,
                r#south: South::None,
            });
        }
        if state_id == 23616 {
            return Some(TuffWall {
                r#east: East::Tall,
                r#waterlogged: false,
                r#north: North::Low,
                r#south: South::Tall,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 23574 {
            return Some(TuffWall {
                r#west: West::None,
                r#up: false,
                r#east: East::Tall,
                r#north: North::None,
                r#waterlogged: false,
                r#south: South::Low,
            });
        }
        if state_id == 23647 {
            return Some(TuffWall {
                r#north: North::Tall,
                r#west: West::Low,
                r#up: false,
                r#east: East::Tall,
                r#south: South::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 23430 {
            return Some(TuffWall {
                r#west: West::None,
                r#up: false,
                r#north: North::Tall,
                r#east: East::None,
                r#south: South::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 23619 {
            return Some(TuffWall {
                r#up: false,
                r#waterlogged: true,
                r#east: East::Tall,
                r#west: West::None,
                r#north: North::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 23594 {
            return Some(TuffWall {
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::Low,
                r#south: South::None,
                r#east: East::Tall,
                r#up: true,
            });
        }
        if state_id == 23356 {
            return Some(TuffWall {
                r#waterlogged: true,
                r#east: East::None,
                r#north: North::None,
                r#west: West::Low,
                r#south: South::Low,
                r#up: false,
            });
        }
        if state_id == 23480 {
            return Some(TuffWall {
                r#up: false,
                r#waterlogged: false,
                r#north: North::None,
                r#south: South::Tall,
                r#east: East::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 23355 {
            return Some(TuffWall {
                r#east: East::None,
                r#north: North::None,
                r#south: South::Low,
                r#waterlogged: true,
                r#up: false,
                r#west: West::None,
            });
        }
        if state_id == 23608 {
            return Some(TuffWall {
                r#east: East::Tall,
                r#west: West::Low,
                r#north: North::Low,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 23401 {
            return Some(TuffWall {
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::None,
                r#north: North::Low,
                r#south: South::Tall,
                r#up: true,
            });
        }
        if state_id == 23576 {
            return Some(TuffWall {
                r#up: false,
                r#south: South::Low,
                r#west: West::Tall,
                r#north: North::None,
                r#east: East::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 23583 {
            return Some(TuffWall {
                r#up: false,
                r#east: East::Tall,
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::Tall,
                r#north: North::None,
            });
        }
        if state_id == 23378 {
            return Some(TuffWall {
                r#south: South::None,
                r#east: East::None,
                r#up: true,
                r#waterlogged: false,
                r#north: North::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 23386 {
            return Some(TuffWall {
                r#east: East::None,
                r#north: North::Low,
                r#waterlogged: true,
                r#south: South::Low,
                r#west: West::Low,
                r#up: true,
            });
        }
        if state_id == 23471 {
            return Some(TuffWall {
                r#north: North::None,
                r#up: true,
                r#west: West::Tall,
                r#east: East::Low,
                r#south: South::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 23474 {
            return Some(TuffWall {
                r#east: East::Low,
                r#up: true,
                r#north: North::None,
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 23627 {
            return Some(TuffWall {
                r#waterlogged: true,
                r#south: South::None,
                r#up: true,
                r#west: West::Tall,
                r#north: North::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 23592 {
            return Some(TuffWall {
                r#north: North::Low,
                r#up: true,
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::Tall,
                r#south: South::None,
            });
        }
        if state_id == 23490 {
            return Some(TuffWall {
                r#south: South::None,
                r#east: East::Low,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 23617 {
            return Some(TuffWall {
                r#west: West::Low,
                r#north: North::Low,
                r#up: true,
                r#east: East::Tall,
                r#waterlogged: false,
                r#south: South::Tall,
            });
        }
        if state_id == 23514 {
            return Some(TuffWall {
                r#west: West::None,
                r#south: South::Tall,
                r#east: East::Low,
                r#north: North::Low,
                r#waterlogged: false,
                r#up: false,
            });
        }
        if state_id == 23634 {
            return Some(TuffWall {
                r#east: East::Tall,
                r#waterlogged: false,
                r#north: North::Tall,
                r#west: West::None,
                r#south: South::None,
                r#up: false,
            });
        }
        if state_id == 23362 {
            return Some(TuffWall {
                r#north: North::None,
                r#waterlogged: true,
                r#south: South::Tall,
                r#up: true,
                r#east: East::None,
                r#west: West::Low,
            });
        }
        if state_id == 23635 {
            return Some(TuffWall {
                r#up: false,
                r#west: West::Low,
                r#waterlogged: false,
                r#north: North::Tall,
                r#south: South::None,
                r#east: East::Tall,
            });
        }
        if state_id == 23340 {
            return Some(TuffWall {
                r#east: East::None,
                r#north: North::None,
                r#south: South::None,
                r#waterlogged: false,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 23464 {
            return Some(TuffWall {
                r#east: East::Low,
                r#south: South::Low,
                r#west: West::Low,
                r#up: false,
                r#north: North::None,
                r#waterlogged: true,
            });
        }
        if state_id == 23565 {
            return Some(TuffWall {
                r#up: true,
                r#west: West::None,
                r#waterlogged: true,
                r#east: East::Tall,
                r#north: North::None,
                r#south: South::Low,
            });
        }
        if state_id == 23620 {
            return Some(TuffWall {
                r#east: East::Tall,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: true,
                r#south: South::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 23369 {
            return Some(TuffWall {
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::None,
                r#south: South::Tall,
                r#east: East::None,
            });
        }
        if state_id == 23586 {
            return Some(TuffWall {
                r#west: West::None,
                r#south: South::Tall,
                r#north: North::None,
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 23646 {
            return Some(TuffWall {
                r#east: East::Tall,
                r#north: North::Tall,
                r#waterlogged: false,
                r#south: South::Low,
                r#up: false,
                r#west: West::None,
            });
        }
        if state_id == 23466 {
            return Some(TuffWall {
                r#east: East::Low,
                r#north: North::None,
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::None,
                r#up: false,
            });
        }
        if state_id == 23559 {
            return Some(TuffWall {
                r#south: South::None,
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::None,
            });
        }
        if state_id == 23436 {
            return Some(TuffWall {
                r#west: West::None,
                r#north: North::Tall,
                r#south: South::Tall,
                r#waterlogged: false,
                r#up: true,
                r#east: East::None,
            });
        }
        if state_id == 23546 {
            return Some(TuffWall {
                r#north: North::Tall,
                r#east: East::Low,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 23631 {
            return Some(TuffWall {
                r#north: North::Tall,
                r#up: false,
                r#west: West::None,
                r#waterlogged: true,
                r#east: East::Tall,
                r#south: South::None,
            });
        }
        if state_id == 23518 {
            return Some(TuffWall {
                r#north: North::Tall,
                r#up: true,
                r#west: West::Low,
                r#south: South::None,
                r#waterlogged: true,
                r#east: East::Low,
            });
        }
        if state_id == 23555 {
            return Some(TuffWall {
                r#north: North::None,
                r#south: South::None,
                r#up: true,
                r#east: East::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 23446 {
            return Some(TuffWall {
                r#east: East::Low,
                r#south: South::None,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::None,
            });
        }
        if state_id == 23444 {
            return Some(TuffWall {
                r#south: South::Tall,
                r#north: North::Tall,
                r#west: West::Tall,
                r#east: East::None,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 23529 {
            return Some(TuffWall {
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::Low,
                r#south: South::Low,
            });
        }
        if state_id == 23504 {
            return Some(TuffWall {
                r#west: West::Tall,
                r#north: North::Low,
                r#south: South::Low,
                r#east: East::Low,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 23582 {
            return Some(TuffWall {
                r#west: West::Tall,
                r#up: true,
                r#east: East::Tall,
                r#south: South::Tall,
                r#waterlogged: false,
                r#north: North::None,
            });
        }
        if state_id == 23649 {
            return Some(TuffWall {
                r#north: North::Tall,
                r#east: East::Tall,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 23604 {
            return Some(TuffWall {
                r#east: East::Tall,
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::Low,
                r#up: true,
            });
        }
        if state_id == 23567 {
            return Some(TuffWall {
                r#north: North::None,
                r#south: South::Low,
                r#east: East::Tall,
                r#waterlogged: true,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 23622 {
            return Some(TuffWall {
                r#waterlogged: false,
                r#north: North::Low,
                r#up: false,
                r#east: East::Tall,
                r#south: South::Tall,
                r#west: West::None,
            });
        }
        if state_id == 23423 {
            return Some(TuffWall {
                r#waterlogged: true,
                r#east: East::None,
                r#north: North::Tall,
                r#south: South::Low,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 23442 {
            return Some(TuffWall {
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::None,
                r#north: North::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 23611 {
            return Some(TuffWall {
                r#east: East::Tall,
                r#west: West::Low,
                r#south: South::Low,
                r#waterlogged: false,
                r#north: North::Low,
                r#up: false,
            });
        }
        if state_id == 23392 {
            return Some(TuffWall {
                r#east: East::None,
                r#waterlogged: true,
                r#north: North::Low,
                r#west: West::Low,
                r#up: false,
                r#south: South::Low,
            });
        }
        if state_id == 23487 {
            return Some(TuffWall {
                r#north: North::Low,
                r#up: false,
                r#east: East::Low,
                r#west: West::None,
                r#south: South::None,
                r#waterlogged: true,
            });
        }
        if state_id == 23535 {
            return Some(TuffWall {
                r#south: South::Low,
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::Low,
                r#up: false,
            });
        }
        if state_id == 23579 {
            return Some(TuffWall {
                r#west: West::Tall,
                r#up: true,
                r#south: South::Tall,
                r#east: East::Tall,
                r#north: North::None,
                r#waterlogged: true,
            });
        }
        if state_id == 23412 {
            return Some(TuffWall {
                r#north: North::Tall,
                r#east: East::None,
                r#waterlogged: false,
                r#south: South::None,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 23425 {
            return Some(TuffWall {
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::None,
                r#south: South::Low,
            });
        }
        if state_id == 23530 {
            return Some(TuffWall {
                r#east: East::Low,
                r#south: South::Low,
                r#up: true,
                r#waterlogged: true,
                r#north: North::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 23455 {
            return Some(TuffWall {
                r#north: North::None,
                r#south: South::None,
                r#east: East::Low,
                r#up: false,
                r#west: West::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 23643 {
            return Some(TuffWall {
                r#east: East::Tall,
                r#west: West::None,
                r#waterlogged: true,
                r#south: South::Low,
                r#up: false,
                r#north: North::Tall,
            });
        }
        if state_id == 23596 {
            return Some(TuffWall {
                r#south: South::None,
                r#up: false,
                r#waterlogged: true,
                r#east: East::Tall,
                r#north: North::Low,
                r#west: West::Low,
            });
        }
        if state_id == 23337 {
            return Some(TuffWall {
                r#east: East::None,
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::None,
                r#north: North::None,
            });
        }
        if state_id == 23394 {
            return Some(TuffWall {
                r#north: North::Low,
                r#east: East::None,
                r#up: false,
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 23513 {
            return Some(TuffWall {
                r#west: West::Tall,
                r#south: South::Tall,
                r#waterlogged: true,
                r#east: East::Low,
                r#north: North::Low,
                r#up: false,
            });
        }
        if state_id == 23348 {
            return Some(TuffWall {
                r#east: East::None,
                r#north: North::None,
                r#up: false,
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 23373 {
            return Some(TuffWall {
                r#east: East::None,
                r#south: South::None,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 23603 {
            return Some(TuffWall {
                r#west: West::Tall,
                r#south: South::Low,
                r#north: North::Low,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 23359 {
            return Some(TuffWall {
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::Low,
                r#east: East::None,
                r#north: North::None,
                r#up: false,
            });
        }
        if state_id == 23416 {
            return Some(TuffWall {
                r#north: North::Tall,
                r#south: South::None,
                r#east: East::None,
                r#west: West::Low,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 23343 {
            return Some(TuffWall {
                r#waterlogged: true,
                r#north: North::None,
                r#up: false,
                r#south: South::None,
                r#west: West::None,
                r#east: East::None,
            });
        }
        if state_id == 23391 {
            return Some(TuffWall {
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::Low,
                r#north: North::Low,
                r#east: East::None,
            });
        }
        if state_id == 23605 {
            return Some(TuffWall {
                r#east: East::Tall,
                r#south: South::Low,
                r#up: true,
                r#west: West::Low,
                r#north: North::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 23492 {
            return Some(TuffWall {
                r#south: South::None,
                r#north: North::Low,
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::Low,
                r#up: false,
            });
        }
        if state_id == 23645 {
            return Some(TuffWall {
                r#west: West::Tall,
                r#up: false,
                r#east: East::Tall,
                r#waterlogged: true,
                r#north: North::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 23417 {
            return Some(TuffWall {
                r#south: South::None,
                r#north: North::Tall,
                r#waterlogged: true,
                r#east: East::None,
                r#up: false,
                r#west: West::Tall,
            });
        }
        if state_id == 23405 {
            return Some(TuffWall {
                r#west: West::Tall,
                r#north: North::Low,
                r#waterlogged: true,
                r#east: East::None,
                r#south: South::Tall,
                r#up: false,
            });
        }
        if state_id == 23454 {
            return Some(TuffWall {
                r#up: false,
                r#waterlogged: false,
                r#east: East::Low,
                r#north: North::None,
                r#south: South::None,
                r#west: West::None,
            });
        }
        if state_id == 23653 {
            return Some(TuffWall {
                r#up: true,
                r#west: West::Low,
                r#waterlogged: false,
                r#north: North::Tall,
                r#east: East::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 23660 {
            return Some(TuffWall {
                r#east: East::Tall,
                r#waterlogged: false,
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: false,
                r#west: West::Tall,
            });
        }
        if state_id == 23479 {
            return Some(TuffWall {
                r#up: false,
                r#waterlogged: false,
                r#north: North::None,
                r#west: West::Low,
                r#south: South::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 23512 {
            return Some(TuffWall {
                r#up: false,
                r#west: West::Low,
                r#east: East::Low,
                r#south: South::Tall,
                r#waterlogged: true,
                r#north: North::Low,
            });
        }
        if state_id == 23406 {
            return Some(TuffWall {
                r#waterlogged: false,
                r#south: South::Tall,
                r#east: East::None,
                r#north: North::Low,
                r#up: false,
                r#west: West::None,
            });
        }
        if state_id == 23658 {
            return Some(TuffWall {
                r#south: South::Tall,
                r#waterlogged: false,
                r#north: North::Tall,
                r#up: false,
                r#east: East::Tall,
                r#west: West::None,
            });
        }
        if state_id == 23358 {
            return Some(TuffWall {
                r#west: West::None,
                r#up: false,
                r#south: South::Low,
                r#east: East::None,
                r#north: North::None,
                r#waterlogged: false,
            });
        }
        if state_id == 23383 {
            return Some(TuffWall {
                r#east: East::None,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::None,
            });
        }
        if state_id == 23338 {
            return Some(TuffWall {
                r#east: East::None,
                r#south: South::None,
                r#north: North::None,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 23402 {
            return Some(TuffWall {
                r#east: East::None,
                r#up: true,
                r#north: North::Low,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 23493 {
            return Some(TuffWall {
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::Low,
                r#east: East::Low,
                r#south: South::Low,
            });
        }
        return None;
    }
}

