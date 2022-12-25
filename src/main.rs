use bevy::{
    prelude::*,
    window::PresentMode, 
    time::FixedTimestep,
};
use rand::{rngs::SmallRng, SeedableRng, Rng};

const SCREEN_WIDTH: f32 = 1080.;
const SCREEN_HEIGHT: f32 = 720.;

const GRID_WIDTH: usize = 1010; // 1080 / 10, each cell is 10x10
const GRID_HEIGHT: usize = 74; // 720 / 10

#[derive(Component)]
struct Cell {
    alive: bool
}

#[derive(Default, Resource)]
struct SpriteImages {
    alive_cell: Handle<Image>,
    dead_cell: Handle<Image>,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // build grid
    let mut rng = SmallRng::from_entropy();
    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            let alive = rng.gen_bool(0.35);
            commands
            .spawn((
                Cell { 
                    alive
                }, 
                SpriteBundle {
                    sprite: Sprite {
                        ..Default::default()
                    },
                    texture: if alive { asset_server.load("alive_cell.png") } else { asset_server.load("dead_cell.png") },
                    transform: Transform { 
                        translation: Vec3::new((x*10) as f32, (y*10) as f32, 0.0),
                        scale: Vec3::new(1.0, 1.0, 0.0),
                        ..Default::default()
                    },
                    ..Default::default()
                }
            ));
        }
    }
    
    // load assets
    commands
        .insert_resource(SpriteImages {
            alive_cell: asset_server.load("alive_cell.png"),
            dead_cell: asset_server.load("dead_cell.png"),
        }
    );

    // set up the camera
    commands.spawn(Camera2dBundle {
        transform: Transform { 
            translation: Vec3::new(SCREEN_WIDTH / 2.0, SCREEN_HEIGHT / 2.0, 0.0),
            ..Default::default()
        },
        ..Default::default()
    });
}

fn update(
    mut cells: Query<(&mut Cell, &mut Handle<Image>, &Transform)>,
    sprite_images: Res<SpriteImages>
) {
    let mut cur = Vec::new();
    let mut new = Vec::new();

    cells.iter().for_each(|(cell, _, _)| {
        cur.push(cell.alive);
    });

    
    for i in 0..cur.len() {
        let x = i % GRID_WIDTH;
        let y = i / GRID_WIDTH;

        if x >= 1 && x < GRID_WIDTH-1 && y >= 1 && y < GRID_HEIGHT-1 {
            let neighbor_count = 
                cur[x - 1 + (y - 1) * GRID_WIDTH] as u8 + 
                cur[x     + (y - 1) * GRID_WIDTH] as u8 + 
                cur[x + 1 + (y - 1) * GRID_WIDTH] as u8 + 
                cur[x - 1 + y       * GRID_WIDTH] as u8 + 
                cur[x + 1 + y       * GRID_WIDTH] as u8 + 
                cur[x - 1 + (y + 1) * GRID_WIDTH] as u8 + 
                cur[x     + (y + 1) * GRID_WIDTH] as u8 + 
                cur[x + 1 + (y + 1) * GRID_WIDTH] as u8;
    
    
            if cur[i] && (neighbor_count == 2 || neighbor_count == 3) {
                new.push(true);
            } else if !cur[i] && neighbor_count == 3 {
                new.push(false);
            } else {
                new.push(false);
            }
        } else {
            new.push(false);
        }
    }

    for (i, (mut cell, mut sprite, _)) in cells.iter_mut().enumerate() {
        if cell.alive != new[i] {
            cell.alive = new[i];

            if cell.alive {
                *sprite = sprite_images.alive_cell.clone();
            } else {
                *sprite = sprite_images.dead_cell.clone();
            }
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Conway's Game of Life!".to_string(),
                width: SCREEN_WIDTH,
                height: SCREEN_HEIGHT,
                present_mode: PresentMode::AutoVsync,
                ..default()
            },
            ..default()
        }))
        .add_startup_system(setup)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.75))
                .with_system(update)
        )
        .run();
}