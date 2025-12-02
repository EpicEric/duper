/** biome-ignore-all lint/suspicious/noExplicitAny: Serialization/Deserialization involves a lot of `any` */
import duperNapi from "./napi";

const duperSymbol: unique symbol = Symbol();

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
      value: any;
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
    identifier?: string,
  ) => {
    if (typeof value !== "object") {
      throw new Error(`Cannot cast value to object: ${value}`);
    }
    return {
      [duperSymbol]: "$__duper" as const,
      type: "Object" as const,
      value,
      identifier,
      toJSON: () =>
        Object.fromEntries(
          Object.entries(value).map(([key, val]) => [
            key,
            (val as any).toJSON(),
          ]),
        ),
    };
  },
  /**
   * Creates a Duper array.
   *
   * @param value The value list.
   * @param identifier The identifier.
   */
  Array: (value: (DuperValue | NonDuper)[], identifier?: string) => {
    if (!Array.isArray(value)) {
      throw new Error(`Cannot cast value to array: ${value}`);
    }
    return {
      [duperSymbol]: "$__duper" as const,
      type: "Array" as const,
      value,
      identifier,
      toJSON: () => value.map((val) => (val as any).toJSON()),
    };
  },
  /**
   * Creates a Duper tuple.
   *
   * @param value The value list.
   * @param identifier The identifier.
   */
  Tuple: (value: (DuperValue | NonDuper)[], identifier?: string) => {
    if (!Array.isArray(value)) {
      throw new Error(`Cannot cast value to tuple: ${value}`);
    }
    return {
      [duperSymbol]: "$__duper" as const,
      type: "Tuple" as const,
      value,
      identifier,
      toJSON: () => value.map((val) => (val as any).toJSON()),
    };
  },
  /**
   * Creates a Duper string.
   *
   * @param value The value.
   * @param identifier The identifier.
   */
  String: (value: string, identifier?: string) => {
    if (typeof value !== "string") {
      throw new Error(`Cannot cast value to string: ${value}`);
    }
    return {
      [duperSymbol]: "$__duper" as const,
      type: "String" as const,
      value,
      identifier,
      toJSON: () => value,
    };
  },
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
          utf8.forEach((val, i) => {
            array[i] = val;
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
          value.forEach((val, i) => {
            array[i] = val;
          });
          return array;
        },
      };
    } else if (Array.isArray(value)) {
      const array = new Uint8Array(value.length);
      array.set(value);
      return {
        [duperSymbol]: "$__duper" as const,
        type: "Bytes" as const,
        value: array,
        identifier,
        toJSON: () => value,
      };
    } else {
      throw new Error(`Cannot cast value to bytes: ${value}`);
    }
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
                : value.toInstant();
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
                : value.toZonedDateTimeISO();
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
                : value.toPlainDate();
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
                : value.toPlainTime();
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
                : value.toPlainDateTime();
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
                : value.toPlainYearMonth();
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
                : value.toPlainMonthDay();
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
                : null;
          if (v === null) {
            throw new Error(`Cannot cast value to Temporal duration: ${value}`);
          }
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
    if (typeof value === "string") {
      return {
        [duperSymbol]: "$__duper" as const,
        type: "Temporal" as const,
        value,
        identifier,
        toJSON: () => value,
      };
    }
    throw new Error(`Cannot cast value to Temporal value: ${value}`);
  },
  /**
   * Creates a Duper integer.
   *
   * @param value The value.
   * @param identifier The identifier.
   */
  Integer: (value: bigint | number | string, identifier?: string) => {
    const bigintValue = BigInt(value);
    return {
      [duperSymbol]: "$__duper" as const,
      type: "Integer" as const,
      value: bigintValue,
      identifier,
      toJSON: () => {
        const float = Number(bigintValue);
        return BigInt(float) === bigintValue ? float : value.toString();
      },
    };
  },
  /**
   * Creates a Duper float.
   *
   * @param value The value.
   * @param identifier The identifier.
   */
  Float: (value: bigint | number, identifier?: string) => {
    const float = Number(value);
    if (!Number.isFinite(float)) {
      throw new Error(`Cannot cast value to finite float: ${value}`);
    }
    return {
      [duperSymbol]: "$__duper" as const,
      type: "Float" as const,
      value: float,
      identifier,
      toJSON: () => value,
    };
  },
  /**
   * Creates a Duper boolean.
   *
   * @param value The value.
   * @param identifier The identifier.
   */
  Boolean: (value: boolean, identifier?: string) => {
    if (typeof value !== "boolean") {
      throw new Error(`Cannot cast value to boolean: ${value}`);
    }
    return {
      [duperSymbol]: "$__duper" as const,
      type: "Boolean" as const,
      value,
      identifier,
      toJSON: () => value,
    };
  },
  /**
   * Creates a Duper null value.
   *
   * @param _value The value.
   * @param identifier The identifier.
   */
  Null: (_value?: null, identifier?: string) => ({
    [duperSymbol]: "$__duper" as const,
    type: "Null" as const,
    value: null,
    identifier,
    toJSON: () => null,
  }),
};

