use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GlowLichen {
    pub waterlogged: bool,
    pub north: bool,
    pub east: bool,
    pub down: bool,
    pub up: bool,
    pub west: bool,
    pub south: bool,
}


impl BlockState for GlowLichen {
    fn to_id(&self) -> i32 {
        if self.r#east == false && self.r#up == true && self.r#south == false && self.r#waterlogged == true && self.r#north == true && self.r#down == false && self.r#west == true { return 8293; }
        if self.r#north == false && self.r#south == true && self.r#waterlogged == false && self.r#down == false && self.r#east == false && self.r#up == true && self.r#west == true { return 8303; }
        if self.r#north == false && self.r#east == true && self.r#south == true && self.r#up == true && self.r#waterlogged == false && self.r#west == false && self.r#down == true { return 8208; }
        if self.r#down == true && self.r#north == true && self.r#up == false && self.r#south == true && self.r#waterlogged == false && self.r#west == false && self.r#east == true { return 8196; }
        if self.r#down == false && self.r#west == true && self.r#north == true && self.r#south == false && self.r#up == true && self.r#east == true && self.r#waterlogged == false { return 8263; }
        if self.r#down == true && self.r#south == false && self.r#waterlogged == false && self.r#west == true && self.r#north == false && self.r#east == true && self.r#up == false { return 8219; }
        if self.r#down == false && self.r#up == false && self.r#waterlogged == false && self.r#south == true && self.r#west == false && self.r#east == false && self.r#north == false { return 8308; }
        if self.r#up == false && self.r#waterlogged == true && self.r#west == true && self.r#down == false && self.r#north == true && self.r#south == false && self.r#east == true { return 8265; }
        if self.r#up == true && self.r#down == true && self.r#east == false && self.r#north == true && self.r#waterlogged == false && self.r#south == true && self.r#west == true { return 8223; }
        if self.r#east == false && self.r#down == false && self.r#up == false && self.r#waterlogged == true && self.r#west == false && self.r#south == true && self.r#north == true { return 8290; }
        if self.r#down == true && self.r#south == false && self.r#up == false && self.r#north == false && self.r#waterlogged == true && self.r#west == true && self.r#east == true { return 8217; }
        if self.r#up == false && self.r#east == false && self.r#north == true && self.r#down == false && self.r#waterlogged == false && self.r#west == true && self.r#south == false { return 8299; }
        if self.r#west == true && self.r#north == false && self.r#south == false && self.r#up == true && self.r#down == true && self.r#waterlogged == true && self.r#east == true { return 8213; }
        if self.r#up == true && self.r#north == true && self.r#down == true && self.r#east == false && self.r#waterlogged == true && self.r#west == true && self.r#south == true { return 8221; }
        if self.r#up == false && self.r#west == false && self.r#north == true && self.r#down == true && self.r#south == false && self.r#east == false && self.r#waterlogged == true { return 8234; }
        if self.r#down == true && self.r#north == false && self.r#west == true && self.r#up == true && self.r#east == true && self.r#south == true && self.r#waterlogged == true { return 8205; }
        if self.r#up == true && self.r#west == false && self.r#north == false && self.r#east == false && self.r#south == true && self.r#down == true && self.r#waterlogged == false { return 8240; }
        if self.r#south == false && self.r#waterlogged == true && self.r#north == false && self.r#down == true && self.r#up == false && self.r#west == false && self.r#east == false { return 8250; }
        if self.r#up == true && self.r#east == true && self.r#waterlogged == false && self.r#north == false && self.r#west == false && self.r#down == false && self.r#south == false { return 8280; }
        if self.r#south == false && self.r#down == false && self.r#east == true && self.r#up == false && self.r#north == false && self.r#waterlogged == true && self.r#west == false { return 8282; }
        if self.r#east == false && self.r#waterlogged == true && self.r#south == false && self.r#north == false && self.r#up == false && self.r#west == true && self.r#down == true { return 8249; }
        if self.r#south == false && self.r#north == true && self.r#up == false && self.r#down == false && self.r#east == false && self.r#waterlogged == false && self.r#west == false { return 8300; }
        if self.r#south == false && self.r#waterlogged == true && self.r#north == false && self.r#west == true && self.r#down == false && self.r#east == false && self.r#up == false { return 8313; }
        if self.r#east == false && self.r#up == true && self.r#waterlogged == true && self.r#west == true && self.r#down == true && self.r#north == false && self.r#south == true { return 8237; }
        if self.r#south == false && self.r#waterlogged == true && self.r#down == false && self.r#east == false && self.r#north == false && self.r#west == false && self.r#up == true { return 8310; }
        if self.r#south == false && self.r#waterlogged == false && self.r#north == false && self.r#west == false && self.r#up == true && self.r#east == true && self.r#down == true { return 8216; }
        if self.r#south == true && self.r#north == false && self.r#down == false && self.r#up == true && self.r#east == true && self.r#waterlogged == true && self.r#west == false { return 8270; }
        if self.r#down == false && self.r#up == true && self.r#west == true && self.r#north == false && self.r#east == true && self.r#south == false && self.r#waterlogged == false { return 8279; }
        if self.r#north == false && self.r#down == true && self.r#east == false && self.r#up == false && self.r#south == true && self.r#waterlogged == true && self.r#west == false { return 8242; }
        if self.r#down == true && self.r#east == true && self.r#north == false && self.r#south == true && self.r#up == false && self.r#waterlogged == true && self.r#west == true { return 8209; }
        if self.r#waterlogged == true && self.r#south == true && self.r#west == true && self.r#east == true && self.r#north == true && self.r#up == true && self.r#down == false { return 8253; }
        if self.r#west == true && self.r#down == false && self.r#north == true && self.r#east == true && self.r#up == true && self.r#south == false && self.r#waterlogged == true { return 8261; }
        if self.r#west == true && self.r#waterlogged == true && self.r#up == false && self.r#down == false && self.r#east == false && self.r#north == false && self.r#south == true { return 8305; }
        if self.r#down == false && self.r#up == false && self.r#waterlogged == false && self.r#north == true && self.r#west == false && self.r#south == true && self.r#east == true { return 8260; }
        if self.r#east == false && self.r#waterlogged == true && self.r#north == false && self.r#south == true && self.r#down == false && self.r#west == true && self.r#up == true { return 8301; }
        if self.r#east == false && self.r#down == true && self.r#south == true && self.r#up == false && self.r#north == false && self.r#west == true && self.r#waterlogged == false { return 8243; }
        if self.r#east == false && self.r#west == false && self.r#south == true && self.r#north == false && self.r#waterlogged == false && self.r#up == false && self.r#down == true { return 8244; }
        if self.r#west == true && self.r#south == true && self.r#north == true && self.r#down == false && self.r#up == false && self.r#east == true && self.r#waterlogged == false { return 8259; }
        if self.r#down == false && self.r#north == false && self.r#south == false && self.r#waterlogged == true && self.r#west == true && self.r#up == true && self.r#east == true { return 8277; }
        if self.r#east == true && self.r#south == false && self.r#north == true && self.r#up == true && self.r#down == true && self.r#waterlogged == true && self.r#west == false { return 8198; }
        if self.r#south == true && self.r#down == false && self.r#up == true && self.r#east == false && self.r#west == false && self.r#north == false && self.r#waterlogged == false { return 8304; }
        if self.r#east == false && self.r#north == true && self.r#south == false && self.r#up == true && self.r#down == true && self.r#waterlogged == false && self.r#west == true { return 8231; }
        if self.r#down == true && self.r#east == true && self.r#waterlogged == true && self.r#up == true && self.r#west == true && self.r#north == true && self.r#south == true { return 8189; }
        if self.r#waterlogged == false && self.r#west == true && self.r#down == false && self.r#south == false && self.r#up == false && self.r#east == false && self.r#north == false { return 8315; }
        if self.r#up == false && self.r#waterlogged == false && self.r#down == true && self.r#east == false && self.r#south == false && self.r#north == false && self.r#west == false { return 8252; }
        if self.r#south == false && self.r#west == true && self.r#down == false && self.r#waterlogged == true && self.r#north == false && self.r#east == false && self.r#up == true { return 8309; }
        if self.r#south == true && self.r#east == true && self.r#up == true && self.r#waterlogged == true && self.r#west == false && self.r#down == false && self.r#north == true { return 8254; }
        if self.r#waterlogged == true && self.r#north == false && self.r#east == true && self.r#south == true && self.r#west == false && self.r#down == false && self.r#up == false { return 8274; }
        if self.r#waterlogged == false && self.r#down == true && self.r#up == false && self.r#west == true && self.r#north == false && self.r#east == false && self.r#south == false { return 8251; }
        if self.r#south == false && self.r#down == false && self.r#up == false && self.r#waterlogged == true && self.r#east == false && self.r#north == false && self.r#west == false { return 8314; }
        if self.r#west == true && self.r#down == false && self.r#waterlogged == true && self.r#north == false && self.r#south == false && self.r#east == true && self.r#up == false { return 8281; }
        if self.r#west == false && self.r#waterlogged == true && self.r#up == true && self.r#south == false && self.r#north == false && self.r#down == true && self.r#east == false { return 8246; }
        if self.r#waterlogged == true && self.r#north == true && self.r#south == false && self.r#up == false && self.r#east == false && self.r#west == true && self.r#down == true { return 8233; }
        if self.r#down == false && self.r#up == true && self.r#east == true && self.r#north == false && self.r#south == true && self.r#waterlogged == true && self.r#west == true { return 8269; }
        if self.r#west == true && self.r#south == true && self.r#east == false && self.r#down == true && self.r#up == true && self.r#waterlogged == false && self.r#north == false { return 8239; }
        if self.r#north == true && self.r#up == true && self.r#east == true && self.r#down == true && self.r#waterlogged == true && self.r#west == true && self.r#south == false { return 8197; }
        if self.r#down == true && self.r#east == false && self.r#waterlogged == false && self.r#north == true && self.r#up == false && self.r#south == true && self.r#west == false { return 8228; }
        if self.r#east == true && self.r#waterlogged == true && self.r#west == false && self.r#down == false && self.r#north == true && self.r#south == false && self.r#up == true { return 8262; }
        if self.r#down == true && self.r#west == true && self.r#south == false && self.r#north == false && self.r#waterlogged == false && self.r#east == false && self.r#up == true { return 8247; }
        if self.r#up == false && self.r#west == true && self.r#down == false && self.r#east == true && self.r#waterlogged == true && self.r#north == true && self.r#south == true { return 8257; }
        if self.r#down == true && self.r#east == false && self.r#south == true && self.r#west == false && self.r#waterlogged == true && self.r#north == false && self.r#up == true { return 8238; }
        if self.r#waterlogged == false && self.r#west == true && self.r#east == false && self.r#down == false && self.r#north == true && self.r#south == true && self.r#up == true { return 8287; }
        if self.r#east == false && self.r#up == false && self.r#south == true && self.r#waterlogged == true && self.r#down == true && self.r#west == true && self.r#north == false { return 8241; }
        if self.r#waterlogged == false && self.r#down == false && self.r#north == false && self.r#south == true && self.r#up == true && self.r#west == true && self.r#east == true { return 8271; }
        if self.r#east == true && self.r#up == false && self.r#down == true && self.r#south == true && self.r#waterlogged == false && self.r#north == true && self.r#west == true { return 8195; }
        if self.r#down == true && self.r#east == true && self.r#north == false && self.r#south == false && self.r#up == false && self.r#waterlogged == true && self.r#west == false { return 8218; }
        if self.r#north == false && self.r#east == true && self.r#south == false && self.r#waterlogged == false && self.r#west == true && self.r#down == false && self.r#up == false { return 8283; }
        if self.r#south == true && self.r#up == true && self.r#north == false && self.r#east == true && self.r#waterlogged == false && self.r#west == false && self.r#down == false { return 8272; }
        if self.r#down == true && self.r#east == true && self.r#up == true && self.r#waterlogged == true && self.r#south == false && self.r#north == false && self.r#west == false { return 8214; }
        if self.r#east == true && self.r#north == false && self.r#down == true && self.r#up == false && self.r#waterlogged == true && self.r#west == false && self.r#south == true { return 8210; }
        if self.r#north == true && self.r#down == false && self.r#south == false && self.r#up == true && self.r#waterlogged == false && self.r#west == true && self.r#east == false { return 8295; }
        if self.r#up == true && self.r#waterlogged == false && self.r#down == true && self.r#south == true && self.r#west == false && self.r#east == false && self.r#north == true { return 8224; }
        if self.r#west == true && self.r#down == false && self.r#up == false && self.r#east == false && self.r#waterlogged == true && self.r#north == true && self.r#south == false { return 8297; }
        if self.r#up == true && self.r#waterlogged == true && self.r#west == false && self.r#south == false && self.r#north == true && self.r#down == true && self.r#east == false { return 8230; }
        if self.r#east == true && self.r#down == false && self.r#up == true && self.r#waterlogged == true && self.r#north == false && self.r#west == false && self.r#south == false { return 8278; }
        if self.r#south == false && self.r#up == true && self.r#west == true && self.r#down == true && self.r#east == false && self.r#north == false && self.r#waterlogged == true { return 8245; }
        if self.r#down == false && self.r#south == true && self.r#up == true && self.r#west == true && self.r#north == true && self.r#waterlogged == true && self.r#east == false { return 8285; }
        if self.r#south == true && self.r#west == false && self.r#up == false && self.r#waterlogged == false && self.r#north == false && self.r#east == true && self.r#down == true { return 8212; }
        if self.r#down == false && self.r#north == true && self.r#up == true && self.r#east == false && self.r#waterlogged == false && self.r#west == false && self.r#south == true { return 8288; }
        if self.r#down == true && self.r#south == false && self.r#north == true && self.r#west == false && self.r#east == true && self.r#up == true && self.r#waterlogged == false { return 8200; }
        if self.r#waterlogged == true && self.r#south == false && self.r#east == false && self.r#down == true && self.r#north == true && self.r#up == true && self.r#west == true { return 8229; }
        if self.r#west == true && self.r#down == false && self.r#south == true && self.r#up == false && self.r#east == true && self.r#north == false && self.r#waterlogged == true { return 8273; }
        if self.r#east == true && self.r#north == false && self.r#south == true && self.r#waterlogged == false && self.r#up == false && self.r#down == false && self.r#west == true { return 8275; }
        if self.r#down == true && self.r#north == true && self.r#up == false && self.r#waterlogged == true && self.r#west == false && self.r#south == false && self.r#east == true { return 8202; }
        if self.r#west == true && self.r#east == false && self.r#waterlogged == false && self.r#down == true && self.r#up == false && self.r#north == true && self.r#south == true { return 8227; }
        if self.r#west == false && self.r#down == true && self.r#south == false && self.r#east == false && self.r#north == true && self.r#up == true && self.r#waterlogged == false { return 8232; }
        if self.r#south == false && self.r#north == true && self.r#up == false && self.r#waterlogged == true && self.r#west == true && self.r#down == true && self.r#east == true { return 8201; }
        if self.r#waterlogged == false && self.r#down == true && self.r#west == true && self.r#north == false && self.r#south == true && self.r#up == true && self.r#east == true { return 8207; }
        if self.r#down == false && self.r#waterlogged == false && self.r#west == false && self.r#up == false && self.r#east == false && self.r#south == true && self.r#north == true { return 8292; }
        if self.r#down == true && self.r#east == true && self.r#south == true && self.r#waterlogged == false && self.r#west == false && self.r#north == true && self.r#up == true { return 8192; }
        if self.r#waterlogged == false && self.r#west == false && self.r#up == false && self.r#down == false && self.r#north == false && self.r#south == false && self.r#east == false { return 8316; }
        if self.r#down == false && self.r#north == true && self.r#up == true && self.r#west == false && self.r#south == false && self.r#waterlogged == false && self.r#east == false { return 8296; }
        if self.r#waterlogged == false && self.r#east == true && self.r#south == true && self.r#up == true && self.r#down == false && self.r#north == true && self.r#west == true { return 8255; }
        if self.r#waterlogged == false && self.r#north == false && self.r#up == false && self.r#down == false && self.r#east == false && self.r#south == true && self.r#west == true { return 8307; }
        if self.r#up == true && self.r#west == true && self.r#south == false && self.r#east == true && self.r#north == true && self.r#waterlogged == false && self.r#down == true { return 8199; }
        if self.r#east == true && self.r#south == true && self.r#west == false && self.r#down == true && self.r#north == true && self.r#up == false && self.r#waterlogged == true { return 8194; }
        if self.r#waterlogged == false && self.r#south == true && self.r#north == false && self.r#down == true && self.r#west == true && self.r#east == true && self.r#up == false { return 8211; }
        if self.r#west == false && self.r#north == true && self.r#down == true && self.r#up == false && self.r#waterlogged == true && self.r#east == false && self.r#south == true { return 8226; }
        if self.r#down == true && self.r#up == false && self.r#west == true && self.r#north == true && self.r#south == false && self.r#east == true && self.r#waterlogged == false { return 8203; }
        if self.r#east == true && self.r#waterlogged == false && self.r#north == true && self.r#south == false && self.r#west == false && self.r#up == false && self.r#down == true { return 8204; }
        if self.r#south == true && self.r#down == true && self.r#waterlogged == true && self.r#north == false && self.r#east == true && self.r#up == true && self.r#west == false { return 8206; }
        if self.r#down == false && self.r#up == false && self.r#south == false && self.r#east == true && self.r#waterlogged == false && self.r#north == false && self.r#west == false { return 8284; }
        if self.r#north == false && self.r#waterlogged == false && self.r#east == true && self.r#west == true && self.r#up == true && self.r#down == true && self.r#south == false { return 8215; }
        if self.r#west == true && self.r#waterlogged == true && self.r#south == true && self.r#east == false && self.r#down == false && self.r#north == true && self.r#up == false { return 8289; }
        if self.r#south == true && self.r#north == true && self.r#up == true && self.r#east == true && self.r#waterlogged == false && self.r#down == false && self.r#west == false { return 8256; }
        if self.r#north == true && self.r#south == true && self.r#down == true && self.r#up == true && self.r#waterlogged == true && self.r#east == false && self.r#west == false { return 8222; }
        if self.r#south == false && self.r#west == false && self.r#north == true && self.r#down == false && self.r#east == true && self.r#up == false && self.r#waterlogged == false { return 8268; }
        if self.r#down == false && self.r#east == false && self.r#waterlogged == true && self.r#west == false && self.r#up == true && self.r#north == false && self.r#south == true { return 8302; }
        if self.r#west == true && self.r#down == false && self.r#east == false && self.r#north == false && self.r#south == false && self.r#up == true && self.r#waterlogged == false { return 8311; }
        if self.r#east == false && self.r#south == false && self.r#down == false && self.r#north == false && self.r#up == true && self.r#waterlogged == false && self.r#west == false { return 8312; }
        if self.r#south == true && self.r#up == true && self.r#waterlogged == false && self.r#west == true && self.r#east == true && self.r#down == true && self.r#north == true { return 8191; }
        if self.r#east == false && self.r#up == false && self.r#down == true && self.r#south == false && self.r#west == true && self.r#waterlogged == false && self.r#north == true { return 8235; }
        if self.r#down == false && self.r#east == false && self.r#up == false && self.r#west == false && self.r#north == true && self.r#south == false && self.r#waterlogged == true { return 8298; }
        if self.r#up == true && self.r#south == true && self.r#north == true && self.r#east == false && self.r#waterlogged == true && self.r#down == false && self.r#west == false { return 8286; }
        if self.r#north == false && self.r#down == true && self.r#up == false && self.r#east == true && self.r#south == false && self.r#waterlogged == false && self.r#west == false { return 8220; }
        if self.r#west == false && self.r#south == false && self.r#east == true && self.r#up == false && self.r#waterlogged == true && self.r#north == true && self.r#down == false { return 8266; }
        if self.r#down == false && self.r#up == false && self.r#north == false && self.r#west == false && self.r#east == true && self.r#south == true && self.r#waterlogged == false { return 8276; }
        if self.r#up == true && self.r#east == false && self.r#down == true && self.r#south == false && self.r#waterlogged == false && self.r#west == false && self.r#north == false { return 8248; }
        if self.r#south == true && self.r#east == false && self.r#north == false && self.r#down == false && self.r#up == false && self.r#waterlogged == true && self.r#west == false { return 8306; }
        if self.r#east == true && self.r#down == true && self.r#up == false && self.r#waterlogged == true && self.r#north == true && self.r#west == true && self.r#south == true { return 8193; }
        if self.r#south == false && self.r#up == false && self.r#east == false && self.r#waterlogged == false && self.r#west == false && self.r#down == true && self.r#north == true { return 8236; }
        if self.r#waterlogged == true && self.r#down == true && self.r#south == true && self.r#west == false && self.r#east == true && self.r#north == true && self.r#up == true { return 8190; }
        if self.r#up == false && self.r#east == true && self.r#waterlogged == false && self.r#north == true && self.r#west == true && self.r#down == false && self.r#south == false { return 8267; }
        if self.r#east == true && self.r#west == false && self.r#south == true && self.r#waterlogged == true && self.r#north == true && self.r#up == false && self.r#down == false { return 8258; }
        if self.r#up == true && self.r#west == false && self.r#south == false && self.r#east == true && self.r#north == true && self.r#waterlogged == false && self.r#down == false { return 8264; }
        if self.r#east == false && self.r#south == true && self.r#down == false && self.r#up == false && self.r#north == true && self.r#waterlogged == false && self.r#west == true { return 8291; }
        if self.r#south == true && self.r#east == false && self.r#west == true && self.r#waterlogged == true && self.r#up == false && self.r#north == true && self.r#down == true { return 8225; }
        if self.r#waterlogged == true && self.r#west == false && self.r#north == true && self.r#down == false && self.r#south == false && self.r#up == true && self.r#east == false { return 8294; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 8293 {
            return Some(GlowLichen {
                r#east: false,
                r#up: true,
                r#south: false,
                r#waterlogged: true,
                r#north: true,
                r#down: false,
                r#west: true,
            });
        }
        if state_id == 8303 {
            return Some(GlowLichen {
                r#north: false,
                r#south: true,
                r#waterlogged: false,
                r#down: false,
                r#east: false,
                r#up: true,
                r#west: true,
            });
        }
        if state_id == 8208 {
            return Some(GlowLichen {
                r#north: false,
                r#east: true,
                r#south: true,
                r#up: true,
                r#waterlogged: false,
                r#west: false,
                r#down: true,
            });
        }
        if state_id == 8196 {
            return Some(GlowLichen {
                r#down: true,
                r#north: true,
                r#up: false,
                r#south: true,
                r#waterlogged: false,
                r#west: false,
                r#east: true,
            });
        }
        if state_id == 8263 {
            return Some(GlowLichen {
                r#down: false,
                r#west: true,
                r#north: true,
                r#south: false,
                r#up: true,
                r#east: true,
                r#waterlogged: false,
            });
        }
        if state_id == 8219 {
            return Some(GlowLichen {
                r#down: true,
                r#south: false,
                r#waterlogged: false,
                r#west: true,
                r#north: false,
                r#east: true,
                r#up: false,
            });
        }
        if state_id == 8308 {
            return Some(GlowLichen {
                r#down: false,
                r#up: false,
                r#waterlogged: false,
                r#south: true,
                r#west: false,
                r#east: false,
                r#north: false,
            });
        }
        if state_id == 8265 {
            return Some(GlowLichen {
                r#up: false,
                r#waterlogged: true,
                r#west: true,
                r#down: false,
                r#north: true,
                r#south: false,
                r#east: true,
            });
        }
        if state_id == 8223 {
            return Some(GlowLichen {
                r#up: true,
                r#down: true,
                r#east: false,
                r#north: true,
                r#waterlogged: false,
                r#south: true,
                r#west: true,
            });
        }
        if state_id == 8290 {
            return Some(GlowLichen {
                r#east: false,
                r#down: false,
                r#up: false,
                r#waterlogged: true,
                r#west: false,
                r#south: true,
                r#north: true,
            });
        }
        if state_id == 8217 {
            return Some(GlowLichen {
                r#down: true,
                r#south: false,
                r#up: false,
                r#north: false,
                r#waterlogged: true,
                r#west: true,
                r#east: true,
            });
        }
        if state_id == 8299 {
            return Some(GlowLichen {
                r#up: false,
                r#east: false,
                r#north: true,
                r#down: false,
                r#waterlogged: false,
                r#west: true,
                r#south: false,
            });
        }
        if state_id == 8213 {
            return Some(GlowLichen {
                r#west: true,
                r#north: false,
                r#south: false,
                r#up: true,
                r#down: true,
                r#waterlogged: true,
                r#east: true,
            });
        }
        if state_id == 8221 {
            return Some(GlowLichen {
                r#up: true,
                r#north: true,
                r#down: true,
                r#east: false,
                r#waterlogged: true,
                r#west: true,
                r#south: true,
            });
        }
        if state_id == 8234 {
            return Some(GlowLichen {
                r#up: false,
                r#west: false,
                r#north: true,
                r#down: true,
                r#south: false,
                r#east: false,
                r#waterlogged: true,
            });
        }
        if state_id == 8205 {
            return Some(GlowLichen {
                r#down: true,
                r#north: false,
                r#west: true,
                r#up: true,
                r#east: true,
                r#south: true,
                r#waterlogged: true,
            });
        }
        if state_id == 8240 {
            return Some(GlowLichen {
                r#up: true,
                r#west: false,
                r#north: false,
                r#east: false,
                r#south: true,
                r#down: true,
                r#waterlogged: false,
            });
        }
        if state_id == 8250 {
            return Some(GlowLichen {
                r#south: false,
                r#waterlogged: true,
                r#north: false,
                r#down: true,
                r#up: false,
                r#west: false,
                r#east: false,
            });
        }
        if state_id == 8280 {
            return Some(GlowLichen {
                r#up: true,
                r#east: true,
                r#waterlogged: false,
                r#north: false,
                r#west: false,
                r#down: false,
                r#south: false,
            });
        }
        if state_id == 8282 {
            return Some(GlowLichen {
                r#south: false,
                r#down: false,
                r#east: true,
                r#up: false,
                r#north: false,
                r#waterlogged: true,
                r#west: false,
            });
        }
        if state_id == 8249 {
            return Some(GlowLichen {
                r#east: false,
                r#waterlogged: true,
                r#south: false,
                r#north: false,
                r#up: false,
                r#west: true,
                r#down: true,
            });
        }
        if state_id == 8300 {
            return Some(GlowLichen {
                r#south: false,
                r#north: true,
                r#up: false,
                r#down: false,
                r#east: false,
                r#waterlogged: false,
                r#west: false,
            });
        }
        if state_id == 8313 {
            return Some(GlowLichen {
                r#south: false,
                r#waterlogged: true,
                r#north: false,
                r#west: true,
                r#down: false,
                r#east: false,
                r#up: false,
            });
        }
        if state_id == 8237 {
            return Some(GlowLichen {
                r#east: false,
                r#up: true,
                r#waterlogged: true,
                r#west: true,
                r#down: true,
                r#north: false,
                r#south: true,
            });
        }
        if state_id == 8310 {
            return Some(GlowLichen {
                r#south: false,
                r#waterlogged: true,
                r#down: false,
                r#east: false,
                r#north: false,
                r#west: false,
                r#up: true,
            });
        }
        if state_id == 8216 {
            return Some(GlowLichen {
                r#south: false,
                r#waterlogged: false,
                r#north: false,
                r#west: false,
                r#up: true,
                r#east: true,
                r#down: true,
            });
        }
        if state_id == 8270 {
            return Some(GlowLichen {
                r#south: true,
                r#north: false,
                r#down: false,
                r#up: true,
                r#east: true,
                r#waterlogged: true,
                r#west: false,
            });
        }
        if state_id == 8279 {
            return Some(GlowLichen {
                r#down: false,
                r#up: true,
                r#west: true,
                r#north: false,
                r#east: true,
                r#south: false,
                r#waterlogged: false,
            });
        }
        if state_id == 8242 {
            return Some(GlowLichen {
                r#north: false,
                r#down: true,
                r#east: false,
                r#up: false,
                r#south: true,
                r#waterlogged: true,
                r#west: false,
            });
        }
        if state_id == 8209 {
            return Some(GlowLichen {
                r#down: true,
                r#east: true,
                r#north: false,
                r#south: true,
                r#up: false,
                r#waterlogged: true,
                r#west: true,
            });
        }
        if state_id == 8253 {
            return Some(GlowLichen {
                r#waterlogged: true,
                r#south: true,
                r#west: true,
                r#east: true,
                r#north: true,
                r#up: true,
                r#down: false,
            });
        }
        if state_id == 8261 {
            return Some(GlowLichen {
                r#west: true,
                r#down: false,
                r#north: true,
                r#east: true,
                r#up: true,
                r#south: false,
                r#waterlogged: true,
            });
        }
        if state_id == 8305 {
            return Some(GlowLichen {
                r#west: true,
                r#waterlogged: true,
                r#up: false,
                r#down: false,
                r#east: false,
                r#north: false,
                r#south: true,
            });
        }
        if state_id == 8260 {
            return Some(GlowLichen {
                r#down: false,
                r#up: false,
                r#waterlogged: false,
                r#north: true,
                r#west: false,
                r#south: true,
                r#east: true,
            });
        }
        if state_id == 8301 {
            return Some(GlowLichen {
                r#east: false,
                r#waterlogged: true,
                r#north: false,
                r#south: true,
                r#down: false,
                r#west: true,
                r#up: true,
            });
        }
        if state_id == 8243 {
            return Some(GlowLichen {
                r#east: false,
                r#down: true,
                r#south: true,
                r#up: false,
                r#north: false,
                r#west: true,
                r#waterlogged: false,
            });
        }
        if state_id == 8244 {
            return Some(GlowLichen {
                r#east: false,
                r#west: false,
                r#south: true,
                r#north: false,
                r#waterlogged: false,
                r#up: false,
                r#down: true,
            });
        }
        if state_id == 8259 {
            return Some(GlowLichen {
                r#west: true,
                r#south: true,
                r#north: true,
                r#down: false,
                r#up: false,
                r#east: true,
                r#waterlogged: false,
            });
        }
        if state_id == 8277 {
            return Some(GlowLichen {
                r#down: false,
                r#north: false,
                r#south: false,
                r#waterlogged: true,
                r#west: true,
                r#up: true,
                r#east: true,
            });
        }
        if state_id == 8198 {
            return Some(GlowLichen {
                r#east: true,
                r#south: false,
                r#north: true,
                r#up: true,
                r#down: true,
                r#waterlogged: true,
                r#west: false,
            });
        }
        if state_id == 8304 {
            return Some(GlowLichen {
                r#south: true,
                r#down: false,
                r#up: true,
                r#east: false,
                r#west: false,
                r#north: false,
                r#waterlogged: false,
            });
        }
        if state_id == 8231 {
            return Some(GlowLichen {
                r#east: false,
                r#north: true,
                r#south: false,
                r#up: true,
                r#down: true,
                r#waterlogged: false,
                r#west: true,
            });
        }
        if state_id == 8189 {
            return Some(GlowLichen {
                r#down: true,
                r#east: true,
                r#waterlogged: true,
                r#up: true,
                r#west: true,
                r#north: true,
                r#south: true,
            });
        }
        if state_id == 8315 {
            return Some(GlowLichen {
                r#waterlogged: false,
                r#west: true,
                r#down: false,
                r#south: false,
                r#up: false,
                r#east: false,
                r#north: false,
            });
        }
        if state_id == 8252 {
            return Some(GlowLichen {
                r#up: false,
                r#waterlogged: false,
                r#down: true,
                r#east: false,
                r#south: false,
                r#north: false,
                r#west: false,
            });
        }
        if state_id == 8309 {
            return Some(GlowLichen {
                r#south: false,
                r#west: true,
                r#down: false,
                r#waterlogged: true,
                r#north: false,
                r#east: false,
                r#up: true,
            });
        }
        if state_id == 8254 {
            return Some(GlowLichen {
                r#south: true,
                r#east: true,
                r#up: true,
                r#waterlogged: true,
                r#west: false,
                r#down: false,
                r#north: true,
            });
        }
        if state_id == 8274 {
            return Some(GlowLichen {
                r#waterlogged: true,
                r#north: false,
                r#east: true,
                r#south: true,
                r#west: false,
                r#down: false,
                r#up: false,
            });
        }
        if state_id == 8251 {
            return Some(GlowLichen {
                r#waterlogged: false,
                r#down: true,
                r#up: false,
                r#west: true,
                r#north: false,
                r#east: false,
                r#south: false,
            });
        }
        if state_id == 8314 {
            return Some(GlowLichen {
                r#south: false,
                r#down: false,
                r#up: false,
                r#waterlogged: true,
                r#east: false,
                r#north: false,
                r#west: false,
            });
        }
        if state_id == 8281 {
            return Some(GlowLichen {
                r#west: true,
                r#down: false,
                r#waterlogged: true,
                r#north: false,
                r#south: false,
                r#east: true,
                r#up: false,
            });
        }
        if state_id == 8246 {
            return Some(GlowLichen {
                r#west: false,
                r#waterlogged: true,
                r#up: true,
                r#south: false,
                r#north: false,
                r#down: true,
                r#east: false,
            });
        }
        if state_id == 8233 {
            return Some(GlowLichen {
                r#waterlogged: true,
                r#north: true,
                r#south: false,
                r#up: false,
                r#east: false,
                r#west: true,
                r#down: true,
            });
        }
        if state_id == 8269 {
            return Some(GlowLichen {
                r#down: false,
                r#up: true,
                r#east: true,
                r#north: false,
                r#south: true,
                r#waterlogged: true,
                r#west: true,
            });
        }
        if state_id == 8239 {
            return Some(GlowLichen {
                r#west: true,
                r#south: true,
                r#east: false,
                r#down: true,
                r#up: true,
                r#waterlogged: false,
                r#north: false,
            });
        }
        if state_id == 8197 {
            return Some(GlowLichen {
                r#north: true,
                r#up: true,
                r#east: true,
                r#down: true,
                r#waterlogged: true,
                r#west: true,
                r#south: false,
            });
        }
        if state_id == 8228 {
            return Some(GlowLichen {
                r#down: true,
                r#east: false,
                r#waterlogged: false,
                r#north: true,
                r#up: false,
                r#south: true,
                r#west: false,
            });
        }
        if state_id == 8262 {
            return Some(GlowLichen {
                r#east: true,
                r#waterlogged: true,
                r#west: false,
                r#down: false,
                r#north: true,
                r#south: false,
                r#up: true,
            });
        }
        if state_id == 8247 {
            return Some(GlowLichen {
                r#down: true,
                r#west: true,
                r#south: false,
                r#north: false,
                r#waterlogged: false,
                r#east: false,
                r#up: true,
            });
        }
        if state_id == 8257 {
            return Some(GlowLichen {
                r#up: false,
                r#west: true,
                r#down: false,
                r#east: true,
                r#waterlogged: true,
                r#north: true,
                r#south: true,
            });
        }
        if state_id == 8238 {
            return Some(GlowLichen {
                r#down: true,
                r#east: false,
                r#south: true,
                r#west: false,
                r#waterlogged: true,
                r#north: false,
                r#up: true,
            });
        }
        if state_id == 8287 {
            return Some(GlowLichen {
                r#waterlogged: false,
                r#west: true,
                r#east: false,
                r#down: false,
                r#north: true,
                r#south: true,
                r#up: true,
            });
        }
        if state_id == 8241 {
            return Some(GlowLichen {
                r#east: false,
                r#up: false,
                r#south: true,
                r#waterlogged: true,
                r#down: true,
                r#west: true,
                r#north: false,
            });
        }
        if state_id == 8271 {
            return Some(GlowLichen {
                r#waterlogged: false,
                r#down: false,
                r#north: false,
                r#south: true,
                r#up: true,
                r#west: true,
                r#east: true,
            });
        }
        if state_id == 8195 {
            return Some(GlowLichen {
                r#east: true,
                r#up: false,
                r#down: true,
                r#south: true,
                r#waterlogged: false,
                r#north: true,
                r#west: true,
            });
        }
        if state_id == 8218 {
            return Some(GlowLichen {
                r#down: true,
                r#east: true,
                r#north: false,
                r#south: false,
                r#up: false,
                r#waterlogged: true,
                r#west: false,
            });
        }
        if state_id == 8283 {
            return Some(GlowLichen {
                r#north: false,
                r#east: true,
                r#south: false,
                r#waterlogged: false,
                r#west: true,
                r#down: false,
                r#up: false,
            });
        }
        if state_id == 8272 {
            return Some(GlowLichen {
                r#south: true,
                r#up: true,
                r#north: false,
                r#east: true,
                r#waterlogged: false,
                r#west: false,
                r#down: false,
            });
        }
        if state_id == 8214 {
            return Some(GlowLichen {
                r#down: true,
                r#east: true,
                r#up: true,
                r#waterlogged: true,
                r#south: false,
                r#north: false,
                r#west: false,
            });
        }
        if state_id == 8210 {
            return Some(GlowLichen {
                r#east: true,
                r#north: false,
                r#down: true,
                r#up: false,
                r#waterlogged: true,
                r#west: false,
                r#south: true,
            });
        }
        if state_id == 8295 {
            return Some(GlowLichen {
                r#north: true,
                r#down: false,
                r#south: false,
                r#up: true,
                r#waterlogged: false,
                r#west: true,
                r#east: false,
            });
        }
        if state_id == 8224 {
            return Some(GlowLichen {
                r#up: true,
                r#waterlogged: false,
                r#down: true,
                r#south: true,
                r#west: false,
                r#east: false,
                r#north: true,
            });
        }
        if state_id == 8297 {
            return Some(GlowLichen {
                r#west: true,
                r#down: false,
                r#up: false,
                r#east: false,
                r#waterlogged: true,
                r#north: true,
                r#south: false,
            });
        }
        if state_id == 8230 {
            return Some(GlowLichen {
                r#up: true,
                r#waterlogged: true,
                r#west: false,
                r#south: false,
                r#north: true,
                r#down: true,
                r#east: false,
            });
        }
        if state_id == 8278 {
            return Some(GlowLichen {
                r#east: true,
                r#down: false,
                r#up: true,
                r#waterlogged: true,
                r#north: false,
                r#west: false,
                r#south: false,
            });
        }
        if state_id == 8245 {
            return Some(GlowLichen {
                r#south: false,
                r#up: true,
                r#west: true,
                r#down: true,
                r#east: false,
                r#north: false,
                r#waterlogged: true,
            });
        }
        if state_id == 8285 {
            return Some(GlowLichen {
                r#down: false,
                r#south: true,
                r#up: true,
                r#west: true,
                r#north: true,
                r#waterlogged: true,
                r#east: false,
            });
        }
        if state_id == 8212 {
            return Some(GlowLichen {
                r#south: true,
                r#west: false,
                r#up: false,
                r#waterlogged: false,
                r#north: false,
                r#east: true,
                r#down: true,
            });
        }
        if state_id == 8288 {
            return Some(GlowLichen {
                r#down: false,
                r#north: true,
                r#up: true,
                r#east: false,
                r#waterlogged: false,
                r#west: false,
                r#south: true,
            });
        }
        if state_id == 8200 {
            return Some(GlowLichen {
                r#down: true,
                r#south: false,
                r#north: true,
                r#west: false,
                r#east: true,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 8229 {
            return Some(GlowLichen {
                r#waterlogged: true,
                r#south: false,
                r#east: false,
                r#down: true,
                r#north: true,
                r#up: true,
                r#west: true,
            });
        }
        if state_id == 8273 {
            return Some(GlowLichen {
                r#west: true,
                r#down: false,
                r#south: true,
                r#up: false,
                r#east: true,
                r#north: false,
                r#waterlogged: true,
            });
        }
        if state_id == 8275 {
            return Some(GlowLichen {
                r#east: true,
                r#north: false,
                r#south: true,
                r#waterlogged: false,
                r#up: false,
                r#down: false,
                r#west: true,
            });
        }
        if state_id == 8202 {
            return Some(GlowLichen {
                r#down: true,
                r#north: true,
                r#up: false,
                r#waterlogged: true,
                r#west: false,
                r#south: false,
                r#east: true,
            });
        }
        if state_id == 8227 {
            return Some(GlowLichen {
                r#west: true,
                r#east: false,
                r#waterlogged: false,
                r#down: true,
                r#up: false,
                r#north: true,
                r#south: true,
            });
        }
        if state_id == 8232 {
            return Some(GlowLichen {
                r#west: false,
                r#down: true,
                r#south: false,
                r#east: false,
                r#north: true,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 8201 {
            return Some(GlowLichen {
                r#south: false,
                r#north: true,
                r#up: false,
                r#waterlogged: true,
                r#west: true,
                r#down: true,
                r#east: true,
            });
        }
        if state_id == 8207 {
            return Some(GlowLichen {
                r#waterlogged: false,
                r#down: true,
                r#west: true,
                r#north: false,
                r#south: true,
                r#up: true,
                r#east: true,
            });
        }
        if state_id == 8292 {
            return Some(GlowLichen {
                r#down: false,
                r#waterlogged: false,
                r#west: false,
                r#up: false,
                r#east: false,
                r#south: true,
                r#north: true,
            });
        }
        if state_id == 8192 {
            return Some(GlowLichen {
                r#down: true,
                r#east: true,
                r#south: true,
                r#waterlogged: false,
                r#west: false,
                r#north: true,
                r#up: true,
            });
        }
        if state_id == 8316 {
            return Some(GlowLichen {
                r#waterlogged: false,
                r#west: false,
                r#up: false,
                r#down: false,
                r#north: false,
                r#south: false,
                r#east: false,
            });
        }
        if state_id == 8296 {
            return Some(GlowLichen {
                r#down: false,
                r#north: true,
                r#up: true,
                r#west: false,
                r#south: false,
                r#waterlogged: false,
                r#east: false,
            });
        }
        if state_id == 8255 {
            return Some(GlowLichen {
                r#waterlogged: false,
                r#east: true,
                r#south: true,
                r#up: true,
                r#down: false,
                r#north: true,
                r#west: true,
            });
        }
        if state_id == 8307 {
            return Some(GlowLichen {
                r#waterlogged: false,
                r#north: false,
                r#up: false,
                r#down: false,
                r#east: false,
                r#south: true,
                r#west: true,
            });
        }
        if state_id == 8199 {
            return Some(GlowLichen {
                r#up: true,
                r#west: true,
                r#south: false,
                r#east: true,
                r#north: true,
                r#waterlogged: false,
                r#down: true,
            });
        }
        if state_id == 8194 {
            return Some(GlowLichen {
                r#east: true,
                r#south: true,
                r#west: false,
                r#down: true,
                r#north: true,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 8211 {
            return Some(GlowLichen {
                r#waterlogged: false,
                r#south: true,
                r#north: false,
                r#down: true,
                r#west: true,
                r#east: true,
                r#up: false,
            });
        }
        if state_id == 8226 {
            return Some(GlowLichen {
                r#west: false,
                r#north: true,
                r#down: true,
                r#up: false,
                r#waterlogged: true,
                r#east: false,
                r#south: true,
            });
        }
        if state_id == 8203 {
            return Some(GlowLichen {
                r#down: true,
                r#up: false,
                r#west: true,
                r#north: true,
                r#south: false,
                r#east: true,
                r#waterlogged: false,
            });
        }
        if state_id == 8204 {
            return Some(GlowLichen {
                r#east: true,
                r#waterlogged: false,
                r#north: true,
                r#south: false,
                r#west: false,
                r#up: false,
                r#down: true,
            });
        }
        if state_id == 8206 {
            return Some(GlowLichen {
                r#south: true,
                r#down: true,
                r#waterlogged: true,
                r#north: false,
                r#east: true,
                r#up: true,
                r#west: false,
            });
        }
        if state_id == 8284 {
            return Some(GlowLichen {
                r#down: false,
                r#up: false,
                r#south: false,
                r#east: true,
                r#waterlogged: false,
                r#north: false,
                r#west: false,
            });
        }
        if state_id == 8215 {
            return Some(GlowLichen {
                r#north: false,
                r#waterlogged: false,
                r#east: true,
                r#west: true,
                r#up: true,
                r#down: true,
                r#south: false,
            });
        }
        if state_id == 8289 {
            return Some(GlowLichen {
                r#west: true,
                r#waterlogged: true,
                r#south: true,
                r#east: false,
                r#down: false,
                r#north: true,
                r#up: false,
            });
        }
        if state_id == 8256 {
            return Some(GlowLichen {
                r#south: true,
                r#north: true,
                r#up: true,
                r#east: true,
                r#waterlogged: false,
                r#down: false,
                r#west: false,
            });
        }
        if state_id == 8222 {
            return Some(GlowLichen {
                r#north: true,
                r#south: true,
                r#down: true,
                r#up: true,
                r#waterlogged: true,
                r#east: false,
                r#west: false,
            });
        }
        if state_id == 8268 {
            return Some(GlowLichen {
                r#south: false,
                r#west: false,
                r#north: true,
                r#down: false,
                r#east: true,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 8302 {
            return Some(GlowLichen {
                r#down: false,
                r#east: false,
                r#waterlogged: true,
                r#west: false,
                r#up: true,
                r#north: false,
                r#south: true,
            });
        }
        if state_id == 8311 {
            return Some(GlowLichen {
                r#west: true,
                r#down: false,
                r#east: false,
                r#north: false,
                r#south: false,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 8312 {
            return Some(GlowLichen {
                r#east: false,
                r#south: false,
                r#down: false,
                r#north: false,
                r#up: true,
                r#waterlogged: false,
                r#west: false,
            });
        }
        if state_id == 8191 {
            return Some(GlowLichen {
                r#south: true,
                r#up: true,
                r#waterlogged: false,
                r#west: true,
                r#east: true,
                r#down: true,
                r#north: true,
            });
        }
        if state_id == 8235 {
            return Some(GlowLichen {
                r#east: false,
                r#up: false,
                r#down: true,
                r#south: false,
                r#west: true,
                r#waterlogged: false,
                r#north: true,
            });
        }
        if state_id == 8298 {
            return Some(GlowLichen {
                r#down: false,
                r#east: false,
                r#up: false,
                r#west: false,
                r#north: true,
                r#south: false,
                r#waterlogged: true,
            });
        }
        if state_id == 8286 {
            return Some(GlowLichen {
                r#up: true,
                r#south: true,
                r#north: true,
                r#east: false,
                r#waterlogged: true,
                r#down: false,
                r#west: false,
            });
        }
        if state_id == 8220 {
            return Some(GlowLichen {
                r#north: false,
                r#down: true,
                r#up: false,
                r#east: true,
                r#south: false,
                r#waterlogged: false,
                r#west: false,
            });
        }
        if state_id == 8266 {
            return Some(GlowLichen {
                r#west: false,
                r#south: false,
                r#east: true,
                r#up: false,
                r#waterlogged: true,
                r#north: true,
                r#down: false,
            });
        }
        if state_id == 8276 {
            return Some(GlowLichen {
                r#down: false,
                r#up: false,
                r#north: false,
                r#west: false,
                r#east: true,
                r#south: true,
                r#waterlogged: false,
            });
        }
        if state_id == 8248 {
            return Some(GlowLichen {
                r#up: true,
                r#east: false,
                r#down: true,
                r#south: false,
                r#waterlogged: false,
                r#west: false,
                r#north: false,
            });
        }
        if state_id == 8306 {
            return Some(GlowLichen {
                r#south: true,
                r#east: false,
                r#north: false,
                r#down: false,
                r#up: false,
                r#waterlogged: true,
                r#west: false,
            });
        }
        if state_id == 8193 {
            return Some(GlowLichen {
                r#east: true,
                r#down: true,
                r#up: false,
                r#waterlogged: true,
                r#north: true,
                r#west: true,
                r#south: true,
            });
        }
        if state_id == 8236 {
            return Some(GlowLichen {
                r#south: false,
                r#up: false,
                r#east: false,
                r#waterlogged: false,
                r#west: false,
                r#down: true,
                r#north: true,
            });
        }
        if state_id == 8190 {
            return Some(GlowLichen {
                r#waterlogged: true,
                r#down: true,
                r#south: true,
                r#west: false,
                r#east: true,
                r#north: true,
                r#up: true,
            });
        }
        if state_id == 8267 {
            return Some(GlowLichen {
                r#up: false,
                r#east: true,
                r#waterlogged: false,
                r#north: true,
                r#west: true,
                r#down: false,
                r#south: false,
            });
        }
        if state_id == 8258 {
            return Some(GlowLichen {
                r#east: true,
                r#west: false,
                r#south: true,
                r#waterlogged: true,
                r#north: true,
                r#up: false,
                r#down: false,
            });
        }
        if state_id == 8264 {
            return Some(GlowLichen {
                r#up: true,
                r#west: false,
                r#south: false,
                r#east: true,
                r#north: true,
                r#waterlogged: false,
                r#down: false,
            });
        }
        if state_id == 8291 {
            return Some(GlowLichen {
                r#east: false,
                r#south: true,
                r#down: false,
                r#up: false,
                r#north: true,
                r#waterlogged: false,
                r#west: true,
            });
        }
        if state_id == 8225 {
            return Some(GlowLichen {
                r#south: true,
                r#east: false,
                r#west: true,
                r#waterlogged: true,
                r#up: false,
                r#north: true,
                r#down: true,
            });
        }
        if state_id == 8294 {
            return Some(GlowLichen {
                r#waterlogged: true,
                r#west: false,
                r#north: true,
                r#down: false,
                r#south: false,
                r#up: true,
                r#east: false,
            });
        }
        return None;
    }
}

