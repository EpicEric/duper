# JavaScript guide

Get started with Duper in JavaScript/TypeScript with the [`@duper-js/wasm`](https://www.npmjs.com/package/@duper-js/wasm) package.

## Installation

### Vite

```bash
npm install --save @duper-js/wasm
npm install --save-dev vite vite-plugin-top-level-await vite-plugin-wasm
```

Add the following to `vite.config.js`:

```javascript
import {
  defineConfig
} from 'vite'
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

Now you can use Duper:

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
