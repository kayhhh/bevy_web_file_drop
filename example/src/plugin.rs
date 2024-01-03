use bevy::prelude::*;

pub struct ExamplePlugin;

impl Plugin for ExamplePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ActiveImage>()
            .add_systems(Startup, setup)
            .add_systems(Update, (read_file_drops, set_sprite));
    }
}

#[derive(Resource, Default)]
struct ActiveImage(Handle<Image>);

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
