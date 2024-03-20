use crate::cache_system::cache_frame_count;
use crate::formatter::FrameCounterPrefixFormatter;
use bevy::core::FrameCount;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::utils::tracing;
use bevy::utils::tracing::Subscriber;
use std::fmt;
use std::sync::atomic::{AtomicU32, Ordering};
use tracing_log::LogTracer;
use tracing_subscriber::fmt::{format, FmtContext, FormatEvent, FormatFields};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::{EnvFilter, Registry};

pub(crate) fn register_subscriber() {
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
        .event_format(FrameCounterPrefixFormatter::default())
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
