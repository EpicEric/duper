#![doc(html_logo_url = "https://duper.dev.br/logos/duper-100-100.png")]
//! A composable [`tracing_subscriber`] layer to emit Duper events.
//!
//! You can install it to a [`tracing_subscriber::registry`] just like any other
//! filter or formatter:
//!
//! ```
//! use tracing_duper::DuperLayer;
//! use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
//!
//! tracing_subscriber::registry()
//!     // ... add filtering layers ...
//!     .with(DuperLayer::new().with_span_timings(true))
//!     .init();
//! ```
//!
//! See [`DuperLayer`] for all the available configuration.
//!
//! Now, you can emit [`tracing`] spans and events as usual. If you'd like to
//! emit Duper values, use `$duper.` as the prefix of the field, and set the
//! value to a Duper-formatted string.
//!
//! ```ignore
//! use tracing::{debug, warn};
//!
//! #[tracing::instrument]
//! pub fn send_gifts(count: &mut usize) {
//!     if *count < 12 {
//!         warn!("too few gifts... try again later");
//!     } else {
//!         debug!(
//!             user_id = &b"santa"[..],
//!             "$duper.delivery_date" = "(PlainMonthDay('12-25'), \"Christmas\")",
//!             "sending {count} gifts"
//!         );
//!         std::thread::sleep(std::time::Duration::from_millis(100));
//!         *count = 0;
//!     }
//! }
//! ```
//!
//! This should print logs like the following to stdout:
//!
//! ```text
//! {level:"WARN",timestamp:Instant('2025-12-04T13:29:33.947380870-03:00'),target:"simple",span:{count:10,span_id:1},spans:[{count:10,span_id:1}],fields:{message:"too few gifts... try again later"}}
//! {level:"INFO",timestamp:Instant('2025-12-04T13:29:33.947608295-03:00'),target:"simple",span:{count:10,span_id:1},spans:[{count:10,span_id:1}],fields:{span_event:"closed","span_time.busy":Duration('PT0.000265196S'),"span_time.idle":Duration('PT0.000003567S')}}
//! {level:"DEBUG",timestamp:Instant('2025-12-04T13:29:33.948805983-03:00'),target:"simple",span:{count:23,span_id:2251799813685249},spans:[{count:23,span_id:2251799813685249}],fields:{delivery_date:(PlainMonthDay('12-25'),"Christmas"),message:"sending 23 gifts",user_id:b"santa"}}
//! {level:"INFO",timestamp:Instant('2025-12-04T13:29:34.049418692-03:00'),target:"simple",span:{count:23,span_id:2251799813685249},spans:[{count:23,span_id:2251799813685249}],fields:{span_event:"closed","span_time.busy":Duration('PT0.100555973S'),"span_time.idle":Duration('PT0.000001603S')}}
//! ```
//!
//! To create Duper values programmatically, look into [`duper`] and [`duper::Serializer`], or
//! the [`serde_duper`] crate.
//!
//! ## Feature flags
//!
//! - `chrono`: Use [`chrono`] to format log timestamps. Enabled by default.
//!

use std::{
    borrow::Cow,
    collections::BTreeMap,
    io::{self, Write},
    marker::PhantomData,
    time::Instant,
};

#[cfg(feature = "chrono")]
use chrono::{Local, Utc};
#[cfg(feature = "chrono")]
use duper::DuperTemporal;
use duper::{
    DuperArray, DuperBytes, DuperIdentifier, DuperInner, DuperKey, DuperObject, DuperString,
    DuperValue, Serializer,
};
use tracing_core::{Event, Subscriber, field};
use tracing_subscriber::{Layer, field::VisitOutput, registry::LookupSpan};

/// A trait to allow implementing timestamp generators.
pub trait DuperTimer {
    /// Returns the current timestamp as a Duper value, or `None` if one cannot
    /// be generated.
    fn get_timestamp(&self) -> Option<DuperValue<'static>>;
}

impl DuperTimer for () {
    fn get_timestamp(&self) -> Option<DuperValue<'static>> {
        None
    }
}

#[cfg(feature = "chrono")]
/// An Instant timestamp generator that uses [`chrono`] and includes the UTC timezone.
pub struct ChronoUtcTimer;

