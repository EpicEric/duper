#![doc(html_logo_url = "https://duper.dev.br/logos/duper-100-100.png")]

use std::{borrow::Cow, collections::BTreeMap};

use chrono::{Local, Utc};
use duper::{
    DuperArray, DuperBytes, DuperIdentifier, DuperInner, DuperKey, DuperObject, DuperString,
    DuperTemporal, DuperValue, Serializer,
};
use tracing_core::{Event, Subscriber};
use tracing_subscriber::{
    field::{RecordFields, VisitOutput},
    fmt::{FormatEvent, FormatFields, FormattedFields},
    registry::LookupSpan,
};

pub trait DuperTimer {
    fn get_time(&self) -> Option<DuperValue<'static>>;
}

impl DuperTimer for () {
    fn get_time(&self) -> Option<DuperValue<'static>> {
        None
    }
}

#[cfg(feature = "chrono")]
#[derive(Debug)]
pub struct ChronoUtcTimer;

#[cfg(feature = "chrono")]
impl DuperTimer for ChronoUtcTimer {
    fn get_time(&self) -> Option<DuperValue<'static>> {
        Some(DuperValue {
            identifier: Some(DuperIdentifier::try_from("Instant").expect("valid identifier")),
            inner: DuperInner::Temporal(
                DuperTemporal::try_instant_from(Cow::Owned(Utc::now().to_rfc3339()))
                    .expect("valid ISO-8601 Instant"),
            ),
        })
    }
}

#[cfg(feature = "chrono")]
#[derive(Debug)]
pub struct ChronoLocalTimer;

#[cfg(feature = "chrono")]
impl DuperTimer for ChronoLocalTimer {
    fn get_time(&self) -> Option<DuperValue<'static>> {
        Some(DuperValue {
            identifier: Some(DuperIdentifier::try_from("Instant").expect("valid identifier")),
            inner: DuperInner::Temporal(
                DuperTemporal::try_instant_from(Cow::Owned(Local::now().to_rfc3339()))
                    .expect("valid ISO-8601 Instant"),
            ),
        })
    }
}

#[cfg(feature = "chrono")]
#[derive(Debug)]
pub struct DuperSubscriber<Timer: std::fmt::Debug = ChronoLocalTimer> {
    timer: Timer,
    display_timestamp: bool,
    display_level: bool,
    display_target: bool,
    display_file: bool,
    display_line: bool,
    flatten_event: bool,
    display_current_span: bool,
    display_span_list: bool,
}

#[cfg(not(any(feature = "chrono")))]
#[derive(Debug)]
pub struct DuperSubscriber<Timer: std::fmt::Debug = ()> {
    timer: Timer,
    display_timestamp: bool,
    display_level: bool,
    display_target: bool,
    display_file: bool,
    display_line: bool,
    flatten_event: bool,
    display_current_span: bool,
    display_span_list: bool,
}

impl Default for DuperSubscriber {
    fn default() -> Self {
        #[cfg(feature = "chrono")]
        {
            Self {
                timer: ChronoLocalTimer,
                display_timestamp: true,
                display_level: true,
                display_target: true,
                display_file: false,
                display_line: false,
                flatten_event: false,
                display_current_span: true,
                display_span_list: true,
            }
        }
        #[cfg(not(any(feature = "chrono")))]
        {
            Self {
                timer: (),
                display_timestamp: true,
                display_level: true,
                display_target: true,
                display_file: false,
                display_line: false,
                flatten_event: false,
                display_current_span: true,
                display_span_list: true,
            }
        }
    }
}

impl DuperSubscriber {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<Timer: std::fmt::Debug> DuperSubscriber<Timer> {
    pub fn with_timer<Timer2: std::fmt::Debug>(self, timer: Timer2) -> DuperSubscriber<Timer2> {
        DuperSubscriber {
            timer,
            display_timestamp: self.display_timestamp,
            display_level: self.display_level,
            display_target: self.display_target,
            display_file: self.display_file,
            display_line: self.display_line,
            flatten_event: self.flatten_event,
            display_current_span: self.display_current_span,
            display_span_list: self.display_span_list,
        }
    }

    pub fn no_timer<Timer2>(self) -> DuperSubscriber<()> {
        DuperSubscriber {
            timer: (),
            display_timestamp: self.display_timestamp,
            display_level: self.display_level,
            display_target: self.display_target,
            display_file: self.display_file,
            display_line: self.display_line,
            flatten_event: self.flatten_event,
            display_current_span: self.display_current_span,
            display_span_list: self.display_span_list,
        }
    }

    pub fn with_timestamp(self, display_timestamp: bool) -> Self {
        Self {
            display_timestamp,
            ..self
        }
    }

