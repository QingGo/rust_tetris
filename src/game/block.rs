pub enum BlockType {
    BlockI,
    BlockO,
}

pub struct Color([u8; 4]);

impl Color {
    pub const BLACK: Color = Color([0, 0, 0, 0xFF]);
    pub const BLUE: Color = Color([48, 199, 239, 0xFF]);
}

pub trait Block {
    fn new() -> Self;
    fn rotate(&mut self);
    fn move_left(&mut self);
    fn move_right(&mut self);
    fn get_blocks(&self) -> Vec<(i32, i32)>;
}

const INIT_LEFT_UP_POS: (i32, i32) = (3, 0);

struct BlockI {
    left_up_pos: (i32, i32),
    raw_block: [[bool; 4]; 4],
    color: Color,
}

fn rotate_4(raw_block: &[[bool; 4]; 4]) -> [[bool; 4]; 4] {
    let mut new_raw_block = [[false; 4]; 4];
    for (y, row) in raw_block.iter().enumerate() {
        for (x, is_exist) in row.iter().enumerate() {
            if *is_exist {
                // [0, 0] -> [3, 0]
                // [0, 1] -> [1, 0]
                // [1, 0] -> [3, 1]
                new_raw_block[3 - y][x] = true;
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

impl Block for BlockI {
    fn new() -> BlockI {
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
            color: Color::BLUE,
        }
    }

    fn rotate(&mut self) {
        let new_raw_block = rotate_4(&self.raw_block);
        self.raw_block = new_raw_block;
    }
    fn move_left(&mut self) {}
    fn move_right(&mut self) {}
    fn get_blocks(&self) -> Vec<(i32, i32)> {
        get_blocks(&self.raw_block)
    }
}
