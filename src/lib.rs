//! Bevy plugin adding better support for drag and drop files in the web.
//! Bevy has built in [drag and drop support](https://docs.rs/bevy/latest/bevy/prelude/enum.FileDragAndDrop.html), but when used in a web build they cause a panic and don't cancel the default browser behaviors.
//! This plugin adds some custom JavaScript glue around the canvas to catch these events and relay them safely to Bevy.
//!
//! ## Usage
//!
//! After adding the plugin, you can use the same Bevy `FileDragAndDrop` events to read dropped files.
//! The only difference is that the `path_buf` field of the `DroppedFile` event will be a blob URL instead of a local file path.
//! This blob URL will be loadable as a Bevy asset using [bevy_blob_loader](https://docs.rs/bevy_blob_loader/latest/bevy_blob_loader/).
//!
//! You might not have to change any code at all! Just add in the plugin and dropped files should work in both native and web builds.

use bevy::prelude::*;
use wasm_bindgen::prelude::*;

pub struct WebFileDropPlugin;

impl Plugin for WebFileDropPlugin {
    #[cfg(not(target_family = "wasm"))]
    fn build(&self, _app: &mut App) {}

    #[cfg(target_family = "wasm")]
    fn build(&self, app: &mut App) {
        {
            use bevy_blob_loader::BlobLoaderPlugin;

            app.add_plugins(BlobLoaderPlugin)
                .add_systems(Startup, init_js)
                .add_systems(Update, read_dropped_files);
        }
    }
}

#[wasm_bindgen(module = "/src/drop.js")]
extern "C" {
    fn init();
    fn next_dropped_file() -> Option<Vec<String>>;
}

#[cfg(target_family = "wasm")]
fn init_js() {
    init();
}

#[cfg(target_family = "wasm")]
fn read_dropped_files(
    mut writer: EventWriter<FileDragAndDrop>,
    windows: Query<Entity, With<Window>>,
) {
    use std::path::Path;

    use bevy_blob_loader::path::serialize_url;

    if let Some(vec) = next_dropped_file() {
        let url = &vec[0];
        let ext = &vec[1];

        info!("Got dropped {} file: {}", ext, url);

        for window in windows.iter() {
            // Convert the blob URL to a loadable path
            let serialized = serialize_url(url, ext);
            let path = Path::new(&serialized);
            let path_buf = path.to_path_buf();

            writer.send(FileDragAndDrop::DroppedFile { window, path_buf });
        }
    }
}