    pub fn with_level(self, display_level: bool) -> Self {
        Self {
            display_level,
            ..self
        }
    }

    pub fn with_target(self, display_target: bool) -> Self {
        Self {
            display_target,
            ..self
        }
    }

    pub fn with_file(self, display_file: bool) -> Self {
        Self {
            display_file,
            ..self
        }
    }

    pub fn with_line(self, display_line: bool) -> Self {
        Self {
            display_line,
            ..self
        }
    }

    pub fn flatten_event(self, flatten_event: bool) -> Self {
        Self {
            flatten_event,
            ..self
        }
    }

    pub fn with_current_span(self, display_current_span: bool) -> Self {
        Self {
            display_current_span,
            ..self
        }
    }

    pub fn with_span_list(self, display_span_list: bool) -> Self {
        Self {
            display_span_list,
            ..self
        }
    }
}

impl<S, N, Timer> FormatEvent<S, N> for DuperSubscriber<Timer>
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
    Timer: DuperTimer + std::fmt::Debug,
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

        if self.display_timestamp
            && let Some(timestamp) = self.timer.get_time()
        {
            log.push((DuperKey::from("timestamp"), timestamp));
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

        if self.display_current_span || self.display_span_list {
            let current_span = event
                .parent()
                .and_then(|id| ctx.span(id))
                .or_else(|| ctx.lookup_current());
            if let Some(ref span) = current_span {
                let extensions = span.extensions();
                if self.display_current_span
                    && let Some(fields) = extensions.get::<FormattedFields<N>>()
                {
                    let value = duper::DuperParser::parse_duper_trunk(fields.as_str())
                        .expect("valid Duper value from FormattedFields");
                    log.push((DuperKey::from("span"), value.static_clone()));
                }
                if self.display_span_list {
                    let mut spans = vec![];
                    ctx.visit_spans(|span| {
                        let extensions = span.extensions();
                        if let Some(fields) = extensions.get::<FormattedFields<N>>() {
                            let value = duper::DuperParser::parse_duper_trunk(fields.as_str())
                                .expect("valid Duper value from FormattedFields");
                            spans.push(value.static_clone());
                        }
                        std::fmt::Result::Ok(())
                    })?;
                    log.push((
                        DuperKey::from("spans"),
                        DuperValue {
                            identifier: None,
                            inner: DuperInner::Array(DuperArray::from(spans)),
                        },
                    ));
                }
            }
        }

        let mut fields_visitor = DuperVisitor::new();
        event.record(&mut fields_visitor);
        let duper = fields_visitor.finish();
        if self.flatten_event
            && let DuperInner::Object(object) = duper.inner
        {
            log.extend(object.into_inner());
        } else {
            log.push((DuperKey::from("fields"), duper));
        }

        let mut serializer = Serializer::new(false, true);
        writeln!(
            writer,
            "{}",
            serializer.serialize(&DuperValue {
                identifier: None,
                inner: DuperInner::Object(DuperObject::from_lossy(log)),
            })
        )
    }
}

pub struct DuperFields;

impl<'writer> FormatFields<'writer> for DuperFields {
    fn format_fields<R: RecordFields>(
        &self,
        mut writer: tracing_subscriber::fmt::format::Writer<'writer>,
        fields: R,
    ) -> std::fmt::Result {
        let mut visitor = DuperVisitor::new();
        fields.record(&mut visitor);
        writer.write_str(&duper::Serializer::new(false, true).serialize(&visitor.finish()))
    }

    fn add_fields(
        &self,
        current: &'writer mut FormattedFields<Self>,
        fields: &tracing_core::span::Record<'_>,
    ) -> std::fmt::Result {
        if current.fields.is_empty() {
            self.format_fields(current.as_writer(), fields)
        } else {
            let mut visitor = DuperVisitor::new();
            let value = duper::DuperParser::parse_duper_trunk(current.as_str())
                .expect("valid Duper value in current FormattedFields");
            let DuperInner::Object(value) = value.inner else {
                unreachable!("invalid Duper value, expected object");
            };
            visitor.values.extend(value.into_inner());
            fields.record(&mut visitor);
            current.fields = duper::Serializer::new(false, true).serialize(&visitor.finish());
            Ok(())
        }
    }
}

pub struct DuperVisitor<'a> {
    values: BTreeMap<DuperKey<'a>, DuperValue<'a>>,
}

impl DuperVisitor<'_> {
    fn new() -> Self {
        Self {
            values: BTreeMap::new(),
        }
    }
}

