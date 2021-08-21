mod game;
mod settings;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};

use game::game::Game;
use settings::settings::*;

// to-do
// 游戏开始/失败界面，图片/音乐，内部窗口，分数
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
                others => g.reveive_event(others),
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

fn init_sdl() -> Result<(Canvas<Window>, sdl2::Sdl), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("Rust Tetris", INNER_WIDTH, INNER_HIGHT)
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
