use super::color::ColorRaw;
use crate::settings::settings::*;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

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

fn rotate_4_clockwise(raw_block: &[[bool; 4]; 4]) -> [[bool; 4]; 4] {
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

// fn rotate_4_anticlockwise(raw_block: &[[bool; 4]; 4]) -> [[bool; 4]; 4] {
//     let mut new_raw_block = [[false; 4]; 4];
//     for (y, row) in raw_block.iter().enumerate() {
//         for (x, is_exist) in row.iter().enumerate() {
//             if *is_exist {
//                 // [0, 0] -> [3, 0]
//                 // [0, 1] -> [2, 0]
//                 // [1, 0] -> [3, 1]
//                 // println!("rotate {:?} -> {:?}", (x, y), (y, 3 - x));
//                 new_raw_block[x][3-y] = true;
//             }
//         }
//     }
//     new_raw_block
// }

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
            fall_distance_per_tick: FALL_SPEED_NORMAL,
            total_fall_distance: 0,
        }
    }

    pub fn rotate(&mut self, blocks: &Vec<Vec<bool>>) {
        let new_raw_block = rotate_4_clockwise(&self.raw_block);
        let old_raw_block = self.raw_block;
        self.raw_block = new_raw_block;
        if self.check_collison(blocks){
            self.raw_block = old_raw_block;
        }
    }
    pub fn move_left(&mut self, blocks: &Vec<Vec<bool>>) {
        self.left_up_pos.0 -= 1;
        if self.check_collison(blocks){
            self.left_up_pos.0 += 1;
        }
    }
    pub fn move_right(&mut self, blocks: &Vec<Vec<bool>>) {
        self.left_up_pos.0 += 1;
        if self.check_collison(blocks){
            self.left_up_pos.0 -= 1;
        }
    }
    pub fn quick_down(&mut self) {
        self.fall_distance_per_tick = FALL_SPEED_QUICK;
    }
    pub fn get_blocks(&self) -> Vec<(i32, i32)> {
        // println!("self.left_up_pos {:?}", self.left_up_pos);
        get_blocks(&self.raw_block)
            .iter()
            .map(|x| (x.0 + self.left_up_pos.0, x.1 + self.left_up_pos.1))
            .collect::<Vec<_>>()
    }

    pub fn get_color(&self) -> ColorRaw {
        self.color
    }

    pub fn update(&mut self, blocks: &Vec<Vec<bool>>) -> bool {
        self.total_fall_distance += self.fall_distance_per_tick;
        if self.total_fall_distance >= BLOCK_SIZE {
            self.left_up_pos.1 += 1;
            self.total_fall_distance = 0;
        }
        // 落地並且多于一个周期
        if self.check_collison(blocks) {
            self.left_up_pos.1 -= 1;
            return true;
        }
        false
    }

    pub fn render(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        self.color.set_canvas(canvas);
        for block in self.get_blocks() {
            // println!("block {:?}", block);
            canvas.fill_rect(Rect::new(
                BLOCK_SIZE as i32 * block.0,
                BLOCK_SIZE as i32 * block.1,
                BLOCK_SIZE,
                BLOCK_SIZE,
            ))?;
        }
        Ok(())
    }

    fn check_collison(&self, blocks: &Vec<Vec<bool>>) -> bool {
        let max_height = blocks.len() as i32;
        let max_width = blocks[0].len() as i32;
        for block in self.get_blocks() {
            let (x, y) = block;
            if y > max_height - 1 {
                return true;
            }
            if x < 0 || x > max_width - 1{
                return true;
            }
            if blocks[y as usize][x as usize] {
                return true;
            }
        }
        false
    }
}
