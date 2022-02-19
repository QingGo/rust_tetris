mod settings;

use bevy::core::FixedTimestep;
use bevy::input::{keyboard::KeyCode, Input};
use bevy::reflect::TypeUuid;
use bevy::render::mesh::VertexAttributeValues;
use bevy::{prelude::*, transform};

use crate::settings::settings::*;

// (0, 0) is in the bottom-left corner
#[derive(Component)]
struct Position {
    x: i64,
    y: i64,
}

#[derive(Component)]
struct FixBlock {
    blocks: Vec<Vec<bool>>,
}

#[derive(Component)]
struct Block;

#[derive(Component)]
struct Score {
    value: u32,
}

// global resources
struct GameRules {
    width: f32,
    height: f32,
}

fn get_border(size_x: f32, size_y: f32, t_x: f32, t_y: f32) -> SpriteBundle {
    SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.0, 0.0, 0.0),
            custom_size: Some(Vec2::new(size_x, size_y)),
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(t_x, t_y, 0.0),
            ..Default::default()
        },
        ..Default::default()
    }
}

fn startup_system(mut commands: Commands) {
    commands.insert_resource(GameRules {
        width: WIDTH_BLOCKS_COUNT,
        height: HEIGHT_BLOCKS_COUNT,
    });
    commands.spawn().insert(Score { value: 0 });
    // 让摄像头中心往右上移动，使得左下角为 (0, 0)
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(Transform::from_xyz(WIDTH / 2.0, HIGHT / 2.0, 1000.0));

    // 框框
    commands.spawn_batch(vec![
        get_border(3.0, INNER_HIGHT, MARGIN, MARGIN + INNER_HIGHT / 2.0), // left
        get_border(
            3.0,
            INNER_HIGHT,
            MARGIN + INNER_WIDTH,
            MARGIN + INNER_HIGHT / 2.0,
        ), // right
        get_border(INNER_WIDTH, 3.0, MARGIN + INNER_WIDTH / 2.0, MARGIN), // top
        get_border(
            INNER_WIDTH,
            3.0,
            MARGIN + INNER_WIDTH / 2.0,
            MARGIN + INNER_HIGHT,
        ), // bottom
    ]);
    // 方块
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                custom_size: Some(Vec2::new(2.0 * BLOCK_SIZE, 2.0 * BLOCK_SIZE)),
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(
                    MARGIN + INNER_WIDTH / 2.0,
                    MARGIN - BLOCK_SIZE + INNER_HIGHT,
                    0.0,
                ),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Block);
}

fn block_move_system(keys: Res<Input<KeyCode>>, mut sprite_query: Query<(&Block, &mut Transform)>) {
    // 检测是否碰到边缘或底部方块，如果碰到边缘，则不能移动
    let (_block, mut transform) = sprite_query.single_mut();
    if keys.pressed(KeyCode::Up) {
        info!("UP key pressed");
        // transform.translation.y += BLOCK_SIZE;
    } else if keys.pressed(KeyCode::Down) {
        info!("DOWN key pressed");
        transform.translation.y -= BLOCK_SIZE;
    } else if keys.pressed(KeyCode::Left) {
        info!("LEFT key pressed");
        transform.translation.x -= BLOCK_SIZE;
    } else if keys.pressed(KeyCode::Right) {
        info!("RIGHT key pressed");
        transform.translation.x += BLOCK_SIZE;
    }
}

struct NewBlockEvent();

fn gravity_system(
    mut commands: Commands,
    mut sprite_query: Query<(Entity, &Block, &mut Transform)>,
    mut ev_newblock: EventWriter<NewBlockEvent>,
) {
    let (entity, _block, mut transform) = sprite_query.single_mut();
    transform.translation.y -= BLOCK_SIZE;
    // 碰到地面就不动了，并生成新方块，后续应该检测是否碰到底部方块
    if transform.translation.y < MARGIN + BLOCK_SIZE * 2.0 {
        // 也许改在这个删除旧的实体并使用新实体
        // 新实体要加上删除规则和加分规则
        commands.entity(entity).remove::<Block>();
        ev_newblock.send(NewBlockEvent());
    }
}

fn generate_new_block(mut commands: Commands, mut ev_newblock: EventReader<NewBlockEvent>) {
    for _ev in ev_newblock.iter() {
        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.25, 0.25, 0.75),
                    custom_size: Some(Vec2::new(2.0 * BLOCK_SIZE, 2.0 * BLOCK_SIZE)),
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(
                        MARGIN + INNER_WIDTH / 2.0,
                        MARGIN - BLOCK_SIZE + INNER_HIGHT,
                        0.0,
                    ),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Block);
    }
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Tetris".to_string(),
            width: WIDTH,
            height: HIGHT,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_event::<NewBlockEvent>()
        .add_startup_system(startup_system)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.0 / 16.0))
                .with_system(block_move_system),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.0 / 2.0))
                .with_system(gravity_system)
                .with_system(generate_new_block),
        )
        .run();
}
