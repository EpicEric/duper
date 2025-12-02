import { DuperValue, parse, stringify } from "..";

describe("parse then stringify", () => {
  const input = `
    Product({
      dimensions: (18.5, 15.2, 7),  // In centimeters
      weight: Kilograms(0.285),
      image_thumbnail: Png(b64"iVBORw0KGgoAAAANSUhEUgAAAGQ="),
      tags: ["electronics", "audio", "wireless"],
      date: ZonedDateTime('2022-02-28T11:06:00.092121729+08:00[Asia/Shanghai][u-ca=chinese]'),
    })
  `;

  it("doesn't modify original object", () => {
    const duper = parse(input);
    expect(stringify(duper)).toMatchSnapshot();
  });

  it("swaps value", () => {
    const duper = parse(input);
    const newTags = DuperValue.Tuple([
      DuperValue.String("music"),
      DuperValue.String("hi-fi", "DeprecatedTag"),
    ]);
    duper.value.tags = newTags;
    expect(stringify(duper)).toMatchSnapshot();
  });

  it("removes identifier", () => {
    const duper = parse(input);
    duper.value.image_thumbnail.identifier = null;
    expect(stringify(duper)).toMatchSnapshot();
  });

  it("swaps identifier", () => {
    const duper = parse(input);
    duper.value.weight.identifier = "Pounds";
    expect(stringify(duper)).toMatchSnapshot();
  });

  it("removes value", () => {
    const duper = parse(input);
    delete duper.value.dimensions;
    expect(stringify(duper)).toMatchSnapshot();
  });

  it("inserts object entry", () => {
    const duper = parse(input);
    duper.value.test = DuperValue.Object({ foo: "bar" });
    expect(stringify(duper)).toMatchSnapshot();
  });

  it("strips identifiers", () => {
    const duper = parse(input);
    expect(stringify(duper, { stripIdentifiers: true })).toMatchSnapshot();
  });

  it("pretty-prints output", () => {
    const duper = parse(input);
    expect(stringify(duper, { indent: "  " })).toMatchSnapshot();
  });

  it("minifies output", () => {
    const duper = parse(input);
    expect(
      stringify(duper, { stripIdentifiers: true, minify: true }),
    ).toMatchSnapshot();
  });

  it("fails on both minify and indent", () => {
    const duper = parse(input);
    expect(() =>
      stringify(duper, { indent: 2, minify: true } as any),
    ).toThrow();
  });
});

describe("stringify then parse", () => {
  const duper = DuperValue.Object(
    {
      chunk: DuperValue.Bytes(new Uint8Array(0), "Stream"),
      port: DuperValue.Integer(5173),
      connections: DuperValue.Array([
        DuperValue.String("192.168.0.50:12345", "IPv4Socket"),
        DuperValue.String("[2001:1d8::1]:29876", "IPv46ocket"),
      ]),
      date: DuperValue.Temporal("2025-11-08"),
    },
    "Tcp",
  );

  it("properly serializes then deserializes to Duper", () => {
    const serialized = stringify(duper);
    expect(serialized).toMatchSnapshot();
    const deserialized = parse(serialized);
    expect(deserialized.identifier).toEqual("Tcp");
    expect(deserialized.type).toEqual("Object");
    expect(deserialized.value.chunk.type).toEqual("Bytes");
    expect(deserialized.value.chunk.identifier).toEqual("Stream");
    expect(deserialized.value.port.type).toEqual("Integer");
    expect(deserialized.value.port.identifier).toBeUndefined();
    expect(deserialized.value.port.value).toEqual(5173n);
    expect(deserialized.value.connections.type).toEqual("Array");
    expect(deserialized.value.connections.value.length).toEqual(2);
    expect(deserialized.value.date.type).toEqual("Temporal");
    expect(deserialized.value.date.value).toEqual("2025-11-08");
  });

  it("properly serializes then deserializes to JSON-safe", () => {
    const serialized = stringify(duper);
    const deserialized = parse(serialized, true);
    expect(deserialized).toMatchSnapshot();
  });
});
