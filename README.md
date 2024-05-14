# Bevy Frame Count Subscriber

## Features

Adds prefix to all log messages with the current frame count. This is useful in determining if certain systems are running on the same frame.

### Usage

### Automatically

```rust
use bevy::log::LogPlugin;
use bevy::prelude::*;

use bevy_frame_count_subscriber::plugin::FrameCountSubscriberPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.build().disable::<LogPlugin>())
        .add_plugins(FrameCountSubscriberPlugin)
        .add_systems(Update, || info!("test"))
        .run()
    ;
}
```

### Manually

```rust
use bevy::log::LogPlugin;
use bevy::prelude::*;

use bevy_frame_count_subscriber::plugin::FrameCountSubscriberPluginManual;
use bevy_frame_count_subscriber::subscriber_layer::frame_count_layer;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.build().set(LogPlugin {
            custom_layer: |app| {
                Some(Box::new(vec![
                    /* other layers here */
                    frame_count_layer(app),
                ]))
            },
        }))
        .add_plugins(FrameCountSubscriberPluginManual)
        .add_systems(Update, || info!("test"))
        .run()
    ;
}
```