type NapiValue = {
  identifier: string | null;
} & (
  | {
      type: "Object";
      inner: Record<string, NapiValue>;
    }
  | {
      type: "Array" | "Tuple";
      inner: NapiValue[];
    }
  | {
      type: "String";
      inner: string;
    }
  | {
      type: "Bytes";
      inner: Uint8Array;
    }
  | {
      type: "Temporal";
      inner: string;
    }
  | {
      type: "Integer";
      inner: bigint;
    }
  | {
      type: "Float";
      inner: number;
    }
  | {
      type: "Boolean";
      inner: boolean;
    }
  | {
      type: "Null";
      inner: null;
    }
);

function toNapi(
  value: DuperValue | NonDuper,
  // convertingToJSON?: boolean,
): NapiValue {
  let convertingToJSON = false;
  while (true) {
    if (
      !convertingToJSON &&
      value &&
      typeof value === "object" &&
      value[duperSymbol] === "$__duper"
    ) {
      switch (value.type) {
        case "Object": {
          const inner = Object.fromEntries(
            Object.entries(value.value).map(([key, val]) => [key, toNapi(val)]),
          );
          return {
            identifier: value.identifier || null,
            type: "Object",
            inner,
          };
        }
        case "Array": {
          const inner = value.value.map((val) => toNapi(val));
          return {
            identifier: value.identifier || null,
            type: "Array",
            inner,
          };
        }
        case "Tuple": {
          const inner = value.value.map((val) => toNapi(val));
          return {
            identifier: value.identifier || null,
            type: "Tuple",
            inner,
          };
        }
        case "String": {
          return {
            identifier: value.identifier || null,
            type: "String",
            inner: value.value,
          };
        }
        case "Bytes": {
          return {
            identifier: value.identifier || null,
            type: "Bytes",
            inner: value.value,
          };
        }
        case "Temporal": {
          return {
            identifier: value.identifier || null,
            type: "Temporal",
            inner: value.value.toString(),
          };
        }
        case "Integer": {
          return {
            identifier: value.identifier || null,
            type: "Integer",
            inner: value.value,
          };
        }
        case "Float": {
          return {
            identifier: value.identifier || null,
            type: "Float",
            inner: value.value,
          };
        }
        case "Boolean": {
          return {
            identifier: value.identifier || null,
            type: "Boolean",
            inner: value.value,
          };
        }
        case "Null": {
          return {
            identifier: value.identifier || null,
            type: "Null",
            inner: null,
          };
        }
        default: {
          const _value: never = value;
          throw new Error(`Unknown Duper value ${_value}`);
        }
      }
    } else if (value === null || value === undefined) {
      return {
        identifier: null,
        type: "Null",
        inner: null,
      };
    } else if (typeof value === "boolean") {
      return {
        identifier: null,
        type: "Boolean",
        inner: value,
      };
    } else if (typeof value === "bigint") {
      return {
        identifier: null,
        type: "Integer",
        inner: value,
      };
    } else if (typeof value === "number") {
      return {
        identifier: null,
        type: "Float",
        inner: value,
      };
    } else if (value instanceof Uint8Array) {
      return {
        identifier: null,
        type: "Bytes",
        inner: value,
      };
    } else if (typeof value === "string") {
      return {
        identifier: null,
        type: "String",
        inner: value,
      };
    } else if (
      "Temporal" in globalThis &&
      value instanceof (globalThis as any).Temporal.Instant
    ) {
      return {
        identifier: "Instant",
        type: "Temporal",
        inner: value.toString(),
      };
    } else if (
      "Temporal" in globalThis &&
      value instanceof (globalThis as any).Temporal.ZonedDateTime
    ) {
      return {
        identifier: "ZonedDateTime",
        type: "Temporal",
        inner: value.toString(),
      };
    } else if (
      "Temporal" in globalThis &&
      value instanceof (globalThis as any).Temporal.PlainDate
    ) {
      return {
        identifier: "PlainDate",
        type: "Temporal",
        inner: value.toString(),
      };
    } else if (
      "Temporal" in globalThis &&
      value instanceof (globalThis as any).Temporal.PlainTime
    ) {
      return {
        identifier: "PlainTime",
        type: "Temporal",
        inner: value.toString(),
      };
    } else if (
      "Temporal" in globalThis &&
      value instanceof (globalThis as any).Temporal.PlainDateTime
    ) {
      return {
        identifier: "PlainDateTime",
        type: "Temporal",
        inner: value.toString(),
      };
    } else if (
      "Temporal" in globalThis &&
      value instanceof (globalThis as any).Temporal.PlainYearMonth
    ) {
      return {
        identifier: "PlainYearMonth",
        type: "Temporal",
        inner: value.toString(),
      };
    } else if (
      "Temporal" in globalThis &&
      value instanceof (globalThis as any).Temporal.PlainMonthDay
    ) {
      return {
        identifier: "PlainMonthDay",
        type: "Temporal",
        inner: value.toString(),
      };
    } else if (
      "Temporal" in globalThis &&
      value instanceof (globalThis as any).Temporal.Duration
    ) {
      return {
        identifier: "Duration",
        type: "Temporal",
        inner: value.toString(),
      };
    } else if (value instanceof Date) {
      throw new Error(
        `Invalid Date value; convert it into a Temporal value first`,
      );
    } else if (Array.isArray(value)) {
      const inner = value.map((val) => toNapi(val));
      return {
        identifier: null,
        type: "Array",
        inner,
      };
    } else if (
      !convertingToJSON &&
      value &&
      typeof value === "object" &&
      "toJSON" in value &&
      typeof value.toJSON === "function"
    ) {
      convertingToJSON = true;
      continue;
    } else if (typeof value !== "function" && typeof value !== "symbol") {
      const inner = Object.fromEntries(
        Object.entries(value).map(([key, val]) => [key, toNapi(val as any)]),
      );
      return {
        identifier: null,
        type: "Object",
        inner,
      };
    }
    throw new Error(`Unknown value ${String(value)}`);
  }
}