#[cfg(feature = "chrono")]
impl DuperTimer for ChronoUtcTimer {
    fn get_timestamp(&self) -> Option<DuperValue<'static>> {
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
/// An Instant timestamp generator that uses [`chrono`] and includes the local timezone.
pub struct ChronoLocalTimer;

#[cfg(feature = "chrono")]
impl DuperTimer for ChronoLocalTimer {
    fn get_timestamp(&self) -> Option<DuperValue<'static>> {
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
/// A [`tracing_subscriber::Layer`] that generates Duper-formatted logs to the
/// specified writer.
///
/// By default, logs will be written to stdout, using [`ChronoLocalTimer`] as
/// the timestamp generator.
pub struct DuperLayer<S, W = fn() -> io::Stdout, Timer = ChronoLocalTimer> {
    make_writer: W,
    timer: Timer,
    flatten_event: bool,
    display_timestamp: bool,
    display_level: bool,
    display_target: bool,
    display_file: bool,
    display_line: bool,
    display_span_timings: bool,
    display_current_span: bool,
    display_span_list: bool,
    _subscriber: PhantomData<S>,
}

#[cfg(not(feature = "chrono"))]
/// A [`tracing_subscriber::Layer`] that generates Duper-formatted logs to the
/// specified writer.
///
/// By default, logs will be written to stdout, with no timestamp generator.
pub struct DuperLayer<S, W = fn() -> io::Stdout, Timer = ()> {
    make_writer: W,
    timer: Timer,
    flatten_event: bool,
    display_timestamp: bool,
    display_level: bool,
    display_target: bool,
    display_file: bool,
    display_line: bool,
    display_span_timings: bool,
    display_current_span: bool,
    display_span_list: bool,
    _subscriber: PhantomData<S>,
}

impl<S> Default for DuperLayer<S> {
    fn default() -> Self {
        #[cfg(feature = "chrono")]
        {
            Self {
                make_writer: io::stdout,
                timer: ChronoLocalTimer,
                flatten_event: false,
                display_timestamp: true,
                display_level: true,
                display_target: true,
                display_file: false,
                display_line: false,
                display_span_timings: false,
                display_current_span: true,
                display_span_list: true,
                _subscriber: Default::default(),
            }
        }
        #[cfg(not(feature = "chrono"))]
        {
            Self {
                make_writer: io::stdout,
                timer: (),
                flatten_event: false,
                display_timestamp: true,
                display_level: true,
                display_target: true,
                display_file: false,
                display_line: false,
                display_span_timings: false,
                display_current_span: true,
                display_span_list: true,
                _subscriber: Default::default(),
            }
        }
    }
}

impl<S> DuperLayer<S> {
    /// Creates a new Duper layer with the default settings.
    pub fn new() -> Self {
        Self::default()
    }
}

impl<S, W, Timer> DuperLayer<S, W, Timer> {
    /// Sets the timer to use with this layer.
    pub fn with_timer<Timer2>(self, timer: Timer2) -> DuperLayer<S, W, Timer2> {
        DuperLayer {
            timer,
            make_writer: self.make_writer,
            flatten_event: self.flatten_event,
            display_timestamp: self.display_timestamp,
            display_level: self.display_level,
            display_target: self.display_target,
            display_file: self.display_file,
            display_line: self.display_line,
            display_span_timings: self.display_span_timings,
            display_current_span: self.display_current_span,
            display_span_list: self.display_span_list,
            _subscriber: Default::default(),
        }
    }

    /// Clear the timer from this layer, consequently disabling timestamps.
    pub fn without_timer(self) -> DuperLayer<S, W, ()> {
        DuperLayer {
            timer: (),
            make_writer: self.make_writer,
            flatten_event: self.flatten_event,
            display_timestamp: self.display_timestamp,
            display_level: self.display_level,
            display_target: self.display_target,
            display_file: self.display_file,
            display_line: self.display_line,
            display_span_timings: self.display_span_timings,
            display_current_span: self.display_current_span,
            display_span_list: self.display_span_list,
            _subscriber: Default::default(),
        }
    }

    /// Sets the writer factory function to use with this layer.
    pub fn with_writer<W2>(self, make_writer: W2) -> DuperLayer<S, W2, Timer> {
        DuperLayer {
            make_writer,
            timer: self.timer,
            flatten_event: self.flatten_event,
            display_timestamp: self.display_timestamp,
            display_level: self.display_level,
            display_target: self.display_target,
            display_file: self.display_file,
            display_line: self.display_line,
            display_span_timings: self.display_span_timings,
            display_current_span: self.display_current_span,
            display_span_list: self.display_span_list,
            _subscriber: Default::default(),
        }
    }

