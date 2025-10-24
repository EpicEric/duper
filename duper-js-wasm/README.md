<p align="center">
    <img src="https://duper.dev.br/logos/duper-400.png" alt="The Duper logo, with a confident spectacled mole wearing a flailing blue cape." /> <br>
</p>
<h1 align="center">@duper-js/wasm</h1>

<p align="center">
    <a href="https://www.npmjs.com/package/%40duper-js%2Fwasm"><img alt="PyPI version" src="https://img.shields.io/npm/v/%40duper-js%2Fwasm?style=flat&logo=npm&logoColor=white&label=%40duper-js%2Fwasm"></a>
    <a href="https://github.com/EpicEric/duper"><img alt="GitHub license" src="https://img.shields.io/github/license/EpicEric/duper"></a>
</p>

Duper support for JavaScript/TypeScript via WASM.

[Check out the official website for Duper.](https://duper.dev.br)

## Installation

```bash
npm install --save @duper-js/wasm
```

## Examples

```typescript
import { parse, stringify } from "@duper-js/wasm";

const duper_data = `
APIResponse({
  status: 200,
  headers: {
    content_type: "application/duper",
    cache_control: "max-age=3600",
  },
  body: {
    users: [
      User({
        id: Uuid("7039311b-02d2-4849-a6de-900d4dbe9acb"),
        name: "Alice",
        email: Email("alice@example.com"),
        roles: ["admin", "user"],
        metadata: {
          last_login: DateTime("2024-01-15T10:30:00Z"),
          ip: IPV4("173.255.230.79"),
        },
      }),
    ],
  },
})
`;

const obj = parse(duper_data);

console.log(stringify, { stripIdentifiers: false });
console.log(JSON.stringify(obj));
```
