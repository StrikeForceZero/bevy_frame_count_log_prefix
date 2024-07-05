# Bevy Frame Count Log Prefix

## Features

Adds prefix to all log messages with the current frame count. This is useful in determining if certain systems are running on the same frame.

### Usage

### Automatically

```rust
use bevy::log::LogPlugin;
use bevy::prelude::*;

use bevy_frame_count_log_prefix::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.build().disable::<LogPlugin>())
        .add_plugins(FrameCountLogPrefixPlugin)
        .add_systems(Update, || info!("test"))
        .run()
    ;
}
```

### Manually

```rust
use bevy::log::LogPlugin;
use bevy::prelude::*;

use bevy_frame_count_log_prefix::prelude::*;

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
        .add_plugins(FrameCountLogPrefixManualPlugin)
        .add_systems(Update, || info!("test"))
        .run()
    ;
}
```

### Custom Formatter

```rust
use bevy::log::LogPlugin;
use bevy::prelude::*;

use bevy_frame_count_log_prefix::prelude::*;

struct CustomFormatter;

impl FormatFrameCount for CustomFormatter {
    fn debug_name(&self) -> &'static str {
        "CustomFormatter"
    }
    fn write(&self, f: &mut Formatter<'_>, frame_count: u32) -> std::fmt::Result {
        write!(f, "{frame_count} ")
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.build().disable::<LogPlugin>())
        .insert_resource(FrameCountLogPrefixConfig {
            formatter: CustomFormatter.into(),
        })
        .add_plugins(FrameCountLogPrefixPlugin)
        .add_systems(Update, || info!("test"))
        .run()
    ;
}
```
