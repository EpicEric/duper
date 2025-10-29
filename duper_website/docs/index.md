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
  - title: Ergonomic
    details: Optional quotes for keys, trailing commas, comments, and clean syntax.
    icon: 🎨
  - title: Rich types
    details: Tuples, bytes, raw strings, and more beyond basic JSON.
    icon: 🔧
  - title: JSON-compatible
    details: Every valid JSON file is automatically valid Duper.
    icon: 🔄
  - title: Self-documenting
    details: Identifiers provide guidance, readability, and validation.
    icon: 📝
---

<script setup>
import DuperEditor from './components/DuperEditor.vue';

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
  avatar: Png(b"\\x89PNG\\r\\n\\x1a\\n\\x00\\x00\\x00\\rIHDR"),
  bio: r#"Hello! I'm a super "duper" user!"#,
  last_logins: [
    (IPv4Address("192.168.1.100"), DateTime("2024-03-20T14:30:00Z")),
  ],
})`;
</script>

## Why Duper?

Duper excels in a variety of use cases:

- **Configuration files**: Duper's explicit types and identifiers serve as helpful guides when users need to modify values.
- **REST APIs**: Self-documenting identifiers make Duper feel natural in API payloads and responses.
- **Data interchange**: With support for bytes, raw data, minimal syntax, and JSON compatibility, Duper is ideal for data exchange between systems.

## Playground

<DuperEditor :initial="initial" />

## Comparison

| Feature         | Duper | JSON | JSON5 | TOML | YAML |
| --------------- | ----- | ---- | ----- | ---- | ---- |
| Comments        | ✅    | ❌   | ✅    | ✅   | ✅   |
| Trailing commas | ✅    | ❌   | ✅    | ✅   | ✅   |
| Unquoted keys   | ✅    | ❌   | ✅    | ✅   | ✅   |
| Integers        | ✅    | ❌   | ❌    | ✅   | ✅   |
| Tuples          | ✅    | ❌   | ❌    | ❌   | ❌   |
| Bytes           | ✅    | ❌   | ❌    | ❌   | ⚠️   |
| Raw strings     | ✅    | ❌   | ❌    | ✅   | ✅   |
| Identifiers     | ✅    | ❌   | ❌    | ❌   | ❌   |
| Unambiguous     | ✅    | ✅   | ✅    | ⚠️   | ❌   |
| Simple          | ✅    | ✅   | ✅    | ✅   | ❌   |
| Popular         | ❌    | ✅   | ❌    | ✅   | ✅   |