function fromNapi(value: NapiValue): DuperValue {
  switch (value.type) {
    case "Object": {
      const obj = Object.fromEntries(
        Object.entries(value.inner).map(([key, value]) => [
          key,
          fromNapi(value),
        ]),
      );
      return DuperValue.Object(obj, value.identifier ?? undefined);
    }
    case "Array": {
      return DuperValue.Array(
        value.inner.map((val) => fromNapi(val)),
        value.identifier ?? undefined,
      );
    }
    case "Tuple": {
      return DuperValue.Tuple(
        value.inner.map((val) => fromNapi(val)),
        value.identifier ?? undefined,
      );
    }
    case "String": {
      return DuperValue.String(value.inner, value.identifier ?? undefined);
    }
    case "Bytes": {
      return DuperValue.Bytes(value.inner, value.identifier ?? undefined);
    }
    case "Temporal": {
      return DuperValue.Temporal(value.inner, value.identifier ?? undefined);
    }
    case "Integer": {
      return DuperValue.Integer(value.inner, value.identifier ?? undefined);
    }
    case "Float": {
      return DuperValue.Float(value.inner, value.identifier ?? undefined);
    }
    case "Boolean": {
      return DuperValue.Boolean(value.inner, value.identifier ?? undefined);
    }
    case "Null": {
      return DuperValue.Null(null, value.identifier ?? undefined);
    }
    default: {
      const _value: never = value;
      throw new Error(`Unknown Duper value ${_value}`);
    }
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
  return duperNapi.serialize(
    toNapi(value),
    options && {
      indent:
        typeof options.indent === "number"
          ? " ".repeat(options.indent)
          : options.indent,
      stripIdentifiers: options.stripIdentifiers ?? false,
      minify: options.minify ?? false,
    },
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
  const parsed = duperNapi.parse(value, true);
  const transformed = fromNapi(parsed as NapiValue);
  if (jsonSafe) {
    return transformed.toJSON();
  }
  return transformed;
}