    /// Sets whether event fields will be flattened into the top-level of the
    /// log.
    ///
    /// NOTE: Keys that collide with existing ones will be discarded.
    pub fn flatten_event(self, flatten_event: bool) -> Self {
        Self {
            flatten_event,
            ..self
        }
    }

    /// Sets whether timestamps will be displayed in logs or not.
    pub fn with_timestamp(self, display_timestamp: bool) -> Self {
        Self {
            display_timestamp,
            ..self
        }
    }

    /// Sets whether log levels will be displayed in logs or not.
    pub fn with_level(self, display_level: bool) -> Self {
        Self {
            display_level,
            ..self
        }
    }

    /// Sets whether the target (i.e. the source of the trace) will be displayed
    /// in logs or not.
    pub fn with_target(self, display_target: bool) -> Self {
        Self {
            display_target,
            ..self
        }
    }

    /// Sets whether the file of the trace will be displayed in logs or not.
    pub fn with_file(self, display_file: bool) -> Self {
        Self {
            display_file,
            ..self
        }
    }

    /// Sets whether the line number of the trace will be displayed in logs or
    /// not.
    pub fn with_line(self, display_line: bool) -> Self {
        Self {
            display_line,
            ..self
        }
    }

    /// Sets whether the span timings will be displayed in logs or not.
    ///
    /// This has no effect if `display_current_span` and `display_span_list` are
    /// both unset.
    pub fn with_span_timings(self, display_span_timings: bool) -> Self {
        Self {
            display_span_timings,
            ..self
        }
    }

    /// Sets whether the current span will be displayed in logs or not.
    pub fn with_current_span(self, display_current_span: bool) -> Self {
        Self {
            display_current_span,
            ..self
        }
    }

    /// Sets whether the complete span list (current span, then each parent
    /// span) will be displayed in logs or not.
    pub fn with_span_list(self, display_span_list: bool) -> Self {
        Self {
            display_span_list,
            ..self
        }
    }

