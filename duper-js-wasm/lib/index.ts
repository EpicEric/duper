import * as duperFfi from "../pkg/duper_js_wasm";

const duperSymbol: unique symbol = Symbol();

/**
 * A Temporal value, i.e. one of:
 * - `Temporal.Instant`
 * - `Temporal.ZonedDateTime`
 * - `Temporal.PlainDate`
 * - `Temporal.PlainTime`
 * - `Temporal.PlainDateTime`
 * - `Temporal.PlainYearMonth`
 * - `Temporal.PlainMonthDay`
 * - `Temporal.Duration`
 */
interface TemporalValue {
  toString(): string;
}

/**
 * A valid Duper value.
 */
export type DuperValue =
  | {
      [duperSymbol]: "$__duper";
      type: "Object";
      value: Record<string, DuperValue | NonDuper>;
      identifier?: string;
      toJSON(): any;
    }
  | {
      [duperSymbol]: "$__duper";
      type: "Array";
      value: (DuperValue | NonDuper)[];
      identifier?: string;
      toJSON(): any;
    }
  | {
      [duperSymbol]: "$__duper";
      type: "Tuple";
      value: (DuperValue | NonDuper)[];
      identifier?: string;
      toJSON(): any;
    }
  | {
      [duperSymbol]: "$__duper";
      type: "String";
      value: string;
      identifier?: string;
      toJSON(): any;
    }
  | {
      [duperSymbol]: "$__duper";
      type: "Bytes";
      value: Uint8Array;
      identifier?: string;
      toJSON(): any;
    }
  | {
      [duperSymbol]: "$__duper";
      type: "Temporal";
      value: string | TemporalValue;
      identifier?: string;
      toJSON(): any;
    }
  | {
      [duperSymbol]: "$__duper";
      type: "Integer";
      value: bigint;
      identifier?: string;
      toJSON(): any;
    }
  | {
      [duperSymbol]: "$__duper";
      type: "Float";
      value: number;
      identifier?: string;
      toJSON(): any;
    }
  | {
      [duperSymbol]: "$__duper";
      type: "Boolean";
      value: boolean;
      identifier?: string;
      toJSON(): any;
    }
  | {
      [duperSymbol]: "$__duper";
      type: "Null";
      value: null;
      identifier?: string;
      toJSON(): any;
    };

type NonDuper =
  | null
  | undefined
  | string
  | boolean
  | number
  | bigint
  | symbol
  | { [duperSymbol]?: never; [key: string]: unknown };

/**
 * The possible types that a Duper value may have.
 */
export type DuperType = DuperValue extends { type: infer T } ? T : never;

