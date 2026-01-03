# Changelog

## 0.6.1 (2026-01-03)

### Added

- Add `yoke::Yokeable` derive to `DuperValue` (gated behind the `yoke` feature).
- Add `ToJson` visitor (gated behind the `json-compat` feature).

## 0.6.0 (2025-12-23)

### Changed

- **Breaking:** Force real values for floats.

## 0.5.0 (2025-12-07)

### Added

- `serde::to_string_compact` which minifies and doesn't remove identifiers.

### Changed

- **Breaking:** Change meta-`serde` types to titlecase.
- **Breaking:** Change visitors to receive borrowed values.

### Fixed

- Fix meta-(de)serialization.
- **Breaking:** Enforce invariants through major refactor of the type system.

## 0.4.3 (2025-11-30)

### Changed

- Expose most parser functions to the public API.

## 0.4.2 (2025-11-25)

### Fixed

- Fix key formatting.

### Changed

- Implement changes from specification 0.4.1.
- Better conformity to specification.

## 0.4.1 (2025-11-22)

### Added

- Expose `escape`, `format`, and `validate` modules.

### Fixed

- Missing handling for PlainTime in Temporal parser.

## 0.4.0 (2025-11-11)

### Added

- **Breaking**: Add `minify` option to `Serializer`.
- **Breaking**: Add `b64"..."` syntax for Base64-encoded bytes.
- **Breaking**: Add `'...'` and `Type('...')` syntax for Temporal values.

### Fixed

- Not handling sequential comments.

## 0.3.0 (2025-10-30)

### Changed

- **Breaking**: Swap `pest` parser for `chumsky`.
- **Breaking:** Implement changes from specification 0.3.0.

## 0.2.2 (2025-10-28)

### Added

- Add `serde` feature flag.
- Add `Serialize`/`Deserialize` implementations to `DuperValue` and related.
- Add custom `serialize_meta`/`deserialize_meta` methods.

### Changed

- Move `serde_duper` serializer and deserializer interfaces into this crate.

### Fixed

- Removed extra line break when pretty printing identifiers.

## 0.2.1 (2025-10-25)

### Changed

- Update logo links.

## 0.2.0 (2025-10-23)

### Changed

- **Breaking**: Implement changes from specification 0.2.0.

### Fixed

- Fix bug when parsing identifiers with underscores or hyphens.

### Removed

- **Breaking**: Unused `serde` feature.

## 0.1.1 (2025-10-22)

### Changed

- Implement changes from specification 0.1.1.
- Update README.
- Reuse buffers for visitors.

### Removed

- Unused types module.

## 0.1.0 (2025-10-20)

Initial release.
