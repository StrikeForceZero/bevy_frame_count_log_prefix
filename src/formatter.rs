use std::any::type_name;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::sync::Arc;

use bevy::utils::tracing::Subscriber;
use tracing_subscriber::fmt::{FmtContext, format, FormatEvent, FormatFields};
use tracing_subscriber::registry::LookupSpan;

use crate::statics::get_frame_count;

pub trait FormatFrameCount {
    fn debug_name(&self) -> &'static str {
        type_name::<Self>()
    }

    fn write(&self, f: &mut Formatter<'_>, frame_count: u32) -> fmt::Result;
}

pub(crate) fn default_frame_count_prefix_formatter(frame_count: u32) -> impl Display {
    struct DefaultFormatFrameCountForwarder {
        frame_count: u32,
    }

    impl Display for DefaultFormatFrameCountForwarder {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "[frame:{}] ", self.frame_count)
        }
    }

    DefaultFormatFrameCountForwarder { frame_count }
}

pub const DEFAULT_FRAME_COUNTER_PREFIX_FORMATTER: FrameCounterPrefixFormatter =
    FrameCounterPrefixFormatter {
        formatter: None,
    };

#[derive(Default, Clone)]
pub struct FrameCounterPrefixFormatter {
    formatter: Option<Arc<dyn FormatFrameCount + Send + Sync>>,
}

impl Debug for FrameCounterPrefixFormatter {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("FrameCounterPrefixFormatter");
        if let Some(formatter) = &self.formatter {
            d.field("formatter", &formatter.debug_name()).finish()
        } else {
            d.finish_non_exhaustive()
        }
    }
}

impl FrameCounterPrefixFormatter {
    pub fn new(formatter: impl FormatFrameCount + Send + Sync + 'static) -> Self {
        Self {
            formatter: Some(Arc::new(formatter)),
        }
    }
    pub fn set_frame_count_prefix_formatter(
        &mut self,
        formatter: Option<impl FormatFrameCount + Send + Sync + 'static>,
    ) {
        self.formatter = formatter.map(Arc::new).map(|param| param as _);
    }
}

impl<S, N> FormatEvent<S, N> for FrameCounterPrefixFormatter
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        _ctx: &FmtContext<'_, S, N>,
        mut writer: format::Writer<'_>,
        _event: &tracing::Event<'_>,
    ) -> fmt::Result {
        // Write the prefix before the rest of the event
        if let Some(formatter) = &self.formatter {
            struct DynFormatFrameCountForwarder<'a> {
                frame_count: u32,
                formatter: &'a dyn FormatFrameCount,
            }

            impl Display for DynFormatFrameCountForwarder<'_> {
                fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                    self.formatter.write(f, self.frame_count)
                }
            }

            write!(
                writer,
                "{}",
                DynFormatFrameCountForwarder {
                    frame_count: get_frame_count(),
                    formatter: &**formatter,
                }
            )
        } else {
            write!(
                writer,
                "{}",
                default_frame_count_prefix_formatter(get_frame_count())
            )
        }
    }
}