export const DuperValue = {
  /**
   * Creates a Duper object.
   *
   * @param value The key/value mapping.
   * @param identifier The identifier.
   */
  Object: (
    value: Record<string, DuperValue | NonDuper>,
    identifier?: string
  ) => ({
    [duperSymbol]: "$__duper" as const,
    type: "Object" as const,
    value,
    identifier,
    toJSON: () =>
      Object.fromEntries(
        Object.entries(value).map(([key, val]) => [key, (val as any).toJSON()])
      ),
  }),
  /**
   * Creates a Duper array.
   *
   * @param value The value list.
   * @param identifier The identifier.
   */
  Array: (value: (DuperValue | NonDuper)[], identifier?: string) => ({
    [duperSymbol]: "$__duper" as const,
    type: "Array" as const,
    value,
    identifier,
    toJSON: () => value.map((val) => (val as any).toJSON()),
  }),
  /**
   * Creates a Duper tuple.
   *
   * @param value The value list.
   * @param identifier The identifier.
   */
  Tuple: (value: (DuperValue | NonDuper)[], identifier?: string) => ({
    [duperSymbol]: "$__duper" as const,
    type: "Tuple" as const,
    value,
    identifier,
    toJSON: () => value.map((val) => (val as any).toJSON()),
  }),
  /**
   * Creates a Duper string.
   *
   * @param value The value.
   * @param identifier The identifier.
   */
  String: (value: string, identifier?: string) => ({
    [duperSymbol]: "$__duper" as const,
    type: "String" as const,
    value,
    identifier,
    toJSON: () => value,
  }),
  /**
   * Creates a Duper byte string.
   *
   * @param value The value.
   * @param identifier The identifier.
   */
  Bytes: (value: Uint8Array | string | number[], identifier?: string) => {
    if (typeof value === "string") {
      const utf8 = new Uint8Array(value.length);
      new TextEncoder().encodeInto(value, utf8);
      return {
        [duperSymbol]: "$__duper" as const,
        type: "Bytes" as const,
        value: utf8,
        identifier,
        toJSON: () => {
          const array = new Array<number>(utf8.byteLength);
          utf8.forEach((val) => {
            array.push(val);
          });
          return array;
        },
      };
    } else if (value instanceof Uint8Array) {
      return {
        [duperSymbol]: "$__duper" as const,
        type: "Bytes" as const,
        value,
        identifier,
        toJSON: () => {
          const array = new Array<number>(value.byteLength);
          value.forEach((val) => {
            array.push(val);
          });
          return array;
        },
      };
    }
    const array = new Uint8Array(value.length);
    array.set(value);
    return {
      [duperSymbol]: "$__duper" as const,
      type: "Bytes" as const,
      value: array,
      identifier,
      toJSON: () => value,
    };
  },
  /**
   * Creates a Duper Temporal value.
   *
   * @param value The value.
   * @param identifier The identifier.
   */
  Temporal: (value: any, identifier?: string) => {
    if ("Temporal" in globalThis) {
      const Temporal = (globalThis as any).Temporal;
      switch (identifier) {
        case "Instant": {
          const v =
            value instanceof Temporal.Instant
              ? value
              : typeof value === "string"
              ? Temporal.Instant.from(value)
              : value;
          return {
            [duperSymbol]: "$__duper" as const,
            type: "Temporal" as const,
            value: v,
            identifier,
            toJSON: () => v.toJSON(),
          };
        }
        case "ZonedDateTime": {
          const v =
            value instanceof Temporal.ZonedDateTime
              ? value
              : typeof value === "string"
              ? Temporal.ZonedDateTime.from(value)
              : value;
          return {
            [duperSymbol]: "$__duper" as const,
            type: "Temporal" as const,
            value: v,
            identifier,
            toJSON: () => v.toJSON(),
          };
        }
        case "PlainDate": {
          const v =
            value instanceof Temporal.PlainDate
              ? value
              : typeof value === "string"
              ? Temporal.PlainDate.from(value)
              : value;
          return {
            [duperSymbol]: "$__duper" as const,
            type: "Temporal" as const,
            value: v,
            identifier,
            toJSON: () => v.toJSON(),
          };
        }
        case "PlainTime": {
          const v =
            value instanceof Temporal.PlainTime
              ? value
              : typeof value === "string"
              ? Temporal.PlainTime.from(value)
              : value;
          return {
            [duperSymbol]: "$__duper" as const,
            type: "Temporal" as const,
            value: v,
            identifier,
            toJSON: () => v.toJSON(),
          };
        }
        case "PlainDateTime": {
          const v =
            value instanceof Temporal.PlainDateTime
              ? value
              : typeof value === "string"
              ? Temporal.PlainDateTime.from(value)
              : value;
          return {
            [duperSymbol]: "$__duper" as const,
            type: "Temporal" as const,
            value: v,
            identifier,
            toJSON: () => v.toJSON(),
          };
        }
        case "PlainYearMonth": {
          const v =
            value instanceof Temporal.PlainYearMonth
              ? value
              : typeof value === "string"
              ? Temporal.PlainYearMonth.from(value)
              : value;
          return {
            [duperSymbol]: "$__duper" as const,
            type: "Temporal" as const,
            value: v,
            identifier,
            toJSON: () => v.toJSON(),
          };
        }
        case "PlainMonthDay": {
          const v =
            value instanceof Temporal.PlainMonthDay
              ? value
              : typeof value === "string"
              ? Temporal.PlainMonthDay.from(value)
              : value;
          return {
            [duperSymbol]: "$__duper" as const,
            type: "Temporal" as const,
            value: v,
            identifier,
            toJSON: () => v.toJSON(),
          };
        }
        case "Duration": {
          const v =
            value instanceof Temporal.Duration
              ? value
              : typeof value === "string"
              ? Temporal.Duration.from(value)
              : value;
          return {
            [duperSymbol]: "$__duper" as const,
            type: "Temporal" as const,
            value: v,
            identifier,
            toJSON: () => v.toJSON(),
          };
        }
      }
    }
    return {
      [duperSymbol]: "$__duper" as const,
      type: "Temporal" as const,
      value,
      identifier,
      toJSON: () => value,
    };
  },
  /**
   * Creates a Duper integer.
   *
   * @param value The value.
   * @param identifier The identifier.
   */
  Integer: (value: bigint | number | string, identifier?: string) => ({
    [duperSymbol]: "$__duper" as const,
    type: "Integer" as const,
    value: BigInt(value),
    identifier,
    toJSON: () => {
      const float = Number(value);
      return float == value ? float : value.toString();
    },
  }),
  /**
   * Creates a Duper integer.
   *
   * @param value The value.
   * @param identifier The identifier.
   */
  Float: (value: bigint | number, identifier?: string) => ({
    [duperSymbol]: "$__duper" as const,
    type: "Float" as const,
    value: Number(value),
    identifier,
    toJSON: () => value,
  }),
  /**
   * Creates a Duper boolean.
   *
   * @param value The value.
   * @param identifier The identifier.
   */
  Boolean: (value: boolean, identifier?: string) => ({
    [duperSymbol]: "$__duper" as const,
    type: "Boolean" as const,
    value,
    identifier,
    toJSON: () => value,
  }),
  /**
   * Creates a Duper null value.
   *
   * @param value The value.
   * @param identifier The identifier.
   */
  Null: (value?: null, identifier?: string) => ({
    [duperSymbol]: "$__duper" as const,
    type: "Null" as const,
    value: null,
    identifier,
    toJSON: () => null,
  }),
};

