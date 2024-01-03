use bevy::{asset::AssetMetaCheck, prelude::*};
use bevy_web_file_drop::WebFileDropPlugin;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
fn start() {
    App::new()
        .insert_resource(AssetMetaCheck::Never)
        .init_resource::<ActiveImage>()
        .add_plugins((
            // Plugin must be added before AssetPlugin (which is in DefaultPlugins)
            WebFileDropPlugin,
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    fit_canvas_to_parent: true,
                    ..default()
                }),
                ..default()
            }),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, (read_file_drops, set_sprite))
        .run();
}

#[derive(Resource, Default)]
pub struct ActiveImage(Handle<Image>);

/// Spawn a camera and a sprite
fn setup(
    asset_server: Res<AssetServer>,
    mut active_image: ResMut<ActiveImage>,
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle::default());

    let handle = asset_server.load("drip.png");
    active_image.0 = handle.clone();

    commands.spawn(SpriteBundle {
        texture: handle,
        transform: Transform::from_xyz(100., 0., 0.),
        ..default()
    });
}

/// Read dropped files and update the active image
fn read_file_drops(
    asset_server: Res<AssetServer>,
    mut active_image: ResMut<ActiveImage>,
    mut events: EventReader<FileDragAndDrop>,
) {
    for event in events.read() {
        if let FileDragAndDrop::DroppedFile { path_buf, .. } = event {
            let path = String::from(path_buf.to_str().unwrap());
            info!("Loading image: {}", path);
            active_image.0 = asset_server.load(path);
        }
    }
}

/// Set the sprite to the active image
fn set_sprite(
    active_image: Res<ActiveImage>,
    mut commands: Commands,
    sprites: Query<(Entity, &Handle<Image>), With<Sprite>>,
) {
    for (entity, handle) in sprites.iter() {
        if *handle == active_image.0 {
            continue;
        }

        info!("Spawning new sprite");

        commands.entity(entity).despawn();

        commands.spawn(SpriteBundle {
            texture: active_image.0.clone(),
            transform: Transform::from_xyz(100., 0., 0.),
            ..default()
        });
    }
}
