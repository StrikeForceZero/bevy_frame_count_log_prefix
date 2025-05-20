use bevy_app::prelude::*;
use bevy_log::{BoxedLayer, LogPlugin};
use tracing_subscriber::Layer;

use crate::config::FrameCountLogPrefixConfig;
use crate::formatter::{FrameCountFormatter, DEFAULT_FRAME_COUNT_FORMATTER};

pub(crate) fn create_filter_from_app(app: &App) -> FrameCountFormatter {
    create_filter(app.world().get_resource::<FrameCountLogPrefixConfig>())
}

pub(crate) fn create_filter(
    config: Option<&FrameCountLogPrefixConfig>,
) -> FrameCountFormatter {
    if let Some(config) = config {
        config.formatter.clone()
    } else {
        DEFAULT_FRAME_COUNT_FORMATTER
    }
}

pub fn frame_count_layer(app: &mut App) -> BoxedLayer {
    // create format layer and replace event_formatter with frame count injector
    tracing_subscriber::fmt::Layer::default()
        .event_format(create_filter_from_app(app))
        .with_writer(std::io::stderr)
        .boxed()
}

fn custom_layer(app: &mut App) -> Option<BoxedLayer> {
    Some(Box::new(vec![
        /* rustfmt multi line */
        frame_count_layer(app),
    ]))
}

pub(crate) fn add_log_plugin_with_custom_layer(app: &mut App) -> &mut App {
    app.add_plugins(LogPlugin {
        custom_layer,
        ..Default::default()
    })
}
