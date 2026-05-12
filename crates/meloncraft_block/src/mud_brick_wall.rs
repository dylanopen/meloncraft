use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MudBrickWall {
    pub up: bool,
    pub waterlogged: bool,
    pub r#south: South,
    pub r#north: North,
    pub r#west: West,
    pub r#east: East,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum South {
    None,
    Low,
    Tall,
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
pub enum East {
    None,
    Low,
    Tall,
}

impl BlockState for MudBrickWall {
    fn to_id(&self) -> i32 {
        if self.r#west == West::Low && self.r#south == South::None && self.r#east == East::None && self.r#north == North::None && self.r#up == true && self.r#waterlogged == true { return 18237; }
        if self.r#south == South::Tall && self.r#waterlogged == true && self.r#up == true && self.r#north == North::None && self.r#east == East::None && self.r#west == West::Low { return 18261; }
        if self.r#north == North::Tall && self.r#south == South::Tall && self.r#waterlogged == false && self.r#east == East::Low && self.r#up == false && self.r#west == West::Low { return 18450; }
        if self.r#north == North::Tall && self.r#south == South::None && self.r#waterlogged == false && self.r#west == West::Tall && self.r#east == East::Low && self.r#up == false { return 18427; }
        if self.r#east == East::Low && self.r#west == West::Low && self.r#north == North::None && self.r#waterlogged == true && self.r#up == false && self.r#south == South::None { return 18351; }
        if self.r#east == East::Tall && self.r#north == North::Low && self.r#south == South::Low && self.r#up == true && self.r#west == West::Tall && self.r#waterlogged == false { return 18505; }
        if self.r#east == East::Low && self.r#west == West::None && self.r#up == true && self.r#north == North::None && self.r#waterlogged == false && self.r#south == South::None { return 18347; }
        if self.r#north == North::None && self.r#south == South::None && self.r#east == East::None && self.r#up == true && self.r#west == West::None && self.r#waterlogged == true { return 18236; }
        if self.r#north == North::Low && self.r#up == false && self.r#south == South::None && self.r#east == East::Tall && self.r#waterlogged == true && self.r#west == West::Tall { return 18496; }
        if self.r#east == East::Tall && self.r#south == South::None && self.r#up == false && self.r#north == North::Tall && self.r#waterlogged == false && self.r#west == West::None { return 18533; }
        if self.r#north == North::Tall && self.r#up == true && self.r#west == West::Low && self.r#waterlogged == false && self.r#east == East::Low && self.r#south == South::None { return 18420; }
        if self.r#east == East::Low && self.r#up == false && self.r#north == North::Tall && self.r#south == South::None && self.r#west == West::None && self.r#waterlogged == true { return 18422; }
        if self.r#waterlogged == true && self.r#north == North::Low && self.r#east == East::None && self.r#west == West::Low && self.r#south == South::Low && self.r#up == true { return 18285; }
        if self.r#north == North::None && self.r#up == false && self.r#west == West::None && self.r#waterlogged == false && self.r#east == East::Tall && self.r#south == South::None { return 18461; }
        if self.r#up == false && self.r#west == West::Low && self.r#east == East::None && self.r#waterlogged == true && self.r#south == South::None && self.r#north == North::Low { return 18279; }
        if self.r#east == East::Low && self.r#up == false && self.r#waterlogged == false && self.r#west == West::Low && self.r#north == North::Low && self.r#south == South::Tall { return 18414; }
        if self.r#north == North::None && self.r#waterlogged == false && self.r#west == West::None && self.r#east == East::None && self.r#south == South::Tall && self.r#up == false { return 18269; }
        if self.r#waterlogged == false && self.r#north == North::None && self.r#west == West::None && self.r#east == East::Low && self.r#up == true && self.r#south == South::Low { return 18359; }
        if self.r#east == East::Tall && self.r#waterlogged == false && self.r#west == West::None && self.r#up == true && self.r#north == North::Tall && self.r#south == South::None { return 18527; }
        if self.r#waterlogged == true && self.r#west == West::None && self.r#south == South::Low && self.r#north == North::Tall && self.r#east == East::Tall && self.r#up == false { return 18542; }
        if self.r#east == East::Tall && self.r#south == South::Low && self.r#north == North::Tall && self.r#up == false && self.r#waterlogged == false && self.r#west == West::Low { return 18546; }
        if self.r#north == North::Tall && self.r#up == false && self.r#east == East::None && self.r#west == West::Low && self.r#waterlogged == true && self.r#south == South::Low { return 18327; }
        if self.r#west == West::Low && self.r#east == East::None && self.r#north == North::None && self.r#waterlogged == true && self.r#up == false && self.r#south == South::Low { return 18255; }
        if self.r#east == East::None && self.r#north == North::Low && self.r#up == true && self.r#waterlogged == false && self.r#west == West::None && self.r#south == South::Tall { return 18299; }
        if self.r#north == North::None && self.r#south == South::Tall && self.r#east == East::Low && self.r#up == true && self.r#waterlogged == true && self.r#west == West::None { return 18368; }
        if self.r#up == false && self.r#west == West::Tall && self.r#north == North::Low && self.r#waterlogged == false && self.r#east == East::Tall && self.r#south == South::None { return 18499; }
        if self.r#up == true && self.r#waterlogged == true && self.r#north == North::Tall && self.r#west == West::None && self.r#south == South::Tall && self.r#east == East::None { return 18332; }
        if self.r#up == true && self.r#south == South::Low && self.r#waterlogged == false && self.r#west == West::Tall && self.r#east == East::None && self.r#north == North::Low { return 18289; }
        if self.r#east == East::Low && self.r#north == North::Low && self.r#up == false && self.r#waterlogged == false && self.r#south == South::None && self.r#west == West::None { return 18389; }
        if self.r#south == South::Low && self.r#north == North::Low && self.r#east == East::Tall && self.r#up == true && self.r#waterlogged == true && self.r#west == West::None { return 18500; }
        if self.r#west == West::Tall && self.r#up == true && self.r#south == South::None && self.r#east == East::Low && self.r#north == North::None && self.r#waterlogged == false { return 18349; }
        if self.r#west == West::Low && self.r#waterlogged == false && self.r#north == North::Tall && self.r#east == East::Tall && self.r#south == South::Low && self.r#up == true { return 18540; }
        if self.r#east == East::Low && self.r#north == North::Tall && self.r#waterlogged == true && self.r#west == West::None && self.r#up == false && self.r#south == South::Low { return 18434; }
        if self.r#north == North::Tall && self.r#south == South::None && self.r#waterlogged == true && self.r#west == West::Tall && self.r#up == true && self.r#east == East::None { return 18310; }
        if self.r#north == North::None && self.r#south == South::Low && self.r#waterlogged == false && self.r#west == West::Low && self.r#up == true && self.r#east == East::Low { return 18360; }
        if self.r#up == true && self.r#waterlogged == false && self.r#west == West::Low && self.r#north == North::Tall && self.r#east == East::None && self.r#south == South::Tall { return 18336; }
        if self.r#south == South::Low && self.r#east == East::None && self.r#up == true && self.r#waterlogged == true && self.r#west == West::None && self.r#north == North::None { return 18248; }
        if self.r#north == North::None && self.r#south == South::Low && self.r#up == true && self.r#east == East::Low && self.r#waterlogged == false && self.r#west == West::Tall { return 18361; }
        if self.r#north == North::Tall && self.r#waterlogged == false && self.r#west == West::Tall && self.r#up == false && self.r#east == East::None && self.r#south == South::Low { return 18331; }
        if self.r#west == West::None && self.r#south == South::Low && self.r#north == North::Tall && self.r#east == East::Tall && self.r#up == false && self.r#waterlogged == false { return 18545; }
        if self.r#up == false && self.r#north == North::None && self.r#west == West::Tall && self.r#east == East::None && self.r#waterlogged == false && self.r#south == South::Tall { return 18271; }
        if self.r#east == East::Low && self.r#up == false && self.r#south == South::None && self.r#waterlogged == false && self.r#west == West::Tall && self.r#north == North::None { return 18355; }
        if self.r#north == North::Tall && self.r#west == West::Tall && self.r#up == false && self.r#waterlogged == true && self.r#south == South::Tall && self.r#east == East::Low { return 18448; }
        if self.r#south == South::Tall && self.r#up == false && self.r#east == East::Low && self.r#west == West::Tall && self.r#waterlogged == false && self.r#north == North::None { return 18379; }
        if self.r#up == true && self.r#east == East::Tall && self.r#south == South::None && self.r#north == North::None && self.r#waterlogged == false && self.r#west == West::Low { return 18456; }
        if self.r#waterlogged == true && self.r#west == West::Tall && self.r#east == East::Tall && self.r#north == North::Tall && self.r#up == false && self.r#south == South::Low { return 18544; }
        if self.r#up == false && self.r#north == North::Low && self.r#waterlogged == false && self.r#west == West::Tall && self.r#south == South::Low && self.r#east == East::Low { return 18403; }
        if self.r#up == true && self.r#east == East::Low && self.r#north == North::Tall && self.r#waterlogged == false && self.r#west == West::Tall && self.r#south == South::Tall { return 18445; }
        if self.r#waterlogged == false && self.r#south == South::None && self.r#east == East::None && self.r#up == false && self.r#west == West::Tall && self.r#north == North::Low { return 18283; }
        if self.r#south == South::Low && self.r#west == West::Tall && self.r#up == true && self.r#waterlogged == true && self.r#east == East::Low && self.r#north == North::Low { return 18394; }
        if self.r#up == true && self.r#south == South::Low && self.r#waterlogged == false && self.r#north == North::None && self.r#east == East::None && self.r#west == West::None { return 18251; }
        if self.r#east == East::Tall && self.r#waterlogged == true && self.r#north == North::None && self.r#west == West::Tall && self.r#up == false && self.r#south == South::None { return 18460; }
        if self.r#up == false && self.r#south == South::None && self.r#east == East::None && self.r#north == North::None && self.r#waterlogged == false && self.r#west == West::Tall { return 18247; }
        if self.r#north == North::None && self.r#east == East::None && self.r#up == false && self.r#south == South::Low && self.r#waterlogged == false && self.r#west == West::None { return 18257; }
        if self.r#south == South::Low && self.r#north == North::Low && self.r#waterlogged == false && self.r#east == East::None && self.r#up == true && self.r#west == West::None { return 18287; }
        if self.r#waterlogged == false && self.r#west == West::Low && self.r#north == North::None && self.r#up == true && self.r#south == South::Low && self.r#east == East::Tall { return 18468; }
        if self.r#north == North::Low && self.r#west == West::None && self.r#south == South::Low && self.r#up == false && self.r#east == East::None && self.r#waterlogged == false { return 18293; }
        if self.r#west == West::Tall && self.r#east == East::None && self.r#south == South::Low && self.r#up == false && self.r#waterlogged == true && self.r#north == North::Tall { return 18328; }
        if self.r#east == East::Low && self.r#north == North::None && self.r#waterlogged == true && self.r#south == South::None && self.r#up == false && self.r#west == West::Tall { return 18352; }
        if self.r#east == East::None && self.r#west == West::Low && self.r#south == South::None && self.r#up == false && self.r#waterlogged == false && self.r#north == North::Tall { return 18318; }
        if self.r#north == North::Low && self.r#south == South::Tall && self.r#waterlogged == false && self.r#west == West::Tall && self.r#up == false && self.r#east == East::Tall { return 18523; }
        if self.r#west == West::Tall && self.r#north == North::None && self.r#east == East::Tall && self.r#waterlogged == true && self.r#up == true && self.r#south == South::None { return 18454; }
        if self.r#east == East::Low && self.r#south == South::Low && self.r#waterlogged == true && self.r#north == North::Tall && self.r#up == false && self.r#west == West::Tall { return 18436; }
        if self.r#waterlogged == true && self.r#north == North::Low && self.r#west == West::Low && self.r#east == East::Low && self.r#south == South::Tall && self.r#up == false { return 18411; }
        if self.r#west == West::None && self.r#north == North::Tall && self.r#east == East::Tall && self.r#waterlogged == false && self.r#south == South::Tall && self.r#up == true { return 18551; }
        if self.r#south == South::None && self.r#east == East::Tall && self.r#waterlogged == true && self.r#up == true && self.r#west == West::None && self.r#north == North::Tall { return 18524; }
        if self.r#east == East::Low && self.r#waterlogged == false && self.r#south == South::Low && self.r#west == West::None && self.r#north == North::Low && self.r#up == false { return 18401; }
        if self.r#up == false && self.r#north == North::Low && self.r#west == West::Low && self.r#waterlogged == false && self.r#east == East::Low && self.r#south == South::Low { return 18402; }
        if self.r#north == North::Tall && self.r#waterlogged == true && self.r#south == South::None && self.r#west == West::Low && self.r#east == East::Low && self.r#up == false { return 18423; }
        if self.r#east == East::None && self.r#up == true && self.r#waterlogged == false && self.r#north == North::None && self.r#west == West::None && self.r#south == South::Tall { return 18263; }
        if self.r#waterlogged == true && self.r#west == West::Low && self.r#east == East::Tall && self.r#north == North::Low && self.r#south == South::Tall && self.r#up == true { return 18513; }
        if self.r#north == North::Low && self.r#waterlogged == true && self.r#south == South::Tall && self.r#up == false && self.r#east == East::Tall && self.r#west == West::Low { return 18519; }
        if self.r#west == West::None && self.r#up == true && self.r#east == East::Low && self.r#south == South::Low && self.r#north == North::Tall && self.r#waterlogged == true { return 18428; }
        if self.r#east == East::None && self.r#north == North::Low && self.r#up == false && self.r#south == South::None && self.r#waterlogged == true && self.r#west == West::Tall { return 18280; }
        if self.r#west == West::Low && self.r#waterlogged == false && self.r#east == East::Tall && self.r#north == North::Low && self.r#south == South::Low && self.r#up == true { return 18504; }
        if self.r#east == East::None && self.r#north == North::Low && self.r#waterlogged == false && self.r#west == West::Low && self.r#south == South::None && self.r#up == false { return 18282; }
        if self.r#east == East::Tall && self.r#waterlogged == false && self.r#north == North::None && self.r#south == South::Tall && self.r#west == West::Tall && self.r#up == false { return 18487; }
        if self.r#south == South::Low && self.r#up == true && self.r#east == East::Low && self.r#waterlogged == false && self.r#west == West::Tall && self.r#north == North::Tall { return 18433; }
        if self.r#waterlogged == true && self.r#west == West::Low && self.r#north == North::Low && self.r#east == East::None && self.r#south == South::Tall && self.r#up == true { return 18297; }
        if self.r#south == South::None && self.r#east == East::Tall && self.r#north == North::Tall && self.r#up == true && self.r#waterlogged == true && self.r#west == West::Low { return 18525; }
        if self.r#north == North::Tall && self.r#waterlogged == true && self.r#east == East::Low && self.r#up == true && self.r#south == South::Tall && self.r#west == West::Tall { return 18442; }
        if self.r#west == West::Low && self.r#waterlogged == false && self.r#north == North::None && self.r#south == South::None && self.r#up == false && self.r#east == East::Low { return 18354; }
        if self.r#waterlogged == true && self.r#east == East::Low && self.r#south == South::None && self.r#north == North::None && self.r#up == true && self.r#west == West::None { return 18344; }
        if self.r#north == North::Low && self.r#up == false && self.r#east == East::Low && self.r#waterlogged == true && self.r#west == West::None && self.r#south == South::Low { return 18398; }
        if self.r#west == West::None && self.r#waterlogged == true && self.r#north == North::Low && self.r#south == South::Low && self.r#up == false && self.r#east == East::Tall { return 18506; }
        if self.r#up == false && self.r#west == West::None && self.r#east == East::Tall && self.r#north == North::Tall && self.r#south == South::Tall && self.r#waterlogged == false { return 18557; }
        if self.r#west == West::Low && self.r#up == false && self.r#north == North::None && self.r#east == East::Tall && self.r#south == South::None && self.r#waterlogged == false { return 18462; }
        if self.r#north == North::Low && self.r#up == true && self.r#waterlogged == false && self.r#east == East::Low && self.r#west == West::Low && self.r#south == South::Tall { return 18408; }
        if self.r#south == South::Low && self.r#east == East::None && self.r#up == false && self.r#west == West::None && self.r#north == North::None && self.r#waterlogged == true { return 18254; }
        if self.r#south == South::Low && self.r#north == North::Low && self.r#west == West::Tall && self.r#east == East::Low && self.r#up == false && self.r#waterlogged == true { return 18400; }
        if self.r#waterlogged == false && self.r#north == North::Low && self.r#up == true && self.r#east == East::Tall && self.r#west == West::None && self.r#south == South::Tall { return 18515; }
        if self.r#north == North::Low && self.r#up == false && self.r#east == East::Tall && self.r#south == South::Tall && self.r#west == West::None && self.r#waterlogged == false { return 18521; }
        if self.r#east == East::None && self.r#south == South::None && self.r#north == North::Tall && self.r#up == true && self.r#waterlogged == false && self.r#west == West::Tall { return 18313; }
        if self.r#north == North::Low && self.r#waterlogged == false && self.r#west == West::None && self.r#up == false && self.r#east == East::Tall && self.r#south == South::None { return 18497; }
        if self.r#north == North::Tall && self.r#up == false && self.r#east == East::Low && self.r#waterlogged == false && self.r#south == South::Low && self.r#west == West::None { return 18437; }
        if self.r#waterlogged == true && self.r#north == North::None && self.r#west == West::Tall && self.r#south == South::Low && self.r#up == false && self.r#east == East::Low { return 18364; }
        if self.r#north == North::Tall && self.r#south == South::Tall && self.r#east == East::Tall && self.r#up == true && self.r#waterlogged == true && self.r#west == West::Tall { return 18550; }
        if self.r#up == true && self.r#west == West::Low && self.r#waterlogged == false && self.r#south == South::Low && self.r#east == East::Low && self.r#north == North::Low { return 18396; }
        if self.r#east == East::None && self.r#north == North::Tall && self.r#south == South::None && self.r#west == West::Low && self.r#waterlogged == true && self.r#up == true { return 18309; }
        if self.r#south == South::None && self.r#up == false && self.r#north == North::Low && self.r#waterlogged == false && self.r#west == West::None && self.r#east == East::None { return 18281; }
        if self.r#up == false && self.r#south == South::Low && self.r#north == North::None && self.r#east == East::None && self.r#waterlogged == false && self.r#west == West::Tall { return 18259; }
        if self.r#up == false && self.r#waterlogged == true && self.r#east == East::None && self.r#north == North::None && self.r#west == West::None && self.r#south == South::Tall { return 18266; }
        if self.r#south == South::Low && self.r#west == West::Tall && self.r#up == false && self.r#north == North::Low && self.r#east == East::None && self.r#waterlogged == false { return 18295; }
        if self.r#east == East::None && self.r#waterlogged == false && self.r#north == North::None && self.r#up == true && self.r#south == South::None && self.r#west == West::Tall { return 18241; }
        if self.r#up == true && self.r#south == South::Tall && self.r#north == North::Low && self.r#waterlogged == false && self.r#east == East::None && self.r#west == West::Tall { return 18301; }
        if self.r#waterlogged == false && self.r#up == false && self.r#west == West::None && self.r#north == North::Low && self.r#east == East::Low && self.r#south == South::Tall { return 18413; }
        if self.r#north == North::Low && self.r#waterlogged == true && self.r#west == West::Tall && self.r#east == East::Low && self.r#up == false && self.r#south == South::Tall { return 18412; }
        if self.r#up == true && self.r#waterlogged == true && self.r#south == South::Low && self.r#east == East::Low && self.r#north == North::Tall && self.r#west == West::Tall { return 18430; }
        if self.r#up == true && self.r#north == North::None && self.r#west == West::None && self.r#east == East::Tall && self.r#south == South::Tall && self.r#waterlogged == true { return 18476; }
        if self.r#up == true && self.r#north == North::Tall && self.r#east == East::Tall && self.r#waterlogged == false && self.r#west == West::Tall && self.r#south == South::None { return 18529; }
        if self.r#east == East::None && self.r#west == West::Tall && self.r#south == South::None && self.r#up == false && self.r#waterlogged == false && self.r#north == North::Tall { return 18319; }
        if self.r#up == false && self.r#waterlogged == false && self.r#north == North::None && self.r#south == South::Low && self.r#west == West::Low && self.r#east == East::Low { return 18366; }
        if self.r#west == West::None && self.r#north == North::Low && self.r#up == true && self.r#east == East::Low && self.r#south == South::Low && self.r#waterlogged == false { return 18395; }
        if self.r#up == true && self.r#east == East::Low && self.r#waterlogged == true && self.r#west == West::Low && self.r#north == North::Low && self.r#south == South::Low { return 18393; }
        if self.r#waterlogged == true && self.r#south == South::None && self.r#up == false && self.r#north == North::None && self.r#west == West::Low && self.r#east == East::None { return 18243; }
        if self.r#west == West::Tall && self.r#north == North::Tall && self.r#waterlogged == false && self.r#south == South::None && self.r#east == East::Low && self.r#up == true { return 18421; }
        if self.r#south == South::Low && self.r#north == North::Low && self.r#up == false && self.r#east == East::None && self.r#waterlogged == false && self.r#west == West::Low { return 18294; }
        if self.r#waterlogged == true && self.r#up == true && self.r#east == East::Tall && self.r#north == North::Tall && self.r#south == South::Low && self.r#west == West::Low { return 18537; }
        if self.r#up == true && self.r#waterlogged == true && self.r#west == West::Tall && self.r#east == East::None && self.r#south == South::Tall && self.r#north == North::Tall { return 18334; }
        if self.r#south == South::None && self.r#east == East::Tall && self.r#north == North::Tall && self.r#up == true && self.r#waterlogged == false && self.r#west == West::Low { return 18528; }
        if self.r#south == South::None && self.r#east == East::Tall && self.r#north == North::None && self.r#waterlogged == false && self.r#west == West::None && self.r#up == true { return 18455; }
        if self.r#north == North::Tall && self.r#west == West::Low && self.r#south == South::Tall && self.r#up == true && self.r#east == East::None && self.r#waterlogged == true { return 18333; }
        if self.r#east == East::Tall && self.r#south == South::None && self.r#up == false && self.r#waterlogged == true && self.r#north == North::Tall && self.r#west == West::None { return 18530; }
        if self.r#north == North::None && self.r#waterlogged == true && self.r#east == East::Tall && self.r#up == false && self.r#south == South::Tall && self.r#west == West::Tall { return 18484; }
        if self.r#east == East::Tall && self.r#waterlogged == false && self.r#north == North::Tall && self.r#south == South::Tall && self.r#west == West::Low && self.r#up == false { return 18558; }
        if self.r#east == East::Tall && self.r#west == West::Tall && self.r#north == North::Tall && self.r#south == South::Low && self.r#up == true && self.r#waterlogged == true { return 18538; }
        if self.r#west == West::Tall && self.r#up == true && self.r#waterlogged == false && self.r#east == East::Tall && self.r#north == North::Tall && self.r#south == South::Low { return 18541; }
        if self.r#up == true && self.r#east == East::Tall && self.r#west == West::None && self.r#waterlogged == false && self.r#north == North::Tall && self.r#south == South::Low { return 18539; }
        if self.r#up == false && self.r#south == South::Low && self.r#west == West::None && self.r#north == North::Tall && self.r#east == East::None && self.r#waterlogged == true { return 18326; }
        if self.r#up == true && self.r#east == East::Low && self.r#north == North::Low && self.r#west == West::Tall && self.r#south == South::Tall && self.r#waterlogged == true { return 18406; }
        if self.r#waterlogged == true && self.r#up == true && self.r#east == East::Low && self.r#north == North::None && self.r#south == South::None && self.r#west == West::Tall { return 18346; }
        if self.r#west == West::Tall && self.r#north == North::Tall && self.r#south == South::Low && self.r#up == false && self.r#waterlogged == false && self.r#east == East::Low { return 18439; }
        if self.r#west == West::Tall && self.r#waterlogged == false && self.r#north == North::Tall && self.r#south == South::Low && self.r#east == East::None && self.r#up == true { return 18325; }
        if self.r#north == North::None && self.r#south == South::Tall && self.r#west == West::None && self.r#up == true && self.r#east == East::Low && self.r#waterlogged == false { return 18371; }
        if self.r#south == South::Low && self.r#north == North::Tall && self.r#waterlogged == true && self.r#east == East::Tall && self.r#west == West::Low && self.r#up == false { return 18543; }
        if self.r#west == West::Low && self.r#east == East::Low && self.r#up == false && self.r#north == North::None && self.r#waterlogged == false && self.r#south == South::Tall { return 18378; }
        if self.r#west == West::None && self.r#east == East::None && self.r#north == North::Tall && self.r#waterlogged == false && self.r#south == South::None && self.r#up == false { return 18317; }
        if self.r#east == East::None && self.r#waterlogged == true && self.r#west == West::None && self.r#up == true && self.r#south == South::Low && self.r#north == North::Low { return 18284; }
        if self.r#up == true && self.r#west == West::None && self.r#south == South::None && self.r#north == North::Tall && self.r#east == East::Low && self.r#waterlogged == true { return 18416; }
        if self.r#up == true && self.r#west == West::Tall && self.r#east == East::Tall && self.r#north == North::None && self.r#waterlogged == true && self.r#south == South::Low { return 18466; }
        if self.r#up == false && self.r#east == East::Low && self.r#west == West::Low && self.r#south == South::Low && self.r#waterlogged == true && self.r#north == North::None { return 18363; }
        if self.r#north == North::Tall && self.r#up == true && self.r#waterlogged == false && self.r#west == West::None && self.r#east == East::Low && self.r#south == South::Tall { return 18443; }
        if self.r#waterlogged == true && self.r#south == South::Low && self.r#east == East::Tall && self.r#up == true && self.r#west == West::Tall && self.r#north == North::Low { return 18502; }
        if self.r#waterlogged == false && self.r#south == South::Tall && self.r#west == West::Tall && self.r#east == East::None && self.r#up == true && self.r#north == North::None { return 18265; }
        if self.r#west == West::None && self.r#north == North::Tall && self.r#up == false && self.r#east == East::Low && self.r#waterlogged == false && self.r#south == South::None { return 18425; }
        if self.r#north == North::None && self.r#west == West::None && self.r#up == true && self.r#east == East::Tall && self.r#south == South::Tall && self.r#waterlogged == false { return 18479; }
        if self.r#west == West::Low && self.r#north == North::Tall && self.r#up == false && self.r#east == East::Low && self.r#waterlogged == false && self.r#south == South::Low { return 18438; }
        if self.r#south == South::Low && self.r#east == East::None && self.r#north == North::None && self.r#up == true && self.r#waterlogged == true && self.r#west == West::Tall { return 18250; }
        if self.r#waterlogged == true && self.r#east == East::None && self.r#north == North::Tall && self.r#up == false && self.r#south == South::Tall && self.r#west == West::None { return 18338; }
        if self.r#south == South::Tall && self.r#waterlogged == false && self.r#west == West::None && self.r#north == North::Tall && self.r#up == false && self.r#east == East::Low { return 18449; }
        if self.r#waterlogged == false && self.r#west == West::Tall && self.r#up == true && self.r#north == North::None && self.r#south == South::Low && self.r#east == East::Tall { return 18469; }
        if self.r#north == North::Low && self.r#waterlogged == true && self.r#west == West::None && self.r#south == South::None && self.r#up == false && self.r#east == East::Tall { return 18494; }
        if self.r#waterlogged == false && self.r#south == South::Low && self.r#east == East::Low && self.r#up == true && self.r#west == West::Low && self.r#north == North::Tall { return 18432; }
        if self.r#west == West::Low && self.r#up == true && self.r#north == North::None && self.r#waterlogged == false && self.r#south == South::Tall && self.r#east == East::Tall { return 18480; }
        if self.r#east == East::Tall && self.r#north == North::None && self.r#west == West::None && self.r#waterlogged == true && self.r#south == South::Low && self.r#up == false { return 18470; }
        if self.r#up == true && self.r#waterlogged == true && self.r#west == West::None && self.r#south == South::Low && self.r#east == East::Tall && self.r#north == North::Tall { return 18536; }
        if self.r#north == North::Tall && self.r#waterlogged == false && self.r#south == South::None && self.r#up == false && self.r#west == West::Low && self.r#east == East::Tall { return 18534; }
        if self.r#west == West::None && self.r#south == South::Tall && self.r#north == North::Tall && self.r#east == East::None && self.r#up == false && self.r#waterlogged == false { return 18341; }
        if self.r#up == true && self.r#waterlogged == false && self.r#west == West::Low && self.r#east == East::None && self.r#north == North::Low && self.r#south == South::Low { return 18288; }
        if self.r#east == East::Tall && self.r#south == South::None && self.r#up == false && self.r#west == West::Low && self.r#north == North::None && self.r#waterlogged == true { return 18459; }
        if self.r#south == South::Tall && self.r#waterlogged == true && self.r#west == West::Tall && self.r#east == East::None && self.r#up == false && self.r#north == North::Low { return 18304; }
        if self.r#waterlogged == true && self.r#east == East::Tall && self.r#up == false && self.r#west == West::Tall && self.r#north == North::Tall && self.r#south == South::Tall { return 18556; }
        if self.r#west == West::Tall && self.r#north == North::Tall && self.r#waterlogged == true && self.r#up == true && self.r#east == East::Low && self.r#south == South::None { return 18418; }
        if self.r#west == West::Tall && self.r#south == South::None && self.r#north == North::Low && self.r#east == East::Tall && self.r#up == true && self.r#waterlogged == true { return 18490; }
        if self.r#south == South::Tall && self.r#east == East::None && self.r#north == North::Low && self.r#west == West::Low && self.r#waterlogged == true && self.r#up == false { return 18303; }
        if self.r#west == West::Low && self.r#up == true && self.r#east == East::Low && self.r#north == North::None && self.r#south == South::None && self.r#waterlogged == false { return 18348; }
        if self.r#east == East::Low && self.r#west == West::Tall && self.r#up == false && self.r#south == South::None && self.r#waterlogged == true && self.r#north == North::Low { return 18388; }
        if self.r#east == East::None && self.r#up == false && self.r#north == North::None && self.r#west == West::Low && self.r#waterlogged == false && self.r#south == South::None { return 18246; }
        if self.r#up == true && self.r#north == North::Low && self.r#waterlogged == true && self.r#east == East::Low && self.r#south == South::Low && self.r#west == West::None { return 18392; }
        if self.r#east == East::Tall && self.r#up == true && self.r#waterlogged == false && self.r#west == West::Tall && self.r#south == South::None && self.r#north == North::None { return 18457; }
        if self.r#up == true && self.r#waterlogged == true && self.r#west == West::Tall && self.r#south == South::Low && self.r#east == East::None && self.r#north == North::Tall { return 18322; }
        if self.r#west == West::Low && self.r#north == North::Low && self.r#up == false && self.r#east == East::Tall && self.r#south == South::Low && self.r#waterlogged == true { return 18507; }
        if self.r#south == South::None && self.r#up == false && self.r#waterlogged == true && self.r#east == East::None && self.r#north == North::None && self.r#west == West::Tall { return 18244; }
        if self.r#east == East::Tall && self.r#north == North::Low && self.r#south == South::Tall && self.r#up == true && self.r#waterlogged == true && self.r#west == West::None { return 18512; }
        if self.r#east == East::Low && self.r#up == false && self.r#north == North::None && self.r#south == South::Tall && self.r#west == West::None && self.r#waterlogged == true { return 18374; }
        if self.r#up == true && self.r#west == West::None && self.r#east == East::Low && self.r#south == South::Low && self.r#north == North::None && self.r#waterlogged == true { return 18356; }
        if self.r#up == true && self.r#south == South::None && self.r#north == North::Tall && self.r#west == West::None && self.r#waterlogged == true && self.r#east == East::None { return 18308; }
        if self.r#up == false && self.r#south == South::Tall && self.r#waterlogged == true && self.r#west == West::None && self.r#east == East::Tall && self.r#north == North::None { return 18482; }
        if self.r#east == East::None && self.r#north == North::Tall && self.r#up == false && self.r#waterlogged == true && self.r#south == South::None && self.r#west == West::Low { return 18315; }
        if self.r#north == North::None && self.r#west == West::Tall && self.r#south == South::Low && self.r#up == false && self.r#waterlogged == false && self.r#east == East::Low { return 18367; }
        if self.r#west == West::None && self.r#east == East::Tall && self.r#waterlogged == false && self.r#north == North::None && self.r#south == South::Low && self.r#up == true { return 18467; }
        if self.r#north == North::None && self.r#up == false && self.r#waterlogged == true && self.r#west == West::Low && self.r#south == South::Low && self.r#east == East::Tall { return 18471; }
        if self.r#east == East::None && self.r#north == North::Low && self.r#waterlogged == false && self.r#south == South::Tall && self.r#up == false && self.r#west == West::None { return 18305; }
        if self.r#east == East::None && self.r#west == West::None && self.r#waterlogged == true && self.r#south == South::None && self.r#up == false && self.r#north == North::None { return 18242; }
        if self.r#south == South::Tall && self.r#up == true && self.r#west == West::None && self.r#waterlogged == true && self.r#north == North::Tall && self.r#east == East::Low { return 18440; }
        if self.r#north == North::None && self.r#waterlogged == true && self.r#up == true && self.r#west == West::Tall && self.r#south == South::Tall && self.r#east == East::None { return 18262; }
        if self.r#waterlogged == true && self.r#up == false && self.r#east == East::Low && self.r#south == South::Low && self.r#west == West::Low && self.r#north == North::Low { return 18399; }
        if self.r#west == West::Low && self.r#south == South::Tall && self.r#waterlogged == false && self.r#east == East::Low && self.r#north == North::None && self.r#up == true { return 18372; }
        if self.r#south == South::Tall && self.r#west == West::Tall && self.r#east == East::Low && self.r#waterlogged == false && self.r#north == North::None && self.r#up == true { return 18373; }
        if self.r#waterlogged == false && self.r#south == South::Tall && self.r#north == North::Low && self.r#east == East::Tall && self.r#up == true && self.r#west == West::Low { return 18516; }
        if self.r#up == true && self.r#east == East::None && self.r#north == North::Tall && self.r#south == South::None && self.r#waterlogged == false && self.r#west == West::Low { return 18312; }
        if self.r#north == North::Low && self.r#south == South::Low && self.r#east == East::Tall && self.r#up == true && self.r#waterlogged == true && self.r#west == West::Low { return 18501; }
        if self.r#north == North::None && self.r#waterlogged == true && self.r#south == South::Low && self.r#east == East::Tall && self.r#up == false && self.r#west == West::Tall { return 18472; }
        if self.r#east == East::Tall && self.r#south == South::None && self.r#west == West::Tall && self.r#north == North::None && self.r#up == false && self.r#waterlogged == false { return 18463; }
        if self.r#up == false && self.r#north == North::Low && self.r#east == East::Tall && self.r#south == South::None && self.r#waterlogged == true && self.r#west == West::Low { return 18495; }
        if self.r#west == West::Tall && self.r#north == North::Low && self.r#south == South::Tall && self.r#up == false && self.r#east == East::None && self.r#waterlogged == false { return 18307; }
        if self.r#up == false && self.r#east == East::None && self.r#north == North::Tall && self.r#waterlogged == true && self.r#south == South::Tall && self.r#west == West::Low { return 18339; }
        if self.r#east == East::None && self.r#west == West::Tall && self.r#south == South::None && self.r#north == North::None && self.r#up == true && self.r#waterlogged == true { return 18238; }
        if self.r#north == North::Low && self.r#south == South::None && self.r#waterlogged == false && self.r#east == East::Low && self.r#west == West::Tall && self.r#up == false { return 18391; }
        if self.r#up == false && self.r#waterlogged == true && self.r#west == West::None && self.r#south == South::Tall && self.r#east == East::Low && self.r#north == North::Low { return 18410; }
        if self.r#waterlogged == true && self.r#west == West::Low && self.r#south == South::Low && self.r#east == East::Low && self.r#up == false && self.r#north == North::Tall { return 18435; }
        if self.r#up == false && self.r#east == East::None && self.r#waterlogged == true && self.r#south == South::Low && self.r#west == West::Tall && self.r#north == North::None { return 18256; }
        if self.r#up == false && self.r#north == North::None && self.r#west == West::Low && self.r#east == East::None && self.r#south == South::Low && self.r#waterlogged == false { return 18258; }
        if self.r#east == East::None && self.r#north == North::Low && self.r#west == West::Tall && self.r#south == South::Low && self.r#up == true && self.r#waterlogged == true { return 18286; }
        if self.r#waterlogged == true && self.r#south == South::Tall && self.r#west == West::Low && self.r#north == North::None && self.r#up == true && self.r#east == East::Low { return 18369; }
        if self.r#up == true && self.r#waterlogged == true && self.r#east == East::Tall && self.r#west == West::None && self.r#south == South::Low && self.r#north == North::None { return 18464; }
        if self.r#east == East::None && self.r#south == South::Low && self.r#waterlogged == false && self.r#west == West::Low && self.r#up == false && self.r#north == North::Tall { return 18330; }
        if self.r#east == East::None && self.r#up == true && self.r#waterlogged == false && self.r#north == North::None && self.r#south == South::Low && self.r#west == West::Tall { return 18253; }
        if self.r#east == East::Tall && self.r#north == North::Low && self.r#south == South::Low && self.r#up == false && self.r#waterlogged == true && self.r#west == West::Tall { return 18508; }
        if self.r#west == West::Low && self.r#up == true && self.r#south == South::Low && self.r#east == East::None && self.r#north == North::Tall && self.r#waterlogged == false { return 18324; }
        if self.r#south == South::None && self.r#north == North::Tall && self.r#waterlogged == true && self.r#east == East::None && self.r#up == false && self.r#west == West::Tall { return 18316; }
        if self.r#west == West::None && self.r#waterlogged == false && self.r#east == East::None && self.r#north == North::None && self.r#south == South::None && self.r#up == false { return 18245; }
        if self.r#up == true && self.r#south == South::Low && self.r#waterlogged == false && self.r#east == East::Low && self.r#north == North::Tall && self.r#west == West::None { return 18431; }
        if self.r#east == East::Tall && self.r#waterlogged == false && self.r#up == false && self.r#north == North::None && self.r#west == West::Low && self.r#south == South::Tall { return 18486; }
        if self.r#south == South::Tall && self.r#east == East::Tall && self.r#up == false && self.r#west == West::Low && self.r#north == North::Low && self.r#waterlogged == false { return 18522; }
        if self.r#up == false && self.r#east == East::None && self.r#waterlogged == false && self.r#west == West::Tall && self.r#south == South::Tall && self.r#north == North::Tall { return 18343; }
        if self.r#south == South::Low && self.r#east == East::None && self.r#up == false && self.r#north == North::Tall && self.r#west == West::None && self.r#waterlogged == false { return 18329; }
        if self.r#north == North::Tall && self.r#up == true && self.r#waterlogged == true && self.r#east == East::Low && self.r#west == West::Low && self.r#south == South::None { return 18417; }
        if self.r#up == true && self.r#east == East::Tall && self.r#south == South::Tall && self.r#west == West::None && self.r#north == North::Tall && self.r#waterlogged == true { return 18548; }
        if self.r#north == North::Low && self.r#up == true && self.r#west == West::Tall && self.r#waterlogged == false && self.r#east == East::Low && self.r#south == South::Tall { return 18409; }
        if self.r#west == West::Low && self.r#up == false && self.r#east == East::None && self.r#south == South::Tall && self.r#north == North::Low && self.r#waterlogged == false { return 18306; }
        if self.r#east == East::Tall && self.r#up == false && self.r#waterlogged == true && self.r#west == West::None && self.r#south == South::None && self.r#north == North::None { return 18458; }
        if self.r#up == true && self.r#north == North::Low && self.r#west == West::Tall && self.r#east == East::Tall && self.r#waterlogged == false && self.r#south == South::Tall { return 18517; }
        if self.r#up == false && self.r#east == East::Tall && self.r#south == South::None && self.r#waterlogged == true && self.r#north == North::Tall && self.r#west == West::Tall { return 18532; }
        if self.r#up == false && self.r#east == East::Tall && self.r#waterlogged == true && self.r#west == West::Low && self.r#north == North::Tall && self.r#south == South::Tall { return 18555; }
        if self.r#south == South::Low && self.r#up == true && self.r#north == North::Low && self.r#east == East::Tall && self.r#waterlogged == false && self.r#west == West::None { return 18503; }
        if self.r#up == true && self.r#south == South::None && self.r#waterlogged == false && self.r#west == West::Low && self.r#east == East::Tall && self.r#north == North::Low { return 18492; }
        if self.r#waterlogged == true && self.r#south == South::None && self.r#east == East::Low && self.r#north == North::Tall && self.r#up == false && self.r#west == West::Tall { return 18424; }
        if self.r#south == South::Low && self.r#east == East::Tall && self.r#waterlogged == false && self.r#west == West::Low && self.r#north == North::None && self.r#up == false { return 18474; }
        if self.r#west == West::Low && self.r#north == North::Tall && self.r#east == East::Low && self.r#up == false && self.r#waterlogged == true && self.r#south == South::Tall { return 18447; }
        if self.r#west == West::Tall && self.r#north == North::Low && self.r#waterlogged == false && self.r#south == South::Low && self.r#east == East::Tall && self.r#up == false { return 18511; }
        if self.r#south == South::Low && self.r#east == East::Tall && self.r#up == false && self.r#waterlogged == false && self.r#west == West::Tall && self.r#north == North::None { return 18475; }
        if self.r#south == South::Low && self.r#east == East::Tall && self.r#west == West::Tall && self.r#north == North::Tall && self.r#up == false && self.r#waterlogged == false { return 18547; }
        if self.r#north == North::Low && self.r#waterlogged == false && self.r#west == West::None && self.r#south == South::None && self.r#east == East::Tall && self.r#up == true { return 18491; }
        if self.r#east == East::None && self.r#west == West::None && self.r#north == North::None && self.r#waterlogged == true && self.r#south == South::Tall && self.r#up == true { return 18260; }
        if self.r#waterlogged == false && self.r#west == West::None && self.r#north == North::Tall && self.r#east == East::None && self.r#south == South::None && self.r#up == true { return 18311; }
        if self.r#south == South::Tall && self.r#up == false && self.r#north == North::Tall && self.r#waterlogged == true && self.r#west == West::Tall && self.r#east == East::None { return 18340; }
        if self.r#north == North::Low && self.r#up == true && self.r#waterlogged == true && self.r#south == South::None && self.r#east == East::Tall && self.r#west == West::None { return 18488; }
        if self.r#east == East::None && self.r#north == North::Tall && self.r#up == false && self.r#west == West::None && self.r#waterlogged == true && self.r#south == South::None { return 18314; }
        if self.r#east == East::Low && self.r#south == South::None && self.r#west == West::Low && self.r#waterlogged == true && self.r#up == true && self.r#north == North::Low { return 18381; }
        if self.r#east == East::Tall && self.r#north == North::Low && self.r#waterlogged == false && self.r#west == West::Low && self.r#up == false && self.r#south == South::None { return 18498; }
        if self.r#up == false && self.r#east == East::Tall && self.r#waterlogged == false && self.r#west == West::Low && self.r#north == North::Low && self.r#south == South::Low { return 18510; }
        if self.r#north == North::None && self.r#waterlogged == true && self.r#west == West::Low && self.r#up == false && self.r#east == East::None && self.r#south == South::Tall { return 18267; }
        if self.r#north == North::None && self.r#waterlogged == true && self.r#east == East::Low && self.r#south == South::Tall && self.r#up == false && self.r#west == West::Tall { return 18376; }
        if self.r#east == East::Low && self.r#up == true && self.r#waterlogged == false && self.r#north == North::Low && self.r#west == West::Tall && self.r#south == South::Low { return 18397; }
        if self.r#north == North::Low && self.r#west == West::Low && self.r#up == false && self.r#waterlogged == true && self.r#east == East::None && self.r#south == South::Low { return 18291; }
        if self.r#south == South::None && self.r#waterlogged == true && self.r#north == North::Low && self.r#east == East::Low && self.r#up == false && self.r#west == West::None { return 18386; }
        if self.r#north == North::None && self.r#waterlogged == true && self.r#south == South::Low && self.r#east == East::None && self.r#west == West::Low && self.r#up == true { return 18249; }
        if self.r#waterlogged == true && self.r#east == East::None && self.r#up == false && self.r#south == South::Tall && self.r#north == North::Low && self.r#west == West::None { return 18302; }
        if self.r#west == West::None && self.r#east == East::None && self.r#up == true && self.r#waterlogged == false && self.r#south == South::Low && self.r#north == North::Tall { return 18323; }
        if self.r#up == false && self.r#west == West::None && self.r#south == South::Tall && self.r#waterlogged == true && self.r#north == North::Tall && self.r#east == East::Tall { return 18554; }
        if self.r#east == East::Tall && self.r#up == true && self.r#waterlogged == true && self.r#north == North::Tall && self.r#west == West::Tall && self.r#south == South::None { return 18526; }
        if self.r#up == true && self.r#east == East::None && self.r#north == North::Low && self.r#waterlogged == false && self.r#west == West::Low && self.r#south == South::None { return 18276; }
        if self.r#south == South::Low && self.r#waterlogged == true && self.r#east == East::Low && self.r#up == true && self.r#west == West::Tall && self.r#north == North::None { return 18358; }
        if self.r#south == South::Low && self.r#west == West::None && self.r#north == North::Low && self.r#up == false && self.r#waterlogged == true && self.r#east == East::None { return 18290; }
        if self.r#waterlogged == true && self.r#west == West::Tall && self.r#south == South::Tall && self.r#east == East::Tall && self.r#north == North::Low && self.r#up == false { return 18520; }
        if self.r#east == East::Tall && self.r#west == West::None && self.r#up == false && self.r#north == North::Low && self.r#waterlogged == false && self.r#south == South::Low { return 18509; }
        if self.r#up == true && self.r#east == East::Low && self.r#west == West::None && self.r#north == North::Tall && self.r#waterlogged == false && self.r#south == South::None { return 18419; }
        if self.r#west == West::Low && self.r#up == false && self.r#east == East::Low && self.r#waterlogged == true && self.r#north == North::None && self.r#south == South::Tall { return 18375; }
        if self.r#north == North::Low && self.r#waterlogged == true && self.r#east == East::Low && self.r#south == South::None && self.r#up == true && self.r#west == West::None { return 18380; }
        if self.r#waterlogged == true && self.r#west == West::None && self.r#south == South::None && self.r#north == North::None && self.r#up == true && self.r#east == East::Tall { return 18452; }
        if self.r#east == East::Tall && self.r#up == false && self.r#south == South::Tall && self.r#north == North::Low && self.r#waterlogged == true && self.r#west == West::None { return 18518; }
        if self.r#up == true && self.r#east == East::Tall && self.r#waterlogged == false && self.r#west == West::Low && self.r#north == North::Tall && self.r#south == South::Tall { return 18552; }
        if self.r#up == false && self.r#waterlogged == false && self.r#south == South::None && self.r#east == East::Low && self.r#west == West::Low && self.r#north == North::Tall { return 18426; }
        if self.r#south == South::Low && self.r#east == East::Tall && self.r#west == West::None && self.r#up == false && self.r#waterlogged == false && self.r#north == North::None { return 18473; }
        if self.r#west == West::None && self.r#east == East::Tall && self.r#up == false && self.r#waterlogged == false && self.r#south == South::Tall && self.r#north == North::None { return 18485; }
        if self.r#waterlogged == true && self.r#south == South::Tall && self.r#east == East::Tall && self.r#west == West::Low && self.r#north == North::None && self.r#up == true { return 18477; }
        if self.r#north == North::Tall && self.r#south == South::Tall && self.r#waterlogged == true && self.r#west == West::Low && self.r#east == East::Tall && self.r#up == true { return 18549; }
        if self.r#waterlogged == false && self.r#south == South::Tall && self.r#north == North::Tall && self.r#up == true && self.r#west == West::Tall && self.r#east == East::Tall { return 18553; }
        if self.r#waterlogged == true && self.r#south == South::Tall && self.r#up == false && self.r#west == West::Low && self.r#east == East::Tall && self.r#north == North::None { return 18483; }
        if self.r#waterlogged == false && self.r#west == West::None && self.r#east == East::None && self.r#south == South::None && self.r#north == North::Low && self.r#up == true { return 18275; }
        if self.r#waterlogged == true && self.r#west == West::Low && self.r#east == East::Low && self.r#north == North::None && self.r#south == South::None && self.r#up == true { return 18345; }
        if self.r#west == West::None && self.r#waterlogged == true && self.r#south == South::None && self.r#north == North::Low && self.r#up == true && self.r#east == East::None { return 18272; }
        if self.r#up == true && self.r#east == East::None && self.r#south == South::Low && self.r#waterlogged == false && self.r#west == West::Low && self.r#north == North::None { return 18252; }
        if self.r#up == false && self.r#waterlogged == true && self.r#west == West::None && self.r#north == North::Low && self.r#east == East::None && self.r#south == South::None { return 18278; }
        if self.r#east == East::None && self.r#south == South::None && self.r#west == West::Tall && self.r#north == North::Low && self.r#up == true && self.r#waterlogged == true { return 18274; }
        if self.r#north == North::Low && self.r#south == South::Low && self.r#up == false && self.r#east == East::None && self.r#waterlogged == true && self.r#west == West::Tall { return 18292; }
        if self.r#north == North::None && self.r#west == West::None && self.r#up == false && self.r#south == South::Low && self.r#east == East::Low && self.r#waterlogged == false { return 18365; }
        if self.r#west == West::Low && self.r#north == North::Tall && self.r#up == true && self.r#waterlogged == true && self.r#south == South::Tall && self.r#east == East::Low { return 18441; }
        if self.r#east == East::Tall && self.r#up == false && self.r#south == South::None && self.r#waterlogged == false && self.r#west == West::Tall && self.r#north == North::Tall { return 18535; }
        if self.r#north == North::Low && self.r#waterlogged == false && self.r#south == South::Tall && self.r#east == East::Low && self.r#up == true && self.r#west == West::None { return 18407; }
        if self.r#east == East::Low && self.r#north == North::Low && self.r#south == South::Tall && self.r#up == true && self.r#waterlogged == true && self.r#west == West::None { return 18404; }
        if self.r#east == East::Tall && self.r#up == false && self.r#north == North::Tall && self.r#south == South::Tall && self.r#west == West::Tall && self.r#waterlogged == false { return 18559; }
        if self.r#north == North::None && self.r#waterlogged == true && self.r#west == West::None && self.r#east == East::Low && self.r#south == South::None && self.r#up == false { return 18350; }
        if self.r#north == North::None && self.r#south == South::None && self.r#up == true && self.r#waterlogged == false && self.r#east == East::None && self.r#west == West::Low { return 18240; }
        if self.r#east == East::None && self.r#up == false && self.r#north == North::None && self.r#south == South::Tall && self.r#waterlogged == false && self.r#west == West::Low { return 18270; }
        if self.r#up == true && self.r#waterlogged == true && self.r#south == South::Low && self.r#west == West::Low && self.r#east == East::None && self.r#north == North::Tall { return 18321; }
        if self.r#up == false && self.r#waterlogged == false && self.r#south == South::None && self.r#north == North::None && self.r#east == East::Low && self.r#west == West::None { return 18353; }
        if self.r#up == true && self.r#waterlogged == false && self.r#north == North::Low && self.r#east == East::Tall && self.r#south == South::None && self.r#west == West::Tall { return 18493; }
        if self.r#east == East::Low && self.r#north == North::None && self.r#waterlogged == true && self.r#up == true && self.r#west == West::Low && self.r#south == South::Low { return 18357; }
        if self.r#north == North::Tall && self.r#west == West::Low && self.r#east == East::Low && self.r#south == South::Tall && self.r#waterlogged == false && self.r#up == true { return 18444; }
        if self.r#waterlogged == false && self.r#south == South::None && self.r#west == West::None && self.r#north == North::None && self.r#east == East::None && self.r#up == true { return 18239; }
        if self.r#up == false && self.r#waterlogged == false && self.r#north == North::Low && self.r#east == East::Low && self.r#west == West::Tall && self.r#south == South::Tall { return 18415; }
        if self.r#east == East::Low && self.r#waterlogged == true && self.r#up == true && self.r#south == South::None && self.r#west == West::Tall && self.r#north == North::Low { return 18382; }
        if self.r#west == West::None && self.r#north == North::Low && self.r#waterlogged == false && self.r#east == East::Low && self.r#south == South::None && self.r#up == true { return 18383; }
        if self.r#south == South::Tall && self.r#east == East::Tall && self.r#west == West::Tall && self.r#up == true && self.r#waterlogged == true && self.r#north == North::None { return 18478; }
        if self.r#south == South::None && self.r#north == North::Low && self.r#up == true && self.r#east == East::Low && self.r#west == West::Low && self.r#waterlogged == false { return 18384; }
        if self.r#up == true && self.r#waterlogged == true && self.r#east == East::None && self.r#west == West::Low && self.r#north == North::Low && self.r#south == South::None { return 18273; }
        if self.r#south == South::Tall && self.r#east == East::None && self.r#north == North::Tall && self.r#up == true && self.r#waterlogged == false && self.r#west == West::Tall { return 18337; }
        if self.r#north == North::None && self.r#east == East::Low && self.r#up == false && self.r#west == West::None && self.r#waterlogged == false && self.r#south == South::Tall { return 18377; }
        if self.r#up == true && self.r#south == South::Tall && self.r#west == West::None && self.r#north == North::Low && self.r#waterlogged == true && self.r#east == East::None { return 18296; }
        if self.r#up == true && self.r#west == West::Low && self.r#waterlogged == true && self.r#east == East::Tall && self.r#south == South::Low && self.r#north == North::None { return 18465; }
        if self.r#east == East::Low && self.r#up == false && self.r#waterlogged == true && self.r#north == North::Tall && self.r#west == West::None && self.r#south == South::Tall { return 18446; }
        if self.r#waterlogged == true && self.r#west == West::None && self.r#up == true && self.r#north == North::Tall && self.r#south == South::Low && self.r#east == East::None { return 18320; }
        if self.r#waterlogged == true && self.r#east == East::Tall && self.r#south == South::None && self.r#north == North::None && self.r#west == West::Low && self.r#up == true { return 18453; }
        if self.r#waterlogged == true && self.r#west == West::Low && self.r#south == South::None && self.r#east == East::Tall && self.r#up == false && self.r#north == North::Tall { return 18531; }
        if self.r#west == West::Tall && self.r#south == South::Tall && self.r#up == true && self.r#east == East::None && self.r#north == North::Low && self.r#waterlogged == true { return 18298; }
        if self.r#north == North::None && self.r#south == South::Low && self.r#up == false && self.r#west == West::None && self.r#east == East::Low && self.r#waterlogged == true { return 18362; }
        if self.r#west == West::Tall && self.r#waterlogged == false && self.r#north == North::Low && self.r#east == East::None && self.r#south == South::None && self.r#up == true { return 18277; }
        if self.r#north == North::None && self.r#south == South::Tall && self.r#up == true && self.r#waterlogged == true && self.r#east == East::Low && self.r#west == West::Tall { return 18370; }
        if self.r#south == South::Tall && self.r#east == East::None && self.r#up == false && self.r#waterlogged == true && self.r#west == West::Tall && self.r#north == North::None { return 18268; }
        if self.r#north == North::Tall && self.r#west == West::Low && self.r#waterlogged == true && self.r#south == South::Low && self.r#east == East::Low && self.r#up == true { return 18429; }
        if self.r#waterlogged == false && self.r#up == true && self.r#south == South::Tall && self.r#north == North::None && self.r#east == East::None && self.r#west == West::Low { return 18264; }
        if self.r#up == true && self.r#west == West::None && self.r#north == North::Tall && self.r#south == South::Tall && self.r#waterlogged == false && self.r#east == East::None { return 18335; }
        if self.r#east == East::Low && self.r#north == North::Tall && self.r#west == West::Tall && self.r#up == false && self.r#south == South::Tall && self.r#waterlogged == false { return 18451; }
        if self.r#east == East::Tall && self.r#north == North::None && self.r#south == South::Tall && self.r#up == true && self.r#west == West::Tall && self.r#waterlogged == false { return 18481; }
        if self.r#waterlogged == false && self.r#south == South::None && self.r#north == North::Low && self.r#up == true && self.r#west == West::Tall && self.r#east == East::Low { return 18385; }
        if self.r#west == West::Low && self.r#east == East::Low && self.r#waterlogged == true && self.r#north == North::Low && self.r#south == South::None && self.r#up == false { return 18387; }
        if self.r#west == West::Low && self.r#east == East::Low && self.r#south == South::Tall && self.r#up == true && self.r#waterlogged == true && self.r#north == North::Low { return 18405; }
        if self.r#north == North::Low && self.r#east == East::None && self.r#up == true && self.r#west == West::Low && self.r#waterlogged == false && self.r#south == South::Tall { return 18300; }
        if self.r#up == false && self.r#south == South::None && self.r#north == North::Low && self.r#west == West::Low && self.r#east == East::Low && self.r#waterlogged == false { return 18390; }
        if self.r#waterlogged == true && self.r#west == West::Low && self.r#east == East::Tall && self.r#north == North::Low && self.r#south == South::None && self.r#up == true { return 18489; }
        if self.r#north == North::Low && self.r#west == West::Tall && self.r#waterlogged == true && self.r#east == East::Tall && self.r#south == South::Tall && self.r#up == true { return 18514; }
        if self.r#north == North::Tall && self.r#waterlogged == false && self.r#east == East::None && self.r#south == South::Tall && self.r#west == West::Low && self.r#up == false { return 18342; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 18237 {
            return Some(MudBrickWall {
                r#west: West::Low,
                r#south: South::None,
                r#east: East::None,
                r#north: North::None,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 18261 {
            return Some(MudBrickWall {
                r#south: South::Tall,
                r#waterlogged: true,
                r#up: true,
                r#north: North::None,
                r#east: East::None,
                r#west: West::Low,
            });
        }
        if state_id == 18450 {
            return Some(MudBrickWall {
                r#north: North::Tall,
                r#south: South::Tall,
                r#waterlogged: false,
                r#east: East::Low,
                r#up: false,
                r#west: West::Low,
            });
        }
        if state_id == 18427 {
            return Some(MudBrickWall {
                r#north: North::Tall,
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::Low,
                r#up: false,
            });
        }
        if state_id == 18351 {
            return Some(MudBrickWall {
                r#east: East::Low,
                r#west: West::Low,
                r#north: North::None,
                r#waterlogged: true,
                r#up: false,
                r#south: South::None,
            });
        }
        if state_id == 18505 {
            return Some(MudBrickWall {
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::Low,
                r#up: true,
                r#west: West::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 18347 {
            return Some(MudBrickWall {
                r#east: East::Low,
                r#west: West::None,
                r#up: true,
                r#north: North::None,
                r#waterlogged: false,
                r#south: South::None,
            });
        }
        if state_id == 18236 {
            return Some(MudBrickWall {
                r#north: North::None,
                r#south: South::None,
                r#east: East::None,
                r#up: true,
                r#west: West::None,
                r#waterlogged: true,
            });
        }
        if state_id == 18496 {
            return Some(MudBrickWall {
                r#north: North::Low,
                r#up: false,
                r#south: South::None,
                r#east: East::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 18533 {
            return Some(MudBrickWall {
                r#east: East::Tall,
                r#south: South::None,
                r#up: false,
                r#north: North::Tall,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 18420 {
            return Some(MudBrickWall {
                r#north: North::Tall,
                r#up: true,
                r#west: West::Low,
                r#waterlogged: false,
                r#east: East::Low,
                r#south: South::None,
            });
        }
        if state_id == 18422 {
            return Some(MudBrickWall {
                r#east: East::Low,
                r#up: false,
                r#north: North::Tall,
                r#south: South::None,
                r#west: West::None,
                r#waterlogged: true,
            });
        }
        if state_id == 18285 {
            return Some(MudBrickWall {
                r#waterlogged: true,
                r#north: North::Low,
                r#east: East::None,
                r#west: West::Low,
                r#south: South::Low,
                r#up: true,
            });
        }
        if state_id == 18461 {
            return Some(MudBrickWall {
                r#north: North::None,
                r#up: false,
                r#west: West::None,
                r#waterlogged: false,
                r#east: East::Tall,
                r#south: South::None,
            });
        }
        if state_id == 18279 {
            return Some(MudBrickWall {
                r#up: false,
                r#west: West::Low,
                r#east: East::None,
                r#waterlogged: true,
                r#south: South::None,
                r#north: North::Low,
            });
        }
        if state_id == 18414 {
            return Some(MudBrickWall {
                r#east: East::Low,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 18269 {
            return Some(MudBrickWall {
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::None,
                r#south: South::Tall,
                r#up: false,
            });
        }
        if state_id == 18359 {
            return Some(MudBrickWall {
                r#waterlogged: false,
                r#north: North::None,
                r#west: West::None,
                r#east: East::Low,
                r#up: true,
                r#south: South::Low,
            });
        }
        if state_id == 18527 {
            return Some(MudBrickWall {
                r#east: East::Tall,
                r#waterlogged: false,
                r#west: West::None,
                r#up: true,
                r#north: North::Tall,
                r#south: South::None,
            });
        }
        if state_id == 18542 {
            return Some(MudBrickWall {
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::Low,
                r#north: North::Tall,
                r#east: East::Tall,
                r#up: false,
            });
        }
        if state_id == 18546 {
            return Some(MudBrickWall {
                r#east: East::Tall,
                r#south: South::Low,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 18327 {
            return Some(MudBrickWall {
                r#north: North::Tall,
                r#up: false,
                r#east: East::None,
                r#west: West::Low,
                r#waterlogged: true,
                r#south: South::Low,
            });
        }
        if state_id == 18255 {
            return Some(MudBrickWall {
                r#west: West::Low,
                r#east: East::None,
                r#north: North::None,
                r#waterlogged: true,
                r#up: false,
                r#south: South::Low,
            });
        }
        if state_id == 18299 {
            return Some(MudBrickWall {
                r#east: East::None,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::Tall,
            });
        }
        if state_id == 18368 {
            return Some(MudBrickWall {
                r#north: North::None,
                r#south: South::Tall,
                r#east: East::Low,
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 18499 {
            return Some(MudBrickWall {
                r#up: false,
                r#west: West::Tall,
                r#north: North::Low,
                r#waterlogged: false,
                r#east: East::Tall,
                r#south: South::None,
            });
        }
        if state_id == 18332 {
            return Some(MudBrickWall {
                r#up: true,
                r#waterlogged: true,
                r#north: North::Tall,
                r#west: West::None,
                r#south: South::Tall,
                r#east: East::None,
            });
        }
        if state_id == 18289 {
            return Some(MudBrickWall {
                r#up: true,
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::None,
                r#north: North::Low,
            });
        }
        if state_id == 18389 {
            return Some(MudBrickWall {
                r#east: East::Low,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: false,
                r#south: South::None,
                r#west: West::None,
            });
        }
        if state_id == 18500 {
            return Some(MudBrickWall {
                r#south: South::Low,
                r#north: North::Low,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 18349 {
            return Some(MudBrickWall {
                r#west: West::Tall,
                r#up: true,
                r#south: South::None,
                r#east: East::Low,
                r#north: North::None,
                r#waterlogged: false,
            });
        }
        if state_id == 18540 {
            return Some(MudBrickWall {
                r#west: West::Low,
                r#waterlogged: false,
                r#north: North::Tall,
                r#east: East::Tall,
                r#south: South::Low,
                r#up: true,
            });
        }
        if state_id == 18434 {
            return Some(MudBrickWall {
                r#east: East::Low,
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::None,
                r#up: false,
                r#south: South::Low,
            });
        }
        if state_id == 18310 {
            return Some(MudBrickWall {
                r#north: North::Tall,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::Tall,
                r#up: true,
                r#east: East::None,
            });
        }
        if state_id == 18360 {
            return Some(MudBrickWall {
                r#north: North::None,
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::Low,
                r#up: true,
                r#east: East::Low,
            });
        }
        if state_id == 18336 {
            return Some(MudBrickWall {
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::Tall,
                r#east: East::None,
                r#south: South::Tall,
            });
        }
        if state_id == 18248 {
            return Some(MudBrickWall {
                r#south: South::Low,
                r#east: East::None,
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::None,
            });
        }
        if state_id == 18361 {
            return Some(MudBrickWall {
                r#north: North::None,
                r#south: South::Low,
                r#up: true,
                r#east: East::Low,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 18331 {
            return Some(MudBrickWall {
                r#north: North::Tall,
                r#waterlogged: false,
                r#west: West::Tall,
                r#up: false,
                r#east: East::None,
                r#south: South::Low,
            });
        }
        if state_id == 18545 {
            return Some(MudBrickWall {
                r#west: West::None,
                r#south: South::Low,
                r#north: North::Tall,
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 18271 {
            return Some(MudBrickWall {
                r#up: false,
                r#north: North::None,
                r#west: West::Tall,
                r#east: East::None,
                r#waterlogged: false,
                r#south: South::Tall,
            });
        }
        if state_id == 18355 {
            return Some(MudBrickWall {
                r#east: East::Low,
                r#up: false,
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::None,
            });
        }
        if state_id == 18448 {
            return Some(MudBrickWall {
                r#north: North::Tall,
                r#west: West::Tall,
                r#up: false,
                r#waterlogged: true,
                r#south: South::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 18379 {
            return Some(MudBrickWall {
                r#south: South::Tall,
                r#up: false,
                r#east: East::Low,
                r#west: West::Tall,
                r#waterlogged: false,
                r#north: North::None,
            });
        }
        if state_id == 18456 {
            return Some(MudBrickWall {
                r#up: true,
                r#east: East::Tall,
                r#south: South::None,
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 18544 {
            return Some(MudBrickWall {
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::Tall,
                r#north: North::Tall,
                r#up: false,
                r#south: South::Low,
            });
        }
        if state_id == 18403 {
            return Some(MudBrickWall {
                r#up: false,
                r#north: North::Low,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::Low,
                r#east: East::Low,
            });
        }
        if state_id == 18445 {
            return Some(MudBrickWall {
                r#up: true,
                r#east: East::Low,
                r#north: North::Tall,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 18283 {
            return Some(MudBrickWall {
                r#waterlogged: false,
                r#south: South::None,
                r#east: East::None,
                r#up: false,
                r#west: West::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 18394 {
            return Some(MudBrickWall {
                r#south: South::Low,
                r#west: West::Tall,
                r#up: true,
                r#waterlogged: true,
                r#east: East::Low,
                r#north: North::Low,
            });
        }
        if state_id == 18251 {
            return Some(MudBrickWall {
                r#up: true,
                r#south: South::Low,
                r#waterlogged: false,
                r#north: North::None,
                r#east: East::None,
                r#west: West::None,
            });
        }
        if state_id == 18460 {
            return Some(MudBrickWall {
                r#east: East::Tall,
                r#waterlogged: true,
                r#north: North::None,
                r#west: West::Tall,
                r#up: false,
                r#south: South::None,
            });
        }
        if state_id == 18247 {
            return Some(MudBrickWall {
                r#up: false,
                r#south: South::None,
                r#east: East::None,
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 18257 {
            return Some(MudBrickWall {
                r#north: North::None,
                r#east: East::None,
                r#up: false,
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 18287 {
            return Some(MudBrickWall {
                r#south: South::Low,
                r#north: North::Low,
                r#waterlogged: false,
                r#east: East::None,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 18468 {
            return Some(MudBrickWall {
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::None,
                r#up: true,
                r#south: South::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 18293 {
            return Some(MudBrickWall {
                r#north: North::Low,
                r#west: West::None,
                r#south: South::Low,
                r#up: false,
                r#east: East::None,
                r#waterlogged: false,
            });
        }
        if state_id == 18328 {
            return Some(MudBrickWall {
                r#west: West::Tall,
                r#east: East::None,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: true,
                r#north: North::Tall,
            });
        }
        if state_id == 18352 {
            return Some(MudBrickWall {
                r#east: East::Low,
                r#north: North::None,
                r#waterlogged: true,
                r#south: South::None,
                r#up: false,
                r#west: West::Tall,
            });
        }
        if state_id == 18318 {
            return Some(MudBrickWall {
                r#east: East::None,
                r#west: West::Low,
                r#south: South::None,
                r#up: false,
                r#waterlogged: false,
                r#north: North::Tall,
            });
        }
        if state_id == 18523 {
            return Some(MudBrickWall {
                r#north: North::Low,
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::Tall,
                r#up: false,
                r#east: East::Tall,
            });
        }
        if state_id == 18454 {
            return Some(MudBrickWall {
                r#west: West::Tall,
                r#north: North::None,
                r#east: East::Tall,
                r#waterlogged: true,
                r#up: true,
                r#south: South::None,
            });
        }
        if state_id == 18436 {
            return Some(MudBrickWall {
                r#east: East::Low,
                r#south: South::Low,
                r#waterlogged: true,
                r#north: North::Tall,
                r#up: false,
                r#west: West::Tall,
            });
        }
        if state_id == 18411 {
            return Some(MudBrickWall {
                r#waterlogged: true,
                r#north: North::Low,
                r#west: West::Low,
                r#east: East::Low,
                r#south: South::Tall,
                r#up: false,
            });
        }
        if state_id == 18551 {
            return Some(MudBrickWall {
                r#west: West::None,
                r#north: North::Tall,
                r#east: East::Tall,
                r#waterlogged: false,
                r#south: South::Tall,
                r#up: true,
            });
        }
        if state_id == 18524 {
            return Some(MudBrickWall {
                r#south: South::None,
                r#east: East::Tall,
                r#waterlogged: true,
                r#up: true,
                r#west: West::None,
                r#north: North::Tall,
            });
        }
        if state_id == 18401 {
            return Some(MudBrickWall {
                r#east: East::Low,
                r#waterlogged: false,
                r#south: South::Low,
                r#west: West::None,
                r#north: North::Low,
                r#up: false,
            });
        }
        if state_id == 18402 {
            return Some(MudBrickWall {
                r#up: false,
                r#north: North::Low,
                r#west: West::Low,
                r#waterlogged: false,
                r#east: East::Low,
                r#south: South::Low,
            });
        }
        if state_id == 18423 {
            return Some(MudBrickWall {
                r#north: North::Tall,
                r#waterlogged: true,
                r#south: South::None,
                r#west: West::Low,
                r#east: East::Low,
                r#up: false,
            });
        }
        if state_id == 18263 {
            return Some(MudBrickWall {
                r#east: East::None,
                r#up: true,
                r#waterlogged: false,
                r#north: North::None,
                r#west: West::None,
                r#south: South::Tall,
            });
        }
        if state_id == 18513 {
            return Some(MudBrickWall {
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::Tall,
                r#up: true,
            });
        }
        if state_id == 18519 {
            return Some(MudBrickWall {
                r#north: North::Low,
                r#waterlogged: true,
                r#south: South::Tall,
                r#up: false,
                r#east: East::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 18428 {
            return Some(MudBrickWall {
                r#west: West::None,
                r#up: true,
                r#east: East::Low,
                r#south: South::Low,
                r#north: North::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 18280 {
            return Some(MudBrickWall {
                r#east: East::None,
                r#north: North::Low,
                r#up: false,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 18504 {
            return Some(MudBrickWall {
                r#west: West::Low,
                r#waterlogged: false,
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::Low,
                r#up: true,
            });
        }
        if state_id == 18282 {
            return Some(MudBrickWall {
                r#east: East::None,
                r#north: North::Low,
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::None,
                r#up: false,
            });
        }
        if state_id == 18487 {
            return Some(MudBrickWall {
                r#east: East::Tall,
                r#waterlogged: false,
                r#north: North::None,
                r#south: South::Tall,
                r#west: West::Tall,
                r#up: false,
            });
        }
        if state_id == 18433 {
            return Some(MudBrickWall {
                r#south: South::Low,
                r#up: true,
                r#east: East::Low,
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 18297 {
            return Some(MudBrickWall {
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::Low,
                r#east: East::None,
                r#south: South::Tall,
                r#up: true,
            });
        }
        if state_id == 18525 {
            return Some(MudBrickWall {
                r#south: South::None,
                r#east: East::Tall,
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 18442 {
            return Some(MudBrickWall {
                r#north: North::Tall,
                r#waterlogged: true,
                r#east: East::Low,
                r#up: true,
                r#south: South::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 18354 {
            return Some(MudBrickWall {
                r#west: West::Low,
                r#waterlogged: false,
                r#north: North::None,
                r#south: South::None,
                r#up: false,
                r#east: East::Low,
            });
        }
        if state_id == 18344 {
            return Some(MudBrickWall {
                r#waterlogged: true,
                r#east: East::Low,
                r#south: South::None,
                r#north: North::None,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 18398 {
            return Some(MudBrickWall {
                r#north: North::Low,
                r#up: false,
                r#east: East::Low,
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::Low,
            });
        }
        if state_id == 18506 {
            return Some(MudBrickWall {
                r#west: West::None,
                r#waterlogged: true,
                r#north: North::Low,
                r#south: South::Low,
                r#up: false,
                r#east: East::Tall,
            });
        }
        if state_id == 18557 {
            return Some(MudBrickWall {
                r#up: false,
                r#west: West::None,
                r#east: East::Tall,
                r#north: North::Tall,
                r#south: South::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 18462 {
            return Some(MudBrickWall {
                r#west: West::Low,
                r#up: false,
                r#north: North::None,
                r#east: East::Tall,
                r#south: South::None,
                r#waterlogged: false,
            });
        }
        if state_id == 18408 {
            return Some(MudBrickWall {
                r#north: North::Low,
                r#up: true,
                r#waterlogged: false,
                r#east: East::Low,
                r#west: West::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 18254 {
            return Some(MudBrickWall {
                r#south: South::Low,
                r#east: East::None,
                r#up: false,
                r#west: West::None,
                r#north: North::None,
                r#waterlogged: true,
            });
        }
        if state_id == 18400 {
            return Some(MudBrickWall {
                r#south: South::Low,
                r#north: North::Low,
                r#west: West::Tall,
                r#east: East::Low,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 18515 {
            return Some(MudBrickWall {
                r#waterlogged: false,
                r#north: North::Low,
                r#up: true,
                r#east: East::Tall,
                r#west: West::None,
                r#south: South::Tall,
            });
        }
        if state_id == 18521 {
            return Some(MudBrickWall {
                r#north: North::Low,
                r#up: false,
                r#east: East::Tall,
                r#south: South::Tall,
                r#west: West::None,
                r#waterlogged: false,
            });
        }
        if state_id == 18313 {
            return Some(MudBrickWall {
                r#east: East::None,
                r#south: South::None,
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 18497 {
            return Some(MudBrickWall {
                r#north: North::Low,
                r#waterlogged: false,
                r#west: West::None,
                r#up: false,
                r#east: East::Tall,
                r#south: South::None,
            });
        }
        if state_id == 18437 {
            return Some(MudBrickWall {
                r#north: North::Tall,
                r#up: false,
                r#east: East::Low,
                r#waterlogged: false,
                r#south: South::Low,
                r#west: West::None,
            });
        }
        if state_id == 18364 {
            return Some(MudBrickWall {
                r#waterlogged: true,
                r#north: North::None,
                r#west: West::Tall,
                r#south: South::Low,
                r#up: false,
                r#east: East::Low,
            });
        }
        if state_id == 18550 {
            return Some(MudBrickWall {
                r#north: North::Tall,
                r#south: South::Tall,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 18396 {
            return Some(MudBrickWall {
                r#up: true,
                r#west: West::Low,
                r#waterlogged: false,
                r#south: South::Low,
                r#east: East::Low,
                r#north: North::Low,
            });
        }
        if state_id == 18309 {
            return Some(MudBrickWall {
                r#east: East::None,
                r#north: North::Tall,
                r#south: South::None,
                r#west: West::Low,
                r#waterlogged: true,
                r#up: true,
            });
        }
        if state_id == 18281 {
            return Some(MudBrickWall {
                r#south: South::None,
                r#up: false,
                r#north: North::Low,
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::None,
            });
        }
        if state_id == 18259 {
            return Some(MudBrickWall {
                r#up: false,
                r#south: South::Low,
                r#north: North::None,
                r#east: East::None,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 18266 {
            return Some(MudBrickWall {
                r#up: false,
                r#waterlogged: true,
                r#east: East::None,
                r#north: North::None,
                r#west: West::None,
                r#south: South::Tall,
            });
        }
        if state_id == 18295 {
            return Some(MudBrickWall {
                r#south: South::Low,
                r#west: West::Tall,
                r#up: false,
                r#north: North::Low,
                r#east: East::None,
                r#waterlogged: false,
            });
        }
        if state_id == 18241 {
            return Some(MudBrickWall {
                r#east: East::None,
                r#waterlogged: false,
                r#north: North::None,
                r#up: true,
                r#south: South::None,
                r#west: West::Tall,
            });
        }
        if state_id == 18301 {
            return Some(MudBrickWall {
                r#up: true,
                r#south: South::Tall,
                r#north: North::Low,
                r#waterlogged: false,
                r#east: East::None,
                r#west: West::Tall,
            });
        }
        if state_id == 18413 {
            return Some(MudBrickWall {
                r#waterlogged: false,
                r#up: false,
                r#west: West::None,
                r#north: North::Low,
                r#east: East::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 18412 {
            return Some(MudBrickWall {
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::Low,
                r#up: false,
                r#south: South::Tall,
            });
        }
        if state_id == 18430 {
            return Some(MudBrickWall {
                r#up: true,
                r#waterlogged: true,
                r#south: South::Low,
                r#east: East::Low,
                r#north: North::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 18476 {
            return Some(MudBrickWall {
                r#up: true,
                r#north: North::None,
                r#west: West::None,
                r#east: East::Tall,
                r#south: South::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 18529 {
            return Some(MudBrickWall {
                r#up: true,
                r#north: North::Tall,
                r#east: East::Tall,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::None,
            });
        }
        if state_id == 18319 {
            return Some(MudBrickWall {
                r#east: East::None,
                r#west: West::Tall,
                r#south: South::None,
                r#up: false,
                r#waterlogged: false,
                r#north: North::Tall,
            });
        }
        if state_id == 18366 {
            return Some(MudBrickWall {
                r#up: false,
                r#waterlogged: false,
                r#north: North::None,
                r#south: South::Low,
                r#west: West::Low,
                r#east: East::Low,
            });
        }
        if state_id == 18395 {
            return Some(MudBrickWall {
                r#west: West::None,
                r#north: North::Low,
                r#up: true,
                r#east: East::Low,
                r#south: South::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 18393 {
            return Some(MudBrickWall {
                r#up: true,
                r#east: East::Low,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::Low,
                r#south: South::Low,
            });
        }
        if state_id == 18243 {
            return Some(MudBrickWall {
                r#waterlogged: true,
                r#south: South::None,
                r#up: false,
                r#north: North::None,
                r#west: West::Low,
                r#east: East::None,
            });
        }
        if state_id == 18421 {
            return Some(MudBrickWall {
                r#west: West::Tall,
                r#north: North::Tall,
                r#waterlogged: false,
                r#south: South::None,
                r#east: East::Low,
                r#up: true,
            });
        }
        if state_id == 18294 {
            return Some(MudBrickWall {
                r#south: South::Low,
                r#north: North::Low,
                r#up: false,
                r#east: East::None,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 18537 {
            return Some(MudBrickWall {
                r#waterlogged: true,
                r#up: true,
                r#east: East::Tall,
                r#north: North::Tall,
                r#south: South::Low,
                r#west: West::Low,
            });
        }
        if state_id == 18334 {
            return Some(MudBrickWall {
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::None,
                r#south: South::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 18528 {
            return Some(MudBrickWall {
                r#south: South::None,
                r#east: East::Tall,
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 18455 {
            return Some(MudBrickWall {
                r#south: South::None,
                r#east: East::Tall,
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::None,
                r#up: true,
            });
        }
        if state_id == 18333 {
            return Some(MudBrickWall {
                r#north: North::Tall,
                r#west: West::Low,
                r#south: South::Tall,
                r#up: true,
                r#east: East::None,
                r#waterlogged: true,
            });
        }
        if state_id == 18530 {
            return Some(MudBrickWall {
                r#east: East::Tall,
                r#south: South::None,
                r#up: false,
                r#waterlogged: true,
                r#north: North::Tall,
                r#west: West::None,
            });
        }
        if state_id == 18484 {
            return Some(MudBrickWall {
                r#north: North::None,
                r#waterlogged: true,
                r#east: East::Tall,
                r#up: false,
                r#south: South::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 18558 {
            return Some(MudBrickWall {
                r#east: East::Tall,
                r#waterlogged: false,
                r#north: North::Tall,
                r#south: South::Tall,
                r#west: West::Low,
                r#up: false,
            });
        }
        if state_id == 18538 {
            return Some(MudBrickWall {
                r#east: East::Tall,
                r#west: West::Tall,
                r#north: North::Tall,
                r#south: South::Low,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 18541 {
            return Some(MudBrickWall {
                r#west: West::Tall,
                r#up: true,
                r#waterlogged: false,
                r#east: East::Tall,
                r#north: North::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 18539 {
            return Some(MudBrickWall {
                r#up: true,
                r#east: East::Tall,
                r#west: West::None,
                r#waterlogged: false,
                r#north: North::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 18326 {
            return Some(MudBrickWall {
                r#up: false,
                r#south: South::Low,
                r#west: West::None,
                r#north: North::Tall,
                r#east: East::None,
                r#waterlogged: true,
            });
        }
        if state_id == 18406 {
            return Some(MudBrickWall {
                r#up: true,
                r#east: East::Low,
                r#north: North::Low,
                r#west: West::Tall,
                r#south: South::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 18346 {
            return Some(MudBrickWall {
                r#waterlogged: true,
                r#up: true,
                r#east: East::Low,
                r#north: North::None,
                r#south: South::None,
                r#west: West::Tall,
            });
        }
        if state_id == 18439 {
            return Some(MudBrickWall {
                r#west: West::Tall,
                r#north: North::Tall,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: false,
                r#east: East::Low,
            });
        }
        if state_id == 18325 {
            return Some(MudBrickWall {
                r#west: West::Tall,
                r#waterlogged: false,
                r#north: North::Tall,
                r#south: South::Low,
                r#east: East::None,
                r#up: true,
            });
        }
        if state_id == 18371 {
            return Some(MudBrickWall {
                r#north: North::None,
                r#south: South::Tall,
                r#west: West::None,
                r#up: true,
                r#east: East::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 18543 {
            return Some(MudBrickWall {
                r#south: South::Low,
                r#north: North::Tall,
                r#waterlogged: true,
                r#east: East::Tall,
                r#west: West::Low,
                r#up: false,
            });
        }
        if state_id == 18378 {
            return Some(MudBrickWall {
                r#west: West::Low,
                r#east: East::Low,
                r#up: false,
                r#north: North::None,
                r#waterlogged: false,
                r#south: South::Tall,
            });
        }
        if state_id == 18317 {
            return Some(MudBrickWall {
                r#west: West::None,
                r#east: East::None,
                r#north: North::Tall,
                r#waterlogged: false,
                r#south: South::None,
                r#up: false,
            });
        }
        if state_id == 18284 {
            return Some(MudBrickWall {
                r#east: East::None,
                r#waterlogged: true,
                r#west: West::None,
                r#up: true,
                r#south: South::Low,
                r#north: North::Low,
            });
        }
        if state_id == 18416 {
            return Some(MudBrickWall {
                r#up: true,
                r#west: West::None,
                r#south: South::None,
                r#north: North::Tall,
                r#east: East::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 18466 {
            return Some(MudBrickWall {
                r#up: true,
                r#west: West::Tall,
                r#east: East::Tall,
                r#north: North::None,
                r#waterlogged: true,
                r#south: South::Low,
            });
        }
        if state_id == 18363 {
            return Some(MudBrickWall {
                r#up: false,
                r#east: East::Low,
                r#west: West::Low,
                r#south: South::Low,
                r#waterlogged: true,
                r#north: North::None,
            });
        }
        if state_id == 18443 {
            return Some(MudBrickWall {
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 18502 {
            return Some(MudBrickWall {
                r#waterlogged: true,
                r#south: South::Low,
                r#east: East::Tall,
                r#up: true,
                r#west: West::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 18265 {
            return Some(MudBrickWall {
                r#waterlogged: false,
                r#south: South::Tall,
                r#west: West::Tall,
                r#east: East::None,
                r#up: true,
                r#north: North::None,
            });
        }
        if state_id == 18425 {
            return Some(MudBrickWall {
                r#west: West::None,
                r#north: North::Tall,
                r#up: false,
                r#east: East::Low,
                r#waterlogged: false,
                r#south: South::None,
            });
        }
        if state_id == 18479 {
            return Some(MudBrickWall {
                r#north: North::None,
                r#west: West::None,
                r#up: true,
                r#east: East::Tall,
                r#south: South::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 18438 {
            return Some(MudBrickWall {
                r#west: West::Low,
                r#north: North::Tall,
                r#up: false,
                r#east: East::Low,
                r#waterlogged: false,
                r#south: South::Low,
            });
        }
        if state_id == 18250 {
            return Some(MudBrickWall {
                r#south: South::Low,
                r#east: East::None,
                r#north: North::None,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 18338 {
            return Some(MudBrickWall {
                r#waterlogged: true,
                r#east: East::None,
                r#north: North::Tall,
                r#up: false,
                r#south: South::Tall,
                r#west: West::None,
            });
        }
        if state_id == 18449 {
            return Some(MudBrickWall {
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::Tall,
                r#up: false,
                r#east: East::Low,
            });
        }
        if state_id == 18469 {
            return Some(MudBrickWall {
                r#waterlogged: false,
                r#west: West::Tall,
                r#up: true,
                r#north: North::None,
                r#south: South::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 18494 {
            return Some(MudBrickWall {
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::None,
                r#up: false,
                r#east: East::Tall,
            });
        }
        if state_id == 18432 {
            return Some(MudBrickWall {
                r#waterlogged: false,
                r#south: South::Low,
                r#east: East::Low,
                r#up: true,
                r#west: West::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 18480 {
            return Some(MudBrickWall {
                r#west: West::Low,
                r#up: true,
                r#north: North::None,
                r#waterlogged: false,
                r#south: South::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 18470 {
            return Some(MudBrickWall {
                r#east: East::Tall,
                r#north: North::None,
                r#west: West::None,
                r#waterlogged: true,
                r#south: South::Low,
                r#up: false,
            });
        }
        if state_id == 18536 {
            return Some(MudBrickWall {
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::Low,
                r#east: East::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 18534 {
            return Some(MudBrickWall {
                r#north: North::Tall,
                r#waterlogged: false,
                r#south: South::None,
                r#up: false,
                r#west: West::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 18341 {
            return Some(MudBrickWall {
                r#west: West::None,
                r#south: South::Tall,
                r#north: North::Tall,
                r#east: East::None,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 18288 {
            return Some(MudBrickWall {
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::None,
                r#north: North::Low,
                r#south: South::Low,
            });
        }
        if state_id == 18459 {
            return Some(MudBrickWall {
                r#east: East::Tall,
                r#south: South::None,
                r#up: false,
                r#west: West::Low,
                r#north: North::None,
                r#waterlogged: true,
            });
        }
        if state_id == 18304 {
            return Some(MudBrickWall {
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::None,
                r#up: false,
                r#north: North::Low,
            });
        }
        if state_id == 18556 {
            return Some(MudBrickWall {
                r#waterlogged: true,
                r#east: East::Tall,
                r#up: false,
                r#west: West::Tall,
                r#north: North::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 18418 {
            return Some(MudBrickWall {
                r#west: West::Tall,
                r#north: North::Tall,
                r#waterlogged: true,
                r#up: true,
                r#east: East::Low,
                r#south: South::None,
            });
        }
        if state_id == 18490 {
            return Some(MudBrickWall {
                r#west: West::Tall,
                r#south: South::None,
                r#north: North::Low,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 18303 {
            return Some(MudBrickWall {
                r#south: South::Tall,
                r#east: East::None,
                r#north: North::Low,
                r#west: West::Low,
                r#waterlogged: true,
                r#up: false,
            });
        }
        if state_id == 18348 {
            return Some(MudBrickWall {
                r#west: West::Low,
                r#up: true,
                r#east: East::Low,
                r#north: North::None,
                r#south: South::None,
                r#waterlogged: false,
            });
        }
        if state_id == 18388 {
            return Some(MudBrickWall {
                r#east: East::Low,
                r#west: West::Tall,
                r#up: false,
                r#south: South::None,
                r#waterlogged: true,
                r#north: North::Low,
            });
        }
        if state_id == 18246 {
            return Some(MudBrickWall {
                r#east: East::None,
                r#up: false,
                r#north: North::None,
                r#west: West::Low,
                r#waterlogged: false,
                r#south: South::None,
            });
        }
        if state_id == 18392 {
            return Some(MudBrickWall {
                r#up: true,
                r#north: North::Low,
                r#waterlogged: true,
                r#east: East::Low,
                r#south: South::Low,
                r#west: West::None,
            });
        }
        if state_id == 18457 {
            return Some(MudBrickWall {
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::None,
                r#north: North::None,
            });
        }
        if state_id == 18322 {
            return Some(MudBrickWall {
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::Low,
                r#east: East::None,
                r#north: North::Tall,
            });
        }
        if state_id == 18507 {
            return Some(MudBrickWall {
                r#west: West::Low,
                r#north: North::Low,
                r#up: false,
                r#east: East::Tall,
                r#south: South::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 18244 {
            return Some(MudBrickWall {
                r#south: South::None,
                r#up: false,
                r#waterlogged: true,
                r#east: East::None,
                r#north: North::None,
                r#west: West::Tall,
            });
        }
        if state_id == 18512 {
            return Some(MudBrickWall {
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 18374 {
            return Some(MudBrickWall {
                r#east: East::Low,
                r#up: false,
                r#north: North::None,
                r#south: South::Tall,
                r#west: West::None,
                r#waterlogged: true,
            });
        }
        if state_id == 18356 {
            return Some(MudBrickWall {
                r#up: true,
                r#west: West::None,
                r#east: East::Low,
                r#south: South::Low,
                r#north: North::None,
                r#waterlogged: true,
            });
        }
        if state_id == 18308 {
            return Some(MudBrickWall {
                r#up: true,
                r#south: South::None,
                r#north: North::Tall,
                r#west: West::None,
                r#waterlogged: true,
                r#east: East::None,
            });
        }
        if state_id == 18482 {
            return Some(MudBrickWall {
                r#up: false,
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::Tall,
                r#north: North::None,
            });
        }
        if state_id == 18315 {
            return Some(MudBrickWall {
                r#east: East::None,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: true,
                r#south: South::None,
                r#west: West::Low,
            });
        }
        if state_id == 18367 {
            return Some(MudBrickWall {
                r#north: North::None,
                r#west: West::Tall,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: false,
                r#east: East::Low,
            });
        }
        if state_id == 18467 {
            return Some(MudBrickWall {
                r#west: West::None,
                r#east: East::Tall,
                r#waterlogged: false,
                r#north: North::None,
                r#south: South::Low,
                r#up: true,
            });
        }
        if state_id == 18471 {
            return Some(MudBrickWall {
                r#north: North::None,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 18305 {
            return Some(MudBrickWall {
                r#east: East::None,
                r#north: North::Low,
                r#waterlogged: false,
                r#south: South::Tall,
                r#up: false,
                r#west: West::None,
            });
        }
        if state_id == 18242 {
            return Some(MudBrickWall {
                r#east: East::None,
                r#west: West::None,
                r#waterlogged: true,
                r#south: South::None,
                r#up: false,
                r#north: North::None,
            });
        }
        if state_id == 18440 {
            return Some(MudBrickWall {
                r#south: South::Tall,
                r#up: true,
                r#west: West::None,
                r#waterlogged: true,
                r#north: North::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 18262 {
            return Some(MudBrickWall {
                r#north: North::None,
                r#waterlogged: true,
                r#up: true,
                r#west: West::Tall,
                r#south: South::Tall,
                r#east: East::None,
            });
        }
        if state_id == 18399 {
            return Some(MudBrickWall {
                r#waterlogged: true,
                r#up: false,
                r#east: East::Low,
                r#south: South::Low,
                r#west: West::Low,
                r#north: North::Low,
            });
        }
        if state_id == 18372 {
            return Some(MudBrickWall {
                r#west: West::Low,
                r#south: South::Tall,
                r#waterlogged: false,
                r#east: East::Low,
                r#north: North::None,
                r#up: true,
            });
        }
        if state_id == 18373 {
            return Some(MudBrickWall {
                r#south: South::Tall,
                r#west: West::Tall,
                r#east: East::Low,
                r#waterlogged: false,
                r#north: North::None,
                r#up: true,
            });
        }
        if state_id == 18516 {
            return Some(MudBrickWall {
                r#waterlogged: false,
                r#south: South::Tall,
                r#north: North::Low,
                r#east: East::Tall,
                r#up: true,
                r#west: West::Low,
            });
        }
        if state_id == 18312 {
            return Some(MudBrickWall {
                r#up: true,
                r#east: East::None,
                r#north: North::Tall,
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 18501 {
            return Some(MudBrickWall {
                r#north: North::Low,
                r#south: South::Low,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 18472 {
            return Some(MudBrickWall {
                r#north: North::None,
                r#waterlogged: true,
                r#south: South::Low,
                r#east: East::Tall,
                r#up: false,
                r#west: West::Tall,
            });
        }
        if state_id == 18463 {
            return Some(MudBrickWall {
                r#east: East::Tall,
                r#south: South::None,
                r#west: West::Tall,
                r#north: North::None,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 18495 {
            return Some(MudBrickWall {
                r#up: false,
                r#north: North::Low,
                r#east: East::Tall,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 18307 {
            return Some(MudBrickWall {
                r#west: West::Tall,
                r#north: North::Low,
                r#south: South::Tall,
                r#up: false,
                r#east: East::None,
                r#waterlogged: false,
            });
        }
        if state_id == 18339 {
            return Some(MudBrickWall {
                r#up: false,
                r#east: East::None,
                r#north: North::Tall,
                r#waterlogged: true,
                r#south: South::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 18238 {
            return Some(MudBrickWall {
                r#east: East::None,
                r#west: West::Tall,
                r#south: South::None,
                r#north: North::None,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 18391 {
            return Some(MudBrickWall {
                r#north: North::Low,
                r#south: South::None,
                r#waterlogged: false,
                r#east: East::Low,
                r#west: West::Tall,
                r#up: false,
            });
        }
        if state_id == 18410 {
            return Some(MudBrickWall {
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::Tall,
                r#east: East::Low,
                r#north: North::Low,
            });
        }
        if state_id == 18435 {
            return Some(MudBrickWall {
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::Low,
                r#east: East::Low,
                r#up: false,
                r#north: North::Tall,
            });
        }
        if state_id == 18256 {
            return Some(MudBrickWall {
                r#up: false,
                r#east: East::None,
                r#waterlogged: true,
                r#south: South::Low,
                r#west: West::Tall,
                r#north: North::None,
            });
        }
        if state_id == 18258 {
            return Some(MudBrickWall {
                r#up: false,
                r#north: North::None,
                r#west: West::Low,
                r#east: East::None,
                r#south: South::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 18286 {
            return Some(MudBrickWall {
                r#east: East::None,
                r#north: North::Low,
                r#west: West::Tall,
                r#south: South::Low,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 18369 {
            return Some(MudBrickWall {
                r#waterlogged: true,
                r#south: South::Tall,
                r#west: West::Low,
                r#north: North::None,
                r#up: true,
                r#east: East::Low,
            });
        }
        if state_id == 18464 {
            return Some(MudBrickWall {
                r#up: true,
                r#waterlogged: true,
                r#east: East::Tall,
                r#west: West::None,
                r#south: South::Low,
                r#north: North::None,
            });
        }
        if state_id == 18330 {
            return Some(MudBrickWall {
                r#east: East::None,
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::Low,
                r#up: false,
                r#north: North::Tall,
            });
        }
        if state_id == 18253 {
            return Some(MudBrickWall {
                r#east: East::None,
                r#up: true,
                r#waterlogged: false,
                r#north: North::None,
                r#south: South::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 18508 {
            return Some(MudBrickWall {
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 18324 {
            return Some(MudBrickWall {
                r#west: West::Low,
                r#up: true,
                r#south: South::Low,
                r#east: East::None,
                r#north: North::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 18316 {
            return Some(MudBrickWall {
                r#south: South::None,
                r#north: North::Tall,
                r#waterlogged: true,
                r#east: East::None,
                r#up: false,
                r#west: West::Tall,
            });
        }
        if state_id == 18245 {
            return Some(MudBrickWall {
                r#west: West::None,
                r#waterlogged: false,
                r#east: East::None,
                r#north: North::None,
                r#south: South::None,
                r#up: false,
            });
        }
        if state_id == 18431 {
            return Some(MudBrickWall {
                r#up: true,
                r#south: South::Low,
                r#waterlogged: false,
                r#east: East::Low,
                r#north: North::Tall,
                r#west: West::None,
            });
        }
        if state_id == 18486 {
            return Some(MudBrickWall {
                r#east: East::Tall,
                r#waterlogged: false,
                r#up: false,
                r#north: North::None,
                r#west: West::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 18522 {
            return Some(MudBrickWall {
                r#south: South::Tall,
                r#east: East::Tall,
                r#up: false,
                r#west: West::Low,
                r#north: North::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 18343 {
            return Some(MudBrickWall {
                r#up: false,
                r#east: East::None,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 18329 {
            return Some(MudBrickWall {
                r#south: South::Low,
                r#east: East::None,
                r#up: false,
                r#north: North::Tall,
                r#west: West::None,
                r#waterlogged: false,
            });
        }
        if state_id == 18417 {
            return Some(MudBrickWall {
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: true,
                r#east: East::Low,
                r#west: West::Low,
                r#south: South::None,
            });
        }
        if state_id == 18548 {
            return Some(MudBrickWall {
                r#up: true,
                r#east: East::Tall,
                r#south: South::Tall,
                r#west: West::None,
                r#north: North::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 18409 {
            return Some(MudBrickWall {
                r#north: North::Low,
                r#up: true,
                r#west: West::Tall,
                r#waterlogged: false,
                r#east: East::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 18306 {
            return Some(MudBrickWall {
                r#west: West::Low,
                r#up: false,
                r#east: East::None,
                r#south: South::Tall,
                r#north: North::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 18458 {
            return Some(MudBrickWall {
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::None,
                r#north: North::None,
            });
        }
        if state_id == 18517 {
            return Some(MudBrickWall {
                r#up: true,
                r#north: North::Low,
                r#west: West::Tall,
                r#east: East::Tall,
                r#waterlogged: false,
                r#south: South::Tall,
            });
        }
        if state_id == 18532 {
            return Some(MudBrickWall {
                r#up: false,
                r#east: East::Tall,
                r#south: South::None,
                r#waterlogged: true,
                r#north: North::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 18555 {
            return Some(MudBrickWall {
                r#up: false,
                r#east: East::Tall,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 18503 {
            return Some(MudBrickWall {
                r#south: South::Low,
                r#up: true,
                r#north: North::Low,
                r#east: East::Tall,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 18492 {
            return Some(MudBrickWall {
                r#up: true,
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 18424 {
            return Some(MudBrickWall {
                r#waterlogged: true,
                r#south: South::None,
                r#east: East::Low,
                r#north: North::Tall,
                r#up: false,
                r#west: West::Tall,
            });
        }
        if state_id == 18474 {
            return Some(MudBrickWall {
                r#south: South::Low,
                r#east: East::Tall,
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::None,
                r#up: false,
            });
        }
        if state_id == 18447 {
            return Some(MudBrickWall {
                r#west: West::Low,
                r#north: North::Tall,
                r#east: East::Low,
                r#up: false,
                r#waterlogged: true,
                r#south: South::Tall,
            });
        }
        if state_id == 18511 {
            return Some(MudBrickWall {
                r#west: West::Tall,
                r#north: North::Low,
                r#waterlogged: false,
                r#south: South::Low,
                r#east: East::Tall,
                r#up: false,
            });
        }
        if state_id == 18475 {
            return Some(MudBrickWall {
                r#south: South::Low,
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::None,
            });
        }
        if state_id == 18547 {
            return Some(MudBrickWall {
                r#south: South::Low,
                r#east: East::Tall,
                r#west: West::Tall,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 18491 {
            return Some(MudBrickWall {
                r#north: North::Low,
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::None,
                r#east: East::Tall,
                r#up: true,
            });
        }
        if state_id == 18260 {
            return Some(MudBrickWall {
                r#east: East::None,
                r#west: West::None,
                r#north: North::None,
                r#waterlogged: true,
                r#south: South::Tall,
                r#up: true,
            });
        }
        if state_id == 18311 {
            return Some(MudBrickWall {
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::Tall,
                r#east: East::None,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 18340 {
            return Some(MudBrickWall {
                r#south: South::Tall,
                r#up: false,
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::None,
            });
        }
        if state_id == 18488 {
            return Some(MudBrickWall {
                r#north: North::Low,
                r#up: true,
                r#waterlogged: true,
                r#south: South::None,
                r#east: East::Tall,
                r#west: West::None,
            });
        }
        if state_id == 18314 {
            return Some(MudBrickWall {
                r#east: East::None,
                r#north: North::Tall,
                r#up: false,
                r#west: West::None,
                r#waterlogged: true,
                r#south: South::None,
            });
        }
        if state_id == 18381 {
            return Some(MudBrickWall {
                r#east: East::Low,
                r#south: South::None,
                r#west: West::Low,
                r#waterlogged: true,
                r#up: true,
                r#north: North::Low,
            });
        }
        if state_id == 18498 {
            return Some(MudBrickWall {
                r#east: East::Tall,
                r#north: North::Low,
                r#waterlogged: false,
                r#west: West::Low,
                r#up: false,
                r#south: South::None,
            });
        }
        if state_id == 18510 {
            return Some(MudBrickWall {
                r#up: false,
                r#east: East::Tall,
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::Low,
                r#south: South::Low,
            });
        }
        if state_id == 18267 {
            return Some(MudBrickWall {
                r#north: North::None,
                r#waterlogged: true,
                r#west: West::Low,
                r#up: false,
                r#east: East::None,
                r#south: South::Tall,
            });
        }
        if state_id == 18376 {
            return Some(MudBrickWall {
                r#north: North::None,
                r#waterlogged: true,
                r#east: East::Low,
                r#south: South::Tall,
                r#up: false,
                r#west: West::Tall,
            });
        }
        if state_id == 18397 {
            return Some(MudBrickWall {
                r#east: East::Low,
                r#up: true,
                r#waterlogged: false,
                r#north: North::Low,
                r#west: West::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 18291 {
            return Some(MudBrickWall {
                r#north: North::Low,
                r#west: West::Low,
                r#up: false,
                r#waterlogged: true,
                r#east: East::None,
                r#south: South::Low,
            });
        }
        if state_id == 18386 {
            return Some(MudBrickWall {
                r#south: South::None,
                r#waterlogged: true,
                r#north: North::Low,
                r#east: East::Low,
                r#up: false,
                r#west: West::None,
            });
        }
        if state_id == 18249 {
            return Some(MudBrickWall {
                r#north: North::None,
                r#waterlogged: true,
                r#south: South::Low,
                r#east: East::None,
                r#west: West::Low,
                r#up: true,
            });
        }
        if state_id == 18302 {
            return Some(MudBrickWall {
                r#waterlogged: true,
                r#east: East::None,
                r#up: false,
                r#south: South::Tall,
                r#north: North::Low,
                r#west: West::None,
            });
        }
        if state_id == 18323 {
            return Some(MudBrickWall {
                r#west: West::None,
                r#east: East::None,
                r#up: true,
                r#waterlogged: false,
                r#south: South::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 18554 {
            return Some(MudBrickWall {
                r#up: false,
                r#west: West::None,
                r#south: South::Tall,
                r#waterlogged: true,
                r#north: North::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 18526 {
            return Some(MudBrickWall {
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: true,
                r#north: North::Tall,
                r#west: West::Tall,
                r#south: South::None,
            });
        }
        if state_id == 18276 {
            return Some(MudBrickWall {
                r#up: true,
                r#east: East::None,
                r#north: North::Low,
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::None,
            });
        }
        if state_id == 18358 {
            return Some(MudBrickWall {
                r#south: South::Low,
                r#waterlogged: true,
                r#east: East::Low,
                r#up: true,
                r#west: West::Tall,
                r#north: North::None,
            });
        }
        if state_id == 18290 {
            return Some(MudBrickWall {
                r#south: South::Low,
                r#west: West::None,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: true,
                r#east: East::None,
            });
        }
        if state_id == 18520 {
            return Some(MudBrickWall {
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::Tall,
                r#east: East::Tall,
                r#north: North::Low,
                r#up: false,
            });
        }
        if state_id == 18509 {
            return Some(MudBrickWall {
                r#east: East::Tall,
                r#west: West::None,
                r#up: false,
                r#north: North::Low,
                r#waterlogged: false,
                r#south: South::Low,
            });
        }
        if state_id == 18419 {
            return Some(MudBrickWall {
                r#up: true,
                r#east: East::Low,
                r#west: West::None,
                r#north: North::Tall,
                r#waterlogged: false,
                r#south: South::None,
            });
        }
        if state_id == 18375 {
            return Some(MudBrickWall {
                r#west: West::Low,
                r#up: false,
                r#east: East::Low,
                r#waterlogged: true,
                r#north: North::None,
                r#south: South::Tall,
            });
        }
        if state_id == 18380 {
            return Some(MudBrickWall {
                r#north: North::Low,
                r#waterlogged: true,
                r#east: East::Low,
                r#south: South::None,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 18452 {
            return Some(MudBrickWall {
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::None,
                r#north: North::None,
                r#up: true,
                r#east: East::Tall,
            });
        }
        if state_id == 18518 {
            return Some(MudBrickWall {
                r#east: East::Tall,
                r#up: false,
                r#south: South::Tall,
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 18552 {
            return Some(MudBrickWall {
                r#up: true,
                r#east: East::Tall,
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 18426 {
            return Some(MudBrickWall {
                r#up: false,
                r#waterlogged: false,
                r#south: South::None,
                r#east: East::Low,
                r#west: West::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 18473 {
            return Some(MudBrickWall {
                r#south: South::Low,
                r#east: East::Tall,
                r#west: West::None,
                r#up: false,
                r#waterlogged: false,
                r#north: North::None,
            });
        }
        if state_id == 18485 {
            return Some(MudBrickWall {
                r#west: West::None,
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: false,
                r#south: South::Tall,
                r#north: North::None,
            });
        }
        if state_id == 18477 {
            return Some(MudBrickWall {
                r#waterlogged: true,
                r#south: South::Tall,
                r#east: East::Tall,
                r#west: West::Low,
                r#north: North::None,
                r#up: true,
            });
        }
        if state_id == 18549 {
            return Some(MudBrickWall {
                r#north: North::Tall,
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::Tall,
                r#up: true,
            });
        }
        if state_id == 18553 {
            return Some(MudBrickWall {
                r#waterlogged: false,
                r#south: South::Tall,
                r#north: North::Tall,
                r#up: true,
                r#west: West::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 18483 {
            return Some(MudBrickWall {
                r#waterlogged: true,
                r#south: South::Tall,
                r#up: false,
                r#west: West::Low,
                r#east: East::Tall,
                r#north: North::None,
            });
        }
        if state_id == 18275 {
            return Some(MudBrickWall {
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::None,
                r#south: South::None,
                r#north: North::Low,
                r#up: true,
            });
        }
        if state_id == 18345 {
            return Some(MudBrickWall {
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::Low,
                r#north: North::None,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 18272 {
            return Some(MudBrickWall {
                r#west: West::None,
                r#waterlogged: true,
                r#south: South::None,
                r#north: North::Low,
                r#up: true,
                r#east: East::None,
            });
        }
        if state_id == 18252 {
            return Some(MudBrickWall {
                r#up: true,
                r#east: East::None,
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::None,
            });
        }
        if state_id == 18278 {
            return Some(MudBrickWall {
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::Low,
                r#east: East::None,
                r#south: South::None,
            });
        }
        if state_id == 18274 {
            return Some(MudBrickWall {
                r#east: East::None,
                r#south: South::None,
                r#west: West::Tall,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 18292 {
            return Some(MudBrickWall {
                r#north: North::Low,
                r#south: South::Low,
                r#up: false,
                r#east: East::None,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 18365 {
            return Some(MudBrickWall {
                r#north: North::None,
                r#west: West::None,
                r#up: false,
                r#south: South::Low,
                r#east: East::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 18441 {
            return Some(MudBrickWall {
                r#west: West::Low,
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: true,
                r#south: South::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 18535 {
            return Some(MudBrickWall {
                r#east: East::Tall,
                r#up: false,
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 18407 {
            return Some(MudBrickWall {
                r#north: North::Low,
                r#waterlogged: false,
                r#south: South::Tall,
                r#east: East::Low,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 18404 {
            return Some(MudBrickWall {
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 18559 {
            return Some(MudBrickWall {
                r#east: East::Tall,
                r#up: false,
                r#north: North::Tall,
                r#south: South::Tall,
                r#west: West::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 18350 {
            return Some(MudBrickWall {
                r#north: North::None,
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::Low,
                r#south: South::None,
                r#up: false,
            });
        }
        if state_id == 18240 {
            return Some(MudBrickWall {
                r#north: North::None,
                r#south: South::None,
                r#up: true,
                r#waterlogged: false,
                r#east: East::None,
                r#west: West::Low,
            });
        }
        if state_id == 18270 {
            return Some(MudBrickWall {
                r#east: East::None,
                r#up: false,
                r#north: North::None,
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 18321 {
            return Some(MudBrickWall {
                r#up: true,
                r#waterlogged: true,
                r#south: South::Low,
                r#west: West::Low,
                r#east: East::None,
                r#north: North::Tall,
            });
        }
        if state_id == 18353 {
            return Some(MudBrickWall {
                r#up: false,
                r#waterlogged: false,
                r#south: South::None,
                r#north: North::None,
                r#east: East::Low,
                r#west: West::None,
            });
        }
        if state_id == 18493 {
            return Some(MudBrickWall {
                r#up: true,
                r#waterlogged: false,
                r#north: North::Low,
                r#east: East::Tall,
                r#south: South::None,
                r#west: West::Tall,
            });
        }
        if state_id == 18357 {
            return Some(MudBrickWall {
                r#east: East::Low,
                r#north: North::None,
                r#waterlogged: true,
                r#up: true,
                r#west: West::Low,
                r#south: South::Low,
            });
        }
        if state_id == 18444 {
            return Some(MudBrickWall {
                r#north: North::Tall,
                r#west: West::Low,
                r#east: East::Low,
                r#south: South::Tall,
                r#waterlogged: false,
                r#up: true,
            });
        }
        if state_id == 18239 {
            return Some(MudBrickWall {
                r#waterlogged: false,
                r#south: South::None,
                r#west: West::None,
                r#north: North::None,
                r#east: East::None,
                r#up: true,
            });
        }
        if state_id == 18415 {
            return Some(MudBrickWall {
                r#up: false,
                r#waterlogged: false,
                r#north: North::Low,
                r#east: East::Low,
                r#west: West::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 18382 {
            return Some(MudBrickWall {
                r#east: East::Low,
                r#waterlogged: true,
                r#up: true,
                r#south: South::None,
                r#west: West::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 18383 {
            return Some(MudBrickWall {
                r#west: West::None,
                r#north: North::Low,
                r#waterlogged: false,
                r#east: East::Low,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 18478 {
            return Some(MudBrickWall {
                r#south: South::Tall,
                r#east: East::Tall,
                r#west: West::Tall,
                r#up: true,
                r#waterlogged: true,
                r#north: North::None,
            });
        }
        if state_id == 18384 {
            return Some(MudBrickWall {
                r#south: South::None,
                r#north: North::Low,
                r#up: true,
                r#east: East::Low,
                r#west: West::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 18273 {
            return Some(MudBrickWall {
                r#up: true,
                r#waterlogged: true,
                r#east: East::None,
                r#west: West::Low,
                r#north: North::Low,
                r#south: South::None,
            });
        }
        if state_id == 18337 {
            return Some(MudBrickWall {
                r#south: South::Tall,
                r#east: East::None,
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 18377 {
            return Some(MudBrickWall {
                r#north: North::None,
                r#east: East::Low,
                r#up: false,
                r#west: West::None,
                r#waterlogged: false,
                r#south: South::Tall,
            });
        }
        if state_id == 18296 {
            return Some(MudBrickWall {
                r#up: true,
                r#south: South::Tall,
                r#west: West::None,
                r#north: North::Low,
                r#waterlogged: true,
                r#east: East::None,
            });
        }
        if state_id == 18465 {
            return Some(MudBrickWall {
                r#up: true,
                r#west: West::Low,
                r#waterlogged: true,
                r#east: East::Tall,
                r#south: South::Low,
                r#north: North::None,
            });
        }
        if state_id == 18446 {
            return Some(MudBrickWall {
                r#east: East::Low,
                r#up: false,
                r#waterlogged: true,
                r#north: North::Tall,
                r#west: West::None,
                r#south: South::Tall,
            });
        }
        if state_id == 18320 {
            return Some(MudBrickWall {
                r#waterlogged: true,
                r#west: West::None,
                r#up: true,
                r#north: North::Tall,
                r#south: South::Low,
                r#east: East::None,
            });
        }
        if state_id == 18453 {
            return Some(MudBrickWall {
                r#waterlogged: true,
                r#east: East::Tall,
                r#south: South::None,
                r#north: North::None,
                r#west: West::Low,
                r#up: true,
            });
        }
        if state_id == 18531 {
            return Some(MudBrickWall {
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::None,
                r#east: East::Tall,
                r#up: false,
                r#north: North::Tall,
            });
        }
        if state_id == 18298 {
            return Some(MudBrickWall {
                r#west: West::Tall,
                r#south: South::Tall,
                r#up: true,
                r#east: East::None,
                r#north: North::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 18362 {
            return Some(MudBrickWall {
                r#north: North::None,
                r#south: South::Low,
                r#up: false,
                r#west: West::None,
                r#east: East::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 18277 {
            return Some(MudBrickWall {
                r#west: West::Tall,
                r#waterlogged: false,
                r#north: North::Low,
                r#east: East::None,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 18370 {
            return Some(MudBrickWall {
                r#north: North::None,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: true,
                r#east: East::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 18268 {
            return Some(MudBrickWall {
                r#south: South::Tall,
                r#east: East::None,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::None,
            });
        }
        if state_id == 18429 {
            return Some(MudBrickWall {
                r#north: North::Tall,
                r#west: West::Low,
                r#waterlogged: true,
                r#south: South::Low,
                r#east: East::Low,
                r#up: true,
            });
        }
        if state_id == 18264 {
            return Some(MudBrickWall {
                r#waterlogged: false,
                r#up: true,
                r#south: South::Tall,
                r#north: North::None,
                r#east: East::None,
                r#west: West::Low,
            });
        }
        if state_id == 18335 {
            return Some(MudBrickWall {
                r#up: true,
                r#west: West::None,
                r#north: North::Tall,
                r#south: South::Tall,
                r#waterlogged: false,
                r#east: East::None,
            });
        }
        if state_id == 18451 {
            return Some(MudBrickWall {
                r#east: East::Low,
                r#north: North::Tall,
                r#west: West::Tall,
                r#up: false,
                r#south: South::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 18481 {
            return Some(MudBrickWall {
                r#east: East::Tall,
                r#north: North::None,
                r#south: South::Tall,
                r#up: true,
                r#west: West::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 18385 {
            return Some(MudBrickWall {
                r#waterlogged: false,
                r#south: South::None,
                r#north: North::Low,
                r#up: true,
                r#west: West::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 18387 {
            return Some(MudBrickWall {
                r#west: West::Low,
                r#east: East::Low,
                r#waterlogged: true,
                r#north: North::Low,
                r#south: South::None,
                r#up: false,
            });
        }
        if state_id == 18405 {
            return Some(MudBrickWall {
                r#west: West::Low,
                r#east: East::Low,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: true,
                r#north: North::Low,
            });
        }
        if state_id == 18300 {
            return Some(MudBrickWall {
                r#north: North::Low,
                r#east: East::None,
                r#up: true,
                r#west: West::Low,
                r#waterlogged: false,
                r#south: South::Tall,
            });
        }
        if state_id == 18390 {
            return Some(MudBrickWall {
                r#up: false,
                r#south: South::None,
                r#north: North::Low,
                r#west: West::Low,
                r#east: East::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 18489 {
            return Some(MudBrickWall {
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 18514 {
            return Some(MudBrickWall {
                r#north: North::Low,
                r#west: West::Tall,
                r#waterlogged: true,
                r#east: East::Tall,
                r#south: South::Tall,
                r#up: true,
            });
        }
        if state_id == 18342 {
            return Some(MudBrickWall {
                r#north: North::Tall,
                r#waterlogged: false,
                r#east: East::None,
                r#south: South::Tall,
                r#west: West::Low,
                r#up: false,
            });
        }
        return None;
    }
}

