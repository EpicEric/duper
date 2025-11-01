import { assert, describe, expect, it } from "vitest";
import { parse, DuperValue } from "..";

describe("parse", () => {
  it("parses an empty object", () => {
    const duper = parse("{}");
    assert.instanceOf(duper, DuperValue);
    expect(duper.type).eq("object");
  });

  it("parses into a JSON-stringifiable object", () => {
    const duper = parse(`
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
        image_thumbnail: Png(b"\\x89PNG\\r\\n\\x1a\\n\\x00\\x00\\x00\\rIHDR\\x00\\x00\\x00\\x64"),
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
    `);
    assert.instanceOf(duper, DuperValue);
    expect(duper.type).eq("object");
    expect(duper.identifier).eq("Product");
    expect(JSON.stringify(duper)).toMatchSnapshot();
  });

  it("parses into a toString-able array", () => {
    const duper = parse(`
      Foo([b"bar", false, 3.14])
    `);
    assert.instanceOf(duper, DuperValue);
    expect(duper.type).eq("array");
    expect(duper.identifier).eq("Foo");
    expect(duper.toString()).toMatchSnapshot();
  });

  it("doesn't parse invalid Duper values", () => {
    assert.throws(() => parse(``));
    assert.throws(() => parse(`{`));
    assert.throws(() => parse(`]`));
    assert.throws(() => parse(`tru`));
    assert.throws(() => parse(`.618`));
    assert.throws(() => parse(`Something(1, 2)`));
    assert.throws(() => parse(`Iñvalid({})`));
  });
});
