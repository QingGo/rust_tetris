mod game;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::Point;
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};

use game::game::Game;

const HIGHT: u32 = 690;
const WIDTH: u32 = 360;
const BLOCK_SIZE: u32 = 30;
const MOVE_BETWEEN_MILLISECOND: u128 = 100;
const POLL_EVENT_INTERVAL_MILLISECOND: u64 = 10;

// to-do
// 什么时候旋转/移动失效
// 自己下落，下落加速
// 堆积和消除的逻辑
// 增加方块类型，且用 trait 实现
fn main() -> Result<(), String> {
    let (mut canvas, sdl_context) = init_sdl()?;
    let mut g = Game::new();
    let mut last_time = get_epoch_ms();
    'mainloop: loop {
        // 获取输入
        for event in sdl_context.event_pump()?.poll_iter() {
            match event {
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                }
                | Event::Quit { .. } => break 'mainloop,
                others => g.reveive_key(others),
            }
        }

        // update the game loop here
        let time_now = get_epoch_ms();
        if time_now.wrapping_sub(last_time) >= MOVE_BETWEEN_MILLISECOND {
            // 更新状态
            // // println!("{} {}", last_time, time_now);
            last_time = time_now;
            g.update();
            // 渲染
            init_background(&mut canvas)?;
            g.render(&mut canvas)?;
            // 展示
            canvas.present();
        }

        std::thread::sleep(Duration::from_millis(POLL_EVENT_INTERVAL_MILLISECOND));
    }
    Ok(())
}

fn get_epoch_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

fn init_sdl() -> Result<(Canvas<Window>, sdl2::Sdl), String>{
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("Rust Tetris", WIDTH, HIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;
    let canvas = window
        .into_canvas()
        .accelerated()
        .build()
        .map_err(|e| e.to_string())?;
    Ok((canvas, sdl_context))
}

fn init_background(canvas: &mut Canvas<Window>) -> Result<(), String> {
    canvas.set_draw_color(Color::RGBA(255, 255, 255, 255));
    canvas.clear();
    // 格子
    canvas.set_draw_color(Color::RGBA(200, 200, 200, 255));
    // 横线
    for height in (BLOCK_SIZE..HIGHT).step_by(BLOCK_SIZE as usize) {
        canvas.draw_line(
            Point::new(0 as i32, height as i32),
            Point::new(WIDTH as i32, height as i32),
        )?;
    }
    // 竖线
    for width in (BLOCK_SIZE..WIDTH).step_by(BLOCK_SIZE as usize) {
        canvas.draw_line(
            Point::new(width as i32, 0 as i32),
            Point::new(width as i32, HIGHT as i32),
        )?;
    }
    Ok(())
}