    /// Sets whether timings for this span (busy, idle) should be tracked. If
    /// set to true, an event will be emitted when the span is closed.
    fn track_timings(&self) -> bool {
        self.display_span_timings && (self.display_current_span || self.display_span_list)
    }
}

impl<S, W, Timer> Layer<S> for DuperLayer<S, W, Timer>
where
    S: Subscriber + for<'span> LookupSpan<'span>,
    W: for<'writer> tracing_subscriber::fmt::writer::MakeWriter<'writer> + 'static,
    Timer: DuperTimer + 'static,
{
    fn on_new_span(
        &self,
        attrs: &tracing_core::span::Attributes<'_>,
        id: &tracing_core::span::Id,
        ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        let span = ctx.span(id).expect("span should exist");
        let mut extensions = span.extensions_mut();

        let mut visitor = DuperVisitor::new();
        visitor.values.insert(
            DuperKey::from("span_id"),
            DuperValue {
                identifier: None,
                inner: DuperInner::Integer(id.into_u64() as i64),
            },
        );
        attrs.record(&mut visitor);
        let fields = visitor.finish();
        extensions.insert(DuperFields(fields));

        if self.track_timings() {
            extensions.insert(Timings::new());
        }
    }

    fn on_record(
        &self,
        id: &tracing_core::span::Id,
        values: &tracing_core::span::Record<'_>,
        ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        let span = ctx.span(id).expect("span should exist");
        let mut extensions = span.extensions_mut();
        if let Some(object) = extensions.get_mut::<DuperFields>() {
            let mut visitor = DuperVisitor::from(std::mem::replace(
                &mut object.0,
                DuperObject::try_from(vec![]).expect("empty object is valid"),
            ));
            values.record(&mut visitor);
            *object = DuperFields(visitor.finish());
            return;
        }

        let mut visitor = DuperVisitor::new();
        values.record(&mut visitor);
        let fields = visitor.finish();
        extensions.insert(DuperFields(fields));
    }

    fn on_event(&self, event: &Event<'_>, ctx: tracing_subscriber::layer::Context<'_, S>) {
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
            && let Some(timestamp) = self.timer.get_timestamp()
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
                    && let Some(fields) = extensions.get::<DuperFields>()
                {
                    log.push((
                        DuperKey::from("span"),
                        DuperValue {
                            identifier: None,
                            inner: DuperInner::Object(fields.0.clone()),
                        },
                    ));
                }
                if self.display_span_list {
                    let mut spans = vec![];
                    if let Some(scope) = ctx.event_scope(event) {
                        for span in scope.from_root() {
                            let extensions = span.extensions();
                            if let Some(fields) = extensions.get::<DuperFields>() {
                                spans.push(DuperValue {
                                    identifier: None,
                                    inner: DuperInner::Object(fields.0.clone()),
                                });
                            }
                        }
                    }
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
        let object = fields_visitor.finish();
        if self.flatten_event {
            log.extend(object.into_inner());
        } else {
            log.push((
                DuperKey::from("fields"),
                DuperValue {
                    identifier: None,
                    inner: DuperInner::Object(object),
                },
            ));
        }

        let mut serializer = Serializer::new(false, true);
        if let Err(error) = writeln!(
            self.make_writer.make_writer_for(event.metadata()),
            "{}",
            serializer.serialize(&DuperValue {
                identifier: None,
                inner: DuperInner::Object(DuperObject::from_lossy(log)),
            })
        ) {
            let _ = error;
        }
    }

    fn on_enter(
        &self,
        id: &tracing_core::span::Id,
        ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        if self.track_timings() {
            let span = ctx.span(id).expect("span should exist");
            if let Some(timings) = span.extensions_mut().get_mut::<Timings>() {
                if timings.entered_count == 0 {
                    let now = Instant::now();
                    timings.idle += (now - timings.last).as_nanos() as i64;
                    timings.last = now;
                }
                timings.entered_count += 1;
            }
        }
    }

    fn on_exit(&self, id: &tracing_core::span::Id, ctx: tracing_subscriber::layer::Context<'_, S>) {
        if self.track_timings() {
            let span = ctx.span(id).expect("span should exist");
            if let Some(timings) = span.extensions_mut().get_mut::<Timings>() {
                timings.entered_count -= 1;
                if timings.entered_count == 0 {
                    let now = Instant::now();
                    timings.busy += (now - timings.last).as_nanos() as i64;
                    timings.last = now;
                }
            }
        }
    }

    fn on_close(&self, id: tracing_core::span::Id, ctx: tracing_subscriber::layer::Context<'_, S>) {
        if self.track_timings() {
            let span = ctx.span(&id).expect("span should exist");
            let mut extensions = span.extensions_mut();
            if let Some(timings) = extensions.remove::<Timings>() {
                let metadata = span.metadata();
                let callsite = metadata.callsite();
                let field_set = field::FieldSet::new(
                    &[
                        "span_event",
                        "$duper.span_time.busy",
                        "$duper.span_time.idle",
                    ],
                    callsite,
                );
                let mut fields = field_set.iter();
                let busy_duration = format!(
                    "Duration('PT{}.{:09}S')",
                    timings.busy / 1_000_000_000,
                    timings.busy % 1_000_000_000
                );
                let idle_duration = format!(
                    "Duration('PT{}.{:09}S')",
                    timings.idle / 1_000_000_000,
                    timings.idle % 1_000_000_000
                );
                let values = [
                    (
                        &fields.next().unwrap(),
                        Some(&"closed" as &dyn field::Value),
                    ),
                    (
                        &fields.next().unwrap(),
                        Some(&busy_duration as &dyn field::Value),
                    ),
                    (
                        &fields.next().unwrap(),
                        Some(&idle_duration as &dyn field::Value),
                    ),
                ];
                let value_set = field_set.value_set(&values);
                let event = Event::new_child_of(id, metadata, &value_set);
                drop(extensions);
                drop(span);
                self.on_event(&event, ctx);
            }
        }
    }
}

struct DuperFields<'a>(DuperObject<'a>);

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

impl<'a> From<DuperObject<'a>> for DuperVisitor<'a> {
    fn from(value: DuperObject<'a>) -> Self {
        Self {
            values: value.into_inner().into_iter().collect(),
        }
    }
}

impl<'a> tracing_subscriber::field::VisitOutput<DuperObject<'a>> for DuperVisitor<'a> {
    fn finish(self) -> DuperObject<'a> {
        DuperObject::from(self.values)
    }
}

