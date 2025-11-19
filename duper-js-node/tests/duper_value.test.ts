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
    expect(() => DuperValue.Object(1234 as any)).toThrow();
    expect(() => DuperValue.Array({} as any)).toThrow();
    expect(() => DuperValue.String(new Uint8Array(0) as any)).toThrow();
    expect(() => DuperValue.Bytes(0xdeadbeef as any)).toThrow();
    expect(() => DuperValue.Temporal(new Date())).toThrow();
    expect(() => DuperValue.Float("fish" as any)).toThrow();
    expect(() => DuperValue.Integer("35n" as any)).toThrow();
    expect(() => DuperValue.Boolean("true" as any)).toThrow();
  });
});
