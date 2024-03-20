use crate::statics::get_frame_count;
use bevy::utils::tracing;
use bevy::utils::tracing::Subscriber;
use std::fmt;
use tracing_subscriber::fmt::{format, FmtContext, FormatEvent, FormatFields};
use tracing_subscriber::registry::LookupSpan;

pub(crate) fn default_frame_count_prefix_formatter(frame_count: u32) -> String {
    format!("[frame:{frame_count}] ")
}

pub type FrameCountPrefixFormatter = fn(count: u32) -> String;

pub(crate) struct FrameCounterPrefixFormatter {
    frame_count_prefix_formatter: FrameCountPrefixFormatter,
    main_formatter: format::Format<format::Full>,
}

impl FrameCounterPrefixFormatter {
    pub(crate) fn set_frame_count_prefix_formatter(
        &mut self,
        formatter: Option<FrameCountPrefixFormatter>,
    ) {
        if let Some(formatter) = formatter {
            self.frame_count_prefix_formatter = formatter;
        } else {
            self.frame_count_prefix_formatter = default_frame_count_prefix_formatter;
        }
    }
}

impl Default for FrameCounterPrefixFormatter {
    fn default() -> Self {
        Self {
            frame_count_prefix_formatter: default_frame_count_prefix_formatter,
            main_formatter: format::Format::default(),
        }
    }
}

impl<S, N> FormatEvent<S, N> for FrameCounterPrefixFormatter
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        ctx: &FmtContext<'_, S, N>,
        mut writer: format::Writer<'_>,
        event: &tracing::Event<'_>,
    ) -> fmt::Result {
        // Write the prefix before the rest of the event
        write!(
            writer,
            "{}",
            (self.frame_count_prefix_formatter)(get_frame_count())
        )?;
        // Use the default event formatter for the rest
        // ctx.field_format().format_fields(writer.by_ref(), event)?;
        self.main_formatter.format_event(ctx, writer, event)?;
        Ok(())
    }
}
