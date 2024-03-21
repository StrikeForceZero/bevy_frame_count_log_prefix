

use std::io::{stderr};




use bevy::log::{BoxedSubscriber, LogPlugin};
use bevy::prelude::*;
use bevy::utils::tracing::{Subscriber};

use tracing_log::LogTracer;


use tracing_subscriber::layer::{SubscriberExt};
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::{EnvFilter, Layer, Registry};


use crate::config::FrameCountSubscriberConfig;
use crate::formatter::FrameCounterPrefixFormatter;

pub fn update_subscriber(subscriber: BoxedSubscriber) -> BoxedSubscriber {
    Box::new(subscriber.with(create_layer(None)))
}

fn create_layer<S: Subscriber + for<'a> LookupSpan<'a>>(
    config: Option<&FrameCountSubscriberConfig>,
) -> impl Layer<S> {
    tracing_subscriber::fmt::Layer::default()
        .event_format(create_filter(config))
        .with_writer(stderr)
}

fn create_filter_from_app(app: &App) -> FrameCounterPrefixFormatter {
    create_filter(app.world.get_resource::<FrameCountSubscriberConfig>())
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

pub(crate) fn register_subscriber(config: Option<&FrameCountSubscriberConfig>) {
    /// derived from https://github.com/bevyengine/bevy/blob/dedf66f72bd8659b744e12b341a7f8de4ed8ba17/crates/bevy_log/src/lib.rs#L129-L228 (MIT/APACHE)
    let finished_subscriber;
    let default_log_plugin = LogPlugin::default();
    let default_filter = { format!("{},{}", default_log_plugin.level, default_log_plugin.filter) };
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new(&default_filter))
        .unwrap();

    // create new subscriber with log level filter
    let subscriber = Registry::default().with(filter_layer);

    // create format layer and replace event_formatter with frame count injector
    let fmt_layer = tracing_subscriber::fmt::Layer::default()
        .event_format(create_filter(config))
        .with_writer(std::io::stderr);

    let subscriber = subscriber.with(fmt_layer);
    finished_subscriber = subscriber;
    let logger_already_set = LogTracer::init().is_err();
    let subscriber_already_set =
        tracing::subscriber::set_global_default(finished_subscriber).is_err();

    match (logger_already_set, subscriber_already_set) {
        (true, true) => warn!(
                "Could not set global logger and tracing subscriber as they are already set. Consider disabling LogPlugin."
            ),
        (true, _) => warn!("Could not set global logger as it is already set. Consider disabling LogPlugin."),
        (_, true) => warn!("Could not set global tracing subscriber as it is already set. Consider disabling LogPlugin."),
        _ => (),
    }
}
