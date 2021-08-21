use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;

#[derive(Debug, Copy, Clone)]
pub struct ColorRaw([u8; 4]);

impl ColorRaw {
    pub const WITHE: ColorRaw = ColorRaw([0xFF, 0xFF, 0xFF, 0xFF]);
    // pub const BLACK: ColorRaw = ColorRaw([0, 0, 0, 0xFF]);
    pub const CYAN: ColorRaw = ColorRaw([48, 199, 239, 0xFF]);
    pub const YELLOW: ColorRaw = ColorRaw([0xeb, 0xe2, 0x4d, 0xFF]);
    pub const BLUE: ColorRaw = ColorRaw([0x00, 0x0C, 0x8A, 0xFF]);
    pub const ORANGE: ColorRaw = ColorRaw([0xF0, 0x98, 0x4F, 0xFF]);
    pub const PURPLE: ColorRaw = ColorRaw([0xed, 0x71, 0xf8, 0xFF]);
    pub const GREEN: ColorRaw = ColorRaw([0x51, 0x9C, 0x3C, 0xFF]);
    pub const RED: ColorRaw = ColorRaw([0x92, 0x1E, 0x13, 0xFF]);

    pub fn set_canvas(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(Color::RGBA(self.0[0], self.0[1], self.0[2], self.0[3]))
    }
}