function toFfi(
  value: DuperValue | NonDuper,
  convertingToJSON?: boolean
): duperFfi.DuperValue {
  if (
    !convertingToJSON &&
    value &&
    typeof value === "object" &&
    value[duperSymbol] === "$__duper"
  ) {
    switch (value.type) {
      case "Object": {
        const array = new Array(Object.keys(value.value).length);
        Object.entries(value.value).forEach(([key, val]) => {
          array.push(new duperFfi.DuperObjectEntry(key, toFfi(val)));
        });
        return new duperFfi.DuperValue(
          duperFfi.DuperValueType.Object,
          value.identifier,
          array
        );
      }
      case "Array": {
        const array = new Array(value.value.length);
        value.value.forEach((val) => {
          array.push(toFfi(val));
        });
        return new duperFfi.DuperValue(
          duperFfi.DuperValueType.Array,
          value.identifier,
          array
        );
      }
      case "Tuple": {
        const array = new Array(value.value.length);
        value.value.forEach((val) => {
          array.push(toFfi(val));
        });
        return new duperFfi.DuperValue(
          duperFfi.DuperValueType.Tuple,
          value.identifier,
          array
        );
      }
      case "String": {
        return new duperFfi.DuperValue(
          duperFfi.DuperValueType.String,
          value.identifier,
          value.value
        );
      }
      case "Bytes": {
        return new duperFfi.DuperValue(
          duperFfi.DuperValueType.Bytes,
          value.identifier,
          value.value
        );
      }
      case "Temporal": {
        return new duperFfi.DuperValue(
          duperFfi.DuperValueType.Temporal,
          value.identifier,
          value.value.toString()
        );
      }
      case "Integer": {
        return new duperFfi.DuperValue(
          duperFfi.DuperValueType.Integer,
          value.identifier,
          value.value
        );
      }
      case "Float": {
        return new duperFfi.DuperValue(
          duperFfi.DuperValueType.Float,
          value.identifier,
          value.value
        );
      }
      case "Boolean": {
        return new duperFfi.DuperValue(
          duperFfi.DuperValueType.Boolean,
          value.identifier,
          value.value
        );
      }
      case "Null": {
        return new duperFfi.DuperValue(
          duperFfi.DuperValueType.Null,
          value.identifier,
          null
        );
      }
      default:
        let _: never = value;
        throw new Error(`Unknown Duper value ${value}`);
    }
  } else if (value === null || value === undefined) {
    return new duperFfi.DuperValue(
      duperFfi.DuperValueType.Null,
      undefined,
      null
    );
  } else if (typeof value === "boolean") {
    return new duperFfi.DuperValue(
      duperFfi.DuperValueType.Boolean,
      undefined,
      value
    );
  } else if (typeof value === "bigint") {
    return new duperFfi.DuperValue(
      duperFfi.DuperValueType.Integer,
      undefined,
      value
    );
  } else if (typeof value === "number") {
    return new duperFfi.DuperValue(
      duperFfi.DuperValueType.Float,
      undefined,
      value
    );
  } else if (value instanceof Uint8Array) {
    return new duperFfi.DuperValue(
      duperFfi.DuperValueType.Bytes,
      undefined,
      value
    );
  } else if (typeof value === "string") {
    return new duperFfi.DuperValue(
      duperFfi.DuperValueType.String,
      undefined,
      value
    );
  } else if (
    "Temporal" in globalThis &&
    value instanceof (globalThis as any).Temporal.Instant
  ) {
    return new duperFfi.DuperValue(
      duperFfi.DuperValueType.Temporal,
      "Instant",
      value.toString()
    );
  } else if (
    "Temporal" in globalThis &&
    value instanceof (globalThis as any).Temporal.ZonedDateTime
  ) {
    return new duperFfi.DuperValue(
      duperFfi.DuperValueType.Temporal,
      "ZonedDateTime",
      value.toString()
    );
  } else if (
    "Temporal" in globalThis &&
    value instanceof (globalThis as any).Temporal.PlainDate
  ) {
    return new duperFfi.DuperValue(
      duperFfi.DuperValueType.Temporal,
      "PlainDate",
      value.toString()
    );
  } else if (
    "Temporal" in globalThis &&
    value instanceof (globalThis as any).Temporal.PlainTime
  ) {
    return new duperFfi.DuperValue(
      duperFfi.DuperValueType.Temporal,
      "PlainTime",
      value.toString()
    );
  } else if (
    "Temporal" in globalThis &&
    value instanceof (globalThis as any).Temporal.PlainDateTime
  ) {
    return new duperFfi.DuperValue(
      duperFfi.DuperValueType.Temporal,
      "PlainDateTime",
      value.toString()
    );
  } else if (
    "Temporal" in globalThis &&
    value instanceof (globalThis as any).Temporal.PlainYearMonth
  ) {
    return new duperFfi.DuperValue(
      duperFfi.DuperValueType.Temporal,
      "PlainYearMonth",
      value.toString()
    );
  } else if (
    "Temporal" in globalThis &&
    value instanceof (globalThis as any).Temporal.PlainMonthDay
  ) {
    return new duperFfi.DuperValue(
      duperFfi.DuperValueType.Temporal,
      "PlainMonthDay",
      value.toString()
    );
  } else if (
    "Temporal" in globalThis &&
    value instanceof (globalThis as any).Temporal.Duration
  ) {
    return new duperFfi.DuperValue(
      duperFfi.DuperValueType.Temporal,
      "Duration",
      value.toString()
    );
  } else if (value instanceof Date) {
    throw new Error(
      `Invalid Date value; convert it into a Temporal value first`
    );
  } else if (Array.isArray(value)) {
    return new duperFfi.DuperValue(
      duperFfi.DuperValueType.Array,
      undefined,
      value.map((val) => toFfi(val))
    );
  } else if (
    !convertingToJSON &&
    value &&
    typeof value === "object" &&
    "toJSON" in value &&
    typeof value.toJSON === "function"
  ) {
    return toFfi(value.toJSON(), true);
  } else if (typeof value !== "function" && typeof value !== "symbol") {
    const array = new Array(Object.keys(value).length);
    Object.entries(value.value as object).forEach(([key, val]) => {
      array.push(new duperFfi.DuperObjectEntry(key, toFfi(val as any)));
    });
    return new duperFfi.DuperValue(
      duperFfi.DuperValueType.Object,
      undefined,
      array
    );
  }
  throw new Error(`Unknown value ${String(value)}`);
}

