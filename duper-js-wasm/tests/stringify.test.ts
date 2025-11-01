import { describe, expect, it } from "vitest";
import { stringify } from "..";

describe("stringify", () => {
  it("stringifies a basic object", () => {
    const data = {
      name: "Wireless Headphones",
      price: 129.99,
      in_stock: true,
      tags: ["electronics", "audio"],
    };
    const duper_string = stringify(data);
    expect(duper_string).toMatchSnapshot();
  });

  it("stringifies nested structures", () => {
    const data = ["foo", { bar: ["baz", { qux: null }] }];
    const duper_string = stringify(data);
    expect(duper_string).toMatchSnapshot();
  });
});
