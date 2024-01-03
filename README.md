# bevy_web_file_drop
Bevy plugin adding better support for drag and drop files in the web.
Bevy has built in [drag and drop events](https://docs.rs/bevy/latest/bevy/prelude/enum.FileDragAndDrop.html), but they cause a panic when used in a web build and don't cancel the default browser behavior.
This plugin adds some custom JavaScript glue around the canvas to catch these events and relay them to Bevy.
