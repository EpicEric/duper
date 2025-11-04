# JavaScript guide

Get started with Duper in JavaScript/TypeScript with the [`@duper-js/wasm`](https://www.npmjs.com/package/@duper-js/wasm) package.

## Installation

### Node.js

```bash
npm install --save @duper-js/wasm
```

### Vite

```bash
npm install --save @duper-js/wasm
npm install --save-dev vite vite-plugin-top-level-await vite-plugin-wasm
```

Add the following to `vite.config.js`:

```javascript
import { defineConfig } from "vite";
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";

export default defineConfig({
  plugins: [
    wasm(),
    topLevelAwait(),
    // Your other plugins...
  ],
});
```

## Usage

```javascript
import { parse, stringify } from "@duper-js/wasm";

const data = {
  name: "Wireless Headphones",
  price: 129.99,
  in_stock: true,
  tags: ["electronics", "audio"],
};

// Convert to Duper format
const duper_string = stringify(data);
console.log(duper_string);

// Convert back from Duper
const restored_data = parse(duper_string);
console.log(restored_data);
```

You can also manipulate parsed Duper values directly:

```javascript
import { parse, stringify, DuperValue } from "@duper-js/wasm";

// Parse some input
const input = `
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
    created_at: DateTime("2023-11-17T21:50:43+00:00"),
  })
`;
const duper = parse(input);
console.log(duper.identifier); // Product
console.log(duper.inner.in_stock.type); // boolean

// Convert to JSON
console.log(JSON.stringify(duper));

// Patch the Duper value and pretty-print it
const newTags = [
  new DuperValue("music"),
  new DuperValue("hi-fi", "DeprecatedTag", "string"),
];
duper.inner.tags.setValue(newTags, "tuple");
duper.inner.image_thumbnail.identifier = null;
duper.inner.weight.identifier = "Pounds";
delete duper.inner.dimensions;
duper.inner.test = new DuperValue({ foo: new DuperValue("bar") });
console.log(stringify(duper, { indent: "  " }));
```
