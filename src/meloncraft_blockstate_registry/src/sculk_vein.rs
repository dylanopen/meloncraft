use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SculkVein {
    pub south: bool,
    pub up: bool,
    pub down: bool,
    pub waterlogged: bool,
    pub east: bool,
    pub north: bool,
    pub west: bool,
}

impl BlockState for SculkVein {
    fn to_id(&self) -> i32 {
        if self.r#east == true
            && self.r#up == false
            && self.r#down == false
            && self.r#south == false
            && self.r#waterlogged == true
            && self.r#west == false
            && self.r#north == true
        {
            return 25046;
        }
        if self.r#east == false
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#west == true
            && self.r#north == true
            && self.r#down == true
            && self.r#south == true
        {
            return 25001;
        }
        if self.r#waterlogged == false
            && self.r#west == false
            && self.r#north == true
            && self.r#down == true
            && self.r#south == true
            && self.r#east == true
            && self.r#up == true
        {
            return 24972;
        }
        if self.r#east == true
            && self.r#down == true
            && self.r#north == false
            && self.r#up == false
            && self.r#south == true
            && self.r#waterlogged == false
            && self.r#west == false
        {
            return 24992;
        }
        if self.r#east == false
            && self.r#north == true
            && self.r#waterlogged == true
            && self.r#down == true
            && self.r#south == true
            && self.r#west == true
            && self.r#up == false
        {
            return 25005;
        }
        if self.r#south == true
            && self.r#north == false
            && self.r#waterlogged == false
            && self.r#east == true
            && self.r#west == false
            && self.r#up == true
            && self.r#down == false
        {
            return 25052;
        }
        if self.r#up == false
            && self.r#waterlogged == true
            && self.r#north == true
            && self.r#west == false
            && self.r#down == true
            && self.r#east == true
            && self.r#south == true
        {
            return 24974;
        }
        if self.r#east == true
            && self.r#south == true
            && self.r#up == true
            && self.r#down == false
            && self.r#west == false
            && self.r#north == true
            && self.r#waterlogged == true
        {
            return 25034;
        }
        if self.r#west == false
            && self.r#waterlogged == false
            && self.r#down == true
            && self.r#east == false
            && self.r#north == false
            && self.r#up == true
            && self.r#south == false
        {
            return 25028;
        }
        if self.r#north == true
            && self.r#south == true
            && self.r#west == false
            && self.r#up == false
            && self.r#down == false
            && self.r#waterlogged == true
            && self.r#east == true
        {
            return 25038;
        }
        if self.r#waterlogged == false
            && self.r#west == true
            && self.r#east == true
            && self.r#down == false
            && self.r#south == true
            && self.r#north == false
            && self.r#up == false
        {
            return 25055;
        }
        if self.r#up == true
            && self.r#north == false
            && self.r#waterlogged == false
            && self.r#west == false
            && self.r#south == false
            && self.r#down == false
            && self.r#east == true
        {
            return 25060;
        }
        if self.r#west == false
            && self.r#south == true
            && self.r#waterlogged == true
            && self.r#east == true
            && self.r#north == false
            && self.r#up == true
            && self.r#down == false
        {
            return 25050;
        }
        if self.r#up == false
            && self.r#waterlogged == false
            && self.r#down == false
            && self.r#north == false
            && self.r#west == true
            && self.r#east == true
            && self.r#south == false
        {
            return 25063;
        }
        if self.r#waterlogged == false
            && self.r#east == true
            && self.r#north == false
            && self.r#west == false
            && self.r#down == false
            && self.r#south == false
            && self.r#up == false
        {
            return 25064;
        }
        if self.r#south == false
            && self.r#west == false
            && self.r#east == true
            && self.r#north == true
            && self.r#down == true
            && self.r#up == false
            && self.r#waterlogged == true
        {
            return 24982;
        }
        if self.r#waterlogged == false
            && self.r#west == true
            && self.r#down == false
            && self.r#south == true
            && self.r#east == true
            && self.r#north == true
            && self.r#up == false
        {
            return 25039;
        }
        if self.r#waterlogged == true
            && self.r#west == true
            && self.r#down == true
            && self.r#north == true
            && self.r#up == true
            && self.r#east == true
            && self.r#south == true
        {
            return 24969;
        }
        if self.r#waterlogged == false
            && self.r#west == true
            && self.r#north == true
            && self.r#south == true
            && self.r#up == false
            && self.r#down == false
            && self.r#east == false
        {
            return 25071;
        }
        if self.r#north == true
            && self.r#down == true
            && self.r#south == true
            && self.r#east == true
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#west == true
        {
            return 24971;
        }
        if self.r#east == false
            && self.r#south == false
            && self.r#up == false
            && self.r#down == true
            && self.r#west == false
            && self.r#waterlogged == false
            && self.r#north == false
        {
            return 25032;
        }
        if self.r#up == false
            && self.r#waterlogged == false
            && self.r#west == true
            && self.r#east == true
            && self.r#north == false
            && self.r#south == true
            && self.r#down == true
        {
            return 24991;
        }
        if self.r#north == false
            && self.r#east == true
            && self.r#waterlogged == true
            && self.r#up == false
            && self.r#south == true
            && self.r#west == true
            && self.r#down == false
        {
            return 25053;
        }
        if self.r#up == true
            && self.r#waterlogged == false
            && self.r#west == false
            && self.r#down == true
            && self.r#east == false
            && self.r#south == true
            && self.r#north == true
        {
            return 25004;
        }
        if self.r#south == false
            && self.r#waterlogged == true
            && self.r#west == true
            && self.r#east == true
            && self.r#north == true
            && self.r#up == true
            && self.r#down == false
        {
            return 25041;
        }
        if self.r#west == true
            && self.r#down == false
            && self.r#south == false
            && self.r#north == true
            && self.r#up == true
            && self.r#east == false
            && self.r#waterlogged == true
        {
            return 25073;
        }
        if self.r#waterlogged == false
            && self.r#east == false
            && self.r#down == true
            && self.r#south == true
            && self.r#west == false
            && self.r#up == false
            && self.r#north == false
        {
            return 25024;
        }
        if self.r#up == true
            && self.r#waterlogged == false
            && self.r#west == false
            && self.r#north == true
            && self.r#east == true
            && self.r#down == false
            && self.r#south == true
        {
            return 25036;
        }
        if self.r#north == false
            && self.r#down == true
            && self.r#up == true
            && self.r#east == false
            && self.r#waterlogged == false
            && self.r#south == true
            && self.r#west == false
        {
            return 25020;
        }
        if self.r#west == false
            && self.r#east == true
            && self.r#south == false
            && self.r#down == true
            && self.r#up == true
            && self.r#north == false
            && self.r#waterlogged == false
        {
            return 24996;
        }
        if self.r#south == false
            && self.r#east == false
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#west == true
            && self.r#north == false
            && self.r#down == false
        {
            return 25091;
        }
        if self.r#down == false
            && self.r#waterlogged == false
            && self.r#east == true
            && self.r#north == false
            && self.r#south == true
            && self.r#west == false
            && self.r#up == false
        {
            return 25056;
        }
        if self.r#north == false
            && self.r#waterlogged == true
            && self.r#down == true
            && self.r#up == true
            && self.r#south == true
            && self.r#east == false
            && self.r#west == true
        {
            return 25017;
        }
        if self.r#down == true
            && self.r#west == false
            && self.r#east == true
            && self.r#south == true
            && self.r#waterlogged == false
            && self.r#up == false
            && self.r#north == true
        {
            return 24976;
        }
        if self.r#down == true
            && self.r#up == true
            && self.r#east == true
            && self.r#south == false
            && self.r#waterlogged == true
            && self.r#north == true
            && self.r#west == true
        {
            return 24977;
        }
        if self.r#east == false
            && self.r#north == false
            && self.r#west == true
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#south == true
            && self.r#down == true
        {
            return 25019;
        }
        if self.r#south == true
            && self.r#down == true
            && self.r#east == true
            && self.r#north == true
            && self.r#waterlogged == true
            && self.r#west == false
            && self.r#up == true
        {
            return 24970;
        }
        if self.r#east == false
            && self.r#north == false
            && self.r#south == true
            && self.r#up == false
            && self.r#down == true
            && self.r#waterlogged == false
            && self.r#west == true
        {
            return 25023;
        }
        if self.r#west == true
            && self.r#down == false
            && self.r#north == false
            && self.r#south == true
            && self.r#east == true
            && self.r#up == true
            && self.r#waterlogged == false
        {
            return 25051;
        }
        if self.r#up == false
            && self.r#north == true
            && self.r#waterlogged == true
            && self.r#south == false
            && self.r#west == true
            && self.r#down == true
            && self.r#east == false
        {
            return 25013;
        }
        if self.r#north == false
            && self.r#south == true
            && self.r#waterlogged == true
            && self.r#down == true
            && self.r#west == false
            && self.r#up == false
            && self.r#east == true
        {
            return 24990;
        }
        if self.r#down == false
            && self.r#waterlogged == false
            && self.r#east == false
            && self.r#up == true
            && self.r#west == false
            && self.r#south == false
            && self.r#north == true
        {
            return 25076;
        }
        if self.r#east == true
            && self.r#west == true
            && self.r#down == true
            && self.r#south == false
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#north == false
        {
            return 24999;
        }
        if self.r#up == false
            && self.r#waterlogged == true
            && self.r#west == true
            && self.r#down == false
            && self.r#north == true
            && self.r#east == false
            && self.r#south == false
        {
            return 25077;
        }
        if self.r#north == true
            && self.r#waterlogged == true
            && self.r#south == true
            && self.r#up == true
            && self.r#east == false
            && self.r#west == false
            && self.r#down == true
        {
            return 25002;
        }
        if self.r#south == false
            && self.r#east == true
            && self.r#north == true
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#west == true
            && self.r#down == false
        {
            return 25043;
        }
        if self.r#north == false
            && self.r#south == false
            && self.r#east == false
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#west == true
            && self.r#down == false
        {
            return 25089;
        }
        if self.r#west == true
            && self.r#waterlogged == false
            && self.r#north == true
            && self.r#south == false
            && self.r#east == false
            && self.r#down == false
            && self.r#up == false
        {
            return 25079;
        }
        if self.r#down == false
            && self.r#east == true
            && self.r#north == false
            && self.r#south == false
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#west == false
        {
            return 25058;
        }
        if self.r#north == false
            && self.r#south == false
            && self.r#west == false
            && self.r#down == false
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#east == false
        {
            return 25090;
        }
        if self.r#north == true
            && self.r#south == true
            && self.r#east == false
            && self.r#waterlogged == true
            && self.r#west == false
            && self.r#down == true
            && self.r#up == false
        {
            return 25006;
        }
        if self.r#west == true
            && self.r#down == true
            && self.r#east == true
            && self.r#north == true
            && self.r#south == true
            && self.r#up == false
            && self.r#waterlogged == false
        {
            return 24975;
        }
        if self.r#south == false
            && self.r#west == false
            && self.r#east == true
            && self.r#waterlogged == false
            && self.r#up == false
            && self.r#down == true
            && self.r#north == false
        {
            return 25000;
        }
        if self.r#north == false
            && self.r#waterlogged == false
            && self.r#down == true
            && self.r#west == true
            && self.r#up == true
            && self.r#south == true
            && self.r#east == true
        {
            return 24987;
        }
        if self.r#down == true
            && self.r#south == false
            && self.r#north == false
            && self.r#waterlogged == true
            && self.r#up == false
            && self.r#east == true
            && self.r#west == false
        {
            return 24998;
        }
        if self.r#up == false
            && self.r#east == false
            && self.r#south == true
            && self.r#west == true
            && self.r#down == true
            && self.r#waterlogged == true
            && self.r#north == false
        {
            return 25021;
        }
        if self.r#north == true
            && self.r#south == false
            && self.r#up == false
            && self.r#down == false
            && self.r#waterlogged == true
            && self.r#west == true
            && self.r#east == true
        {
            return 25045;
        }
        if self.r#east == true
            && self.r#north == true
            && self.r#south == false
            && self.r#west == false
            && self.r#waterlogged == false
            && self.r#down == false
            && self.r#up == true
        {
            return 25044;
        }
        if self.r#north == true
            && self.r#up == true
            && self.r#west == false
            && self.r#south == true
            && self.r#down == false
            && self.r#east == false
            && self.r#waterlogged == true
        {
            return 25066;
        }
        if self.r#up == true
            && self.r#east == false
            && self.r#down == false
            && self.r#north == true
            && self.r#waterlogged == true
            && self.r#south == false
            && self.r#west == false
        {
            return 25074;
        }
        if self.r#east == false
            && self.r#south == false
            && self.r#up == true
            && self.r#north == false
            && self.r#down == true
            && self.r#waterlogged == false
            && self.r#west == true
        {
            return 25027;
        }
        if self.r#waterlogged == true
            && self.r#west == true
            && self.r#down == false
            && self.r#up == false
            && self.r#east == true
            && self.r#north == true
            && self.r#south == true
        {
            return 25037;
        }
        if self.r#up == false
            && self.r#west == false
            && self.r#waterlogged == true
            && self.r#south == false
            && self.r#down == false
            && self.r#east == false
            && self.r#north == false
        {
            return 25094;
        }
        if self.r#waterlogged == false
            && self.r#west == true
            && self.r#south == false
            && self.r#down == true
            && self.r#north == true
            && self.r#up == false
            && self.r#east == true
        {
            return 24983;
        }
        if self.r#east == true
            && self.r#south == true
            && self.r#north == true
            && self.r#down == false
            && self.r#west == true
            && self.r#up == true
            && self.r#waterlogged == false
        {
            return 25035;
        }
        if self.r#up == true
            && self.r#west == false
            && self.r#north == true
            && self.r#waterlogged == false
            && self.r#east == true
            && self.r#down == true
            && self.r#south == false
        {
            return 24980;
        }
        if self.r#south == false
            && self.r#waterlogged == false
            && self.r#west == true
            && self.r#down == true
            && self.r#east == false
            && self.r#north == true
            && self.r#up == false
        {
            return 25015;
        }
        if self.r#down == false
            && self.r#up == false
            && self.r#south == false
            && self.r#waterlogged == false
            && self.r#east == true
            && self.r#west == true
            && self.r#north == true
        {
            return 25047;
        }
        if self.r#up == true
            && self.r#west == false
            && self.r#east == true
            && self.r#waterlogged == false
            && self.r#south == true
            && self.r#north == false
            && self.r#down == true
        {
            return 24988;
        }
        if self.r#east == false
            && self.r#south == true
            && self.r#north == true
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#west == true
            && self.r#down == true
        {
            return 25003;
        }
        if self.r#north == true
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#west == false
            && self.r#south == false
            && self.r#down == true
            && self.r#east == false
        {
            return 25014;
        }
        if self.r#up == false
            && self.r#east == false
            && self.r#waterlogged == true
            && self.r#west == false
            && self.r#down == true
            && self.r#north == false
            && self.r#south == true
        {
            return 25022;
        }
        if self.r#west == true
            && self.r#waterlogged == false
            && self.r#east == true
            && self.r#down == true
            && self.r#south == false
            && self.r#up == true
            && self.r#north == true
        {
            return 24979;
        }
        if self.r#west == true
            && self.r#north == true
            && self.r#up == false
            && self.r#east == true
            && self.r#waterlogged == true
            && self.r#south == true
            && self.r#down == true
        {
            return 24973;
        }
        if self.r#east == false
            && self.r#down == false
            && self.r#north == true
            && self.r#south == true
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#west == false
        {
            return 25072;
        }
        if self.r#south == true
            && self.r#east == false
            && self.r#north == true
            && self.r#down == false
            && self.r#up == false
            && self.r#waterlogged == true
            && self.r#west == true
        {
            return 25069;
        }
        if self.r#down == true
            && self.r#east == false
            && self.r#north == false
            && self.r#south == false
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#west == true
        {
            return 25025;
        }
        if self.r#west == false
            && self.r#north == true
            && self.r#east == false
            && self.r#down == true
            && self.r#up == false
            && self.r#south == false
            && self.r#waterlogged == false
        {
            return 25016;
        }
        if self.r#down == false
            && self.r#west == false
            && self.r#east == true
            && self.r#north == true
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#south == false
        {
            return 25048;
        }
        if self.r#south == true
            && self.r#west == true
            && self.r#north == false
            && self.r#down == false
            && self.r#east == true
            && self.r#up == true
            && self.r#waterlogged == true
        {
            return 25049;
        }
        if self.r#down == false
            && self.r#east == false
            && self.r#south == true
            && self.r#up == true
            && self.r#west == true
            && self.r#north == true
            && self.r#waterlogged == false
        {
            return 25067;
        }
        if self.r#west == false
            && self.r#down == true
            && self.r#east == true
            && self.r#north == false
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#south == true
        {
            return 24986;
        }
        if self.r#up == true
            && self.r#waterlogged == true
            && self.r#down == true
            && self.r#south == false
            && self.r#north == true
            && self.r#west == true
            && self.r#east == false
        {
            return 25009;
        }
        if self.r#west == false
            && self.r#east == false
            && self.r#north == false
            && self.r#down == true
            && self.r#south == true
            && self.r#up == true
            && self.r#waterlogged == true
        {
            return 25018;
        }
        if self.r#east == true
            && self.r#north == false
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#west == true
            && self.r#south == false
            && self.r#down == false
        {
            return 25057;
        }
        if self.r#east == true
            && self.r#north == false
            && self.r#west == false
            && self.r#down == false
            && self.r#south == false
            && self.r#waterlogged == true
            && self.r#up == false
        {
            return 25062;
        }
        if self.r#west == false
            && self.r#waterlogged == false
            && self.r#down == true
            && self.r#south == false
            && self.r#up == true
            && self.r#north == true
            && self.r#east == false
        {
            return 25012;
        }
        if self.r#south == true
            && self.r#down == false
            && self.r#waterlogged == false
            && self.r#north == false
            && self.r#west == true
            && self.r#up == true
            && self.r#east == false
        {
            return 25083;
        }
        if self.r#down == false
            && self.r#up == false
            && self.r#east == false
            && self.r#south == true
            && self.r#waterlogged == false
            && self.r#north == false
            && self.r#west == true
        {
            return 25087;
        }
        if self.r#east == false
            && self.r#up == true
            && self.r#south == false
            && self.r#north == true
            && self.r#down == true
            && self.r#waterlogged == true
            && self.r#west == false
        {
            return 25010;
        }
        if self.r#down == true
            && self.r#waterlogged == true
            && self.r#west == true
            && self.r#south == true
            && self.r#north == false
            && self.r#up == true
            && self.r#east == true
        {
            return 24985;
        }
        if self.r#north == false
            && self.r#up == true
            && self.r#west == false
            && self.r#waterlogged == true
            && self.r#down == true
            && self.r#south == false
            && self.r#east == true
        {
            return 24994;
        }
        if self.r#east == false
            && self.r#down == false
            && self.r#west == true
            && self.r#south == true
            && self.r#up == true
            && self.r#north == false
            && self.r#waterlogged == true
        {
            return 25081;
        }
        if self.r#waterlogged == false
            && self.r#east == false
            && self.r#up == true
            && self.r#west == false
            && self.r#down == false
            && self.r#south == false
            && self.r#north == false
        {
            return 25092;
        }
        if self.r#south == true
            && self.r#waterlogged == true
            && self.r#east == false
            && self.r#north == true
            && self.r#down == false
            && self.r#up == false
            && self.r#west == false
        {
            return 25070;
        }
        if self.r#west == true
            && self.r#up == false
            && self.r#north == false
            && self.r#east == true
            && self.r#south == false
            && self.r#waterlogged == true
            && self.r#down == true
        {
            return 24997;
        }
        if self.r#west == true
            && self.r#waterlogged == true
            && self.r#down == true
            && self.r#north == false
            && self.r#east == false
            && self.r#south == false
            && self.r#up == false
        {
            return 25029;
        }
        if self.r#east == false
            && self.r#west == true
            && self.r#north == false
            && self.r#down == true
            && self.r#waterlogged == false
            && self.r#south == false
            && self.r#up == false
        {
            return 25031;
        }
        if self.r#east == true
            && self.r#south == false
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#west == true
            && self.r#north == false
            && self.r#down == false
        {
            return 25059;
        }
        if self.r#up == true
            && self.r#south == true
            && self.r#down == false
            && self.r#east == false
            && self.r#west == false
            && self.r#waterlogged == true
            && self.r#north == false
        {
            return 25082;
        }
        if self.r#west == true
            && self.r#down == true
            && self.r#waterlogged == true
            && self.r#north == true
            && self.r#south == false
            && self.r#up == false
            && self.r#east == true
        {
            return 24981;
        }
        if self.r#up == false
            && self.r#south == false
            && self.r#waterlogged == false
            && self.r#down == true
            && self.r#west == false
            && self.r#east == true
            && self.r#north == true
        {
            return 24984;
        }
        if self.r#down == true
            && self.r#north == true
            && self.r#up == true
            && self.r#west == false
            && self.r#east == true
            && self.r#south == false
            && self.r#waterlogged == true
        {
            return 24978;
        }
        if self.r#east == true
            && self.r#waterlogged == false
            && self.r#up == true
            && self.r#north == false
            && self.r#south == false
            && self.r#west == true
            && self.r#down == true
        {
            return 24995;
        }
        if self.r#waterlogged == false
            && self.r#down == true
            && self.r#west == true
            && self.r#east == false
            && self.r#north == true
            && self.r#south == false
            && self.r#up == true
        {
            return 25011;
        }
        if self.r#down == false
            && self.r#east == false
            && self.r#waterlogged == true
            && self.r#west == false
            && self.r#north == false
            && self.r#south == true
            && self.r#up == false
        {
            return 25086;
        }
        if self.r#down == false
            && self.r#west == true
            && self.r#waterlogged == false
            && self.r#south == false
            && self.r#up == false
            && self.r#north == false
            && self.r#east == false
        {
            return 25095;
        }
        if self.r#north == true
            && self.r#east == false
            && self.r#up == false
            && self.r#waterlogged == false
            && self.r#south == true
            && self.r#down == true
            && self.r#west == false
        {
            return 25008;
        }
        if self.r#east == true
            && self.r#south == false
            && self.r#west == true
            && self.r#down == false
            && self.r#north == false
            && self.r#up == false
            && self.r#waterlogged == true
        {
            return 25061;
        }
        if self.r#east == false
            && self.r#south == false
            && self.r#up == true
            && self.r#down == true
            && self.r#west == false
            && self.r#north == false
            && self.r#waterlogged == true
        {
            return 25026;
        }
        if self.r#down == false
            && self.r#east == false
            && self.r#south == true
            && self.r#up == false
            && self.r#west == false
            && self.r#waterlogged == false
            && self.r#north == false
        {
            return 25088;
        }
        if self.r#down == false
            && self.r#south == false
            && self.r#waterlogged == true
            && self.r#east == false
            && self.r#west == true
            && self.r#north == false
            && self.r#up == false
        {
            return 25093;
        }
        if self.r#up == false
            && self.r#down == false
            && self.r#waterlogged == false
            && self.r#west == false
            && self.r#east == true
            && self.r#south == true
            && self.r#north == true
        {
            return 25040;
        }
        if self.r#east == false
            && self.r#north == true
            && self.r#up == true
            && self.r#waterlogged == false
            && self.r#west == true
            && self.r#down == false
            && self.r#south == false
        {
            return 25075;
        }
        if self.r#north == true
            && self.r#south == true
            && self.r#up == true
            && self.r#down == false
            && self.r#east == false
            && self.r#waterlogged == true
            && self.r#west == true
        {
            return 25065;
        }
        if self.r#south == false
            && self.r#waterlogged == true
            && self.r#down == true
            && self.r#west == true
            && self.r#up == true
            && self.r#north == false
            && self.r#east == true
        {
            return 24993;
        }
        if self.r#west == false
            && self.r#down == false
            && self.r#east == false
            && self.r#north == true
            && self.r#south == false
            && self.r#up == false
            && self.r#waterlogged == false
        {
            return 25080;
        }
        if self.r#up == false
            && self.r#north == true
            && self.r#waterlogged == false
            && self.r#west == true
            && self.r#down == true
            && self.r#east == false
            && self.r#south == true
        {
            return 25007;
        }
        if self.r#east == false
            && self.r#down == true
            && self.r#waterlogged == true
            && self.r#up == false
            && self.r#north == false
            && self.r#south == false
            && self.r#west == false
        {
            return 25030;
        }
        if self.r#west == false
            && self.r#south == false
            && self.r#waterlogged == false
            && self.r#north == false
            && self.r#down == false
            && self.r#east == false
            && self.r#up == false
        {
            return 25096;
        }
        if self.r#north == false
            && self.r#south == true
            && self.r#up == true
            && self.r#down == false
            && self.r#east == false
            && self.r#waterlogged == false
            && self.r#west == false
        {
            return 25084;
        }
        if self.r#down == true
            && self.r#up == false
            && self.r#east == true
            && self.r#waterlogged == true
            && self.r#west == true
            && self.r#south == true
            && self.r#north == false
        {
            return 24989;
        }
        if self.r#east == false
            && self.r#north == true
            && self.r#south == true
            && self.r#west == false
            && self.r#waterlogged == false
            && self.r#down == false
            && self.r#up == true
        {
            return 25068;
        }
        if self.r#down == false
            && self.r#east == true
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#west == true
            && self.r#north == true
            && self.r#south == true
        {
            return 25033;
        }
        if self.r#west == false
            && self.r#down == false
            && self.r#north == false
            && self.r#east == true
            && self.r#south == true
            && self.r#up == false
            && self.r#waterlogged == true
        {
            return 25054;
        }
        if self.r#north == true
            && self.r#south == false
            && self.r#down == false
            && self.r#waterlogged == true
            && self.r#west == false
            && self.r#east == false
            && self.r#up == false
        {
            return 25078;
        }
        if self.r#west == true
            && self.r#east == false
            && self.r#up == false
            && self.r#north == false
            && self.r#south == true
            && self.r#waterlogged == true
            && self.r#down == false
        {
            return 25085;
        }
        if self.r#down == false
            && self.r#south == false
            && self.r#up == true
            && self.r#waterlogged == true
            && self.r#east == true
            && self.r#west == false
            && self.r#north == true
        {
            return 25042;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 25046 {
            return Some(SculkVein {
                r#east: true,
                r#up: false,
                r#down: false,
                r#south: false,
                r#waterlogged: true,
                r#west: false,
                r#north: true,
            });
        }
        if state_id == 25001 {
            return Some(SculkVein {
                r#east: false,
                r#up: true,
                r#waterlogged: true,
                r#west: true,
                r#north: true,
                r#down: true,
                r#south: true,
            });
        }
        if state_id == 24972 {
            return Some(SculkVein {
                r#waterlogged: false,
                r#west: false,
                r#north: true,
                r#down: true,
                r#south: true,
                r#east: true,
                r#up: true,
            });
        }
        if state_id == 24992 {
            return Some(SculkVein {
                r#east: true,
                r#down: true,
                r#north: false,
                r#up: false,
                r#south: true,
                r#waterlogged: false,
                r#west: false,
            });
        }
        if state_id == 25005 {
            return Some(SculkVein {
                r#east: false,
                r#north: true,
                r#waterlogged: true,
                r#down: true,
                r#south: true,
                r#west: true,
                r#up: false,
            });
        }
        if state_id == 25052 {
            return Some(SculkVein {
                r#south: true,
                r#north: false,
                r#waterlogged: false,
                r#east: true,
                r#west: false,
                r#up: true,
                r#down: false,
            });
        }
        if state_id == 24974 {
            return Some(SculkVein {
                r#up: false,
                r#waterlogged: true,
                r#north: true,
                r#west: false,
                r#down: true,
                r#east: true,
                r#south: true,
            });
        }
        if state_id == 25034 {
            return Some(SculkVein {
                r#east: true,
                r#south: true,
                r#up: true,
                r#down: false,
                r#west: false,
                r#north: true,
                r#waterlogged: true,
            });
        }
        if state_id == 25028 {
            return Some(SculkVein {
                r#west: false,
                r#waterlogged: false,
                r#down: true,
                r#east: false,
                r#north: false,
                r#up: true,
                r#south: false,
            });
        }
        if state_id == 25038 {
            return Some(SculkVein {
                r#north: true,
                r#south: true,
                r#west: false,
                r#up: false,
                r#down: false,
                r#waterlogged: true,
                r#east: true,
            });
        }
        if state_id == 25055 {
            return Some(SculkVein {
                r#waterlogged: false,
                r#west: true,
                r#east: true,
                r#down: false,
                r#south: true,
                r#north: false,
                r#up: false,
            });
        }
        if state_id == 25060 {
            return Some(SculkVein {
                r#up: true,
                r#north: false,
                r#waterlogged: false,
                r#west: false,
                r#south: false,
                r#down: false,
                r#east: true,
            });
        }
        if state_id == 25050 {
            return Some(SculkVein {
                r#west: false,
                r#south: true,
                r#waterlogged: true,
                r#east: true,
                r#north: false,
                r#up: true,
                r#down: false,
            });
        }
        if state_id == 25063 {
            return Some(SculkVein {
                r#up: false,
                r#waterlogged: false,
                r#down: false,
                r#north: false,
                r#west: true,
                r#east: true,
                r#south: false,
            });
        }
        if state_id == 25064 {
            return Some(SculkVein {
                r#waterlogged: false,
                r#east: true,
                r#north: false,
                r#west: false,
                r#down: false,
                r#south: false,
                r#up: false,
            });
        }
        if state_id == 24982 {
            return Some(SculkVein {
                r#south: false,
                r#west: false,
                r#east: true,
                r#north: true,
                r#down: true,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 25039 {
            return Some(SculkVein {
                r#waterlogged: false,
                r#west: true,
                r#down: false,
                r#south: true,
                r#east: true,
                r#north: true,
                r#up: false,
            });
        }
        if state_id == 24969 {
            return Some(SculkVein {
                r#waterlogged: true,
                r#west: true,
                r#down: true,
                r#north: true,
                r#up: true,
                r#east: true,
                r#south: true,
            });
        }
        if state_id == 25071 {
            return Some(SculkVein {
                r#waterlogged: false,
                r#west: true,
                r#north: true,
                r#south: true,
                r#up: false,
                r#down: false,
                r#east: false,
            });
        }
        if state_id == 24971 {
            return Some(SculkVein {
                r#north: true,
                r#down: true,
                r#south: true,
                r#east: true,
                r#up: true,
                r#waterlogged: false,
                r#west: true,
            });
        }
        if state_id == 25032 {
            return Some(SculkVein {
                r#east: false,
                r#south: false,
                r#up: false,
                r#down: true,
                r#west: false,
                r#waterlogged: false,
                r#north: false,
            });
        }
        if state_id == 24991 {
            return Some(SculkVein {
                r#up: false,
                r#waterlogged: false,
                r#west: true,
                r#east: true,
                r#north: false,
                r#south: true,
                r#down: true,
            });
        }
        if state_id == 25053 {
            return Some(SculkVein {
                r#north: false,
                r#east: true,
                r#waterlogged: true,
                r#up: false,
                r#south: true,
                r#west: true,
                r#down: false,
            });
        }
        if state_id == 25004 {
            return Some(SculkVein {
                r#up: true,
                r#waterlogged: false,
                r#west: false,
                r#down: true,
                r#east: false,
                r#south: true,
                r#north: true,
            });
        }
        if state_id == 25041 {
            return Some(SculkVein {
                r#south: false,
                r#waterlogged: true,
                r#west: true,
                r#east: true,
                r#north: true,
                r#up: true,
                r#down: false,
            });
        }
        if state_id == 25073 {
            return Some(SculkVein {
                r#west: true,
                r#down: false,
                r#south: false,
                r#north: true,
                r#up: true,
                r#east: false,
                r#waterlogged: true,
            });
        }
        if state_id == 25024 {
            return Some(SculkVein {
                r#waterlogged: false,
                r#east: false,
                r#down: true,
                r#south: true,
                r#west: false,
                r#up: false,
                r#north: false,
            });
        }
        if state_id == 25036 {
            return Some(SculkVein {
                r#up: true,
                r#waterlogged: false,
                r#west: false,
                r#north: true,
                r#east: true,
                r#down: false,
                r#south: true,
            });
        }
        if state_id == 25020 {
            return Some(SculkVein {
                r#north: false,
                r#down: true,
                r#up: true,
                r#east: false,
                r#waterlogged: false,
                r#south: true,
                r#west: false,
            });
        }
        if state_id == 24996 {
            return Some(SculkVein {
                r#west: false,
                r#east: true,
                r#south: false,
                r#down: true,
                r#up: true,
                r#north: false,
                r#waterlogged: false,
            });
        }
        if state_id == 25091 {
            return Some(SculkVein {
                r#south: false,
                r#east: false,
                r#up: true,
                r#waterlogged: false,
                r#west: true,
                r#north: false,
                r#down: false,
            });
        }
        if state_id == 25056 {
            return Some(SculkVein {
                r#down: false,
                r#waterlogged: false,
                r#east: true,
                r#north: false,
                r#south: true,
                r#west: false,
                r#up: false,
            });
        }
        if state_id == 25017 {
            return Some(SculkVein {
                r#north: false,
                r#waterlogged: true,
                r#down: true,
                r#up: true,
                r#south: true,
                r#east: false,
                r#west: true,
            });
        }
        if state_id == 24976 {
            return Some(SculkVein {
                r#down: true,
                r#west: false,
                r#east: true,
                r#south: true,
                r#waterlogged: false,
                r#up: false,
                r#north: true,
            });
        }
        if state_id == 24977 {
            return Some(SculkVein {
                r#down: true,
                r#up: true,
                r#east: true,
                r#south: false,
                r#waterlogged: true,
                r#north: true,
                r#west: true,
            });
        }
        if state_id == 25019 {
            return Some(SculkVein {
                r#east: false,
                r#north: false,
                r#west: true,
                r#up: true,
                r#waterlogged: false,
                r#south: true,
                r#down: true,
            });
        }
        if state_id == 24970 {
            return Some(SculkVein {
                r#south: true,
                r#down: true,
                r#east: true,
                r#north: true,
                r#waterlogged: true,
                r#west: false,
                r#up: true,
            });
        }
        if state_id == 25023 {
            return Some(SculkVein {
                r#east: false,
                r#north: false,
                r#south: true,
                r#up: false,
                r#down: true,
                r#waterlogged: false,
                r#west: true,
            });
        }
        if state_id == 25051 {
            return Some(SculkVein {
                r#west: true,
                r#down: false,
                r#north: false,
                r#south: true,
                r#east: true,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 25013 {
            return Some(SculkVein {
                r#up: false,
                r#north: true,
                r#waterlogged: true,
                r#south: false,
                r#west: true,
                r#down: true,
                r#east: false,
            });
        }
        if state_id == 24990 {
            return Some(SculkVein {
                r#north: false,
                r#south: true,
                r#waterlogged: true,
                r#down: true,
                r#west: false,
                r#up: false,
                r#east: true,
            });
        }
        if state_id == 25076 {
            return Some(SculkVein {
                r#down: false,
                r#waterlogged: false,
                r#east: false,
                r#up: true,
                r#west: false,
                r#south: false,
                r#north: true,
            });
        }
        if state_id == 24999 {
            return Some(SculkVein {
                r#east: true,
                r#west: true,
                r#down: true,
                r#south: false,
                r#up: false,
                r#waterlogged: false,
                r#north: false,
            });
        }
        if state_id == 25077 {
            return Some(SculkVein {
                r#up: false,
                r#waterlogged: true,
                r#west: true,
                r#down: false,
                r#north: true,
                r#east: false,
                r#south: false,
            });
        }
        if state_id == 25002 {
            return Some(SculkVein {
                r#north: true,
                r#waterlogged: true,
                r#south: true,
                r#up: true,
                r#east: false,
                r#west: false,
                r#down: true,
            });
        }
        if state_id == 25043 {
            return Some(SculkVein {
                r#south: false,
                r#east: true,
                r#north: true,
                r#up: true,
                r#waterlogged: false,
                r#west: true,
                r#down: false,
            });
        }
        if state_id == 25089 {
            return Some(SculkVein {
                r#north: false,
                r#south: false,
                r#east: false,
                r#up: true,
                r#waterlogged: true,
                r#west: true,
                r#down: false,
            });
        }
        if state_id == 25079 {
            return Some(SculkVein {
                r#west: true,
                r#waterlogged: false,
                r#north: true,
                r#south: false,
                r#east: false,
                r#down: false,
                r#up: false,
            });
        }
        if state_id == 25058 {
            return Some(SculkVein {
                r#down: false,
                r#east: true,
                r#north: false,
                r#south: false,
                r#up: true,
                r#waterlogged: true,
                r#west: false,
            });
        }
        if state_id == 25090 {
            return Some(SculkVein {
                r#north: false,
                r#south: false,
                r#west: false,
                r#down: false,
                r#up: true,
                r#waterlogged: true,
                r#east: false,
            });
        }
        if state_id == 25006 {
            return Some(SculkVein {
                r#north: true,
                r#south: true,
                r#east: false,
                r#waterlogged: true,
                r#west: false,
                r#down: true,
                r#up: false,
            });
        }
        if state_id == 24975 {
            return Some(SculkVein {
                r#west: true,
                r#down: true,
                r#east: true,
                r#north: true,
                r#south: true,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 25000 {
            return Some(SculkVein {
                r#south: false,
                r#west: false,
                r#east: true,
                r#waterlogged: false,
                r#up: false,
                r#down: true,
                r#north: false,
            });
        }
        if state_id == 24987 {
            return Some(SculkVein {
                r#north: false,
                r#waterlogged: false,
                r#down: true,
                r#west: true,
                r#up: true,
                r#south: true,
                r#east: true,
            });
        }
        if state_id == 24998 {
            return Some(SculkVein {
                r#down: true,
                r#south: false,
                r#north: false,
                r#waterlogged: true,
                r#up: false,
                r#east: true,
                r#west: false,
            });
        }
        if state_id == 25021 {
            return Some(SculkVein {
                r#up: false,
                r#east: false,
                r#south: true,
                r#west: true,
                r#down: true,
                r#waterlogged: true,
                r#north: false,
            });
        }
        if state_id == 25045 {
            return Some(SculkVein {
                r#north: true,
                r#south: false,
                r#up: false,
                r#down: false,
                r#waterlogged: true,
                r#west: true,
                r#east: true,
            });
        }
        if state_id == 25044 {
            return Some(SculkVein {
                r#east: true,
                r#north: true,
                r#south: false,
                r#west: false,
                r#waterlogged: false,
                r#down: false,
                r#up: true,
            });
        }
        if state_id == 25066 {
            return Some(SculkVein {
                r#north: true,
                r#up: true,
                r#west: false,
                r#south: true,
                r#down: false,
                r#east: false,
                r#waterlogged: true,
            });
        }
        if state_id == 25074 {
            return Some(SculkVein {
                r#up: true,
                r#east: false,
                r#down: false,
                r#north: true,
                r#waterlogged: true,
                r#south: false,
                r#west: false,
            });
        }
        if state_id == 25027 {
            return Some(SculkVein {
                r#east: false,
                r#south: false,
                r#up: true,
                r#north: false,
                r#down: true,
                r#waterlogged: false,
                r#west: true,
            });
        }
        if state_id == 25037 {
            return Some(SculkVein {
                r#waterlogged: true,
                r#west: true,
                r#down: false,
                r#up: false,
                r#east: true,
                r#north: true,
                r#south: true,
            });
        }
        if state_id == 25094 {
            return Some(SculkVein {
                r#up: false,
                r#west: false,
                r#waterlogged: true,
                r#south: false,
                r#down: false,
                r#east: false,
                r#north: false,
            });
        }
        if state_id == 24983 {
            return Some(SculkVein {
                r#waterlogged: false,
                r#west: true,
                r#south: false,
                r#down: true,
                r#north: true,
                r#up: false,
                r#east: true,
            });
        }
        if state_id == 25035 {
            return Some(SculkVein {
                r#east: true,
                r#south: true,
                r#north: true,
                r#down: false,
                r#west: true,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 24980 {
            return Some(SculkVein {
                r#up: true,
                r#west: false,
                r#north: true,
                r#waterlogged: false,
                r#east: true,
                r#down: true,
                r#south: false,
            });
        }
        if state_id == 25015 {
            return Some(SculkVein {
                r#south: false,
                r#waterlogged: false,
                r#west: true,
                r#down: true,
                r#east: false,
                r#north: true,
                r#up: false,
            });
        }
        if state_id == 25047 {
            return Some(SculkVein {
                r#down: false,
                r#up: false,
                r#south: false,
                r#waterlogged: false,
                r#east: true,
                r#west: true,
                r#north: true,
            });
        }
        if state_id == 24988 {
            return Some(SculkVein {
                r#up: true,
                r#west: false,
                r#east: true,
                r#waterlogged: false,
                r#south: true,
                r#north: false,
                r#down: true,
            });
        }
        if state_id == 25003 {
            return Some(SculkVein {
                r#east: false,
                r#south: true,
                r#north: true,
                r#up: true,
                r#waterlogged: false,
                r#west: true,
                r#down: true,
            });
        }
        if state_id == 25014 {
            return Some(SculkVein {
                r#north: true,
                r#up: false,
                r#waterlogged: true,
                r#west: false,
                r#south: false,
                r#down: true,
                r#east: false,
            });
        }
        if state_id == 25022 {
            return Some(SculkVein {
                r#up: false,
                r#east: false,
                r#waterlogged: true,
                r#west: false,
                r#down: true,
                r#north: false,
                r#south: true,
            });
        }
        if state_id == 24979 {
            return Some(SculkVein {
                r#west: true,
                r#waterlogged: false,
                r#east: true,
                r#down: true,
                r#south: false,
                r#up: true,
                r#north: true,
            });
        }
        if state_id == 24973 {
            return Some(SculkVein {
                r#west: true,
                r#north: true,
                r#up: false,
                r#east: true,
                r#waterlogged: true,
                r#south: true,
                r#down: true,
            });
        }
        if state_id == 25072 {
            return Some(SculkVein {
                r#east: false,
                r#down: false,
                r#north: true,
                r#south: true,
                r#up: false,
                r#waterlogged: false,
                r#west: false,
            });
        }
        if state_id == 25069 {
            return Some(SculkVein {
                r#south: true,
                r#east: false,
                r#north: true,
                r#down: false,
                r#up: false,
                r#waterlogged: true,
                r#west: true,
            });
        }
        if state_id == 25025 {
            return Some(SculkVein {
                r#down: true,
                r#east: false,
                r#north: false,
                r#south: false,
                r#up: true,
                r#waterlogged: true,
                r#west: true,
            });
        }
        if state_id == 25016 {
            return Some(SculkVein {
                r#west: false,
                r#north: true,
                r#east: false,
                r#down: true,
                r#up: false,
                r#south: false,
                r#waterlogged: false,
            });
        }
        if state_id == 25048 {
            return Some(SculkVein {
                r#down: false,
                r#west: false,
                r#east: true,
                r#north: true,
                r#up: false,
                r#waterlogged: false,
                r#south: false,
            });
        }
        if state_id == 25049 {
            return Some(SculkVein {
                r#south: true,
                r#west: true,
                r#north: false,
                r#down: false,
                r#east: true,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 25067 {
            return Some(SculkVein {
                r#down: false,
                r#east: false,
                r#south: true,
                r#up: true,
                r#west: true,
                r#north: true,
                r#waterlogged: false,
            });
        }
        if state_id == 24986 {
            return Some(SculkVein {
                r#west: false,
                r#down: true,
                r#east: true,
                r#north: false,
                r#up: true,
                r#waterlogged: true,
                r#south: true,
            });
        }
        if state_id == 25009 {
            return Some(SculkVein {
                r#up: true,
                r#waterlogged: true,
                r#down: true,
                r#south: false,
                r#north: true,
                r#west: true,
                r#east: false,
            });
        }
        if state_id == 25018 {
            return Some(SculkVein {
                r#west: false,
                r#east: false,
                r#north: false,
                r#down: true,
                r#south: true,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 25057 {
            return Some(SculkVein {
                r#east: true,
                r#north: false,
                r#up: true,
                r#waterlogged: true,
                r#west: true,
                r#south: false,
                r#down: false,
            });
        }
        if state_id == 25062 {
            return Some(SculkVein {
                r#east: true,
                r#north: false,
                r#west: false,
                r#down: false,
                r#south: false,
                r#waterlogged: true,
                r#up: false,
            });
        }
        if state_id == 25012 {
            return Some(SculkVein {
                r#west: false,
                r#waterlogged: false,
                r#down: true,
                r#south: false,
                r#up: true,
                r#north: true,
                r#east: false,
            });
        }
        if state_id == 25083 {
            return Some(SculkVein {
                r#south: true,
                r#down: false,
                r#waterlogged: false,
                r#north: false,
                r#west: true,
                r#up: true,
                r#east: false,
            });
        }
        if state_id == 25087 {
            return Some(SculkVein {
                r#down: false,
                r#up: false,
                r#east: false,
                r#south: true,
                r#waterlogged: false,
                r#north: false,
                r#west: true,
            });
        }
        if state_id == 25010 {
            return Some(SculkVein {
                r#east: false,
                r#up: true,
                r#south: false,
                r#north: true,
                r#down: true,
                r#waterlogged: true,
                r#west: false,
            });
        }
        if state_id == 24985 {
            return Some(SculkVein {
                r#down: true,
                r#waterlogged: true,
                r#west: true,
                r#south: true,
                r#north: false,
                r#up: true,
                r#east: true,
            });
        }
        if state_id == 24994 {
            return Some(SculkVein {
                r#north: false,
                r#up: true,
                r#west: false,
                r#waterlogged: true,
                r#down: true,
                r#south: false,
                r#east: true,
            });
        }
        if state_id == 25081 {
            return Some(SculkVein {
                r#east: false,
                r#down: false,
                r#west: true,
                r#south: true,
                r#up: true,
                r#north: false,
                r#waterlogged: true,
            });
        }
        if state_id == 25092 {
            return Some(SculkVein {
                r#waterlogged: false,
                r#east: false,
                r#up: true,
                r#west: false,
                r#down: false,
                r#south: false,
                r#north: false,
            });
        }
        if state_id == 25070 {
            return Some(SculkVein {
                r#south: true,
                r#waterlogged: true,
                r#east: false,
                r#north: true,
                r#down: false,
                r#up: false,
                r#west: false,
            });
        }
        if state_id == 24997 {
            return Some(SculkVein {
                r#west: true,
                r#up: false,
                r#north: false,
                r#east: true,
                r#south: false,
                r#waterlogged: true,
                r#down: true,
            });
        }
        if state_id == 25029 {
            return Some(SculkVein {
                r#west: true,
                r#waterlogged: true,
                r#down: true,
                r#north: false,
                r#east: false,
                r#south: false,
                r#up: false,
            });
        }
        if state_id == 25031 {
            return Some(SculkVein {
                r#east: false,
                r#west: true,
                r#north: false,
                r#down: true,
                r#waterlogged: false,
                r#south: false,
                r#up: false,
            });
        }
        if state_id == 25059 {
            return Some(SculkVein {
                r#east: true,
                r#south: false,
                r#up: true,
                r#waterlogged: false,
                r#west: true,
                r#north: false,
                r#down: false,
            });
        }
        if state_id == 25082 {
            return Some(SculkVein {
                r#up: true,
                r#south: true,
                r#down: false,
                r#east: false,
                r#west: false,
                r#waterlogged: true,
                r#north: false,
            });
        }
        if state_id == 24981 {
            return Some(SculkVein {
                r#west: true,
                r#down: true,
                r#waterlogged: true,
                r#north: true,
                r#south: false,
                r#up: false,
                r#east: true,
            });
        }
        if state_id == 24984 {
            return Some(SculkVein {
                r#up: false,
                r#south: false,
                r#waterlogged: false,
                r#down: true,
                r#west: false,
                r#east: true,
                r#north: true,
            });
        }
        if state_id == 24978 {
            return Some(SculkVein {
                r#down: true,
                r#north: true,
                r#up: true,
                r#west: false,
                r#east: true,
                r#south: false,
                r#waterlogged: true,
            });
        }
        if state_id == 24995 {
            return Some(SculkVein {
                r#east: true,
                r#waterlogged: false,
                r#up: true,
                r#north: false,
                r#south: false,
                r#west: true,
                r#down: true,
            });
        }
        if state_id == 25011 {
            return Some(SculkVein {
                r#waterlogged: false,
                r#down: true,
                r#west: true,
                r#east: false,
                r#north: true,
                r#south: false,
                r#up: true,
            });
        }
        if state_id == 25086 {
            return Some(SculkVein {
                r#down: false,
                r#east: false,
                r#waterlogged: true,
                r#west: false,
                r#north: false,
                r#south: true,
                r#up: false,
            });
        }
        if state_id == 25095 {
            return Some(SculkVein {
                r#down: false,
                r#west: true,
                r#waterlogged: false,
                r#south: false,
                r#up: false,
                r#north: false,
                r#east: false,
            });
        }
        if state_id == 25008 {
            return Some(SculkVein {
                r#north: true,
                r#east: false,
                r#up: false,
                r#waterlogged: false,
                r#south: true,
                r#down: true,
                r#west: false,
            });
        }
        if state_id == 25061 {
            return Some(SculkVein {
                r#east: true,
                r#south: false,
                r#west: true,
                r#down: false,
                r#north: false,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 25026 {
            return Some(SculkVein {
                r#east: false,
                r#south: false,
                r#up: true,
                r#down: true,
                r#west: false,
                r#north: false,
                r#waterlogged: true,
            });
        }
        if state_id == 25088 {
            return Some(SculkVein {
                r#down: false,
                r#east: false,
                r#south: true,
                r#up: false,
                r#west: false,
                r#waterlogged: false,
                r#north: false,
            });
        }
        if state_id == 25093 {
            return Some(SculkVein {
                r#down: false,
                r#south: false,
                r#waterlogged: true,
                r#east: false,
                r#west: true,
                r#north: false,
                r#up: false,
            });
        }
        if state_id == 25040 {
            return Some(SculkVein {
                r#up: false,
                r#down: false,
                r#waterlogged: false,
                r#west: false,
                r#east: true,
                r#south: true,
                r#north: true,
            });
        }
        if state_id == 25075 {
            return Some(SculkVein {
                r#east: false,
                r#north: true,
                r#up: true,
                r#waterlogged: false,
                r#west: true,
                r#down: false,
                r#south: false,
            });
        }
        if state_id == 25065 {
            return Some(SculkVein {
                r#north: true,
                r#south: true,
                r#up: true,
                r#down: false,
                r#east: false,
                r#waterlogged: true,
                r#west: true,
            });
        }
        if state_id == 24993 {
            return Some(SculkVein {
                r#south: false,
                r#waterlogged: true,
                r#down: true,
                r#west: true,
                r#up: true,
                r#north: false,
                r#east: true,
            });
        }
        if state_id == 25080 {
            return Some(SculkVein {
                r#west: false,
                r#down: false,
                r#east: false,
                r#north: true,
                r#south: false,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 25007 {
            return Some(SculkVein {
                r#up: false,
                r#north: true,
                r#waterlogged: false,
                r#west: true,
                r#down: true,
                r#east: false,
                r#south: true,
            });
        }
        if state_id == 25030 {
            return Some(SculkVein {
                r#east: false,
                r#down: true,
                r#waterlogged: true,
                r#up: false,
                r#north: false,
                r#south: false,
                r#west: false,
            });
        }
        if state_id == 25096 {
            return Some(SculkVein {
                r#west: false,
                r#south: false,
                r#waterlogged: false,
                r#north: false,
                r#down: false,
                r#east: false,
                r#up: false,
            });
        }
        if state_id == 25084 {
            return Some(SculkVein {
                r#north: false,
                r#south: true,
                r#up: true,
                r#down: false,
                r#east: false,
                r#waterlogged: false,
                r#west: false,
            });
        }
        if state_id == 24989 {
            return Some(SculkVein {
                r#down: true,
                r#up: false,
                r#east: true,
                r#waterlogged: true,
                r#west: true,
                r#south: true,
                r#north: false,
            });
        }
        if state_id == 25068 {
            return Some(SculkVein {
                r#east: false,
                r#north: true,
                r#south: true,
                r#west: false,
                r#waterlogged: false,
                r#down: false,
                r#up: true,
            });
        }
        if state_id == 25033 {
            return Some(SculkVein {
                r#down: false,
                r#east: true,
                r#up: true,
                r#waterlogged: true,
                r#west: true,
                r#north: true,
                r#south: true,
            });
        }
        if state_id == 25054 {
            return Some(SculkVein {
                r#west: false,
                r#down: false,
                r#north: false,
                r#east: true,
                r#south: true,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 25078 {
            return Some(SculkVein {
                r#north: true,
                r#south: false,
                r#down: false,
                r#waterlogged: true,
                r#west: false,
                r#east: false,
                r#up: false,
            });
        }
        if state_id == 25085 {
            return Some(SculkVein {
                r#west: true,
                r#east: false,
                r#up: false,
                r#north: false,
                r#south: true,
                r#waterlogged: true,
                r#down: false,
            });
        }
        if state_id == 25042 {
            return Some(SculkVein {
                r#down: false,
                r#south: false,
                r#up: true,
                r#waterlogged: true,
                r#east: true,
                r#west: false,
                r#north: true,
            });
        }
        return None;
    }
}
