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
