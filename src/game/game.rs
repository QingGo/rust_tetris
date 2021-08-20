use super::block::BlockI;

use super::color::ColorRaw;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::settings::settings::*;

pub struct Game {
    blocks: Vec<Vec<bool>>,
    blocks_color: Vec<Vec<ColorRaw>>,
    float_block: BlockI,
}


impl Game {
    pub fn new() -> Game {
        Game {
            blocks: vec![vec![false; WIDTH_BLOCKS_COUNT as usize]; HEIGHT_BLOCKS_COUNT as usize],
            blocks_color: vec![
                vec![ColorRaw::WITHE; WIDTH_BLOCKS_COUNT as usize];
                HEIGHT_BLOCKS_COUNT as usize
            ],
            float_block: BlockI::new(),
        }
    }
    pub fn reveive_key(&mut self, event: Event) {
        match event {
            Event::KeyDown {
                keycode: Some(Keycode::Up),
                ..
            } => self.float_block.rotate(&self.blocks),
            Event::KeyDown {
                keycode: Some(Keycode::Down),
                ..
            } => self.float_block.quick_down(),
            Event::KeyDown {
                keycode: Some(Keycode::Left),
                ..
            } => self.float_block.move_left(&self.blocks),
            Event::KeyDown {
                keycode: Some(Keycode::Right),
                ..
            } => self.float_block.move_right(&self.blocks),
            _ => {}
        }
    }
    pub fn update(&mut self) {
        let is_hit_ground = self.float_block.update(&self.blocks);
        if is_hit_ground {
            for block in self.float_block.get_blocks() {
                self.blocks[block.1 as usize][block.0 as usize] = true;
                self.blocks_color[block.1 as usize][block.0 as usize] = self.float_block.get_color();
            }
            self.float_block = BlockI::new();
            // 消除逻辑
            self.clear_lines();

        }
    }

    pub fn render(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        for (y, row) in self.blocks.iter().enumerate() {
            for (x, is_exist) in row.iter().enumerate() {
                if *is_exist {
                    self.blocks_color[y][x].set_canvas(canvas);
                    canvas.fill_rect(Rect::new(
                        BLOCK_SIZE as i32 * x as i32,
                        BLOCK_SIZE as i32 * y as i32,
                        BLOCK_SIZE,
                        BLOCK_SIZE,
                    ))?;
                }
            }
        }
        self.float_block.render(canvas)?;
        Ok(())
    }

    fn clear_lines(&mut self){
        let mut line_nums: Vec<usize> = self.blocks.iter().enumerate().filter(|&(_, row)| !row.iter().any(|exist| !exist)).map(|(i, _)| i).collect();
        // index 从高到低删除
        line_nums.reverse();
        for line_num in line_nums.iter(){
            self.blocks.remove(*line_num);
            self.blocks_color.remove(*line_num);
        }
        // 在零轴填充空白
        for _ in line_nums{
            self.blocks.insert(0, vec![false; WIDTH_BLOCKS_COUNT as usize]);
            self.blocks_color.insert(0, vec![ColorRaw::WITHE; WIDTH_BLOCKS_COUNT as usize]);
        }
    }
}
