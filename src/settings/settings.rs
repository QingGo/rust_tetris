pub const BLOCK_SIZE: f32 = 30.0;
pub const WIDTH_BLOCKS_COUNT: f32 = 12.0;
pub const HEIGHT_BLOCKS_COUNT: f32 = 23.0;
pub const TIMESTEP_2_PER_SECOND: f64 = 30.0 / 60.0;


pub const INNER_HIGHT: f32 = BLOCK_SIZE * HEIGHT_BLOCKS_COUNT;
pub const INNER_WIDTH: f32 = BLOCK_SIZE * WIDTH_BLOCKS_COUNT;

pub const MARGIN: f32 = 20.0;
pub const HIGHT: f32 = BLOCK_SIZE * HEIGHT_BLOCKS_COUNT + 2.0 * MARGIN;
pub const WIDTH: f32 = BLOCK_SIZE * WIDTH_BLOCKS_COUNT * 2.0 + 2.0 * MARGIN;

pub const MOVE_BETWEEN_MILLISECOND: u128 = 50;
pub const POLL_EVENT_INTERVAL_MILLISECOND: u64 = 5;

pub const FALL_SPEED_NORMAL: u32 = 6;
pub const FALL_SPEED_QUICK: u32 = 30;

pub const BASE_CLEAR_LINE_SCORE: u32 = 100;

pub const INIT_LEFT_UP_POS: (i32, i32) = (3, 0);

// pub const BLOCKS_TYPE: Vec<&str> = BLOCKS_SETTING.iter().map(|(key, value)| *key).collect();
