use bevy::{asset::AssetMetaCheck, prelude::*};
use bevy_web_file_drop::WebFileDropPlugin;

mod plugin;

use plugin::ExamplePlugin;

fn main() {
    App::new()
        .add_plugins((
            WebFileDropPlugin,
            DefaultPlugins.set(AssetPlugin {
                meta_check: AssetMetaCheck::Never,
                ..default()
            }),
            ExamplePlugin,
        ))
        .run();
}
