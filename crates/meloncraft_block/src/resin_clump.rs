use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResinClump {
    pub west: bool,
    pub up: bool,
    pub north: bool,
    pub east: bool,
    pub down: bool,
    pub south: bool,
    pub waterlogged: bool,
}


impl BlockState for ResinClump {
    fn to_id(self) -> i32 {
        if block_state.r#down == false && block_state.r#east == false && block_state.r#south == false && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == true && block_state.r#north == true { return 8423; }
        if block_state.r#south == true && block_state.r#north == false && block_state.r#down == false && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == true && block_state.r#east == false { return 8431; }
        if block_state.r#down == false && block_state.r#south == false && block_state.r#north == true && block_state.r#east == false && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == true { return 8427; }
        if block_state.r#west == false && block_state.r#north == true && block_state.r#east == false && block_state.r#up == true && block_state.r#down == false && block_state.r#south == true && block_state.r#waterlogged == false { return 8416; }
        if block_state.r#down == true && block_state.r#south == false && block_state.r#waterlogged == false && block_state.r#north == false && block_state.r#east == true && block_state.r#west == true && block_state.r#up == true { return 8343; }
        if block_state.r#down == true && block_state.r#north == true && block_state.r#waterlogged == true && block_state.r#west == true && block_state.r#east == true && block_state.r#south == true && block_state.r#up == false { return 8321; }
        if block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#down == true && block_state.r#east == true && block_state.r#north == true && block_state.r#south == false && block_state.r#west == false { return 8330; }
        if block_state.r#waterlogged == false && block_state.r#west == false && block_state.r#down == false && block_state.r#south == false && block_state.r#up == true && block_state.r#east == false && block_state.r#north == false { return 8440; }
        if block_state.r#down == false && block_state.r#west == false && block_state.r#east == false && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#north == true && block_state.r#south == true { return 8420; }
        if block_state.r#east == true && block_state.r#up == true && block_state.r#north == true && block_state.r#down == true && block_state.r#south == false && block_state.r#waterlogged == true && block_state.r#west == false { return 8326; }
        if block_state.r#north == true && block_state.r#up == false && block_state.r#east == true && block_state.r#down == false && block_state.r#waterlogged == false && block_state.r#south == false && block_state.r#west == true { return 8395; }
        if block_state.r#south == false && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == false && block_state.r#down == false && block_state.r#east == false && block_state.r#north == false { return 8444; }
        if block_state.r#east == true && block_state.r#down == true && block_state.r#north == false && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == true && block_state.r#south == true { return 8335; }
        if block_state.r#east == true && block_state.r#waterlogged == false && block_state.r#west == true && block_state.r#south == true && block_state.r#north == false && block_state.r#up == true && block_state.r#down == false { return 8399; }
        if block_state.r#up == false && block_state.r#west == false && block_state.r#north == true && block_state.r#waterlogged == true && block_state.r#down == true && block_state.r#east == false && block_state.r#south == true { return 8354; }
        if block_state.r#east == true && block_state.r#down == false && block_state.r#south == true && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == true && block_state.r#north == true { return 8385; }
        if block_state.r#waterlogged == true && block_state.r#down == true && block_state.r#south == true && block_state.r#east == true && block_state.r#west == false && block_state.r#up == true && block_state.r#north == true { return 8318; }
        if block_state.r#waterlogged == true && block_state.r#down == false && block_state.r#south == true && block_state.r#north == false && block_state.r#east == false && block_state.r#up == false && block_state.r#west == true { return 8433; }
        if block_state.r#north == true && block_state.r#south == true && block_state.r#west == false && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#east == true && block_state.r#down == true { return 8320; }
        if block_state.r#down == false && block_state.r#waterlogged == true && block_state.r#south == false && block_state.r#east == false && block_state.r#west == false && block_state.r#up == true && block_state.r#north == true { return 8422; }
        if block_state.r#up == true && block_state.r#down == true && block_state.r#waterlogged == true && block_state.r#west == false && block_state.r#south == false && block_state.r#east == false && block_state.r#north == false { return 8374; }
        if block_state.r#east == true && block_state.r#up == false && block_state.r#west == false && block_state.r#down == false && block_state.r#south == false && block_state.r#waterlogged == false && block_state.r#north == true { return 8396; }
        if block_state.r#waterlogged == true && block_state.r#down == false && block_state.r#north == false && block_state.r#up == true && block_state.r#east == true && block_state.r#south == false && block_state.r#west == true { return 8405; }
        if block_state.r#down == false && block_state.r#up == true && block_state.r#west == true && block_state.r#south == true && block_state.r#waterlogged == false && block_state.r#north == true && block_state.r#east == false { return 8415; }
        if block_state.r#west == true && block_state.r#up == false && block_state.r#down == false && block_state.r#east == false && block_state.r#north == true && block_state.r#south == true && block_state.r#waterlogged == false { return 8419; }
        if block_state.r#up == false && block_state.r#north == false && block_state.r#waterlogged == false && block_state.r#east == true && block_state.r#west == true && block_state.r#down == false && block_state.r#south == true { return 8403; }
        if block_state.r#south == false && block_state.r#west == false && block_state.r#down == false && block_state.r#east == false && block_state.r#up == false && block_state.r#north == true && block_state.r#waterlogged == false { return 8428; }
        if block_state.r#down == true && block_state.r#east == true && block_state.r#south == false && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == false && block_state.r#north == false { return 8348; }
        if block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#north == true && block_state.r#south == false && block_state.r#west == false && block_state.r#down == true && block_state.r#east == true { return 8328; }
        if block_state.r#east == true && block_state.r#north == false && block_state.r#south == true && block_state.r#down == true && block_state.r#west == true && block_state.r#waterlogged == true && block_state.r#up == false { return 8337; }
        if block_state.r#south == false && block_state.r#west == false && block_state.r#down == true && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#east == true && block_state.r#north == false { return 8342; }
        if block_state.r#south == true && block_state.r#waterlogged == false && block_state.r#east == true && block_state.r#up == false && block_state.r#west == false && block_state.r#north == true && block_state.r#down == true { return 8324; }
        if block_state.r#west == true && block_state.r#waterlogged == true && block_state.r#down == false && block_state.r#south == false && block_state.r#north == false && block_state.r#up == false && block_state.r#east == false { return 8441; }
        if block_state.r#east == true && block_state.r#south == false && block_state.r#down == true && block_state.r#west == false && block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#north == true { return 8332; }
        if block_state.r#east == true && block_state.r#waterlogged == false && block_state.r#west == false && block_state.r#down == false && block_state.r#north == false && block_state.r#south == false && block_state.r#up == true { return 8408; }
        if block_state.r#west == true && block_state.r#down == true && block_state.r#up == false && block_state.r#north == false && block_state.r#south == false && block_state.r#waterlogged == true && block_state.r#east == true { return 8345; }
        if block_state.r#south == false && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#north == true && block_state.r#west == true && block_state.r#down == false && block_state.r#east == true { return 8391; }
        if block_state.r#north == false && block_state.r#south == true && block_state.r#waterlogged == false && block_state.r#east == true && block_state.r#up == true && block_state.r#west == false && block_state.r#down == true { return 8336; }
        if block_state.r#east == true && block_state.r#south == false && block_state.r#down == false && block_state.r#north == false && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == true { return 8409; }
        if block_state.r#east == false && block_state.r#south == true && block_state.r#north == false && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == false && block_state.r#down == true { return 8366; }
        if block_state.r#up == false && block_state.r#west == true && block_state.r#east == true && block_state.r#waterlogged == false && block_state.r#north == false && block_state.r#down == true && block_state.r#south == false { return 8347; }
        if block_state.r#up == false && block_state.r#north == false && block_state.r#waterlogged == false && block_state.r#east == false && block_state.r#south == true && block_state.r#west == false && block_state.r#down == true { return 8372; }
        if block_state.r#west == true && block_state.r#north == false && block_state.r#down == false && block_state.r#waterlogged == true && block_state.r#east == true && block_state.r#up == true && block_state.r#south == true { return 8397; }
        if block_state.r#waterlogged == false && block_state.r#east == false && block_state.r#west == true && block_state.r#down == true && block_state.r#south == false && block_state.r#up == false && block_state.r#north == true { return 8363; }
        if block_state.r#waterlogged == true && block_state.r#down == false && block_state.r#east == false && block_state.r#west == true && block_state.r#north == true && block_state.r#up == false && block_state.r#south == true { return 8417; }
        if block_state.r#down == false && block_state.r#waterlogged == true && block_state.r#west == false && block_state.r#south == true && block_state.r#east == false && block_state.r#north == true && block_state.r#up == false { return 8418; }
        if block_state.r#up == false && block_state.r#down == true && block_state.r#west == true && block_state.r#north == true && block_state.r#east == false && block_state.r#south == true && block_state.r#waterlogged == true { return 8353; }
        if block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == true && block_state.r#down == false && block_state.r#east == false && block_state.r#north == false && block_state.r#south == false { return 8439; }
        if block_state.r#west == false && block_state.r#waterlogged == true && block_state.r#south == true && block_state.r#down == false && block_state.r#up == false && block_state.r#north == true && block_state.r#east == true { return 8386; }
        if block_state.r#waterlogged == true && block_state.r#south == true && block_state.r#up == true && block_state.r#west == false && block_state.r#north == false && block_state.r#down == false && block_state.r#east == false { return 8430; }
        if block_state.r#east == false && block_state.r#north == false && block_state.r#waterlogged == true && block_state.r#west == true && block_state.r#up == false && block_state.r#south == true && block_state.r#down == true { return 8369; }
        if block_state.r#west == false && block_state.r#up == true && block_state.r#south == false && block_state.r#down == false && block_state.r#north == false && block_state.r#east == true && block_state.r#waterlogged == true { return 8406; }
        if block_state.r#down == false && block_state.r#east == false && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == true && block_state.r#south == true && block_state.r#north == false { return 8429; }
        if block_state.r#east == false && block_state.r#down == true && block_state.r#north == false && block_state.r#waterlogged == true && block_state.r#west == false && block_state.r#south == false && block_state.r#up == false { return 8378; }
        if block_state.r#west == false && block_state.r#south == true && block_state.r#east == false && block_state.r#waterlogged == false && block_state.r#north == false && block_state.r#down == false && block_state.r#up == true { return 8432; }
        if block_state.r#up == true && block_state.r#west == false && block_state.r#east == false && block_state.r#south == false && block_state.r#waterlogged == false && block_state.r#down == false && block_state.r#north == true { return 8424; }
        if block_state.r#down == true && block_state.r#east == true && block_state.r#waterlogged == true && block_state.r#west == false && block_state.r#south == false && block_state.r#up == false && block_state.r#north == false { return 8346; }
        if block_state.r#up == true && block_state.r#west == true && block_state.r#east == true && block_state.r#waterlogged == true && block_state.r#down == true && block_state.r#north == false && block_state.r#south == true { return 8333; }
        if block_state.r#north == true && block_state.r#south == true && block_state.r#waterlogged == false && block_state.r#down == true && block_state.r#up == false && block_state.r#west == false && block_state.r#east == false { return 8356; }
        if block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#down == true && block_state.r#north == false && block_state.r#east == false && block_state.r#west == true && block_state.r#south == false { return 8373; }
        if block_state.r#down == true && block_state.r#east == false && block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#north == false && block_state.r#south == false && block_state.r#west == false { return 8376; }
        if block_state.r#down == true && block_state.r#north == true && block_state.r#west == true && block_state.r#up == false && block_state.r#east == true && block_state.r#south == false && block_state.r#waterlogged == true { return 8329; }
        if block_state.r#north == true && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#down == false && block_state.r#west == false && block_state.r#south == false && block_state.r#east == true { return 8394; }
        if block_state.r#east == false && block_state.r#south == false && block_state.r#waterlogged == false && block_state.r#west == true && block_state.r#up == true && block_state.r#down == true && block_state.r#north == false { return 8375; }
        if block_state.r#west == true && block_state.r#down == false && block_state.r#east == true && block_state.r#north == false && block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#south == true { return 8401; }
        if block_state.r#north == false && block_state.r#south == true && block_state.r#up == false && block_state.r#down == false && block_state.r#waterlogged == false && block_state.r#west == false && block_state.r#east == false { return 8436; }
        if block_state.r#up == false && block_state.r#west == true && block_state.r#north == true && block_state.r#south == true && block_state.r#down == false && block_state.r#waterlogged == false && block_state.r#east == true { return 8387; }
        if block_state.r#up == true && block_state.r#north == true && block_state.r#west == false && block_state.r#waterlogged == false && block_state.r#down == false && block_state.r#east == true && block_state.r#south == true { return 8384; }
        if block_state.r#south == true && block_state.r#east == false && block_state.r#north == false && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == true && block_state.r#down == true { return 8367; }
        if block_state.r#down == true && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == false && block_state.r#south == true && block_state.r#east == true && block_state.r#north == false { return 8338; }
        if block_state.r#north == false && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#down == true && block_state.r#south == false && block_state.r#west == false && block_state.r#east == false { return 8380; }
        if block_state.r#down == true && block_state.r#south == true && block_state.r#up == false && block_state.r#north == true && block_state.r#waterlogged == false && block_state.r#west == true && block_state.r#east == false { return 8355; }
        if block_state.r#waterlogged == false && block_state.r#east == true && block_state.r#north == true && block_state.r#up == false && block_state.r#down == false && block_state.r#south == true && block_state.r#west == false { return 8388; }
        if block_state.r#east == false && block_state.r#waterlogged == true && block_state.r#south == true && block_state.r#up == true && block_state.r#west == true && block_state.r#down == false && block_state.r#north == true { return 8413; }
        if block_state.r#waterlogged == true && block_state.r#north == false && block_state.r#down == false && block_state.r#east == false && block_state.r#south == false && block_state.r#up == false && block_state.r#west == false { return 8442; }
        if block_state.r#up == false && block_state.r#north == false && block_state.r#waterlogged == true && block_state.r#west == false && block_state.r#east == true && block_state.r#south == false && block_state.r#down == false { return 8410; }
        if block_state.r#down == true && block_state.r#north == false && block_state.r#up == false && block_state.r#south == true && block_state.r#east == true && block_state.r#waterlogged == false && block_state.r#west == true { return 8339; }
        if block_state.r#south == true && block_state.r#down == false && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == true && block_state.r#north == true && block_state.r#east == true { return 8383; }
        if block_state.r#east == false && block_state.r#west == false && block_state.r#down == false && block_state.r#up == false && block_state.r#south == true && block_state.r#waterlogged == true && block_state.r#north == false { return 8434; }
        if block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == false && block_state.r#east == true && block_state.r#down == false && block_state.r#north == false && block_state.r#south == false { return 8412; }
        if block_state.r#north == true && block_state.r#south == false && block_state.r#waterlogged == true && block_state.r#east == false && block_state.r#up == false && block_state.r#down == false && block_state.r#west == false { return 8426; }
        if block_state.r#waterlogged == false && block_state.r#down == false && block_state.r#south == false && block_state.r#west == true && block_state.r#east == true && block_state.r#north == false && block_state.r#up == false { return 8411; }
        if block_state.r#south == true && block_state.r#waterlogged == true && block_state.r#down == true && block_state.r#north == true && block_state.r#west == true && block_state.r#east == true && block_state.r#up == true { return 8317; }
        if block_state.r#up == false && block_state.r#west == false && block_state.r#east == true && block_state.r#north == false && block_state.r#south == true && block_state.r#down == true && block_state.r#waterlogged == false { return 8340; }
        if block_state.r#up == true && block_state.r#down == true && block_state.r#north == true && block_state.r#waterlogged == false && block_state.r#south == true && block_state.r#west == true && block_state.r#east == true { return 8319; }
        if block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#west == true && block_state.r#north == true && block_state.r#east == true && block_state.r#south == false && block_state.r#down == true { return 8327; }
        if block_state.r#waterlogged == true && block_state.r#east == false && block_state.r#down == true && block_state.r#up == true && block_state.r#west == true && block_state.r#north == false && block_state.r#south == true { return 8365; }
        if block_state.r#waterlogged == false && block_state.r#west == true && block_state.r#north == false && block_state.r#down == true && block_state.r#east == false && block_state.r#south == true && block_state.r#up == false { return 8371; }
        if block_state.r#west == false && block_state.r#south == true && block_state.r#north == false && block_state.r#down == false && block_state.r#east == true && block_state.r#up == true && block_state.r#waterlogged == true { return 8398; }
        if block_state.r#south == true && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == false && block_state.r#down == false && block_state.r#east == false && block_state.r#north == true { return 8414; }
        if block_state.r#east == true && block_state.r#south == false && block_state.r#down == true && block_state.r#north == true && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == true { return 8331; }
        if block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == true && block_state.r#north == false && block_state.r#down == true && block_state.r#east == true && block_state.r#south == false { return 8341; }
        if block_state.r#waterlogged == true && block_state.r#west == true && block_state.r#east == false && block_state.r#south == true && block_state.r#down == true && block_state.r#north == true && block_state.r#up == true { return 8349; }
        if block_state.r#waterlogged == true && block_state.r#south == false && block_state.r#east == true && block_state.r#west == true && block_state.r#down == true && block_state.r#north == true && block_state.r#up == true { return 8325; }
        if block_state.r#south == false && block_state.r#north == true && block_state.r#waterlogged == true && block_state.r#west == true && block_state.r#up == true && block_state.r#down == true && block_state.r#east == false { return 8357; }
        if block_state.r#north == true && block_state.r#waterlogged == false && block_state.r#east == true && block_state.r#up == true && block_state.r#west == false && block_state.r#south == false && block_state.r#down == false { return 8392; }
        if block_state.r#down == true && block_state.r#north == false && block_state.r#south == true && block_state.r#up == true && block_state.r#west == false && block_state.r#east == true && block_state.r#waterlogged == true { return 8334; }
        if block_state.r#north == true && block_state.r#east == false && block_state.r#up == false && block_state.r#down == true && block_state.r#west == false && block_state.r#south == false && block_state.r#waterlogged == true { return 8362; }
        if block_state.r#south == true && block_state.r#waterlogged == true && block_state.r#down == false && block_state.r#west == false && block_state.r#east == true && block_state.r#north == false && block_state.r#up == false { return 8402; }
        if block_state.r#down == false && block_state.r#up == true && block_state.r#east == true && block_state.r#south == false && block_state.r#waterlogged == false && block_state.r#west == true && block_state.r#north == false { return 8407; }
        if block_state.r#west == true && block_state.r#east == false && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#north == false && block_state.r#down == true && block_state.r#south == false { return 8379; }
        if block_state.r#west == false && block_state.r#down == false && block_state.r#east == true && block_state.r#south == false && block_state.r#north == true && block_state.r#up == true && block_state.r#waterlogged == true { return 8390; }
        if block_state.r#north == false && block_state.r#south == true && block_state.r#up == false && block_state.r#west == true && block_state.r#waterlogged == false && block_state.r#down == false && block_state.r#east == false { return 8435; }
        if block_state.r#waterlogged == true && block_state.r#south == false && block_state.r#west == true && block_state.r#east == false && block_state.r#north == true && block_state.r#up == false && block_state.r#down == false { return 8425; }
        if block_state.r#east == false && block_state.r#waterlogged == true && block_state.r#down == true && block_state.r#up == false && block_state.r#south == false && block_state.r#north == false && block_state.r#west == true { return 8377; }
        if block_state.r#east == false && block_state.r#north == false && block_state.r#south == true && block_state.r#up == false && block_state.r#down == true && block_state.r#waterlogged == true && block_state.r#west == false { return 8370; }
        if block_state.r#waterlogged == false && block_state.r#down == true && block_state.r#east == false && block_state.r#north == true && block_state.r#up == true && block_state.r#west == true && block_state.r#south == false { return 8359; }
        if block_state.r#south == true && block_state.r#down == true && block_state.r#east == false && block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#west == true && block_state.r#north == true { return 8351; }
        if block_state.r#east == false && block_state.r#north == false && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == true && block_state.r#down == false && block_state.r#south == false { return 8437; }
        if block_state.r#south == false && block_state.r#east == false && block_state.r#down == false && block_state.r#north == false && block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#west == true { return 8443; }
        if block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#north == true && block_state.r#west == true && block_state.r#down == false && block_state.r#east == true && block_state.r#south == true { return 8381; }
        if block_state.r#down == false && block_state.r#waterlogged == true && block_state.r#south == false && block_state.r#north == true && block_state.r#west == true && block_state.r#up == true && block_state.r#east == false { return 8421; }
        if block_state.r#up == false && block_state.r#east == true && block_state.r#waterlogged == true && block_state.r#north == true && block_state.r#west == true && block_state.r#south == false && block_state.r#down == false { return 8393; }
        if block_state.r#up == false && block_state.r#north == true && block_state.r#east == true && block_state.r#south == true && block_state.r#waterlogged == false && block_state.r#west == true && block_state.r#down == true { return 8323; }
        if block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == false && block_state.r#south == false && block_state.r#down == true && block_state.r#east == true && block_state.r#north == false { return 8344; }
        if block_state.r#east == true && block_state.r#down == true && block_state.r#waterlogged == true && block_state.r#west == false && block_state.r#up == false && block_state.r#north == true && block_state.r#south == true { return 8322; }
        if block_state.r#waterlogged == true && block_state.r#west == false && block_state.r#north == true && block_state.r#down == true && block_state.r#east == false && block_state.r#south == true && block_state.r#up == true { return 8350; }
        if block_state.r#up == false && block_state.r#south == true && block_state.r#west == false && block_state.r#waterlogged == false && block_state.r#north == false && block_state.r#east == true && block_state.r#down == false { return 8404; }
        if block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == true && block_state.r#down == true && block_state.r#east == false && block_state.r#north == true && block_state.r#south == false { return 8361; }
        if block_state.r#north == false && block_state.r#south == true && block_state.r#up == true && block_state.r#west == false && block_state.r#waterlogged == false && block_state.r#down == false && block_state.r#east == true { return 8400; }
        if block_state.r#south == false && block_state.r#east == false && block_state.r#north == false && block_state.r#up == true && block_state.r#west == false && block_state.r#waterlogged == true && block_state.r#down == false { return 8438; }
        if block_state.r#south == true && block_state.r#north == true && block_state.r#west == false && block_state.r#east == true && block_state.r#waterlogged == true && block_state.r#down == false && block_state.r#up == true { return 8382; }
        if block_state.r#down == true && block_state.r#west == false && block_state.r#up == true && block_state.r#north == false && block_state.r#south == true && block_state.r#waterlogged == false && block_state.r#east == false { return 8368; }
        if block_state.r#south == true && block_state.r#down == true && block_state.r#north == true && block_state.r#east == false && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == false { return 8352; }
        if block_state.r#down == false && block_state.r#east == true && block_state.r#north == true && block_state.r#south == false && block_state.r#waterlogged == true && block_state.r#west == true && block_state.r#up == true { return 8389; }
        if block_state.r#south == false && block_state.r#east == false && block_state.r#north == true && block_state.r#west == false && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#down == true { return 8360; }
        if block_state.r#west == false && block_state.r#south == false && block_state.r#up == true && block_state.r#down == true && block_state.r#east == false && block_state.r#north == true && block_state.r#waterlogged == true { return 8358; }
        if block_state.r#east == false && block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#south == false && block_state.r#west == false && block_state.r#north == true && block_state.r#down == true { return 8364; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 8423 {
            return Some(ResinClump {
                r#down: false,
                r#east: false,
                r#south: false,
                r#up: true,
                r#waterlogged: false,
                r#west: true,
                r#north: true,
            });
        }
        if state_id == 8431 {
            return Some(ResinClump {
                r#south: true,
                r#north: false,
                r#down: false,
                r#up: true,
                r#waterlogged: false,
                r#west: true,
                r#east: false,
            });
        }
        if state_id == 8427 {
            return Some(ResinClump {
                r#down: false,
                r#south: false,
                r#north: true,
                r#east: false,
                r#up: false,
                r#waterlogged: false,
                r#west: true,
            });
        }
        if state_id == 8416 {
            return Some(ResinClump {
                r#west: false,
                r#north: true,
                r#east: false,
                r#up: true,
                r#down: false,
                r#south: true,
                r#waterlogged: false,
            });
        }
        if state_id == 8343 {
            return Some(ResinClump {
                r#down: true,
                r#south: false,
                r#waterlogged: false,
                r#north: false,
                r#east: true,
                r#west: true,
                r#up: true,
            });
        }
        if state_id == 8321 {
            return Some(ResinClump {
                r#down: true,
                r#north: true,
                r#waterlogged: true,
                r#west: true,
                r#east: true,
                r#south: true,
                r#up: false,
            });
        }
        if state_id == 8330 {
            return Some(ResinClump {
                r#up: false,
                r#waterlogged: true,
                r#down: true,
                r#east: true,
                r#north: true,
                r#south: false,
                r#west: false,
            });
        }
        if state_id == 8440 {
            return Some(ResinClump {
                r#waterlogged: false,
                r#west: false,
                r#down: false,
                r#south: false,
                r#up: true,
                r#east: false,
                r#north: false,
            });
        }
        if state_id == 8420 {
            return Some(ResinClump {
                r#down: false,
                r#west: false,
                r#east: false,
                r#up: false,
                r#waterlogged: false,
                r#north: true,
                r#south: true,
            });
        }
        if state_id == 8326 {
            return Some(ResinClump {
                r#east: true,
                r#up: true,
                r#north: true,
                r#down: true,
                r#south: false,
                r#waterlogged: true,
                r#west: false,
            });
        }
        if state_id == 8395 {
            return Some(ResinClump {
                r#north: true,
                r#up: false,
                r#east: true,
                r#down: false,
                r#waterlogged: false,
                r#south: false,
                r#west: true,
            });
        }
        if state_id == 8444 {
            return Some(ResinClump {
                r#south: false,
                r#up: false,
                r#waterlogged: false,
                r#west: false,
                r#down: false,
                r#east: false,
                r#north: false,
            });
        }
        if state_id == 8335 {
            return Some(ResinClump {
                r#east: true,
                r#down: true,
                r#north: false,
                r#up: true,
                r#waterlogged: false,
                r#west: true,
                r#south: true,
            });
        }
        if state_id == 8399 {
            return Some(ResinClump {
                r#east: true,
                r#waterlogged: false,
                r#west: true,
                r#south: true,
                r#north: false,
                r#up: true,
                r#down: false,
            });
        }
        if state_id == 8354 {
            return Some(ResinClump {
                r#up: false,
                r#west: false,
                r#north: true,
                r#waterlogged: true,
                r#down: true,
                r#east: false,
                r#south: true,
            });
        }
        if state_id == 8385 {
            return Some(ResinClump {
                r#east: true,
                r#down: false,
                r#south: true,
                r#up: false,
                r#waterlogged: true,
                r#west: true,
                r#north: true,
            });
        }
        if state_id == 8318 {
            return Some(ResinClump {
                r#waterlogged: true,
                r#down: true,
                r#south: true,
                r#east: true,
                r#west: false,
                r#up: true,
                r#north: true,
            });
        }
        if state_id == 8433 {
            return Some(ResinClump {
                r#waterlogged: true,
                r#down: false,
                r#south: true,
                r#north: false,
                r#east: false,
                r#up: false,
                r#west: true,
            });
        }
        if state_id == 8320 {
            return Some(ResinClump {
                r#north: true,
                r#south: true,
                r#west: false,
                r#up: true,
                r#waterlogged: false,
                r#east: true,
                r#down: true,
            });
        }
        if state_id == 8422 {
            return Some(ResinClump {
                r#down: false,
                r#waterlogged: true,
                r#south: false,
                r#east: false,
                r#west: false,
                r#up: true,
                r#north: true,
            });
        }
        if state_id == 8374 {
            return Some(ResinClump {
                r#up: true,
                r#down: true,
                r#waterlogged: true,
                r#west: false,
                r#south: false,
                r#east: false,
                r#north: false,
            });
        }
        if state_id == 8396 {
            return Some(ResinClump {
                r#east: true,
                r#up: false,
                r#west: false,
                r#down: false,
                r#south: false,
                r#waterlogged: false,
                r#north: true,
            });
        }
        if state_id == 8405 {
            return Some(ResinClump {
                r#waterlogged: true,
                r#down: false,
                r#north: false,
                r#up: true,
                r#east: true,
                r#south: false,
                r#west: true,
            });
        }
        if state_id == 8415 {
            return Some(ResinClump {
                r#down: false,
                r#up: true,
                r#west: true,
                r#south: true,
                r#waterlogged: false,
                r#north: true,
                r#east: false,
            });
        }
        if state_id == 8419 {
            return Some(ResinClump {
                r#west: true,
                r#up: false,
                r#down: false,
                r#east: false,
                r#north: true,
                r#south: true,
                r#waterlogged: false,
            });
        }
        if state_id == 8403 {
            return Some(ResinClump {
                r#up: false,
                r#north: false,
                r#waterlogged: false,
                r#east: true,
                r#west: true,
                r#down: false,
                r#south: true,
            });
        }
        if state_id == 8428 {
            return Some(ResinClump {
                r#south: false,
                r#west: false,
                r#down: false,
                r#east: false,
                r#up: false,
                r#north: true,
                r#waterlogged: false,
            });
        }
        if state_id == 8348 {
            return Some(ResinClump {
                r#down: true,
                r#east: true,
                r#south: false,
                r#up: false,
                r#waterlogged: false,
                r#west: false,
                r#north: false,
            });
        }
        if state_id == 8328 {
            return Some(ResinClump {
                r#up: true,
                r#waterlogged: false,
                r#north: true,
                r#south: false,
                r#west: false,
                r#down: true,
                r#east: true,
            });
        }
        if state_id == 8337 {
            return Some(ResinClump {
                r#east: true,
                r#north: false,
                r#south: true,
                r#down: true,
                r#west: true,
                r#waterlogged: true,
                r#up: false,
            });
        }
        if state_id == 8342 {
            return Some(ResinClump {
                r#south: false,
                r#west: false,
                r#down: true,
                r#up: true,
                r#waterlogged: true,
                r#east: true,
                r#north: false,
            });
        }
        if state_id == 8324 {
            return Some(ResinClump {
                r#south: true,
                r#waterlogged: false,
                r#east: true,
                r#up: false,
                r#west: false,
                r#north: true,
                r#down: true,
            });
        }
        if state_id == 8441 {
            return Some(ResinClump {
                r#west: true,
                r#waterlogged: true,
                r#down: false,
                r#south: false,
                r#north: false,
                r#up: false,
                r#east: false,
            });
        }
        if state_id == 8332 {
            return Some(ResinClump {
                r#east: true,
                r#south: false,
                r#down: true,
                r#west: false,
                r#waterlogged: false,
                r#up: false,
                r#north: true,
            });
        }
        if state_id == 8408 {
            return Some(ResinClump {
                r#east: true,
                r#waterlogged: false,
                r#west: false,
                r#down: false,
                r#north: false,
                r#south: false,
                r#up: true,
            });
        }
        if state_id == 8345 {
            return Some(ResinClump {
                r#west: true,
                r#down: true,
                r#up: false,
                r#north: false,
                r#south: false,
                r#waterlogged: true,
                r#east: true,
            });
        }
        if state_id == 8391 {
            return Some(ResinClump {
                r#south: false,
                r#up: true,
                r#waterlogged: false,
                r#north: true,
                r#west: true,
                r#down: false,
                r#east: true,
            });
        }
        if state_id == 8336 {
            return Some(ResinClump {
                r#north: false,
                r#south: true,
                r#waterlogged: false,
                r#east: true,
                r#up: true,
                r#west: false,
                r#down: true,
            });
        }
        if state_id == 8409 {
            return Some(ResinClump {
                r#east: true,
                r#south: false,
                r#down: false,
                r#north: false,
                r#up: false,
                r#waterlogged: true,
                r#west: true,
            });
        }
        if state_id == 8366 {
            return Some(ResinClump {
                r#east: false,
                r#south: true,
                r#north: false,
                r#up: true,
                r#waterlogged: true,
                r#west: false,
                r#down: true,
            });
        }
        if state_id == 8347 {
            return Some(ResinClump {
                r#up: false,
                r#west: true,
                r#east: true,
                r#waterlogged: false,
                r#north: false,
                r#down: true,
                r#south: false,
            });
        }
        if state_id == 8372 {
            return Some(ResinClump {
                r#up: false,
                r#north: false,
                r#waterlogged: false,
                r#east: false,
                r#south: true,
                r#west: false,
                r#down: true,
            });
        }
        if state_id == 8397 {
            return Some(ResinClump {
                r#west: true,
                r#north: false,
                r#down: false,
                r#waterlogged: true,
                r#east: true,
                r#up: true,
                r#south: true,
            });
        }
        if state_id == 8363 {
            return Some(ResinClump {
                r#waterlogged: false,
                r#east: false,
                r#west: true,
                r#down: true,
                r#south: false,
                r#up: false,
                r#north: true,
            });
        }
        if state_id == 8417 {
            return Some(ResinClump {
                r#waterlogged: true,
                r#down: false,
                r#east: false,
                r#west: true,
                r#north: true,
                r#up: false,
                r#south: true,
            });
        }
        if state_id == 8418 {
            return Some(ResinClump {
                r#down: false,
                r#waterlogged: true,
                r#west: false,
                r#south: true,
                r#east: false,
                r#north: true,
                r#up: false,
            });
        }
        if state_id == 8353 {
            return Some(ResinClump {
                r#up: false,
                r#down: true,
                r#west: true,
                r#north: true,
                r#east: false,
                r#south: true,
                r#waterlogged: true,
            });
        }
        if state_id == 8439 {
            return Some(ResinClump {
                r#up: true,
                r#waterlogged: false,
                r#west: true,
                r#down: false,
                r#east: false,
                r#north: false,
                r#south: false,
            });
        }
        if state_id == 8386 {
            return Some(ResinClump {
                r#west: false,
                r#waterlogged: true,
                r#south: true,
                r#down: false,
                r#up: false,
                r#north: true,
                r#east: true,
            });
        }
        if state_id == 8430 {
            return Some(ResinClump {
                r#waterlogged: true,
                r#south: true,
                r#up: true,
                r#west: false,
                r#north: false,
                r#down: false,
                r#east: false,
            });
        }
        if state_id == 8369 {
            return Some(ResinClump {
                r#east: false,
                r#north: false,
                r#waterlogged: true,
                r#west: true,
                r#up: false,
                r#south: true,
                r#down: true,
            });
        }
        if state_id == 8406 {
            return Some(ResinClump {
                r#west: false,
                r#up: true,
                r#south: false,
                r#down: false,
                r#north: false,
                r#east: true,
                r#waterlogged: true,
            });
        }
        if state_id == 8429 {
            return Some(ResinClump {
                r#down: false,
                r#east: false,
                r#up: true,
                r#waterlogged: true,
                r#west: true,
                r#south: true,
                r#north: false,
            });
        }
        if state_id == 8378 {
            return Some(ResinClump {
                r#east: false,
                r#down: true,
                r#north: false,
                r#waterlogged: true,
                r#west: false,
                r#south: false,
                r#up: false,
            });
        }
        if state_id == 8432 {
            return Some(ResinClump {
                r#west: false,
                r#south: true,
                r#east: false,
                r#waterlogged: false,
                r#north: false,
                r#down: false,
                r#up: true,
            });
        }
        if state_id == 8424 {
            return Some(ResinClump {
                r#up: true,
                r#west: false,
                r#east: false,
                r#south: false,
                r#waterlogged: false,
                r#down: false,
                r#north: true,
            });
        }
        if state_id == 8346 {
            return Some(ResinClump {
                r#down: true,
                r#east: true,
                r#waterlogged: true,
                r#west: false,
                r#south: false,
                r#up: false,
                r#north: false,
            });
        }
        if state_id == 8333 {
            return Some(ResinClump {
                r#up: true,
                r#west: true,
                r#east: true,
                r#waterlogged: true,
                r#down: true,
                r#north: false,
                r#south: true,
            });
        }
        if state_id == 8356 {
            return Some(ResinClump {
                r#north: true,
                r#south: true,
                r#waterlogged: false,
                r#down: true,
                r#up: false,
                r#west: false,
                r#east: false,
            });
        }
        if state_id == 8373 {
            return Some(ResinClump {
                r#waterlogged: true,
                r#up: true,
                r#down: true,
                r#north: false,
                r#east: false,
                r#west: true,
                r#south: false,
            });
        }
        if state_id == 8376 {
            return Some(ResinClump {
                r#down: true,
                r#east: false,
                r#waterlogged: false,
                r#up: true,
                r#north: false,
                r#south: false,
                r#west: false,
            });
        }
        if state_id == 8329 {
            return Some(ResinClump {
                r#down: true,
                r#north: true,
                r#west: true,
                r#up: false,
                r#east: true,
                r#south: false,
                r#waterlogged: true,
            });
        }
        if state_id == 8394 {
            return Some(ResinClump {
                r#north: true,
                r#up: false,
                r#waterlogged: true,
                r#down: false,
                r#west: false,
                r#south: false,
                r#east: true,
            });
        }
        if state_id == 8375 {
            return Some(ResinClump {
                r#east: false,
                r#south: false,
                r#waterlogged: false,
                r#west: true,
                r#up: true,
                r#down: true,
                r#north: false,
            });
        }
        if state_id == 8401 {
            return Some(ResinClump {
                r#west: true,
                r#down: false,
                r#east: true,
                r#north: false,
                r#waterlogged: true,
                r#up: false,
                r#south: true,
            });
        }
        if state_id == 8436 {
            return Some(ResinClump {
                r#north: false,
                r#south: true,
                r#up: false,
                r#down: false,
                r#waterlogged: false,
                r#west: false,
                r#east: false,
            });
        }
        if state_id == 8387 {
            return Some(ResinClump {
                r#up: false,
                r#west: true,
                r#north: true,
                r#south: true,
                r#down: false,
                r#waterlogged: false,
                r#east: true,
            });
        }
        if state_id == 8384 {
            return Some(ResinClump {
                r#up: true,
                r#north: true,
                r#west: false,
                r#waterlogged: false,
                r#down: false,
                r#east: true,
                r#south: true,
            });
        }
        if state_id == 8367 {
            return Some(ResinClump {
                r#south: true,
                r#east: false,
                r#north: false,
                r#up: true,
                r#waterlogged: false,
                r#west: true,
                r#down: true,
            });
        }
        if state_id == 8338 {
            return Some(ResinClump {
                r#down: true,
                r#up: false,
                r#waterlogged: true,
                r#west: false,
                r#south: true,
                r#east: true,
                r#north: false,
            });
        }
        if state_id == 8380 {
            return Some(ResinClump {
                r#north: false,
                r#up: false,
                r#waterlogged: false,
                r#down: true,
                r#south: false,
                r#west: false,
                r#east: false,
            });
        }
        if state_id == 8355 {
            return Some(ResinClump {
                r#down: true,
                r#south: true,
                r#up: false,
                r#north: true,
                r#waterlogged: false,
                r#west: true,
                r#east: false,
            });
        }
        if state_id == 8388 {
            return Some(ResinClump {
                r#waterlogged: false,
                r#east: true,
                r#north: true,
                r#up: false,
                r#down: false,
                r#south: true,
                r#west: false,
            });
        }
        if state_id == 8413 {
            return Some(ResinClump {
                r#east: false,
                r#waterlogged: true,
                r#south: true,
                r#up: true,
                r#west: true,
                r#down: false,
                r#north: true,
            });
        }
        if state_id == 8442 {
            return Some(ResinClump {
                r#waterlogged: true,
                r#north: false,
                r#down: false,
                r#east: false,
                r#south: false,
                r#up: false,
                r#west: false,
            });
        }
        if state_id == 8410 {
            return Some(ResinClump {
                r#up: false,
                r#north: false,
                r#waterlogged: true,
                r#west: false,
                r#east: true,
                r#south: false,
                r#down: false,
            });
        }
        if state_id == 8339 {
            return Some(ResinClump {
                r#down: true,
                r#north: false,
                r#up: false,
                r#south: true,
                r#east: true,
                r#waterlogged: false,
                r#west: true,
            });
        }
        if state_id == 8383 {
            return Some(ResinClump {
                r#south: true,
                r#down: false,
                r#up: true,
                r#waterlogged: false,
                r#west: true,
                r#north: true,
                r#east: true,
            });
        }
        if state_id == 8434 {
            return Some(ResinClump {
                r#east: false,
                r#west: false,
                r#down: false,
                r#up: false,
                r#south: true,
                r#waterlogged: true,
                r#north: false,
            });
        }
        if state_id == 8412 {
            return Some(ResinClump {
                r#up: false,
                r#waterlogged: false,
                r#west: false,
                r#east: true,
                r#down: false,
                r#north: false,
                r#south: false,
            });
        }
        if state_id == 8426 {
            return Some(ResinClump {
                r#north: true,
                r#south: false,
                r#waterlogged: true,
                r#east: false,
                r#up: false,
                r#down: false,
                r#west: false,
            });
        }
        if state_id == 8411 {
            return Some(ResinClump {
                r#waterlogged: false,
                r#down: false,
                r#south: false,
                r#west: true,
                r#east: true,
                r#north: false,
                r#up: false,
            });
        }
        if state_id == 8317 {
            return Some(ResinClump {
                r#south: true,
                r#waterlogged: true,
                r#down: true,
                r#north: true,
                r#west: true,
                r#east: true,
                r#up: true,
            });
        }
        if state_id == 8340 {
            return Some(ResinClump {
                r#up: false,
                r#west: false,
                r#east: true,
                r#north: false,
                r#south: true,
                r#down: true,
                r#waterlogged: false,
            });
        }
        if state_id == 8319 {
            return Some(ResinClump {
                r#up: true,
                r#down: true,
                r#north: true,
                r#waterlogged: false,
                r#south: true,
                r#west: true,
                r#east: true,
            });
        }
        if state_id == 8327 {
            return Some(ResinClump {
                r#waterlogged: false,
                r#up: true,
                r#west: true,
                r#north: true,
                r#east: true,
                r#south: false,
                r#down: true,
            });
        }
        if state_id == 8365 {
            return Some(ResinClump {
                r#waterlogged: true,
                r#east: false,
                r#down: true,
                r#up: true,
                r#west: true,
                r#north: false,
                r#south: true,
            });
        }
        if state_id == 8371 {
            return Some(ResinClump {
                r#waterlogged: false,
                r#west: true,
                r#north: false,
                r#down: true,
                r#east: false,
                r#south: true,
                r#up: false,
            });
        }
        if state_id == 8398 {
            return Some(ResinClump {
                r#west: false,
                r#south: true,
                r#north: false,
                r#down: false,
                r#east: true,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 8414 {
            return Some(ResinClump {
                r#south: true,
                r#up: true,
                r#waterlogged: true,
                r#west: false,
                r#down: false,
                r#east: false,
                r#north: true,
            });
        }
        if state_id == 8331 {
            return Some(ResinClump {
                r#east: true,
                r#south: false,
                r#down: true,
                r#north: true,
                r#up: false,
                r#waterlogged: false,
                r#west: true,
            });
        }
        if state_id == 8341 {
            return Some(ResinClump {
                r#up: true,
                r#waterlogged: true,
                r#west: true,
                r#north: false,
                r#down: true,
                r#east: true,
                r#south: false,
            });
        }
        if state_id == 8349 {
            return Some(ResinClump {
                r#waterlogged: true,
                r#west: true,
                r#east: false,
                r#south: true,
                r#down: true,
                r#north: true,
                r#up: true,
            });
        }
        if state_id == 8325 {
            return Some(ResinClump {
                r#waterlogged: true,
                r#south: false,
                r#east: true,
                r#west: true,
                r#down: true,
                r#north: true,
                r#up: true,
            });
        }
        if state_id == 8357 {
            return Some(ResinClump {
                r#south: false,
                r#north: true,
                r#waterlogged: true,
                r#west: true,
                r#up: true,
                r#down: true,
                r#east: false,
            });
        }
        if state_id == 8392 {
            return Some(ResinClump {
                r#north: true,
                r#waterlogged: false,
                r#east: true,
                r#up: true,
                r#west: false,
                r#south: false,
                r#down: false,
            });
        }
        if state_id == 8334 {
            return Some(ResinClump {
                r#down: true,
                r#north: false,
                r#south: true,
                r#up: true,
                r#west: false,
                r#east: true,
                r#waterlogged: true,
            });
        }
        if state_id == 8362 {
            return Some(ResinClump {
                r#north: true,
                r#east: false,
                r#up: false,
                r#down: true,
                r#west: false,
                r#south: false,
                r#waterlogged: true,
            });
        }
        if state_id == 8402 {
            return Some(ResinClump {
                r#south: true,
                r#waterlogged: true,
                r#down: false,
                r#west: false,
                r#east: true,
                r#north: false,
                r#up: false,
            });
        }
        if state_id == 8407 {
            return Some(ResinClump {
                r#down: false,
                r#up: true,
                r#east: true,
                r#south: false,
                r#waterlogged: false,
                r#west: true,
                r#north: false,
            });
        }
        if state_id == 8379 {
            return Some(ResinClump {
                r#west: true,
                r#east: false,
                r#up: false,
                r#waterlogged: false,
                r#north: false,
                r#down: true,
                r#south: false,
            });
        }
        if state_id == 8390 {
            return Some(ResinClump {
                r#west: false,
                r#down: false,
                r#east: true,
                r#south: false,
                r#north: true,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 8435 {
            return Some(ResinClump {
                r#north: false,
                r#south: true,
                r#up: false,
                r#west: true,
                r#waterlogged: false,
                r#down: false,
                r#east: false,
            });
        }
        if state_id == 8425 {
            return Some(ResinClump {
                r#waterlogged: true,
                r#south: false,
                r#west: true,
                r#east: false,
                r#north: true,
                r#up: false,
                r#down: false,
            });
        }
        if state_id == 8377 {
            return Some(ResinClump {
                r#east: false,
                r#waterlogged: true,
                r#down: true,
                r#up: false,
                r#south: false,
                r#north: false,
                r#west: true,
            });
        }
        if state_id == 8370 {
            return Some(ResinClump {
                r#east: false,
                r#north: false,
                r#south: true,
                r#up: false,
                r#down: true,
                r#waterlogged: true,
                r#west: false,
            });
        }
        if state_id == 8359 {
            return Some(ResinClump {
                r#waterlogged: false,
                r#down: true,
                r#east: false,
                r#north: true,
                r#up: true,
                r#west: true,
                r#south: false,
            });
        }
        if state_id == 8351 {
            return Some(ResinClump {
                r#south: true,
                r#down: true,
                r#east: false,
                r#waterlogged: false,
                r#up: true,
                r#west: true,
                r#north: true,
            });
        }
        if state_id == 8437 {
            return Some(ResinClump {
                r#east: false,
                r#north: false,
                r#up: true,
                r#waterlogged: true,
                r#west: true,
                r#down: false,
                r#south: false,
            });
        }
        if state_id == 8443 {
            return Some(ResinClump {
                r#south: false,
                r#east: false,
                r#down: false,
                r#north: false,
                r#waterlogged: false,
                r#up: false,
                r#west: true,
            });
        }
        if state_id == 8381 {
            return Some(ResinClump {
                r#up: true,
                r#waterlogged: true,
                r#north: true,
                r#west: true,
                r#down: false,
                r#east: true,
                r#south: true,
            });
        }
        if state_id == 8421 {
            return Some(ResinClump {
                r#down: false,
                r#waterlogged: true,
                r#south: false,
                r#north: true,
                r#west: true,
                r#up: true,
                r#east: false,
            });
        }
        if state_id == 8393 {
            return Some(ResinClump {
                r#up: false,
                r#east: true,
                r#waterlogged: true,
                r#north: true,
                r#west: true,
                r#south: false,
                r#down: false,
            });
        }
        if state_id == 8323 {
            return Some(ResinClump {
                r#up: false,
                r#north: true,
                r#east: true,
                r#south: true,
                r#waterlogged: false,
                r#west: true,
                r#down: true,
            });
        }
        if state_id == 8344 {
            return Some(ResinClump {
                r#up: true,
                r#waterlogged: false,
                r#west: false,
                r#south: false,
                r#down: true,
                r#east: true,
                r#north: false,
            });
        }
        if state_id == 8322 {
            return Some(ResinClump {
                r#east: true,
                r#down: true,
                r#waterlogged: true,
                r#west: false,
                r#up: false,
                r#north: true,
                r#south: true,
            });
        }
        if state_id == 8350 {
            return Some(ResinClump {
                r#waterlogged: true,
                r#west: false,
                r#north: true,
                r#down: true,
                r#east: false,
                r#south: true,
                r#up: true,
            });
        }
        if state_id == 8404 {
            return Some(ResinClump {
                r#up: false,
                r#south: true,
                r#west: false,
                r#waterlogged: false,
                r#north: false,
                r#east: true,
                r#down: false,
            });
        }
        if state_id == 8361 {
            return Some(ResinClump {
                r#up: false,
                r#waterlogged: true,
                r#west: true,
                r#down: true,
                r#east: false,
                r#north: true,
                r#south: false,
            });
        }
        if state_id == 8400 {
            return Some(ResinClump {
                r#north: false,
                r#south: true,
                r#up: true,
                r#west: false,
                r#waterlogged: false,
                r#down: false,
                r#east: true,
            });
        }
        if state_id == 8438 {
            return Some(ResinClump {
                r#south: false,
                r#east: false,
                r#north: false,
                r#up: true,
                r#west: false,
                r#waterlogged: true,
                r#down: false,
            });
        }
        if state_id == 8382 {
            return Some(ResinClump {
                r#south: true,
                r#north: true,
                r#west: false,
                r#east: true,
                r#waterlogged: true,
                r#down: false,
                r#up: true,
            });
        }
        if state_id == 8368 {
            return Some(ResinClump {
                r#down: true,
                r#west: false,
                r#up: true,
                r#north: false,
                r#south: true,
                r#waterlogged: false,
                r#east: false,
            });
        }
        if state_id == 8352 {
            return Some(ResinClump {
                r#south: true,
                r#down: true,
                r#north: true,
                r#east: false,
                r#up: true,
                r#waterlogged: false,
                r#west: false,
            });
        }
        if state_id == 8389 {
            return Some(ResinClump {
                r#down: false,
                r#east: true,
                r#north: true,
                r#south: false,
                r#waterlogged: true,
                r#west: true,
                r#up: true,
            });
        }
        if state_id == 8360 {
            return Some(ResinClump {
                r#south: false,
                r#east: false,
                r#north: true,
                r#west: false,
                r#up: true,
                r#waterlogged: false,
                r#down: true,
            });
        }
        if state_id == 8358 {
            return Some(ResinClump {
                r#west: false,
                r#south: false,
                r#up: true,
                r#down: true,
                r#east: false,
                r#north: true,
                r#waterlogged: true,
            });
        }
        if state_id == 8364 {
            return Some(ResinClump {
                r#east: false,
                r#waterlogged: false,
                r#up: false,
                r#south: false,
                r#west: false,
                r#north: true,
                r#down: true,
            });
        }
        return None;
    }
}

