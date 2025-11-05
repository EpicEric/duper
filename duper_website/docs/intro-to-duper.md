# A visual introduction to Duper in four parts

For this example, we'll assume a specific format for product data in a storefront.

## Level 1

```duper
{
  "product_id": "1dd7b7aa-515e-405f-85a9-8ac812242609",
  "name": "Wireless Bluetooth Headphones",
  "brand": "AudioTech",
  "price": "129.99",
  "dimensions": [18.5, 15.2, 7.8],
  "weight": 0.285,
  "in_stock": true,
  "specifications": {
    "battery_life": "30h",
    "noise_cancellation": true,
    "connectivity": ["Bluetooth 5.0", "3.5mm Jack"]
  },
  "image_thumbnail": [137, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68, 82, 0, 0, 0, 100],
  "tags": ["electronics", "audio", "wireless"],
  "release_date": "2023-11-15",
  "warranty_period": null,
  "customer_ratings": {
    "latest_review": "Absolutely \"\"astounding\"\"!! 😎",
    "average": 4.5,
    "count": 127
  },
  "created_at": "2023-11-17T21:50:43+00:00"
}
```

Plain ol' JSON. This is a valid Duper object, as well.

## Level 2

```duper
{
  product_id: "1dd7b7aa-515e-405f-85a9-8ac812242609",
  name: "Wireless Bluetooth Headphones",
  brand: "AudioTech",
  price: "129.99",
  dimensions: [18.5, 15.2, 7.8],  // In centimeters
  weight: 0.285,
  in_stock: true,
  specifications: {
    battery_life: "30h",
    noise_cancellation: true,
    connectivity: ["Bluetooth 5.0", "3.5mm Jack"],
  },
  image_thumbnail: [137, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68, 82, 0, 0, 0, 100],
  tags: ["electronics", "audio", "wireless"],
  release_date: "2023-11-15",
  /* Warranty is optional */
  warranty_period: null,
  customer_ratings: {
    latest_review: "Absolutely \"\"astounding\"\"!! 😎",
    average: 4.5,
    count: 127,
  },
  created_at: "2023-11-17T21:50:43+00:00",
}
```

We can get rid of the quotes for simple keys, use trailing commas, and include comments. This is similar to [JSON5](https://json5.org/).

## Level 3

```duper
{
  product_id: "1dd7b7aa-515e-405f-85a9-8ac812242609",
  name: "Wireless Bluetooth Headphones",
  brand: "AudioTech",
  price: "129.99",
  dimensions: (18.5, 15.2, 7.8),  // In centimeters
  weight: 0.285,
  in_stock: true,
  specifications: {
    battery_life: "30h",
    noise_cancellation: true,
    connectivity: ["Bluetooth 5.0", "3.5mm Jack"],
  },
  image_thumbnail: b64"iVBORw0KGgoAAAANSUhEUgAAAGQ=",
  tags: ["electronics", "audio", "wireless"],
  release_date: "2023-11-15",
  /* Warranty is optional */
  warranty_period: null,
  customer_ratings: {
    latest_review: r#"Absolutely ""astounding""!! 😎"#,
    average: 4.5,
    count: 127,
  },
  created_at: '2023-11-17T21:50:43+00:00',
}
```

Duper also adds supports for tuples (`(-23.561384, -46.655891)`), bytes (`b"\x1b[1mDuper\x1b[0m"`), raw strings (`r#"I can use "quotes" in here!"#`), raw bytes (`br"/\ Check this out! #wombo_combo"`), and Temporal (`'2022-02-28T11:06:00.092121729+08:00[Asia/Shanghai][u-ca=chinese]'`). Also, integers are a separate type from floats.

## Level 4

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

Finally, Duper has the notion of _identifiers_: optional type-like annotations (`MyIdentifier(...)`) to help with readability, or to suggest that the parser validates the data in a specific manner.

---

Want to learn more? [Check out the specification](/spec).
