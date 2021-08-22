pub const BLOCK_SIZE: u32 = 30;
pub const WIDTH_BLOCKS_COUNT: u32 = 12;
pub const HEIGHT_BLOCKS_COUNT: u32 = 23;

pub const INNER_HIGHT: u32 = BLOCK_SIZE * HEIGHT_BLOCKS_COUNT;
pub const INNER_WIDTH: u32 = BLOCK_SIZE * WIDTH_BLOCKS_COUNT;

pub const MOVE_BETWEEN_MILLISECOND: u128 = 50;
pub const POLL_EVENT_INTERVAL_MILLISECOND: u64 = 5;

pub const FALL_SPEED_NORMAL: u32 = 6;
pub const FALL_SPEED_QUICK: u32 = 30;

pub const INIT_LEFT_UP_POS: (i32, i32) = (3, 0);

// pub const BLOCKS_TYPE: Vec<&str> = BLOCKS_SETTING.iter().map(|(key, value)| *key).collect();
