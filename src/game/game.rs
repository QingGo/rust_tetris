use super::block::BlockI;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::Rect;


const BLOCK_SIZE: u32 = 30;
const WIDTH: u32 = 12;
const HEIGHT: u32 = 23;

pub struct Game {
    blocks: Vec<Vec<bool>>,
    // blocks_color: Vec<Vec<block::Color>>,
    float_block: BlockI,
}

impl Game {
    pub fn new() -> Game {
        Game {
            blocks: vec![vec![false; WIDTH as usize]; HEIGHT as usize],
            float_block: BlockI::new(),
        }
    }
    pub fn reveive_key(&mut self, event: Event) {
        match event {
            Event::KeyDown {
                keycode: Some(Keycode::Up),
                ..
            } => self.float_block.rotate(),
            Event::KeyDown {
                keycode: Some(Keycode::Down),
                ..
            } => self.float_block.move_down(),
            Event::KeyDown {
                keycode: Some(Keycode::Left),
                ..
            } => self.float_block.move_left(),
            Event::KeyDown {
                keycode: Some(Keycode::Right),
                ..
            } => self.float_block.move_right(),
            _ => {}
        }
    }
    pub fn update(&self) {}
    pub fn render(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        let color = self.float_block.get_colors();
        // println!("color {:?}", color);
        canvas.set_draw_color(color);
        for block in self.float_block.get_blocks().iter() {
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
}
