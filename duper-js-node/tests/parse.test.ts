import { parse } from "..";

describe("parse", () => {
  it("parses an empty object", () => {
    const duper = parse("{}");
    expect(duper.type).toEqual("Object");
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
    `);
    expect(duper.type).toEqual("Object");
    expect(duper.identifier).toEqual("Product");
    expect(JSON.stringify(duper)).toMatchSnapshot();
  });

  it("doesn't parse invalid Duper values", () => {
    expect(() => parse(``)).toThrow();
    expect(() => parse(`{`)).toThrow();
    expect(() => parse(`]`)).toThrow();
    expect(() => parse(`tru`)).toThrow();
    expect(() => parse(`.618`)).toThrow();
    expect(() => parse(`Something(1, 2)`)).toThrow();
    expect(() => parse(`Instant('2025-11-16')`)).toThrow();
    expect(() => parse(`Iñvalid({})`)).toThrow();
  });
});
