<p align="center">
    <img src="https://raw.githubusercontent.com/EpicEric/duper/refs/heads/main/logos/duper-400.png" alt="The Duper logo, with a confident spectacled mole wearing a flailing blue cape." /> <br>
</p>
<h1 align="center">Serde Duper</h1>

<p align="center">
    <a href="https://crates.io/crates/serde_duper"><img alt="Crates.io version" src="https://img.shields.io/crates/v/serde_duper?style=flat&logo=rust&logoColor=white&label=serde_duper"></a>
    <a href="https://github.com/EpicEric/duper"><img alt="GitHub license" src="https://img.shields.io/github/license/EpicEric/duper"></a>
</p>

Adds `serde` support for Duper.

[See the main project on GitHub.](https://github.com/EpicEric/duper)

## Example

```rust
use serde::{Deserialize, Serialize};
use serde_duper::types::DuperUuid;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
#[serde(rename = "Status")]
enum UserStatus {
    Disabled,
    PendingApproval,
    Enabled,
}

#[derive(Serialize, Deserialize)]
struct User {
    #[serde(with = "DuperUuid")]
    id: Uuid,
    status: UserStatus,
    last_known_ips: Vec<String>,
}

let u = User {
    id: "314dfe6f-7a76-4c43-80b9-3b0ceb0960c0".parse().unwrap(),
    status: UserStatus::Enabled,
    last_known_ips: vec!["2a02:ec80:700:ed1a::1".to_string()],
};
let d = serde_duper::to_string(&u).unwrap();
println!("{}", d);
// This should print:
//     User({
//       id: Uuid("314dfe6f-7a76-4c43-80b9-3b0ceb0960c0"),
//       status: Status("Enabled"),
//       last_known_ips: ["2a02:ec80:700:ed1a::1"],
//     })
```
