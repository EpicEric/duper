# Changelog

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
