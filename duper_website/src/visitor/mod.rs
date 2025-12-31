mod saphyr;
mod toml;

pub(crate) use saphyr::SaphyrVisitor;
pub(crate) use toml::TomlVisitor;

pub(crate) fn clean_temporal(temporal: &str) -> &str {
    match temporal.split_once('[') {
        Some((start, _)) => start,
        None => temporal,
    }
    .trim()
}
