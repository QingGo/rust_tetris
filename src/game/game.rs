const WIDTH: u32 = 12;
const HEIGHT: u32 = 23;

pub struct Game {
    blocks: Vec<Vec<bool>>,
    // blocks_color: Vec<Vec<block::Color>>,
    // float_block: block::Block,
}

impl Game {
    pub fn new() -> Game {
        Game{
            blocks: Vec::new(),
        }
    }
    pub fn reveive_key(&self){}
    pub fn update(){}

    fn rotate(&self){}
    fn move_left(&self){}
    fn move_right(&self){}
}