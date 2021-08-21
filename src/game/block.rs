use crate::settings::color::ColorRaw;
use crate::settings::settings::*;
use rand;
use rand::seq::SliceRandom;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

// 不同方块看成是 Block 的不同实例
pub struct Block {
    left_up_pos: (i32, i32),
    // 约定长宽相等
    raw_block: Vec<Vec<bool>>,
    color: ColorRaw,
    fall_distance_per_tick: u32,
    total_fall_distance: u32,
}

pub fn get_random_block() -> Block {
    let blocks_setting: Vec<(&str, (Vec<Vec<bool>>, ColorRaw))> = vec![
        (
            "I",
            (
                vec![
                    vec![false, false, false, false],
                    vec![true, true, true, true],
                    vec![false, false, false, false],
                    vec![false, false, false, false],
                ],
                ColorRaw::CYAN,
            ),
        ),
        (
            "J",
            (
                vec![
                    vec![true, false, false],
                    vec![true, true, true],
                    vec![false, false, false],
                ],
                ColorRaw::BLUE,
            ),
        ),
        (
            "L",
            (
                vec![
                    vec![false, false, true],
                    vec![true, true, true],
                    vec![false, false, false],
                ],
                ColorRaw::ORANGE,
            ),
        ),
        (
            "O",
            (
                vec![
                    vec![false, false, false, false],
                    vec![false, true, true, false],
                    vec![false, true, true, false],
                    vec![false, false, false, false],
                ],
                ColorRaw::YELLOW,
            ),
        ),
        (
            "S",
            (
                vec![
                    vec![false, true, true],
                    vec![true, true, false],
                    vec![false, false, false],
                ],
                ColorRaw::GREEN,
            ),
        ),
        (
            "T",
            (
                vec![
                    vec![false, true, false],
                    vec![true, true, true],
                    vec![false, false, false],
                ],
                ColorRaw::PURPLE,
            ),
        ),
        (
            "Z",
            (
                vec![
                    vec![true, true, false],
                    vec![false, true, true],
                    vec![false, false, false],
                ],
                ColorRaw::RED,
            ),
        ),
    ];
    let (_, setting) = blocks_setting.choose(&mut rand::thread_rng()).unwrap();
    let (raw_block, color) = setting.clone();
    // println!("blocks {:?}", raw_block);
    Block::new(raw_block, color)
}

impl Block {
    pub fn new(raw_block: Vec<Vec<bool>>, color: ColorRaw) -> Block {
        let left_up_pos = INIT_LEFT_UP_POS.clone();
        Block {
            left_up_pos: left_up_pos,
            raw_block: raw_block,
            color: color,
            fall_distance_per_tick: FALL_SPEED_NORMAL,
            total_fall_distance: 0,
        }
    }

    pub fn rotate(&mut self, blocks: &Vec<Vec<bool>>) {
        // println!("blocks {:?}", blocks);
        let mut new_raw_block = vec![vec![false; self.raw_block.len()]; self.raw_block.len()];
        for (y, row) in self.raw_block.iter().enumerate() {
            for (x, is_exist) in row.iter().enumerate() {
                if *is_exist {
                    // [0, 0] -> [0, 3]
                    // [0, 1] -> [1, 3]
                    // [1, 0] -> [0, 2]
                    // println!("rotate {:?} -> {:?}", (x, y), (y, 3 - x));
                    new_raw_block[self.raw_block.len() - 1 - x][y] = true;
                }
            }
        }
        let old_raw_block = self.raw_block.clone();
        self.raw_block = new_raw_block;
        if self.check_collison(blocks) {
            self.raw_block = old_raw_block;
        }
    }
    pub fn move_left(&mut self, blocks: &Vec<Vec<bool>>) {
        self.left_up_pos.0 -= 1;
        if self.check_collison(blocks) {
            self.left_up_pos.0 += 1;
        }
    }
    pub fn move_right(&mut self, blocks: &Vec<Vec<bool>>) {
        self.left_up_pos.0 += 1;
        if self.check_collison(blocks) {
            self.left_up_pos.0 -= 1;
        }
    }
    pub fn quick_down(&mut self) {
        self.fall_distance_per_tick = FALL_SPEED_QUICK;
    }
    pub fn get_blocks(&self) -> Vec<(i32, i32)> {
        let mut blocks: Vec<(i32, i32)> = Vec::new();
        for (y, row) in self.raw_block.iter().enumerate() {
            for (x, is_exist) in row.iter().enumerate() {
                if *is_exist {
                    blocks.push((x as i32, y as i32));
                }
            }
        }
        blocks
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

    pub fn check_collison(&self, blocks: &Vec<Vec<bool>>) -> bool {
        let max_height = blocks.len() as i32;
        let max_width = blocks[0].len() as i32;
        for block in self.get_blocks() {
            let (x, y) = block;
            if y > max_height - 1 {
                return true;
            }
            if x < 0 || x > max_width - 1 {
                return true;
            }
            if blocks[y as usize][x as usize] {
                return true;
            }
        }
        false
    }
}
