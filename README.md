<p align="center">
    <img src="https://duper.dev.br/logos/duper-400.png" alt="The Duper logo, with a confident spectacled mole wearing a flailing blue cape." /> <br>
</p>
<h1 align="center">Duper: The format that's super!</h1>

<p align="center">
    <a href="https://github.com/EpicEric/duper"><img alt="GitHub stars" src="https://img.shields.io/github/stars/EpicEric/duper?style=flat&logo=github&logoColor=white"></a>
    <a href="https://github.com/EpicEric/duper"><img alt="GitHub license" src="https://img.shields.io/github/license/EpicEric/duper"></a>
</p>

Duper aims to be a human-friendly extension of JSON with quality-of-life improvements, extra types, and semantic identifiers.

[Check out the official website](https://duper.dev.br) for examples, quick start guides, and more.

```duper
UserProfile({
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
  avatar: Png(b64"iVBORw0KGgoAAAANSUhEUgAAAGQ="),
  bio: r#"Hello! I'm a super "duper" user!"#,
  last_logins: [
    (IPv4Address("192.168.1.100"), Instant('2024-03-20T14:30:00Z')),
  ],
})
```

## Why Duper?

Duper excels in a variety of use cases:

- In configuration files, where users are expected to swap out values, its explicit types can be a helpful guide.
- Thanks to its extended type support and self-documenting identifiers, Duper feels right at home in REST APIs.
- With a simple and readable syntax for logs, Duper is a breath of fresh air for both manual and tool-assisted debugging.

## For implementers

See [the specification](https://duper.dev.br/spec.html) or the [EBNF grammar](https://github.com/EpicEric/duper/blob/main/duper.ebnf) for more details.

## Workspace structure

- [`duper`](./duper/): Core implementation of the Duper parser and serializer in Rust, as well as Serde support. Used by Rust libraries and bindings in other languages.
  - Libraries
    - [`serde_duper`](./serde_duper/): Adds Duper-specific support for extra types for use with Serde, including a proc-macro via [`serde_duper_macros`](./serde_duper_macros/).
    - [`axum_duper`](./axum_duper/): Axum support for Duper requests and responses.
  - Bindings
    - [`duper-python`](./duper-python/): Python bindings using PyO3, including Pydantic and FastAPI support.
    - [`duper_uniffi_`](./duper_uniffi/): Multi-language bindings using UniFFI.
      - [`duper_uniffi/dotnet`](./duper_uniffi/dotnet/): C# / .NET bindings.
      - [`duper-js-wasm`](./duper-js-wasm/): WebAssembly bindings.
- [`tree-sitter-duper`](./tree-sitter-duper/): tree-sitter implementation of Duper.
  - [`duperfmt`](./duperfmt/): Duper formatter based on Topiary.
    - [`duper_lsp`](./duper_lsp/): Duper LSP.
      - [`duper-vs-code`](./duper-vs-code/): Duper extension for Visual Studio Code.
- [`duper_website`](./duper_website/): Official website for Duper, including the specification and WebAssembly-based playground.