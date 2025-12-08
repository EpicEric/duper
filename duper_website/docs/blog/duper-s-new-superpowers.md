---
title: Duper's new superpowers!
---

```duper
BlogPost({
  title: "Duper's new superpowers!",
  publish_date: PlainDate('2025-12-08'),
  author: ("Eric Rodrigues Pires", Email("eric@eric.dev.br")),
})
```

# Duper's new superpowers!

<p align="center">
  <img src="/images/blog/header-duper-s-new-superpowers.webp" alt="The Duper mole, sitting down and reading a paper sheet." />
</p>

[Duper's specification version 0.4.1](/spec) has been released, and with it, a plethora of exciting changes have come to the ecosystem!

For those who don't know, Duper is a modern extension of JSON, with unquoted keys, comments, trailing commas, byte strings, type-like identifiers, and more. Whether you're working with configuration files, logs, REST APIs, or just want an easier way to write JSON files (such as [TextMate grammars](https://github.com/EpicEric/duper/blob/main/duper_website/duper.tmLanguage.duper)) before transpiling, then Duper might be the choice for you.

## A new specification... It's about time!

The new Duper specification includes a lot of clean-up and fixes, as well as a collection of features you've been asking for:

### Temporal support

The initial release of Duper lacked any sort of date-time support, which felt like a step backwards compared to YAML and TOML. Well, no more! Not only does it have support for those types, but many more via full support to the [Temporal specification](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal).

Time support is far from trivial, and the folks from ECMAScript have landed on an official API, already available on Firefox and with support soon coming to [Chrome](https://issues.chromium.org/issues/401065166) and [Node.JS](https://github.com/nodejs/node/issues/57127). And with Duper, you can start using it today! Simply use the new single quotes syntax `'...'` to get started:

```duper
{
  instant: '1994-11-06T19:45:27-03:00',
  duration: 'P1W6DT5.000001S',
  zoned_date_time: '2020-05-22T07:19:35.123456789-04:00[America/Indiana/Indianapolis][u-ca=islamic-umalqura]',

  not_temporal: 'hello world',         // INVALID // [!code error]
  "date doesn't exist": '2025-02-29',  // INVALID // [!code error]
}
```

With Duper's identifiers, you can go even further beyond: values will be type-checked during parsing!

```duper
{
  plain_date_time: PlainDateTime('2007-03-31T10:35:10'),
  subset: PlainYearMonth('1994-11-06T19:45:27-03:00'),  // 👍 PlainYearMonth is a subset of Instant

  wrong_type: Duration('2025-10-31T19:39:02'),  // INVALID // [!code error]
}
```

### Base64 byte strings

Duper had support for byte strings from the start, allowing users to easily pass binary data around. However, it was limited to manually escaped data or raw byte strings:

```duper
{
  ansi_reset: b"\x1b[0m",
  windows_path: br"C:\Windows\System32",
}
```

For longer binary data that is not human-readable, Duper now supports Base64-encoded byte strings for compact transmission (note the `b64` prefix):

```duper
{
  secret: b64"ZHVwZXI=",
}
```

### EBNF grammar and railroad diagram

Despite the comprehensive specification, there was still the need for a formal grammar. The latest specification now includes an [Extended BNF grammar](/spec#w3c-ebnf-grammar), as well as a [railroad diagram](/diagram.html){target="_blank"} generated with [`rr`](https://www.bottlecaps.de/rr/).

## A polyglot mole

Learning new languages is a valuable skill, and for a portable file format, that's no different. Thanks to its core library being written in Rust, Duper has been ported to a number of different languages. Aside from the existing Python and WebAssembly support, it's now available for:

- C# / .NET (via [UniFFI](https://github.com/mozilla/uniffi-rs) and [uniffi-bindgen-cs](https://github.com/NordSecurity/uniffi-bindgen-cs))
- Node.JS (via [NAPI-RS](https://napi.rs))

In fact, even the WASM bindings have been migrated to [uniffi-bindgen-react-native](https://github.com/jhugman/uniffi-bindgen-react-native), hopefully making the code more maintainable in the long-term.

Thank you to the Rust community for the brilliant libraries that have made these ports possible! Our hope is that Duper is available to even more languages and frameworks, ensuring a better user experience for developers all around.

## New tools and gadgets

A modern format requires modern tools, and this update features a bunch of new goodies for all Duper hackers!

### Log analysis with `duperq`

With logging support as one of its main selling points, it's only natural that Duper would develop its own log analysis tool. And that's exactly what [`duperq`](/tools/duperq) is: a fast filter/formatter for processing logs and files from your applications. Inspired by tools like [`jq`](https://jqlang.org/) and [`hl`](https://github.com/pamburus/hl/), it's got powerful filtering capabilities so you can find the information you need with a single query.

If all you have are JSON logs, you can start using `duperq` today - after all, Duper is a superset of JSON!

### Rust tracing with `tracing_duper`

Of course, you can make the most out of `duperq` if your logs are already in Duper. If you're a Rust user and would like to upgrade your `tracing` logs, check out the brand-new [`tracing_duper`](https://docs.rs/tracing_duper/latest/tracing_duper/) crate. Here's a little taste:

```rust
use tracing::{debug, warn};
use tracing_duper::DuperLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tracing::instrument]
fn send_gifts(count: &mut usize) {
    if *count < 12 {
        warn!("too few gifts... try again later");
    } else {
        debug!(
            user_id = &b"santa"[..],
            "$duper.delivery_date" = "(PlainMonthDay('12-25'), \"Christmas\")",
            "sending {count} gifts"
        );
        std::thread::sleep(std::time::Duration::from_millis(100));
        *count = 0;
    }
}

fn main() {
    tracing_subscriber::registry()
        .with(DuperLayer::new().with_span_timings(true))
        .init();
    let mut gifts = 10;
    send_gifts(&mut gifts);
    gifts += 13;
    send_gifts(&mut gifts);
}
```

```duper
{level:"WARN",timestamp:Instant('2025-12-08T07:28:30.395620087-03:00'),target:"simple",span:{count:10,span_id:1},spans:[{count:10,span_id:1}],fields:{message:"too few gifts... try again later"}}
{level:"INFO",timestamp:Instant('2025-12-08T07:28:30.395850438-03:00'),target:"simple",span:{count:10,span_id:1},spans:[{count:10,span_id:1}],fields:{span_event:"closed","span_time.busy":Duration('PT0.000230591S'),"span_time.idle":Duration('PT0.000003567S')}}
{level:"DEBUG",timestamp:Instant('2025-12-08T07:28:30.396951896-03:00'),target:"simple",span:{count:23,span_id:2251799813685249},spans:[{count:23,span_id:2251799813685249}],fields:{delivery_date:(PlainMonthDay('12-25'),"Christmas"),message:"sending 23 gifts",user_id:b"santa"}}
{level:"INFO",timestamp:Instant('2025-12-08T07:28:30.497574820-03:00'),target:"simple",span:{count:23,span_id:2251799813685249},spans:[{count:23,span_id:2251799813685249}],fields:{span_event:"closed","span_time.busy":Duration('PT0.100613786S'),"span_time.idle":Duration('PT0.000002164S')}}
```

### Language server with `duper_lsp`

Language servers provide developers with superpowers when working with a language. Although Duper is simpler than any programming language, it still would benefit from diagnostics, auto-formatting, and syntax highlighting as much as any other encoding.

There's no need to wait! The VSCode / VSCodium extension has been upgraded with a bundled language server via `duper_lsp`, whatever your OS may be. Powered by [`tree-sitter`](https://tree-sitter.github.io/tree-sitter/) and [`async-lsp`](https://docs.rs/async-lsp/latest/async_lsp/), it provides fast feedback, making hand-writing Duper files as easy as you'd expect.

<video controls muted>
  <source src="/videos/blog/lsp-example.mp4" type="video/mp4" />
</video>

If you'd like to use formatting as a standalone CLI instead, check out [`duperfmt`](https://crates.io/crates/duperfmt), the library that powers formatting in the LSP.

## A kaiju-sized refactoring

With great power comes great responsibility, and Duper's newest changes had deep-felt impacts in how the core Rust library was implemented.

Most relevant is how Temporal values added new constraints to identifiers and values - now, type-checking has to be done at parsing. This brought new challenges, and the base format had to be wholly refactored for type safety. In terms of the Rust implementation, Duper values have gone from this:

```rust
pub struct DuperValue<'a> {
    pub identifier: Option<DuperIdentifier<'a>>,
    pub inner: DuperInner<'a>,
}

pub enum DuperInner<'a> {
    Object(DuperObject<'a>),
    Array(DuperArray<'a>),
    Tuple(DuperTuple<'a>),
    String(DuperString<'a>),
    Bytes(DuperBytes<'a>),
    Temporal(DuperTemporal<'a>),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Null,
}

pub enum DuperTemporal<'a> {
    Instant(DuperTemporalInner<'a>),
    ZonedDateTime(DuperTemporalInner<'a>),
    PlainDate(DuperTemporalInner<'a>),
    PlainTime(DuperTemporalInner<'a>),
    PlainDateTime(DuperTemporalInner<'a>),
    PlainYearMonth(DuperTemporalInner<'a>),
    PlainMonthDay(DuperTemporalInner<'a>),
    Duration(DuperTemporalInner<'a>),
    Unspecified(DuperTemporalInner<'a>),
}
```

To this:

```rust
pub enum DuperValue<'a> {
    Object {
        identifier: Option<DuperIdentifier<'a>>,
        inner: DuperObject<'a>,
    },
    Array {
        identifier: Option<DuperIdentifier<'a>>,
        inner: Vec<DuperValue<'a>>,
    },
    Tuple {
        identifier: Option<DuperIdentifier<'a>>,
        inner: Vec<DuperValue<'a>>,
    },
    String {
        identifier: Option<DuperIdentifier<'a>>,
        inner: Cow<'a, str>,
    },
    Bytes {
        identifier: Option<DuperIdentifier<'a>>,
        inner: Cow<'a, [u8]>,
    },
    Temporal(DuperTemporal<'a>),
    Integer {
        identifier: Option<DuperIdentifier<'a>>,
        inner: i64,
    },
    Float {
        identifier: Option<DuperIdentifier<'a>>,
        inner: f64,
    },
    Boolean {
        identifier: Option<DuperIdentifier<'a>>,
        inner: bool,
    },
    Null {
        identifier: Option<DuperIdentifier<'a>>,
    },
}

pub enum DuperTemporal<'a> {
    Instant {
        inner: DuperTemporalInstant<'a>,
    },
    ZonedDateTime {
        inner: DuperTemporalZonedDateTime<'a>,
    },
    PlainDate {
        inner: DuperTemporalPlainDate<'a>,
    },
    PlainTime {
        inner: DuperTemporalPlainTime<'a>,
    },
    PlainDateTime {
        inner: DuperTemporalPlainDateTime<'a>,
    },
    PlainYearMonth {
        inner: DuperTemporalPlainYearMonth<'a>,
    },
    PlainMonthDay {
        inner: DuperTemporalPlainMonthDay<'a>,
    },
    Duration {
        inner: DuperTemporalDuration<'a>,
    },
    Unspecified {
        identifier: Option<DuperTemporalIdentifier<'a>>,
        inner: DuperTemporalUnspecified<'a>,
    },
}
```

Most users won't see a difference upgrading from 0.4 to 0.5, but you can be confident that Duper is now less error-prone and more super than ever.

## ...and that's it!

Whew, that's all for now! There's been a lot to fit in a single update, yet I feel that there could've been more. Granted, Duper remains a single-developer project done entirely on my free time. Still, the comprehensive Rust ecosystem has really expedited the creation of a lot of exciting features.

While the long-term goal is to get the format stable enough for specification v1.0, there are still improvements to be had on the Rust side (mainly reducing compilation times), on general support (more editor extensions, more supported languages), and on improved tooling. Until Duper sees wider adoption, it doesn't make sense to stabilize the format - so if any of this sounded interesting to you, [check out the website](https://duper.dev.br) and give it a try!
