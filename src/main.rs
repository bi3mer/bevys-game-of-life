use bevy::{
    prelude::*,
    window::PresentMode,
};
use rand::{rngs::SmallRng, SeedableRng, Rng};

const SCREEN_WIDTH: f32 = 1080.;
const SCREEN_HEIGHT: f32 = 720.;

const GRID_WIDTH: usize = 108; // 1080 / 10, each cell is 10x10
const GRID_HEIGHT: usize = 72; // 720 / 10

#[derive(Component)]
struct Cell {
    alive: bool,
}

#[derive(Default, Resource)]
struct SpriteImages {
    alive_cell: Handle<Image>,
    dead_cell: Handle<Image>,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // build grid
    let mut rng = SmallRng::from_entropy();
    for y in 1..(GRID_HEIGHT - 1) {
        for x in 1..(GRID_WIDTH - 1) {
            let alive = rng.gen_bool(0.2);
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
    commands.spawn_bundle(Camera2dBundle {
        transform: Transform { 
            translation: Vec3::new(SCREEN_WIDTH / 2.0, SCREEN_HEIGHT / 2.0, 0.0),
            ..Default::default()
        },
        ..Default::default()
    });
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
        .run();
}
