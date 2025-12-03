#![doc(html_logo_url = "https://duper.dev.br/logos/duper-100-100.png")]

use duper::{DuperInner, DuperKey, DuperObject, DuperString, DuperValue, Serializer};
use tracing_core::{Event, Subscriber};
use tracing_subscriber::{
    fmt::{FormatEvent, FormatFields},
    registry::LookupSpan,
};

struct DuperSubscriber<Timer> {
    timer: Timer,
    display_timestamp: bool,
    display_level: bool,
    display_target: bool,
    display_file: bool,
    display_line: bool,
}

impl<S, N, Timer> FormatEvent<S, N> for DuperSubscriber<Timer>
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        ctx: &tracing_subscriber::fmt::FmtContext<'_, S, N>,
        mut writer: tracing_subscriber::fmt::format::Writer<'_>,
        event: &Event<'_>,
    ) -> std::fmt::Result {
        let mut log = vec![];

        let metadata = event.metadata();
        if self.display_level {
            log.push((
                DuperKey::from("level"),
                DuperValue {
                    identifier: None,
                    inner: DuperInner::String(DuperString::from(metadata.level().as_str())),
                },
            ));
        }
        if self.display_target {
            log.push((
                DuperKey::from("target"),
                DuperValue {
                    identifier: None,
                    inner: DuperInner::String(DuperString::from(metadata.target())),
                },
            ));
        }
        if self.display_file {
            log.push((
                DuperKey::from("file"),
                DuperValue {
                    identifier: None,
                    inner: metadata
                        .file()
                        .map(|file| DuperInner::String(DuperString::from(file)))
                        .unwrap_or(DuperInner::Null),
                },
            ));
        }
        if self.display_line {
            log.push((
                DuperKey::from("line"),
                DuperValue {
                    identifier: None,
                    inner: metadata
                        .line()
                        .map(|line| DuperInner::Integer(line.into()))
                        .unwrap_or(DuperInner::Null),
                },
            ));
        }

        let mut serializer = Serializer::new(false, true);
        if let Ok(object) = DuperObject::try_from(log) {
            writeln!(
                writer,
                "{}",
                serializer.serialize(&DuperValue {
                    identifier: None,
                    inner: DuperInner::Object(object),
                })
            )
        } else {
            Ok(())
        }
    }
}
