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
    commands.spawn(Camera2d);

    let image = asset_server.load("drip.png");
    active_image.0 = image.clone();

    commands.spawn((
        Sprite { image, ..default() },
        Transform::from_xyz(100., 0., 0.),
    ));
}

/// Read dropped files and update the active image
fn read_file_drops(
    asset_server: Res<AssetServer>,
    mut active_image: ResMut<ActiveImage>,
    mut events: MessageReader<FileDragAndDrop>,
) {
    for event in events.read() {
        if let FileDragAndDrop::DroppedFile { path_buf, .. } = event {
            #[cfg(target_family = "wasm")]
            let path = String::from(path_buf.to_str().unwrap());
            #[cfg(not(target_family = "wasm"))]
            let path = bevy::asset::AssetPath::from_path(path_buf.as_path());

            info!("Loading image: {}", path);
            active_image.0 = asset_server.load(path);
        }
    }
}

/// Set the sprite to the active image
fn set_sprite(
    active_image: Res<ActiveImage>,
    mut commands: Commands,
    sprites: Query<(Entity, &Sprite)>,
) {
    for (entity, sprite) in sprites.iter() {
        if sprite.image == active_image.0 {
            continue;
        }

        info!("Spawning new sprite");

        commands.entity(entity).despawn();

        commands.spawn((
            Sprite {
                image: active_image.0.clone(),
                ..default()
            },
            Transform::from_xyz(100., 0., 0.),
        ));
    }
}
