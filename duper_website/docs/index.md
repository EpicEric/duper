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
    icon: |
      <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-notebook-pen-icon lucide-notebook-pen"><path d="M13.4 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2v-7.4"/><path d="M2 6h4"/><path d="M2 10h4"/><path d="M2 14h4"/><path d="M2 18h4"/><path d="M21.378 5.626a1 1 0 1 0-3.004-3.004l-5.01 5.012a2 2 0 0 0-.506.854l-.837 2.87a.5.5 0 0 0 .62.62l2.87-.837a2 2 0 0 0 .854-.506z"/></svg>
  - title: Rich types
    details: Tuples, bytes, raw strings, Temporal, and proper integer support.
    icon: |
      <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-boxes-icon lucide-boxes"><path d="M2.97 12.92A2 2 0 0 0 2 14.63v3.24a2 2 0 0 0 .97 1.71l3 1.8a2 2 0 0 0 2.06 0L12 19v-5.5l-5-3-4.03 2.42Z"/><path d="m7 16.5-4.74-2.85"/><path d="m7 16.5 5-3"/><path d="M7 16.5v5.17"/><path d="M12 13.5V19l3.97 2.38a2 2 0 0 0 2.06 0l3-1.8a2 2 0 0 0 .97-1.71v-3.24a2 2 0 0 0-.97-1.71L17 10.5l-5 3Z"/><path d="m17 16.5-5-3"/><path d="m17 16.5 4.74-2.85"/><path d="M17 16.5v5.17"/><path d="M7.97 4.42A2 2 0 0 0 7 6.13v4.37l5 3 5-3V6.13a2 2 0 0 0-.97-1.71l-3-1.8a2 2 0 0 0-2.06 0l-3 1.8Z"/><path d="M12 8 7.26 5.15"/><path d="m12 8 4.74-2.85"/><path d="M12 13.5V8"/></svg>
  - title: Self-documenting
    details: Identifiers provide readability, debuggability, and optional validation.
    icon: |
      <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-file-search-corner-icon lucide-file-search-corner"><path d="M11.1 22H6a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h8a2.4 2.4 0 0 1 1.706.706l3.589 3.588A2.4 2.4 0 0 1 20 8v3.25"/><path d="M14 2v5a1 1 0 0 0 1 1h5"/><path d="m21 22-2.88-2.88"/><circle cx="16" cy="17" r="3"/></svg>
  - title: JSON-compatible
    details: Every valid JSON file is automatically valid Duper.
    icon: |
      <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-braces-icon lucide-braces"><path d="M8 3H7a2 2 0 0 0-2 2v5a2 2 0 0 1-2 2 2 2 0 0 1 2 2v5c0 1.1.9 2 2 2h1"/><path d="M16 21h1a2 2 0 0 0 2-2v-5c0-1.1.9-2 2-2a2 2 0 0 1-2-2V5a2 2 0 0 0-2-2h-1"/></svg>
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

- **Configuration files**: Duper's explicit types and comments serve as helpful guides when users need to modify values.
- **REST APIs**: Self-documenting identifiers make Duper a natural fit for API responses.
- **Logging**: With a simple syntax and extended type support, Duper is a breath of fresh air for both manual and tool-assisted debugging.

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

- <sup>[1]</sup> Using the [`!!binary` scalar type](https://yaml.org/type/binary.html); limited support in implementations.
- <sup>[2]</sup> Fully compliant with the [Temporal specification](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal).
- <sup>[3]</sup> Unquoted strings can get [confused with other scalars](https://www.bram.us/2022/01/11/yaml-the-norway-problem/) and vice-versa.
- <sup>[4]</sup> Tabs allowed in [strings](https://toml.io/en/v1.0.0#string).
