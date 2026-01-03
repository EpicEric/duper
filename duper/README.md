<p align="center">
    <img src="https://duper.dev.br/logos/duper-400.png" alt="The Duper logo, with a confident spectacled mole wearing a flailing blue cape." /> <br>
</p>
<h1 align="center">Duper: The format that's super!</h1>

<p align="center">
    <a href="https://crates.io/crates/duper"><img alt="crates.io version" src="https://img.shields.io/crates/v/duper?style=flat&logo=rust&logoColor=white&label=duper"></a>
    <a href="https://github.com/EpicEric/duper"><img alt="GitHub license" src="https://img.shields.io/github/license/EpicEric/duper"></a>
</p>

Duper aims to be a human-friendly extension of JSON with quality-of-life improvements, extra types, and semantic identifiers.

[Check out the official website for Duper.](https://duper.dev.br)

## An example

```duper
Product({
  product_id: Uuid("1dd7b7aa-515e-405f-85a9-8ac812242609"),
  name: "Wireless Bluetooth Headphones",
  brand: "AudioTech",
  price: Decimal("129.99"),
  dimensions: (18.5, 15.2, 7.8),  // In centimeters
  weight: Kilograms(0.285),
  in_stock: true,
  specifications: {
    battery_life: Duration("30h"),
    noise_cancellation: true,
    connectivity: ["Bluetooth 5.0", "3.5mm Jack"],
  },
  image_thumbnail: Png(b64"iVBORw0KGgoAAAANSUhEUgAAAGQ="),
  tags: ["electronics", "audio", "wireless"],
  release_date: Date("2023-11-15"),
  /* Warranty is optional */
  warranty_period: null,
  customer_ratings: {
    latest_review: r#"Absolutely ""astounding""!! 😎"#,
    average: 4.5,
    count: 127,
  },
  created_at: Instant('2023-11-17T21:50:43+00:00'),
})
```

- Similar to JSON, but with support for unquoted keys, trailing commas, and comments.
- It includes support for extra types: integers, tuples, bytes, raw strings, raw bytes, and Temporal.
- Finally, Duper has the notion of _identifiers_: optional type-like annotations (`MyIdentifier(...)`) to help with readability, or to suggest that the parser handles/validates the data in a specific manner.

## See also:

- [`serde_duper`](https://docs.rs/serde_duper): `serde` serialization / deserialization support for Duper.
- [`axum_duper`](https://docs.rs/axum_duper): Duper extractor / response for `axum`.
