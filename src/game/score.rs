use crate::settings::settings::*;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::render::TextureQuery;
use sdl2::video::Window;

// 记录和展示分数
pub struct Score {
    score_now: u32,
    score_highest: u32,
}

impl Score {
    pub fn new() -> Score {
        Score {
            score_now: 0,
            score_highest: 0,
        }
    }

    pub fn add_score_by_clear_lines(&mut self, lines_num: u32) {
        self.score_now += lines_num * (lines_num + 1) * BASE_CLEAR_LINE_SCORE / 2;
        if self.score_now > self.score_highest {
            self.score_highest = self.score_now;
        }
        // println!("score_now: {}", self.score_now);
    }

    pub fn reset_score(&mut self) {
        self.score_now = 0;
    }

    pub fn render(&self, canvas: &mut Canvas<Window>, offset: (i32, i32)) -> Result<(), String> {
        self.render_text(canvas, (offset.0, offset.1), "Score:")?;
        self.render_text(
            canvas,
            (offset.0, offset.1 + 50),
            &*self.score_now.to_string(),
        )?;
        self.render_text(canvas, (offset.0, offset.1 + 100), "High Score:")?;
        self.render_text(
            canvas,
            (offset.0, offset.1 + 150),
            &*self.score_highest.to_string(),
        )?;
        Ok(())
    }

    fn render_text(
        &self,
        canvas: &mut Canvas<Window>,
        offset: (i32, i32),
        text: &str,
    ) -> Result<(), String> {
        let ttf_context = sdl2::ttf::init().unwrap();
        let font_path = "./assets/font/ascii.ttf";
        let font = ttf_context.load_font(font_path, 128).unwrap();
        let texture_creator = canvas.texture_creator();
        // render a surface, and convert it to a texture bound to the canvas
        let surface = font
            .render(text)
            .blended(Color::RGBA(0, 0, 0, 255))
            .map_err(|e| e.to_string())?;
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;
        let TextureQuery { width, height, .. } = texture.query();
        let target = Rect::new(offset.0, offset.1, width / 3, height / 3);
        canvas.copy(&texture, None, Some(target))?;
        Ok(())
    }
}
