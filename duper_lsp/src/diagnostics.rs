use std::{
    borrow::Cow,
    collections::HashSet,
    net::{IpAddr, Ipv4Addr, Ipv6Addr},
    str::FromStr,
    sync::LazyLock,
};

use base64::Engine;
use ipnet::{IpNet, Ipv4Net, Ipv6Net};
use line_index::{LineCol, LineIndex, WideEncoding};
use lsp_types::{Diagnostic, DiagnosticSeverity};
use regex::Regex;
use tracing::{error, warn};
use tree_sitter::{Query, QueryCursor, StreamingIterator, Tree};

static QUERY_ERRORS: LazyLock<Query> = LazyLock::new(|| {
    let text = r"(ERROR) @error";

    Query::new(&tree_sitter_duper::LANGUAGE.into(), text).expect("valid errors query")
});

static QUERY_MISSING: LazyLock<Query> = LazyLock::new(|| {
    let text = r"(MISSING) @missing";

    Query::new(&tree_sitter_duper::LANGUAGE.into(), text).expect("valid missing query")
});

static QUERY_OBJECT_KEYS: LazyLock<Query> = LazyLock::new(|| {
    let text = r#"
        (object
            .
            [(line_comment) (block_comment)]*
            .
            (
                (object_entry
                    (object_key [
                        (plain_key) @plain
                        (quoted_string
                            (quoted_content) @quote
                        )
                        (raw_string
                            (raw_content) @raw
                        )
                    ])
                )
                _*
            )*
        )"#;

    Query::new(&tree_sitter_duper::LANGUAGE.into(), text).expect("valid object keys query")
});

static QUERY_UNIDENTIFIED_QUOTED_STRINGS: LazyLock<Query> = LazyLock::new(|| {
    let text = r#"
        (duper_value
            (string
                (quoted_string
                    (quoted_content) @quote)))"#;

    Query::new(&tree_sitter_duper::LANGUAGE.into(), text)
        .expect("valid unidentified quoted strings query")
});

static QUERY_QUOTED_BYTES: LazyLock<Query> = LazyLock::new(|| {
    let text = r#"
        (bytes
            (quoted_bytes
                (quoted_content) @quote))"#;

    Query::new(&tree_sitter_duper::LANGUAGE.into(), text).expect("valid quoted bytes query")
});

static QUERY_BASE64_BYTES: LazyLock<Query> = LazyLock::new(|| {
    let text = r#"
        (bytes
            (base64_bytes
                (base64_content) @base64))"#;

    Query::new(&tree_sitter_duper::LANGUAGE.into(), text).expect("valid base64 bytes query")
});

static QUERY_IDENTIFIED_TEMPORAL: LazyLock<Query> = LazyLock::new(|| {
    let text = r#"
        (duper_value
            (identified_value
                (identifier) @identifier
                (temporal
                    (temporal_content) @temporal)))"#;

    Query::new(&tree_sitter_duper::LANGUAGE.into(), text).expect("valid identified Temporal query")
});

static QUERY_UNSPECIFIED_TEMPORAL: LazyLock<Query> = LazyLock::new(|| {
    let text = r#"
        (duper_value
            (temporal
                (temporal_content) @temporal))"#;

    Query::new(&tree_sitter_duper::LANGUAGE.into(), text).expect("valid unspecified Temporal query")
});

static QUERY_IDENTIFIED_STRINGS: LazyLock<Query> = LazyLock::new(|| {
    let text = r#"
        (duper_value
            (identified_value
                (identifier) @identifier
                (string [
                    (quoted_string
                        (quoted_content) @quote
                    )
                    (raw_string
                        (raw_content) @raw
                    )
                ])))"#;

    Query::new(&tree_sitter_duper::LANGUAGE.into(), text).expect("valid identified strings query")
});

static QUERY_INTEGERS: LazyLock<Query> = LazyLock::new(|| {
    let text = r#"
        (integer) @integer
    "#;

    Query::new(&tree_sitter_duper::LANGUAGE.into(), text).expect("valid integers query")
});

