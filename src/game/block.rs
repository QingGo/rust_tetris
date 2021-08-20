use sdl2::pixels::Color;

pub struct ColorRaw([u8; 4]);

impl ColorRaw {
    // pub const BLACK: ColorRaw = ColorRaw([0, 0, 0, 0xFF]);
    pub const BLUE: ColorRaw = ColorRaw([48, 199, 239, 0xFF]);
}

pub trait Block {
    fn new() -> Self;
    fn rotate(&mut self);
    fn move_left(&mut self);
    fn move_right(&mut self);
    fn get_blocks(&self) -> Vec<(i32, i32)>;
}

const INIT_LEFT_UP_POS: (i32, i32) = (3, 0);

pub struct BlockI {
    left_up_pos: (i32, i32),
    raw_block: [[bool; 4]; 4],
    color: ColorRaw,
    fall_distance_per_tick: u32,
    total_fall_distance: u32,
}

fn rotate_4(raw_block: &[[bool; 4]; 4]) -> [[bool; 4]; 4] {
    let mut new_raw_block = [[false; 4]; 4];
    for (y, row) in raw_block.iter().enumerate() {
        for (x, is_exist) in row.iter().enumerate() {
            if *is_exist {
                // [0, 0] -> [0, 3]
                // [0, 1] -> [1, 3]
                // [1, 0] -> [0, 2]
                // println!("rotate {:?} -> {:?}", (x, y), (y, 3 - x));
                new_raw_block[3 - x][y] = true;
            }
        }
    }
    new_raw_block
}

fn get_blocks(raw_block: &[[bool; 4]; 4]) -> Vec<(i32, i32)> {
    let mut blocks: Vec<(i32, i32)> = Vec::new();
    for (y, row) in raw_block.iter().enumerate() {
        for (x, is_exist) in row.iter().enumerate() {
            if *is_exist {
                blocks.push((x as i32, y as i32));
            }
        }
    }
    blocks
}

impl BlockI {
    pub fn new() -> BlockI {
        let left_up_pos = INIT_LEFT_UP_POS.clone();
        let raw_block = [
            [false, false, false, false],
            [true, true, true, true],
            [false, false, false, false],
            [false, false, false, false],
        ];
        BlockI {
            left_up_pos: left_up_pos,
            raw_block: raw_block,
            color: ColorRaw::BLUE,
            fall_distance_per_tick: 5,
            total_fall_distance: 0,
        }
    }

    pub fn rotate(&mut self) {
        let new_raw_block = rotate_4(&self.raw_block);
        // println!("rotate {:?}", new_raw_block);
        self.raw_block = new_raw_block;
        // println!("rotate {:?}", self.raw_block);
    }
    pub fn move_left(&mut self) {
        self.left_up_pos.0 -= 1
    }
    pub fn move_right(&mut self) {
        self.left_up_pos.0 += 1
    }
    pub fn move_down(&mut self) {
        self.left_up_pos.1 += 1
    }
    pub fn get_blocks(&self) -> Vec<(i32, i32)> {
        // println!("self.left_up_pos {:?}", self.left_up_pos);
        get_blocks(&self.raw_block)
            .iter()
            .map(|x| (x.0 + self.left_up_pos.0, x.1 + self.left_up_pos.1))
            .collect::<Vec<_>>()
    }
    pub fn get_colors(&self) -> Color {
        Color::RGBA(
            self.color.0[0],
            self.color.0[1],
            self.color.0[2],
            self.color.0[3],
        )
    }
}
