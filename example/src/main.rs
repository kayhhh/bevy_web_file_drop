use bevy::{asset::AssetMetaCheck, prelude::*};
use bevy_web_file_drop::WebFileDropPlugin;
use wasm_bindgen::prelude::*;

mod plugin;

use plugin::ExamplePlugin;

// Native app
fn main() {
    App::new()
        .add_plugins((DefaultPlugins, ExamplePlugin))
        .run();
}

// Web app
#[wasm_bindgen(start)]
fn start() {
    App::new()
        .insert_resource(AssetMetaCheck::Never)
        .add_plugins((
            WebFileDropPlugin,
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    fit_canvas_to_parent: true,
                    ..default()
                }),
                ..default()
            }),
            ExamplePlugin,
        ))
        .run();
}
