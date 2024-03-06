use bevy::{asset::AssetMetaCheck, prelude::*};
use bevy_web_file_drop::WebFileDropPlugin;

mod plugin;

use plugin::ExamplePlugin;

fn main() {
    App::new()
        .insert_resource(AssetMetaCheck::Never)
        .add_plugins((WebFileDropPlugin, DefaultPlugins, ExamplePlugin))
        .run();
}
