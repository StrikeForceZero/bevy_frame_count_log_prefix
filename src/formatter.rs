use std::any::type_name;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::sync::Arc;

use bevy_log::tracing::Subscriber;
use tracing_subscriber::fmt::{format, FmtContext, FormatEvent, FormatFields};
use tracing_subscriber::registry::LookupSpan;

#[cfg(feature = "fixed_update")]
use crate::statics::get_fixed_update_count;
use crate::statics::get_frame_count;

pub trait FormatFrameCount {
    fn debug_name(&self) -> &'static str {
        type_name::<Self>()
    }

    fn write(
        &self,
        f: &mut Formatter<'_>,
        frame_count: u32,
        #[cfg(feature = "fixed_update")] fixed_update_count: u32,
    ) -> fmt::Result;
}

impl<T: FormatFrameCount + Send + Sync + 'static> From<T> for FrameCountFormatter {
    fn from(value: T) -> Self {
        Self {
            formatter: Some(Arc::new(value)),
        }
    }
}

pub(crate) fn default_frame_count_formatter(
    frame_count: u32,
    #[cfg(feature = "fixed_update")] fixed_update_count: u32,
) -> impl Display {
    struct DefaultFormatFrameCountForwarder {
        frame_count: u32,
        #[cfg(feature = "fixed_update")]
        fixed_update_count: u32,
    }

    impl Display for DefaultFormatFrameCountForwarder {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            #[cfg(not(feature = "fixed_update"))]
            {
                write!(f, "[frame:{}] ", self.frame_count)
            }
            #[cfg(feature = "fixed_update")]
            {
                write!(
                    f,
                    "[frame:{}][fixed:{}] ",
                    self.frame_count, self.fixed_update_count
                )
            }
        }
    }

    DefaultFormatFrameCountForwarder {
        frame_count,
        #[cfg(feature = "fixed_update")]
        fixed_update_count,
    }
}

pub const DEFAULT_FRAME_COUNT_FORMATTER: FrameCountFormatter =
    FrameCountFormatter { formatter: None };

#[derive(Default, Clone)]
pub struct FrameCountFormatter {
    formatter: Option<Arc<dyn FormatFrameCount + Send + Sync>>,
}

impl Debug for FrameCountFormatter {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("FrameCounterPrefixFormatter");
        if let Some(formatter) = &self.formatter {
            d.field("formatter", &formatter.debug_name()).finish()
        } else {
            d.finish_non_exhaustive()
        }
    }
}

impl FrameCountFormatter {
    pub fn new(formatter: impl FormatFrameCount + Send + Sync + 'static) -> Self {
        Self {
            formatter: Some(Arc::new(formatter)),
        }
    }
}

impl<S, N> FormatEvent<S, N> for FrameCountFormatter
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
                #[cfg(feature = "fixed_update")]
                fixed_update_count: u32,
                formatter: &'a dyn FormatFrameCount,
            }

            impl Display for DynFormatFrameCountForwarder<'_> {
                fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                    self.formatter.write(
                        f,
                        self.frame_count,
                        #[cfg(feature = "fixed_update")]
                        self.fixed_update_count,
                    )
                }
            }

            write!(
                writer,
                "{}",
                DynFormatFrameCountForwarder {
                    frame_count: get_frame_count(),
                    #[cfg(feature = "fixed_update")]
                    fixed_update_count: get_fixed_update_count(),
                    formatter: &**formatter,
                }
            )
        } else {
            write!(
                writer,
                "{}",
                default_frame_count_formatter(
                    get_frame_count(),
                    #[cfg(feature = "fixed_update")]
                    get_fixed_update_count()
                )
            )
        }
    }
}