static QUERY_FLOATS: LazyLock<Query> = LazyLock::new(|| {
    let text = r#"
        (float) @float
    "#;

    Query::new(&tree_sitter_duper::LANGUAGE.into(), text).expect("valid floats query")
});

static REGEX_UUID: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)^UUID(v?\d)?$").expect("valid UUID regex"));

static REGEX_DECIMAL: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)^Decimal$").expect("valid Decimal regex"));

static REGEX_IP4ADDR: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)^IPv?4(Addr(ess)?)?$").expect("valid IPv4 address regex"));

static REGEX_IP6ADDR: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)^IPv?6(Addr(ess)?)?$").expect("valid IPv6 address regex"));

static REGEX_IPADDR: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)^IP(Addr(ess)?)?$").expect("valid IP address regex"));

static REGEX_IP4NET: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)^IPv?4Net(work)?$").expect("valid IPv4 network regex"));

static REGEX_IP6NET: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)^IPv?6Net(work)?$").expect("valid IPv6 network regex"));

static REGEX_IPNET: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)^IPNet(work)?$").expect("valid IP network regex"));

pub(crate) fn get_diagnostics(source: &str, tree: &Tree, is_utf8: bool) -> Vec<Diagnostic> {
    let mut diagnostics = vec![];
    let index = LineIndex::new(source);

    // Return errors
    let query = &QUERY_ERRORS;
    let mut cursor = QueryCursor::new();
    let mut captures = cursor.captures(query, tree.root_node(), source.as_bytes());
    while let Some((m, _)) = {
        captures.advance();
        captures.get()
    } {
        diagnostics.extend(m.captures.iter().map(|c| {
            Diagnostic::new(
                to_range(c.node.range(), &index, is_utf8),
                Some(DiagnosticSeverity::ERROR),
                None,
                None,
                "Syntax error".into(),
                None,
                None,
            )
        }));
    }

    // Return missing values
    let query = &QUERY_MISSING;
    let mut cursor = QueryCursor::new();
    let mut captures = cursor.captures(query, tree.root_node(), source.as_bytes());
    while let Some((m, _)) = {
        captures.advance();
        captures.get()
    } {
        diagnostics.extend(m.captures.iter().map(|c| {
            Diagnostic::new(
                to_range(c.node.range(), &index, is_utf8),
                Some(DiagnosticSeverity::ERROR),
                None,
                None,
                "Missing value".into(),
                None,
                None,
            )
        }));
    }

    // Check that objects don't have duplicate keys
    let query = &QUERY_OBJECT_KEYS;
    let mut cursor = QueryCursor::new();
    let mut matches = cursor.matches(query, tree.root_node(), source.as_bytes());
    while let Some(m) = {
        matches.advance();
        matches.get()
    } {
        let mut keys = HashSet::<Cow<'_, str>>::new();
        for capture in m.captures {
            let capture_name = query.capture_names()[capture.index as usize];
            let node = capture.node;
            match capture_name {
                "plain" => match node.utf8_text(source.as_bytes()) {
                    Ok(str) => {
                        if keys.contains(str) {
                            diagnostics.push(Diagnostic::new(
                                to_range(node.range(), &index, is_utf8),
                                Some(DiagnosticSeverity::ERROR),
                                None,
                                None,
                                "Duplicate key".to_string(),
                                None,
                                None,
                            ))
                        } else {
                            keys.insert(Cow::Borrowed(str));
                        }
                    }
                    Err(err) => diagnostics.push(Diagnostic::new(
                        to_range(node.range(), &index, is_utf8),
                        Some(DiagnosticSeverity::ERROR),
                        None,
                        None,
                        format!("Invalid UTF-8: {err}"),
                        None,
                        None,
                    )),
                },
                "quote" => match node.utf8_text(source.as_bytes()) {
                    Ok(escaped_str) => match duper::escape::unescape_str(escaped_str) {
                        Ok(str) => {
                            if keys.contains(str.as_ref()) {
                                diagnostics.push(Diagnostic::new(
                                    to_range(node.range(), &index, is_utf8),
                                    Some(DiagnosticSeverity::ERROR),
                                    None,
                                    None,
                                    "Duplicate key".to_string(),
                                    None,
                                    None,
                                ))
                            } else {
                                keys.insert(str);
                            }
                        }
                        Err(err) => diagnostics.push(Diagnostic::new(
                            to_range(node.range(), &index, is_utf8),
                            Some(DiagnosticSeverity::ERROR),
                            None,
                            None,
                            format!("Invalid string: {err}"),
                            None,
                            None,
                        )),
                    },
                    Err(err) => diagnostics.push(Diagnostic::new(
                        to_range(node.range(), &index, is_utf8),
                        Some(DiagnosticSeverity::ERROR),
                        None,
                        None,
                        format!("Invalid UTF-8: {err}"),
                        None,
                        None,
                    )),
                },
                "raw" => match node.utf8_text(source.as_bytes()) {
                    Ok(str) => {
                        if keys.contains(str) {
                            diagnostics.push(Diagnostic::new(
                                to_range(node.range(), &index, is_utf8),
                                Some(DiagnosticSeverity::ERROR),
                                None,
                                None,
                                "Duplicate key".to_string(),
                                None,
                                None,
                            ))
                        } else {
                            keys.insert(Cow::Borrowed(str));
                        }
                    }
                    Err(err) => diagnostics.push(Diagnostic::new(
                        to_range(node.range(), &index, is_utf8),
                        Some(DiagnosticSeverity::ERROR),
                        None,
                        None,
                        format!("Invalid UTF-8: {err}"),
                        None,
                        None,
                    )),
                },
                _ => {
                    error!(
                        names = ?query.capture_names(),
                        index = capture.index,
                        "Unknown capture name"
                    )
                }
            }
        }
    }

    // Check escaping on unidentified string values
    let query = &QUERY_UNIDENTIFIED_QUOTED_STRINGS;
    let mut cursor = QueryCursor::new();
    let mut matches = cursor.matches(query, tree.root_node(), source.as_bytes());
    while let Some(m) = {
        matches.advance();
        matches.get()
    } {
        for capture in m.captures {
            let capture_name = query.capture_names()[capture.index as usize];
            let node = capture.node;
            match capture_name {
                "quote" => match node.utf8_text(source.as_bytes()) {
                    Ok(escaped_str) => {
                        if let Err(err) = duper::escape::unescape_str(escaped_str) {
                            diagnostics.push(Diagnostic::new(
                                to_range(node.range(), &index, is_utf8),
                                Some(DiagnosticSeverity::ERROR),
                                None,
                                None,
                                format!("Invalid string: {err}"),
                                None,
                                None,
                            ));
                        }
                    }
                    Err(err) => diagnostics.push(Diagnostic::new(
                        to_range(node.range(), &index, is_utf8),
                        Some(DiagnosticSeverity::ERROR),
                        None,
                        None,
                        format!("Invalid UTF-8: {err}"),
                        None,
                        None,
                    )),
                },
                _ => {
                    error!(
                        names = ?query.capture_names(),
                        index = capture.index,
                        "Unknown capture name"
                    )
                }
            }
        }
    }

    // Check escaping on bytes values
    let query = &QUERY_QUOTED_BYTES;
    let mut cursor = QueryCursor::new();
    let mut matches = cursor.matches(query, tree.root_node(), source.as_bytes());
    while let Some(m) = {
        matches.advance();
        matches.get()
    } {
        for capture in m.captures {
            let capture_name = query.capture_names()[capture.index as usize];
            let node = capture.node;
            match capture_name {
                "quote" => match node.utf8_text(source.as_bytes()) {
                    Ok(escaped_bytes) => {
                        if let Err(err) = duper::escape::unescape_bytes(escaped_bytes) {
                            diagnostics.push(Diagnostic::new(
                                to_range(node.range(), &index, is_utf8),
                                Some(DiagnosticSeverity::ERROR),
                                None,
                                None,
                                format!("Invalid bytes: {err}"),
                                None,
                                None,
                            ));
                        }
                    }
                    Err(err) => diagnostics.push(Diagnostic::new(
                        to_range(node.range(), &index, is_utf8),
                        Some(DiagnosticSeverity::ERROR),
                        None,
                        None,
                        format!("Invalid UTF-8: {err}"),
                        None,
                        None,
                    )),
                },
                _ => {
                    error!(
                        names = ?query.capture_names(),
                        index = capture.index,
                        "Unknown capture name"
                    )
                }
            }
        }
    }

    // Check Base64 bytes values
    let query = &QUERY_BASE64_BYTES;
    let mut cursor = QueryCursor::new();
    let mut matches = cursor.matches(query, tree.root_node(), source.as_bytes());
    while let Some(m) = {
        matches.advance();
        matches.get()
    } {
        for capture in m.captures {
            let capture_name = query.capture_names()[capture.index as usize];
            let node = capture.node;
            match capture_name {
                "base64" => match node.utf8_text(source.as_bytes()) {
                    Ok(encoded_bytes) => {
                        let unpadded: String = encoded_bytes
                            .chars()
                            .filter(|c| !c.is_ascii_whitespace())
                            .collect();
                        if base64::engine::GeneralPurpose::new(
                            &base64::alphabet::STANDARD,
                            base64::engine::GeneralPurposeConfig::new().with_decode_padding_mode(
                                base64::engine::DecodePaddingMode::RequireCanonical,
                            ),
                        )
                        .decode(&unpadded)
                        .is_err()
                        {
                            match base64::engine::GeneralPurpose::new(
                                &base64::alphabet::STANDARD,
                                base64::engine::GeneralPurposeConfig::new()
                                    .with_decode_padding_mode(
                                        base64::engine::DecodePaddingMode::Indifferent,
                                    ),
                            )
                            .decode(&unpadded)
                            {
                                Ok(_) => diagnostics.push(Diagnostic::new(
                                    to_range(node.range(), &index, is_utf8),
                                    Some(DiagnosticSeverity::WARNING),
                                    None,
                                    None,
                                    "Missing padding".into(),
                                    None,
                                    None,
                                )),
                                Err(err) => diagnostics.push(Diagnostic::new(
                                    to_range(node.range(), &index, is_utf8),
                                    Some(DiagnosticSeverity::ERROR),
                                    None,
                                    None,
                                    format!("Invalid Base64 bytes: {err}"),
                                    None,
                                    None,
                                )),
                            }
                        }
                    }
                    Err(err) => diagnostics.push(Diagnostic::new(
                        to_range(node.range(), &index, is_utf8),
                        Some(DiagnosticSeverity::ERROR),
                        None,
                        None,
                        format!("Invalid UTF-8: {err}"),
                        None,
                        None,
                    )),
                },
                _ => {
                    error!(
                        names = ?query.capture_names(),
                        index = capture.index,
                        "Unknown capture name"
                    )
                }
            }
        }
    }

    // Check identified Temporal values
    let query = &QUERY_IDENTIFIED_TEMPORAL;
    let mut cursor = QueryCursor::new();
    let mut matches = cursor.matches(query, tree.root_node(), source.as_bytes());
    while let Some(m) = {
        matches.advance();
        matches.get()
    } {
        let mut identifier: Option<(&str, tree_sitter::Node<'_>)> = None;
        let mut value: Option<(&str, tree_sitter::Node<'_>)> = None;
        for capture in m.captures {
            let capture_name = query.capture_names()[capture.index as usize];
            let node = capture.node;
            match capture_name {
                "identifier" => match node.utf8_text(source.as_bytes()) {
                    Ok(parsed) => {
                        identifier = Some((parsed, node));
                    }
                    Err(err) => diagnostics.push(Diagnostic::new(
                        to_range(node.range(), &index, is_utf8),
                        Some(DiagnosticSeverity::ERROR),
                        None,
                        None,
                        format!("Invalid identifier: {err}"),
                        None,
                        None,
                    )),
                },
                "temporal" => match node.utf8_text(source.as_bytes()) {
                    Ok(parsed) => {
                        value = Some((parsed.trim(), node));
                    }
                    Err(err) => diagnostics.push(Diagnostic::new(
                        to_range(node.range(), &index, is_utf8),
                        Some(DiagnosticSeverity::ERROR),
                        None,
                        None,
                        format!("Invalid UTF-8: {err}"),
                        None,
                        None,
                    )),
                },
                _ => {
                    error!(
                        names = ?query.capture_names(),
                        index = capture.index,
                        "Unknown capture name"
                    )
                }
            }
        }
        if let (Some((identifier, identifier_node)), Some((temporal, temporal_node))) =
            (identifier, value)
        {
            match identifier {
                "Instant" => {
                    if !duper::validate::is_valid_instant(temporal) {
                        diagnostics.push(Diagnostic::new(
                            to_range(temporal_node.range(), &index, is_utf8),
                            Some(DiagnosticSeverity::ERROR),
                            None,
                            None,
                            "Invalid Instant".into(),
                            None,
                            None,
                        ));
                    }
                }
                "ZonedDateTime" => {
                    if !duper::validate::is_valid_zoned_date_time(temporal) {
                        diagnostics.push(Diagnostic::new(
                            to_range(temporal_node.range(), &index, is_utf8),
                            Some(DiagnosticSeverity::ERROR),
                            None,
                            None,
                            "Invalid ZonedDateTime".into(),
                            None,
                            None,
                        ));
                    }
                }
                "PlainDate" => {
                    if !duper::validate::is_valid_plain_date(temporal) {
                        diagnostics.push(Diagnostic::new(
                            to_range(temporal_node.range(), &index, is_utf8),
                            Some(DiagnosticSeverity::ERROR),
                            None,
                            None,
                            "Invalid PlainDate".into(),
                            None,
                            None,
                        ));
                    }
                }
                "PlainTime" => {
                    if !duper::validate::is_valid_plain_time(temporal) {
                        diagnostics.push(Diagnostic::new(
                            to_range(temporal_node.range(), &index, is_utf8),
                            Some(DiagnosticSeverity::ERROR),
                            None,
                            None,
                            "Invalid PlainTime".into(),
                            None,
                            None,
                        ));
                    }
                }
                "PlainDateTime" => {
                    if !duper::validate::is_valid_plain_date_time(temporal) {
                        diagnostics.push(Diagnostic::new(
                            to_range(temporal_node.range(), &index, is_utf8),
                            Some(DiagnosticSeverity::ERROR),
                            None,
                            None,
                            "Invalid PlainDateTime".into(),
                            None,
                            None,
                        ));
                    }
                }
                "PlainYearMonth" => {
                    if !duper::validate::is_valid_plain_year_month(temporal) {
                        diagnostics.push(Diagnostic::new(
                            to_range(temporal_node.range(), &index, is_utf8),
                            Some(DiagnosticSeverity::ERROR),
                            None,
                            None,
                            "Invalid PlainYearMonth".into(),
                            None,
                            None,
                        ));
                    }
                }
                "PlainMonthDay" => {
                    if !duper::validate::is_valid_plain_month_day(temporal) {
                        diagnostics.push(Diagnostic::new(
                            to_range(temporal_node.range(), &index, is_utf8),
                            Some(DiagnosticSeverity::ERROR),
                            None,
                            None,
                            "Invalid PlainMonthDay".into(),
                            None,
                            None,
                        ));
                    }
                }
                "Duration" => {
                    if !duper::validate::is_valid_duration(temporal) {
                        diagnostics.push(Diagnostic::new(
                            to_range(temporal_node.range(), &index, is_utf8),
                            Some(DiagnosticSeverity::ERROR),
                            None,
                            None,
                            "Invalid Duration".into(),
                            None,
                            None,
                        ));
                    }
                }
                _ => {
                    diagnostics.push(Diagnostic::new(
                        to_range(identifier_node.range(), &index, is_utf8),
                        Some(DiagnosticSeverity::HINT),
                        None,
                        None,
                        "Identifier has no effect".into(),
                        None,
                        None,
                    ));
                    if !duper::validate::is_valid_unspecified_temporal(temporal) {
                        diagnostics.push(Diagnostic::new(
                            to_range(temporal_node.range(), &index, is_utf8),
                            Some(DiagnosticSeverity::ERROR),
                            None,
                            None,
                            "Invalid Temporal value".to_string(),
                            None,
                            None,
                        ));
                    }
                }
            }
        } else {
            warn!(
                ?identifier,
                ?value,
                "Unexpected condition: Temporal identifier and/or value are None"
            );
        }
    }

    // Check unspecified Temporal values
    let query = &QUERY_UNSPECIFIED_TEMPORAL;
    let mut cursor = QueryCursor::new();
    let mut matches = cursor.matches(query, tree.root_node(), source.as_bytes());
    while let Some(m) = {
        matches.advance();
        matches.get()
    } {
        for capture in m.captures {
            let capture_name = query.capture_names()[capture.index as usize];
            let node = capture.node;
            match capture_name {
                "temporal" => match node.utf8_text(source.as_bytes()) {
                    Ok(parsed) if !duper::validate::is_valid_unspecified_temporal(parsed) => {
                        diagnostics.push(Diagnostic::new(
                            to_range(node.range(), &index, is_utf8),
                            Some(DiagnosticSeverity::ERROR),
                            None,
                            None,
                            "Invalid Temporal".into(),
                            None,
                            None,
                        ));
                    }
                    Ok(_) => (),
                    Err(err) => diagnostics.push(Diagnostic::new(
                        to_range(node.range(), &index, is_utf8),
                        Some(DiagnosticSeverity::ERROR),
                        None,
                        None,
                        format!("Invalid UTF-8: {err}"),
                        None,
                        None,
                    )),
                },
                _ => {
                    error!(
                        names = ?query.capture_names(),
                        index = capture.index,
                        "Unknown capture name"
                    )
                }
            }
        }
    }

    // Check integers
    let query = &QUERY_INTEGERS;
    let mut cursor = QueryCursor::new();
    let mut matches = cursor.matches(query, tree.root_node(), source.as_bytes());
    while let Some(m) = {
        matches.advance();
        matches.get()
    } {
        for capture in m.captures {
            let capture_name = query.capture_names()[capture.index as usize];
            let node = capture.node;
            match capture_name {
                "integer" => match node.utf8_text(source.as_bytes()) {
                    Ok(parsed) if !duper::validate::is_valid_integer(parsed) => {
                        diagnostics.push(Diagnostic::new(
                            to_range(node.range(), &index, is_utf8),
                            Some(DiagnosticSeverity::ERROR),
                            None,
                            None,
                            "Integer cannot be represented with I64\n  = hint: consider using a string instead".into(),
                            None,
                            None,
                        ));
                    }
                    Ok(_) => (),
                    Err(err) => diagnostics.push(Diagnostic::new(
                        to_range(node.range(), &index, is_utf8),
                        Some(DiagnosticSeverity::ERROR),
                        None,
                        None,
                        format!("Invalid UTF-8: {err}"),
                        None,
                        None,
                    )),
                },
                _ => {
                    error!(
                        names = ?query.capture_names(),
                        index = capture.index,
                        "Unknown capture name"
                    )
                }
            }
        }
    }

    // Check floats
    let query = &QUERY_FLOATS;
    let mut cursor = QueryCursor::new();
    let mut matches = cursor.matches(query, tree.root_node(), source.as_bytes());
    while let Some(m) = {
        matches.advance();
        matches.get()
    } {
        for capture in m.captures {
            let capture_name = query.capture_names()[capture.index as usize];
            let node = capture.node;
            match capture_name {
                "float" => match node.utf8_text(source.as_bytes()) {
                    Ok(parsed) if !duper::validate::is_valid_float(parsed) => {
                        diagnostics.push(Diagnostic::new(
                            to_range(node.range(), &index, is_utf8),
                            Some(DiagnosticSeverity::ERROR),
                            None,
                            None,
                            "Float cannot be represented with double".into(),
                            None,
                            None,
                        ));
                    }
                    Ok(_) => (),
                    Err(err) => diagnostics.push(Diagnostic::new(
                        to_range(node.range(), &index, is_utf8),
                        Some(DiagnosticSeverity::ERROR),
                        None,
                        None,
                        format!("Invalid UTF-8: {err}"),
                        None,
                        None,
                    )),
                },
                _ => {
                    error!(
                        names = ?query.capture_names(),
                        index = capture.index,
                        "Unknown capture name"
                    )
                }
            }
        }
    }

    // Check well-known string types
    let query = &QUERY_IDENTIFIED_STRINGS;
    let mut cursor = QueryCursor::new();
    let mut matches = cursor.matches(query, tree.root_node(), source.as_bytes());
    'm: while let Some(m) = {
        matches.advance();
        matches.get()
    } {
        let mut identifier: Option<&str> = None;
        let mut value: Option<(Cow<'_, str>, tree_sitter::Node<'_>)> = None;
        for capture in m.captures {
            let capture_name = query.capture_names()[capture.index as usize];
            let node = capture.node;
            match capture_name {
                "identifier" => match node.utf8_text(source.as_bytes()) {
                    Ok(parsed) => {
                        identifier = Some(parsed);
                    }
                    Err(err) => {
                        diagnostics.push(Diagnostic::new(
                            to_range(node.range(), &index, is_utf8),
                            Some(DiagnosticSeverity::ERROR),
                            None,
                            None,
                            format!("Invalid identifier: {err}"),
                            None,
                            None,
                        ));
                        continue 'm;
                    }
                },
                "quote" => match node.utf8_text(source.as_bytes()) {
                    Ok(escaped_str) => match duper::escape::unescape_str(escaped_str) {
                        Ok(str) => {
                            value = Some((str, node));
                        }
                        Err(err) => {
                            diagnostics.push(Diagnostic::new(
                                to_range(node.range(), &index, is_utf8),
                                Some(DiagnosticSeverity::ERROR),
                                None,
                                None,
                                format!("Invalid UTF-8: {err}"),
                                None,
                                None,
                            ));
                            continue 'm;
                        }
                    },
                    Err(err) => {
                        diagnostics.push(Diagnostic::new(
                            to_range(node.range(), &index, is_utf8),
                            Some(DiagnosticSeverity::ERROR),
                            None,
                            None,
                            format!("Invalid UTF-8: {err}"),
                            None,
                            None,
                        ));
                        continue 'm;
                    }
                },
                "raw" => match node.utf8_text(source.as_bytes()) {
                    Ok(raw) => {
                        value = Some((Cow::Borrowed(raw), node));
                    }
                    Err(err) => {
                        diagnostics.push(Diagnostic::new(
                            to_range(node.range(), &index, is_utf8),
                            Some(DiagnosticSeverity::ERROR),
                            None,
                            None,
                            format!("Invalid UTF-8: {err}"),
                            None,
                            None,
                        ));
                        continue 'm;
                    }
                },
                _ => {
                    error!(
                        names = ?query.capture_names(),
                        index = capture.index,
                        "Unknown capture name"
                    )
                }
            }
        }
        if let (Some(identifier), Some((string, node))) = (identifier, value.as_ref()) {
            if REGEX_UUID.is_match(identifier) {
                if let Err(err) = uuid::Uuid::try_parse(string) {
                    diagnostics.push(Diagnostic::new(
                        to_range(node.range(), &index, is_utf8),
                        Some(DiagnosticSeverity::WARNING),
                        None,
                        None,
                        format!("Invalid UUID: {err}"),
                        None,
                        None,
                    ))
                }
            } else if REGEX_DECIMAL.is_match(identifier) {
                if let Err(err) = rust_decimal::Decimal::from_str(string.as_ref()) {
                    diagnostics.push(Diagnostic::new(
                        to_range(node.range(), &index, is_utf8),
                        Some(DiagnosticSeverity::WARNING),
                        None,
                        None,
                        format!("Invalid Decimal: {err}"),
                        None,
                        None,
                    ))
                }
            } else if REGEX_IP4ADDR.is_match(identifier) {
                if let Err(err) = Ipv4Addr::from_str(string.as_ref()) {
                    diagnostics.push(Diagnostic::new(
                        to_range(node.range(), &index, is_utf8),
                        Some(DiagnosticSeverity::WARNING),
                        None,
                        None,
                        format!("Invalid IPv4 address: {err}"),
                        None,
                        None,
                    ))
                }
            } else if REGEX_IP6ADDR.is_match(identifier) {
                if let Err(err) = Ipv6Addr::from_str(string.as_ref()) {
                    diagnostics.push(Diagnostic::new(
                        to_range(node.range(), &index, is_utf8),
                        Some(DiagnosticSeverity::WARNING),
                        None,
                        None,
                        format!("Invalid IPv6 address: {err}"),
                        None,
                        None,
                    ))
                }
            } else if REGEX_IPADDR.is_match(identifier) {
                if let Err(err) = IpAddr::from_str(string.as_ref()) {
                    diagnostics.push(Diagnostic::new(
                        to_range(node.range(), &index, is_utf8),
                        Some(DiagnosticSeverity::WARNING),
                        None,
                        None,
                        format!("Invalid IP address: {err}"),
                        None,
                        None,
                    ))
                }
            } else if REGEX_IP4NET.is_match(identifier) {
                if let Err(err) = Ipv4Net::from_str(string.as_ref()) {
                    diagnostics.push(Diagnostic::new(
                        to_range(node.range(), &index, is_utf8),
                        Some(DiagnosticSeverity::WARNING),
                        None,
                        None,
                        format!("Invalid IPv4 network: {err}"),
                        None,
                        None,
                    ))
                }
            } else if REGEX_IP6NET.is_match(identifier) {
                if let Err(err) = Ipv6Net::from_str(string.as_ref()) {
                    diagnostics.push(Diagnostic::new(
                        to_range(node.range(), &index, is_utf8),
                        Some(DiagnosticSeverity::WARNING),
                        None,
                        None,
                        format!("Invalid IPv6 network: {err}"),
                        None,
                        None,
                    ))
                }
            } else if REGEX_IPNET.is_match(identifier)
                && let Err(err) = IpNet::from_str(string.as_ref()) {
                    diagnostics.push(Diagnostic::new(
                        to_range(node.range(), &index, is_utf8),
                        Some(DiagnosticSeverity::WARNING),
                        None,
                        None,
                        format!("Invalid IP network: {err}"),
                        None,
                        None,
                    ))
                }
        } else {
            warn!(
                ?identifier,
                ?value,
                "Unexpected condition: String identifier and/or value are None"
            );
        }
    }

    diagnostics
}

fn to_range(range: tree_sitter::Range, index: &LineIndex, is_utf8: bool) -> lsp_types::Range {
    let start_line_col = LineCol {
        line: range.start_point.row as u32,
        col: range.start_point.column as u32,
    };
    let end_line_col = LineCol {
        line: range.end_point.row as u32,
        col: range.end_point.column as u32,
    };
    if is_utf8 {
        lsp_types::Range::new(
            lsp_types::Position {
                line: start_line_col.line,
                character: start_line_col.col,
            },
            lsp_types::Position {
                line: end_line_col.line,
                character: end_line_col.col,
            },
        )
    } else {
        let wide_start_line_col = index
            .to_wide(WideEncoding::Utf16, start_line_col)
            .expect("integer overflow");
        let wide_end_line_col = index
            .to_wide(WideEncoding::Utf16, end_line_col)
            .expect("integer overflow");
        lsp_types::Range::new(
            lsp_types::Position {
                line: wide_start_line_col.line,
                character: wide_start_line_col.col,
            },
            lsp_types::Position {
                line: wide_end_line_col.line,
                character: wide_end_line_col.col,
            },
        )
    }
}