function fromFfi(value: duperFfi.DuperValue): DuperValue {
  switch (value.type) {
    case duperFfi.DuperValueType.Object: {
      const obj: Record<string, DuperValue> = {};
      for (const entry of value.value as duperFfi.DuperObjectEntry[]) {
        obj[entry.key] = fromFfi(entry.value);
      }
      return DuperValue.Object(obj, value.identifier);
    }
    case duperFfi.DuperValueType.Array: {
      return DuperValue.Array(
        (value.value as duperFfi.DuperValue[]).map((val) => fromFfi(val)),
        value.identifier
      );
    }
    case duperFfi.DuperValueType.Tuple: {
      return DuperValue.Tuple(
        (value.value as duperFfi.DuperValue[]).map((val) => fromFfi(val)),
        value.identifier
      );
    }
    case duperFfi.DuperValueType.String: {
      return DuperValue.String(value.value, value.identifier);
    }
    case duperFfi.DuperValueType.Bytes: {
      return DuperValue.Bytes(value.value, value.identifier);
    }
    case duperFfi.DuperValueType.Temporal: {
      return DuperValue.Temporal(value.value, value.identifier);
    }
    case duperFfi.DuperValueType.Integer: {
      return DuperValue.Integer(value.value, value.identifier);
    }
    case duperFfi.DuperValueType.Float: {
      return DuperValue.Float(value.value, value.identifier);
    }
    case duperFfi.DuperValueType.Boolean: {
      return DuperValue.Boolean(value.value, value.identifier);
    }
    case duperFfi.DuperValueType.Null: {
      return DuperValue.Null(null, value.identifier);
    }
    default:
      // let _: never = value;
      throw new Error(`Unknown Duper value ${value}`);
  }
}

