import { assert, describe, expect, it } from "vitest";
import { DuperValue } from "..";

describe("DuperValue", () => {
  it("casts to types provided by user", () => {
    expect(DuperValue.Object({}).type).toEqual("Object");
    expect(DuperValue.Array([]).type).toEqual("Array");
    expect(DuperValue.Tuple([]).type).toEqual("Tuple");
    expect(DuperValue.String("").type).toEqual("String");
    expect(DuperValue.Bytes(new Uint8Array(0)).type).toEqual("Bytes");
    expect(DuperValue.Bytes([0x61, 0x62, 0x63]).type).toEqual("Bytes");
    expect(DuperValue.Bytes("Hello world!").type).toEqual("Bytes");
    if ("Temporal" in globalThis) {
      expect(
        DuperValue.Temporal((globalThis as any).Temporal.Now.instant()).type,
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

  it("raises error for invalid values", () => {
    assert.throws(() => DuperValue.Object(1234 as any));
    assert.throws(() => DuperValue.Array({} as any));
    assert.throws(() => DuperValue.String(new Uint8Array(0) as any));
    assert.throws(() => DuperValue.Bytes(0xdeadbeef as any));
    assert.throws(() => DuperValue.Temporal(new Date()));
    assert.throws(() => DuperValue.Float("fish" as any));
    assert.throws(() => DuperValue.Integer("35n" as any));
    assert.throws(() => DuperValue.Boolean("true" as any));
  });
});
