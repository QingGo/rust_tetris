use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;

#[derive(Debug, Copy, Clone)]
pub struct ColorRaw([u8; 4]);

impl ColorRaw {
    pub const WITHE: ColorRaw = ColorRaw([0xFF, 0xFF, 0xFF, 0xFF]);
    pub const BLACK: ColorRaw = ColorRaw([0, 0, 0, 0xFF]);
    pub const BLUE: ColorRaw = ColorRaw([48, 199, 239, 0xFF]);

    pub fn set_canvas(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(Color::RGBA(self.0[0], self.0[1], self.0[2], self.0[3]))
    }
}