impl tracing_core::field::Visit for DuperVisitor<'_> {
    fn record_debug(&mut self, field: &tracing_core::Field, value: &dyn core::fmt::Debug) {
        let key = DuperKey::from(field.name());
        if self.values.contains_key(&key) {
            return;
        }
        self.values.insert(
            key,
            DuperValue {
                identifier: None,
                inner: DuperInner::String(DuperString::from(format!("{:?}", value))),
            },
        );
    }

    fn record_f64(&mut self, field: &tracing_core::Field, value: f64) {
        let key = DuperKey::from(field.name());
        if self.values.contains_key(&key) {
            return;
        }
        self.values.insert(
            key,
            DuperValue {
                identifier: None,
                inner: DuperInner::Float(value),
            },
        );
    }

    fn record_i64(&mut self, field: &tracing_core::Field, value: i64) {
        let key = DuperKey::from(field.name());
        if self.values.contains_key(&key) {
            return;
        }
        self.values.insert(
            key,
            DuperValue {
                identifier: None,
                inner: DuperInner::Integer(value),
            },
        );
    }

    fn record_u64(&mut self, field: &tracing_core::Field, value: u64) {
        let key = DuperKey::from(field.name());
        if self.values.contains_key(&key) {
            return;
        }
        if let Ok(value) = i64::try_from(value) {
            self.values.insert(
                key,
                DuperValue {
                    identifier: None,
                    inner: DuperInner::Integer(value),
                },
            );
        } else {
            self.values.insert(
                key,
                DuperValue {
                    identifier: Some(DuperIdentifier::try_from("U64").expect("valid identifier")),
                    inner: DuperInner::String(DuperString::from(value.to_string())),
                },
            );
        }
    }

    fn record_i128(&mut self, field: &tracing_core::Field, value: i128) {
        let key = DuperKey::from(field.name());
        if self.values.contains_key(&key) {
            return;
        }
        if let Ok(value) = i64::try_from(value) {
            self.values.insert(
                key,
                DuperValue {
                    identifier: None,
                    inner: DuperInner::Integer(value),
                },
            );
        } else {
            self.values.insert(
                key,
                DuperValue {
                    identifier: Some(DuperIdentifier::try_from("I128").expect("valid identifier")),
                    inner: DuperInner::String(DuperString::from(value.to_string())),
                },
            );
        }
    }

    fn record_u128(&mut self, field: &tracing_core::Field, value: u128) {
        let key = DuperKey::from(field.name());
        if self.values.contains_key(&key) {
            return;
        }

        if let Ok(value) = i64::try_from(value) {
            self.values.insert(
                key,
                DuperValue {
                    identifier: None,
                    inner: DuperInner::Integer(value),
                },
            );
        } else {
            self.values.insert(
                key,
                DuperValue {
                    identifier: Some(DuperIdentifier::try_from("U128").expect("valid identifier")),
                    inner: DuperInner::String(DuperString::from(value.to_string())),
                },
            );
        }
    }

    fn record_bool(&mut self, field: &tracing_core::Field, value: bool) {
        let key = DuperKey::from(field.name());
        if self.values.contains_key(&key) {
            return;
        }
        self.values.insert(
            key,
            DuperValue {
                identifier: None,
                inner: DuperInner::Boolean(value),
            },
        );
    }

    fn record_str(&mut self, field: &tracing_core::Field, value: &str) {
        let key = field.name();
        if let Some(suffix) = key.strip_prefix("$duper.")
            && let Ok(value) = duper::DuperParser::parse_duper_value(value)
        {
            let key = DuperKey::from(suffix);
            if self.values.contains_key(&key) {
                return;
            }
            self.values.insert(key, value.static_clone());
        } else {
            let key = DuperKey::from(key);
            if self.values.contains_key(&key) {
                return;
            }
            self.values.insert(
                key,
                DuperValue {
                    identifier: None,
                    inner: DuperInner::String(DuperString::from(value.to_string())),
                },
            );
        }
    }

    fn record_bytes(&mut self, field: &tracing_core::Field, value: &[u8]) {
        let key = DuperKey::from(field.name());
        if self.values.contains_key(&key) {
            return;
        }
        self.values.insert(
            key,
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
        let key = DuperKey::from(field.name());
        if self.values.contains_key(&key) {
            return;
        }
        self.values.insert(
            key,
            DuperValue {
                identifier: Some(DuperIdentifier::try_from("Error").expect("valid identifier")),
                inner: DuperInner::String(DuperString::from(value.to_string())),
            },
        );
    }
}

struct Timings {
    idle: i64,
    busy: i64,
    last: Instant,
    entered_count: u64,
}

impl Timings {
    fn new() -> Self {
        Self {
            idle: 0,
            busy: 0,
            last: Instant::now(),
            entered_count: 0,
        }
    }
}
