import { assert, describe, expect, it } from "vitest";
import { parse, DuperValue } from "..";

describe("parse", () => {
  it("parses an empty object", () => {
    const duper = parse("{}");
    assert.instanceOf(duper, DuperValue);
    expect(duper.type).eq("object");
  });
});
