---
layout: home

hero:
  name: Duper
  text: The format that's super!
  tagline: A human-friendly extension of JSON with quality-of-life improvements, extra types, and semantic identifiers.
  image:
    src: /logos/duper.svg
    alt: Duper
  actions:
    - theme: brand
      text: Get started
      link: /quick-start
    - theme: alt
      text: View on GitHub
      link: https://github.com/EpicEric/duper

features:
  - title: Hand-writing ergonomics
    details: Trailing commas, comments, and optional quotes for keys.
    icon: 🦾
  - title: Rich types
    details: Tuples, bytes, raw strings, Temporal, and proper integer support.
    icon: 🧰
  - title: JSON-compatible
    details: Every valid JSON file is automatically valid Duper.
    icon: 🧩
  - title: Self-documenting
    details: Identifiers provide readability, debuggability, and optional validation.
    icon: 📝
---

<script setup>
import DuperEditor from "./components/DuperEditor.vue";

const initial = `UserProfile({
  id: Uuid("f111c275-b4ce-4392-8e5b-19067ce39b53"),
  username: "EpicEric",
  email: EmailAddress("eric@duper.dev.br"),
  settings: {
    "dark mode": true,
    language: Locale("pt-BR"),
    metadata: null,
  },
  score: 120.25,
  // Support for bytes, woohoo!
  avatar: Png(b64"iVBORw0KGgoAAAANSUhEUgAAAGQ"),
  bio: r#"Hello! I'm a super "duper" user!"#,
  last_logins: [
    (IPv4Address("192.168.1.100"), Instant('2024-03-20T14:30:00Z')),
  ],
})`;
</script>

## Why Duper?

Duper excels in a variety of use cases:

- **Configuration files**: Duper's explicit types and identifiers serve as helpful guides when users need to modify values.
- **REST APIs**: Self-documenting identifiers make Duper a natural fit for API responses.
- **Data interchange**: With support for bytes, raw data, JSON compatibility, and identifiers that can be turned on for debugging-only, Duper is ideal for data exchange between systems.

## Playground

<DuperEditor :initial="initial" />

## Comparison

| Feature          | Duper             | JSON | JSON5 | YAML              | TOML              | RON |
| ---------------- | ----------------- | ---- | ----- | ----------------- | ----------------- | --- |
| Comments         | ✅                | ❌   | ✅    | ✅                | ✅                | ✅  |
| Trailing commas  | ✅                | ❌   | ✅    | ✅                | ✅                | ✅  |
| Unquoted keys    | ✅                | ❌   | ✅    | ✅                | ✅                | ✅  |
| Integers         | ✅                | ❌   | ❌    | ✅                | ✅                | ✅  |
| Tuples           | ✅                | ❌   | ❌    | ❌                | ❌                | ✅  |
| Bytes            | ✅                | ❌   | ❌    | ✅<sup>\[1]</sup> | ❌                | ✅  |
| Date and time    | ✅<sup>\[2]</sup> | ❌   | ❌    | ✅                | ✅                | ❌  |
| Raw strings      | ✅                | ❌   | ❌    | ✅                | ✅                | ✅  |
| Identifiers/tags | ✅                | ❌   | ❌    | ✅                | ❌                | ✅  |
| Unambiguous      | ✅                | ✅   | ✅    | ❌<sup>\[3]</sup> | ⚠️<sup>\[4]</sup> | ✅  |
| Simple           | ✅                | ✅   | ✅    | ❌                | ✅                | ✅  |
| JSON-compatible  | ✅                | ✅   | ✅    | ✅                | ❌                | ❌  |
| Popular          | ❌                | ✅   | ⚠️    | ✅                | ✅                | ⚠️  |

- <sup>[1]</sup> Using the [`!!binary` scalar type](https://yaml.org/type/binary.html) and base64 text; limited support in implementations.
- <sup>[2]</sup> Compliant with the [Temporal specification](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal).
- <sup>[3]</sup> Unquoted strings can get confused with other scalars and [vice-versa](https://www.bram.us/2022/01/11/yaml-the-norway-problem/).
- <sup>[4]</sup> Tabs allowed in [strings](https://toml.io/en/v1.0.0#string).
