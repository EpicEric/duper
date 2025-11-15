import { assert, describe, expect, it } from "vitest";
import { DuperValue } from "..";

describe("DuperValue", () => {
  it("casts to types provided by user", () => {
    expect(DuperValue.Object({}).type).toEqual("Object");
    expect(DuperValue.Array([]).type).toEqual("Array");
    expect(DuperValue.Tuple([]).type).toEqual("Tuple");
    expect(DuperValue.String("").type).toEqual("String");
    expect(DuperValue.Bytes(new Uint8Array()).type).toEqual("Bytes");
    expect(DuperValue.Bytes([0x61, 0x62, 0x63]).type).toEqual("Bytes");
    expect(DuperValue.Bytes("Hello world!").type).toEqual("Bytes");
    if ("Temporal" in globalThis) {
      expect(
        DuperValue.Temporal((globalThis as any).Temporal.Now.instant()).type
      ).toEqual("Temporal");
    }
    expect(DuperValue.Temporal("2025-11-08").type).toEqual("Temporal");
    expect(DuperValue.Float(34).type).toEqual("Float");
    expect(DuperValue.Integer(34).type).toEqual("Integer");
    expect(DuperValue.Integer(35n).type).toEqual("Integer");
    expect(DuperValue.Boolean(true).type).toEqual("Boolean");
    expect(DuperValue.Null(null).type).toEqual("Null");
    expect(DuperValue.Null().type).toEqual("Null");
  });

  // it("raises error for invalid user-provided types", () => {
  //   assert.throws(() => new DuperValue(1234, null, "object"));
  //   assert.throws(() => new DuperValue({}, null, "array"));
  //   assert.throws(() => new DuperValue(new Uint8Array(), null, "string"));
  //   assert.throws(() => new DuperValue(0xdeadbeef, null, "bytes"));
  //   assert.throws(() => new DuperValue("34", null, "float"));
  //   assert.throws(() => new DuperValue("35n", null, "integer"));
  //   assert.throws(() => new DuperValue("true", null, "boolean"));
  //   assert.throws(() => new DuperValue("null", null, "null"));
  // });

  // it("raises error for invalid values", () => {
  //   assert.throws(() => new DuperValue({ "non-duper value": "hello" }));
  //   assert.throws(
  //     () => new DuperValue({ "non-duper value": "hello" }, null, "object")
  //   );
  //   assert.throws(() => new DuperValue(["non-duper value"]));
  //   assert.throws(() => new DuperValue(["non-duper value"], null, "array"));
  //   assert.throws(() => new DuperValue(["non-duper value"], null, "tuple"));
  //   assert.throws(() => new DuperValue(new Date()));
  //   assert.throws(() => new DuperValue(new Date(), null, "temporal"));
  // });
});