impl std::fmt::Debug for DuperVisitor<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DuperVisitor")
            .field("values", &self.values)
            .finish()
    }
}

impl<'a> tracing_subscriber::field::VisitOutput<DuperValue<'a>> for DuperVisitor<'a> {
    fn finish(self) -> DuperValue<'a> {
        DuperValue {
            identifier: None,
            inner: DuperInner::Object(DuperObject::from(self.values)),
        }
    }
}

impl tracing_core::field::Visit for DuperVisitor<'_> {
    fn record_debug(&mut self, field: &tracing_core::Field, value: &dyn core::fmt::Debug) {
        self.values.insert(
            DuperKey::from(field.name()),
            DuperValue {
                identifier: Some(DuperIdentifier::try_from("Debug").expect("valid identifier")),
                inner: DuperInner::String(DuperString::from(format!("{:?}", value))),
            },
        );
    }

    fn record_f64(&mut self, field: &tracing_core::Field, value: f64) {
        self.values.insert(
            DuperKey::from(field.name()),
            DuperValue {
                identifier: None,
                inner: DuperInner::Float(value),
            },
        );
    }

    fn record_i64(&mut self, field: &tracing_core::Field, value: i64) {
        self.values.insert(
            DuperKey::from(field.name()),
            DuperValue {
                identifier: None,
                inner: DuperInner::Integer(value),
            },
        );
    }

    fn record_u64(&mut self, field: &tracing_core::Field, value: u64) {
        if let Ok(value) = i64::try_from(value) {
            self.values.insert(
                DuperKey::from(field.name()),
                DuperValue {
                    identifier: None,
                    inner: DuperInner::Integer(value),
                },
            );
        } else {
            self.values.insert(
                DuperKey::from(field.name()),
                DuperValue {
                    identifier: Some(DuperIdentifier::try_from("U64").expect("valid identifier")),
                    inner: DuperInner::String(DuperString::from(value.to_string())),
                },
            );
        }
    }

    fn record_i128(&mut self, field: &tracing_core::Field, value: i128) {
        if let Ok(value) = i64::try_from(value) {
            self.values.insert(
                DuperKey::from(field.name()),
                DuperValue {
                    identifier: None,
                    inner: DuperInner::Integer(value),
                },
            );
        } else {
            self.values.insert(
                DuperKey::from(field.name()),
                DuperValue {
                    identifier: Some(DuperIdentifier::try_from("I128").expect("valid identifier")),
                    inner: DuperInner::String(DuperString::from(value.to_string())),
                },
            );
        }
    }

    fn record_u128(&mut self, field: &tracing_core::Field, value: u128) {
        if let Ok(value) = i64::try_from(value) {
            self.values.insert(
                DuperKey::from(field.name()),
                DuperValue {
                    identifier: None,
                    inner: DuperInner::Integer(value),
                },
            );
        } else {
            self.values.insert(
                DuperKey::from(field.name()),
                DuperValue {
                    identifier: Some(DuperIdentifier::try_from("U128").expect("valid identifier")),
                    inner: DuperInner::String(DuperString::from(value.to_string())),
                },
            );
        }
    }

    fn record_bool(&mut self, field: &tracing_core::Field, value: bool) {
        self.values.insert(
            DuperKey::from(field.name()),
            DuperValue {
                identifier: None,
                inner: DuperInner::Boolean(value),
            },
        );
    }

    fn record_str(&mut self, field: &tracing_core::Field, value: &str) {
        self.values.insert(
            DuperKey::from(field.name()),
            DuperValue {
                identifier: None,
                inner: DuperInner::String(DuperString::from(value.to_string())),
            },
        );
    }

    fn record_bytes(&mut self, field: &tracing_core::Field, value: &[u8]) {
        self.values.insert(
            DuperKey::from(field.name()),
            DuperValue {
                identifier: None,
                inner: DuperInner::Bytes(DuperBytes::from(value.to_vec())),
            },
        );
    }

    fn record_error(
        &mut self,
        field: &tracing_core::Field,
        value: &(dyn std::error::Error + 'static),
    ) {
        self.values.insert(
            DuperKey::from(field.name()),
            DuperValue {
                identifier: Some(DuperIdentifier::try_from("Error").expect("valid identifier")),
                inner: DuperInner::String(DuperString::from(value.to_string())),
            },
        );
    }
}

#[cfg(test)]
mod tests {
    use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

    use super::{DuperFields, DuperSubscriber};

    #[test]
    fn create_duper_formatter() {
        let fmt_layer = fmt::layer()
            .fmt_fields(DuperFields)
            .event_format(DuperSubscriber::new());
        tracing_subscriber::registry().with(fmt_layer).init();
    }
}
