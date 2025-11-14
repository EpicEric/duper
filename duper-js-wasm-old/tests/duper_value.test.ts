import { assert, describe, expect, it } from "vitest";
import { DuperValue } from "..";
import { Temporal } from "@js-temporal/polyfill";

describe("DuperValue", () => {
  it("infers types from JS types", () => {
    expect(new DuperValue({}).type).toEqual("object");
    expect(new DuperValue([]).type).toEqual("array");
    expect(new DuperValue("").type).toEqual("string");
    expect(new DuperValue(new Uint8Array()).type).toEqual("bytes");
    expect(new DuperValue(Temporal.Now.instant()).type).toEqual("temporal");
    expect(new DuperValue(34).type).toEqual("float");
    expect(new DuperValue(35n).type).toEqual("integer");
    expect(new DuperValue(true).type).toEqual("boolean");
    expect(new DuperValue(null).type).toEqual("null");
  });

  it("casts to types provided by user", () => {
    expect(new DuperValue({}, null, "object").type).toEqual("object");
    expect(new DuperValue([], null, "array").type).toEqual("array");
    expect(new DuperValue([], null, "tuple").type).toEqual("tuple");
    expect(new DuperValue("", null, "string").type).toEqual("string");
    expect(new DuperValue(new Uint8Array(), null, "bytes").type).toEqual(
      "bytes"
    );
    expect(new DuperValue([0x61, 0x62, 0x63], null, "bytes").type).toEqual(
      "bytes"
    );
    expect(new DuperValue("Hello world!", null, "bytes").type).toEqual("bytes");
    expect(
      new DuperValue(Temporal.Now.instant(), null, "temporal").type
    ).toEqual("temporal");
    expect(new DuperValue("2025-11-08", null, "temporal").type).toEqual(
      "temporal"
    );
    expect(new DuperValue(34, null, "float").type).toEqual("float");
    expect(new DuperValue(34, null, "integer").type).toEqual("integer");
    expect(new DuperValue(35n, null, "integer").type).toEqual("integer");
    expect(new DuperValue(true, null, "boolean").type).toEqual("boolean");
    expect(new DuperValue(null, null, "null").type).toEqual("null");
  });

  it("casts to types provided by user", () => {
    expect(new DuperValue({}, null, "object").type).toEqual("object");
    expect(new DuperValue([], null, "object").type).toEqual("object");
    expect(new DuperValue([], null, "object").inner).toEqual({});
    expect(new DuperValue([], null, "array").type).toEqual("array");
    expect(new DuperValue([], null, "tuple").type).toEqual("tuple");
    expect(new DuperValue("", null, "string").type).toEqual("string");
    expect(new DuperValue(new Uint8Array(), null, "bytes").type).toEqual(
      "bytes"
    );
    expect(new DuperValue([0x61, 0x62, 0x63], null, "bytes").type).toEqual(
      "bytes"
    );
    expect(new DuperValue("Hello world!", null, "bytes").type).toEqual("bytes");
    expect(new DuperValue(34, null, "float").type).toEqual("float");
    expect(new DuperValue(34, null, "integer").type).toEqual("integer");
    expect(new DuperValue(35n, null, "integer").type).toEqual("integer");
    expect(new DuperValue(true, null, "boolean").type).toEqual("boolean");
    expect(new DuperValue(null, null, "null").type).toEqual("null");
  });

  it("raises error for invalid user-provided types", () => {
    assert.throws(() => new DuperValue(1234, null, "object"));
    assert.throws(() => new DuperValue({}, null, "array"));
    assert.throws(() => new DuperValue(new Uint8Array(), null, "string"));
    assert.throws(() => new DuperValue(0xdeadbeef, null, "bytes"));
    assert.throws(() => new DuperValue("34", null, "float"));
    assert.throws(() => new DuperValue("35n", null, "integer"));
    assert.throws(() => new DuperValue("true", null, "boolean"));
    assert.throws(() => new DuperValue("null", null, "null"));
  });

  it("raises error for invalid values", () => {
    assert.throws(() => new DuperValue({ "non-duper value": "hello" }));
    assert.throws(
      () => new DuperValue({ "non-duper value": "hello" }, null, "object")
    );
    assert.throws(() => new DuperValue(["non-duper value"]));
    assert.throws(() => new DuperValue(["non-duper value"], null, "array"));
    assert.throws(() => new DuperValue(["non-duper value"], null, "tuple"));
    assert.throws(() => new DuperValue(new Date()));
    assert.throws(() => new DuperValue(new Date(), null, "temporal"));
  });
});
