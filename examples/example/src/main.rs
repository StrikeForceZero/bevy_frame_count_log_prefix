use bevy::log::{BoxedSubscriber, LogPlugin};
use bevy::prelude::*;

use bevy_frame_count_subscriber::plugin::FrameCountSubscriberPlugin;
use bevy_frame_count_subscriber::subscriber::update_subscriber;

fn main() {
    App::new()
        // .add_plugins(DefaultPlugins.build().disable::<LogPlugin>())
        .add_plugins(DefaultPlugins.set(LogPlugin {
            update_subscriber: Some(update_subscriber),
            ..default()
        }))
        .add_plugins(FrameCountSubscriberPlugin)
        .add_systems(Update, || info!("test"))
        .run()
    ;
}
