use super::block::{get_random_block, Block};

use super::state::GameState;
use crate::settings::color::ColorRaw;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::image::{InitFlag, LoadTexture};
use std::path::Path;

use crate::settings::settings::*;

pub struct Game {
    blocks: Vec<Vec<bool>>,
    blocks_color: Vec<Vec<ColorRaw>>,
    float_block: Block,
    state: GameState,
}

impl Game {
    pub fn new() -> Game {
        Game {
            blocks: vec![vec![false; WIDTH_BLOCKS_COUNT as usize]; HEIGHT_BLOCKS_COUNT as usize],
            blocks_color: vec![
                vec![ColorRaw::WITHE; WIDTH_BLOCKS_COUNT as usize];
                HEIGHT_BLOCKS_COUNT as usize
            ],
            float_block: get_random_block(),
            state: GameState::UNSTART,
        }
    }
    pub fn reveive_event(&mut self, event: Event) {
        match event {
            Event::KeyDown {
                keycode: Some(Keycode::Return),
                ..
            } => match self.state {
                // 开始游戏
                GameState::UNSTART {} => {
                    self.state = GameState::PLAYING;
                    println!("开始游戏");
                }
                _ => {}
            },
            Event::KeyDown {
                keycode: Some(Keycode::R),
                ..
            } => match self.state {
                // 重新开始游戏
                GameState::GAMEOVER {} => {
                    self.state = GameState::PLAYING;
                    self.reset();
                    println!("重新开始游戏");
                }
                _ => {}
            },
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
        match self.state {
            // 开始游戏
            GameState::PLAYING {} => {
                let is_hit_ground = self.float_block.update(&self.blocks);
                if is_hit_ground {
                    for block in self.float_block.get_blocks() {
                        self.blocks[block.1 as usize][block.0 as usize] = true;
                        self.blocks_color[block.1 as usize][block.0 as usize] =
                            self.float_block.get_color();
                    }
                    self.float_block = get_random_block();
                    // 检验游戏失败
                    if self.float_block.check_collison(&self.blocks) {
                        self.state = GameState::GAMEOVER;
                        println!("游戏结束");
                    }
                    // 消除逻辑
                    self.clear_lines();
                }
            }
            _ => {}
        }
    }

    pub fn render(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        match self.state {
            // 开始游戏
            GameState::PLAYING {} => {
                init_game_background(canvas)?;
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
            }
            GameState::UNSTART {} => {
                init_start_interface(canvas)?;
            }
            GameState::GAMEOVER {} => {
                init_gameover_interface(canvas)?;
            }
            _ => {}
        }
        Ok(())
    }

    fn reset(&mut self) {
        self.blocks = vec![vec![false; WIDTH_BLOCKS_COUNT as usize]; HEIGHT_BLOCKS_COUNT as usize];
        self.blocks_color =
            vec![vec![ColorRaw::WITHE; WIDTH_BLOCKS_COUNT as usize]; HEIGHT_BLOCKS_COUNT as usize];
        self.float_block = get_random_block();
        self.state = GameState::PLAYING;
    }

    fn clear_lines(&mut self) {
        let mut line_nums: Vec<usize> = self
            .blocks
            .iter()
            .enumerate()
            .filter(|&(_, row)| !row.iter().any(|exist| !exist))
            .map(|(i, _)| i)
            .collect();
        // index 从高到低删除
        line_nums.reverse();
        for line_num in line_nums.iter() {
            self.blocks.remove(*line_num);
            self.blocks_color.remove(*line_num);
        }
        // 在零轴填充空白
        for _ in line_nums {
            self.blocks
                .insert(0, vec![false; WIDTH_BLOCKS_COUNT as usize]);
            self.blocks_color
                .insert(0, vec![ColorRaw::WITHE; WIDTH_BLOCKS_COUNT as usize]);
        }
    }
}

fn init_game_background(canvas: &mut Canvas<Window>) -> Result<(), String> {
    canvas.set_draw_color(Color::RGBA(255, 255, 255, 255));
    canvas.clear();
    // 格子
    canvas.set_draw_color(Color::RGBA(200, 200, 200, 255));
    // 横线
    for height in (BLOCK_SIZE..INNER_HIGHT).step_by(BLOCK_SIZE as usize) {
        canvas.draw_line(
            Point::new(0 as i32, height as i32),
            Point::new(INNER_WIDTH as i32, height as i32),
        )?;
    }
    // 竖线
    for width in (BLOCK_SIZE..INNER_WIDTH).step_by(BLOCK_SIZE as usize) {
        canvas.draw_line(
            Point::new(width as i32, 0 as i32),
            Point::new(width as i32, INNER_HIGHT as i32),
        )?;
    }
    Ok(())
}

fn init_img(canvas: &mut Canvas<Window>, img_path: &str) -> Result<(), String> {
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture(Path::new(img_path))?;
    canvas.copy(&texture, None, None)?;
    Ok(())
}


fn init_start_interface(canvas: &mut Canvas<Window>, ) -> Result<(), String> {
    init_img(canvas, r"assets/image/start.png")
}

fn init_gameover_interface(canvas: &mut Canvas<Window>) -> Result<(), String> {
    init_img(canvas, r"assets/image/gameover.png")
}