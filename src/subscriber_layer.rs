use bevy::log::{BoxedLayer, LogPlugin};
use bevy::prelude::*;
use tracing_subscriber::Layer;

use crate::config::FrameCountSubscriberConfig;
use crate::formatter::FrameCounterPrefixFormatter;

pub(crate) fn create_filter_from_app(app: &App) -> FrameCounterPrefixFormatter {
    create_filter(app.world().get_resource::<FrameCountSubscriberConfig>())
}

pub(crate) fn create_filter(
    config: Option<&FrameCountSubscriberConfig>,
) -> FrameCounterPrefixFormatter {
    let mut frame_counter_prefix_formatter = FrameCounterPrefixFormatter::default();
    if let Some(config) = config {
        frame_counter_prefix_formatter
            .set_frame_count_prefix_formatter(config.get_frame_count_prefix_formatter());
    }
    frame_counter_prefix_formatter
}

fn custom_layer(app: &mut App) -> Option<BoxedLayer> {    // create format layer and replace event_formatter with frame count injector
    let fmt_layer = tracing_subscriber::fmt::Layer::default()
        .event_format(create_filter_from_app(app))
        .with_writer(std::io::stderr);

    Some(Box::new(vec![
        /* rustfmt multi line */
        fmt_layer.boxed(),
    ]))
}

pub(crate) fn add_log_plugin_with_custom_layer(app: &mut App) -> &mut App {
    app.add_plugins(LogPlugin {
        custom_layer,
        ..default()
    })
}
