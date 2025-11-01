import { describe, expect, it } from "vitest";
import { parse, stringify, DuperValue } from "..";

describe("parse then stringify", () => {
  const input = `
    Product({
      dimensions: (18.5, 15.2, 7.8),  // In centimeters
      weight: Kilograms(0.285),
      image_thumbnail: Png(b"\\x89PNG\\r\\n\\x1a\\n\\x00\\x00\\x00\\rIHDR\\x00\\x00\\x00\\x64"),
      tags: ["electronics", "audio", "wireless"],
    })
  `;

  it("doesn't modify original object", () => {
    const duper = parse(input);
    expect(stringify(duper)).toMatchSnapshot();
  });

  it("swaps value", () => {
    const duper = parse(input);
    const newTags = [
      new DuperValue("music"),
      new DuperValue("hi-fi", "DeprecatedTag", "string"),
    ];
    duper.inner.tags.setValue(newTags, "tuple");
    expect(stringify(duper)).toMatchSnapshot();
  });

  it("removes identifier", () => {
    const duper = parse(input);
    duper.inner.image_thumbnail.identifier = null;
    expect(stringify(duper)).toMatchSnapshot();
  });

  it("swaps identifier", () => {
    const duper = parse(input);
    duper.inner.weight.identifier = "Pounds";
    expect(stringify(duper)).toMatchSnapshot();
  });

  it("removes value", () => {
    const duper = parse(input);
    delete duper.inner.dimensions;
    expect(stringify(duper)).toMatchSnapshot();
  });

  it("inserts object entry", () => {
    const duper = parse(input);
    duper.inner.test = new DuperValue({ foo: new DuperValue("bar") });
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
});

describe("stringify then parse", () => {
  const duper = new DuperValue(
    {
      chunk: new DuperValue(new Uint8Array(), "Stream"),
      port: new DuperValue(5173, null, "integer"),
      connections: new DuperValue([
        new DuperValue("192.168.0.50:12345", "IPv4Socket"),
        new DuperValue("[2001:1d8::1]:29876", "IPv46ocket"),
      ]),
    },
    "Tcp"
  );

  it("properly serializes then deserializes to Duper", () => {
    const serialized = stringify(duper);
    expect(serialized).toMatchSnapshot();
    const deserialized = parse(serialized);
    expect(deserialized.identifier).toEqual("Tcp");
    expect(deserialized.type).toEqual("object");
    expect(deserialized.inner.chunk.type).toEqual("bytes");
    expect(deserialized.inner.chunk.identifier).toEqual("Stream");
    expect(deserialized.inner.port.type).toEqual("integer");
    expect(deserialized.inner.port.identifier).toBeNullable();
    expect(deserialized.inner.port.inner).toEqual(5173n);
    expect(deserialized.inner.connections.type).toEqual("array");
    expect(deserialized.inner.connections.inner.length).toEqual(2);
  });

  it("properly serializes then deserializes to JSON-safe", () => {
    const serialized = stringify(duper);
    expect(serialized).toMatchSnapshot();
    const deserialized = parse(serialized, true);
    expect(deserialized).toMatchSnapshot();
  });
});
