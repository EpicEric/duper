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
- Thanks to its self-documenting identifiers, Duper feels right at home in REST APIs.
- With a simple syntax and extended type support, using Duper for logs is a breath of fresh air for both manual and tool-assisted debugging.

## For implementers

See [the specification](https://duper.dev.br/spec.html) for more details.