/**
 * Options available to the `stringify` function.
 *
 * @property {string | number} [indent] - Optional whitespace string to use as
 * indentation, or the number of spaces to use as indentation.
 * @property {boolean} [stripIdentifiers] - Whether Duper identifiers should be
 * removed from the stringified value.
 * @property {boolean} [minify] - Whether stringify should minify the value. Not
 * compatible with `indent`.
 */
type StringifyOptions =
  | {
      indent?: string | number;
      stripIdentifiers?: boolean;
      minify?: false;
    }
  | {
      indent?: never;
      stripIdentifiers?: boolean;
      minify: true;
    };

/**
 * Converts the provided value into a Duper string.
 *
 * @param value The value to stringify.
 * @param options Options for stringification.
 * @returns The Duper string.
 */
export function stringify(value: any, options?: StringifyOptions): string {
  return duperFfi.serialize(
    toFfi(value),
    options &&
      new duperFfi.SerializeOptions(
        typeof options.indent === "number"
          ? " ".repeat(options.indent)
          : options.indent,
        options.stripIdentifiers ?? false,
        options.minify ?? false
      )
  );
}

/**
 * Parses the provided Duper string into a Duper value, or a JSON-safe alternative if specified.
 *
 * @param value The Duper string to parse.
 * @param jsonSafe Whether to emit a JSON-safe alternative instead of a `DuperValue`.
 * @returns The parsed value.
 */
export function parse(value: string, jsonSafe?: false): DuperValue;
export function parse(value: string, jsonSafe: true): any;
export function parse(value: string, jsonSafe?: boolean): DuperValue | any {
  const parsed = duperFfi.parse(value, true);
  const transformed = fromFfi(parsed);
  if (jsonSafe) {
    return transformed.toJSON();
  }
  return transformed;
}
