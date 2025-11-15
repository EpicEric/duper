import * as duperFfi from "./index.web";

await duperFfi.uniffiInitAsync();

export type DuperError = duperFfi.DuperError;

const duperSymbol: unique symbol = Symbol();

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
      value: string;
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

type NonDuper = { [duperSymbol]?: never; [key: string]: unknown };

export type DuperType = DuperValue extends { type: infer T } ? T : never;

export const DuperValue = {
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
        Object.entries(value).map(([key, val]) => [key, (val.toJSON as any)()])
      ),
  }),
  Array: (value: (DuperValue | NonDuper)[], identifier?: string) => ({
    [duperSymbol]: "$__duper" as const,
    type: "Array" as const,
    value,
    identifier,
    toJSON: () => value.map((val) => (val.toJSON as any)()),
  }),
  Tuple: (value: (DuperValue | NonDuper)[], identifier?: string) => ({
    [duperSymbol]: "$__duper" as const,
    type: "Tuple" as const,
    value,
    identifier,
    toJSON: () => value.map((val) => (val.toJSON as any)()),
  }),
  String: (value: string, identifier?: string) => ({
    [duperSymbol]: "$__duper" as const,
    type: "String" as const,
    value,
    identifier,
    toJSON: () => value,
  }),
  Bytes: (value: Uint8Array, identifier?: string) => ({
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
  }),
  Temporal: (value: string, identifier?: string) => ({
    [duperSymbol]: "$__duper" as const,
    type: "Temporal" as const,
    value,
    identifier,
    toJSON: () => value,
  }),
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
  Float: (value: bigint | number, identifier?: string) => ({
    [duperSymbol]: "$__duper" as const,
    type: "Float" as const,
    value: Number(value),
    identifier,
    toJSON: () => value,
  }),
  Boolean: (value: boolean, identifier?: string) => ({
    [duperSymbol]: "$__duper" as const,
    type: "Boolean" as const,
    value,
    identifier,
    toJSON: () => value,
  }),
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
  if (!convertingToJSON && value[duperSymbol] === "$__duper") {
    switch (value.type) {
      case "Object": {
        const map = new Map();
        Object.entries(value.value).forEach(([key, val]) => {
          map.set(key, toFfi(val));
        });
        return duperFfi.DuperValue.Object.new({
          identifier: value.identifier,
          value: map,
        });
      }
      case "Array": {
        const array = new Array(value.value.length);
        value.value.forEach((val) => {
          array.push(toFfi(val));
        });
        return duperFfi.DuperValue.Array.new({
          identifier: value.identifier,
          value: array,
        });
      }
      case "Tuple": {
        const array = new Array(value.value.length);
        value.value.forEach((val) => {
          array.push(toFfi(val));
        });
        return duperFfi.DuperValue.Tuple.new({
          identifier: value.identifier,
          value: array,
        });
      }
      case "String": {
        return duperFfi.DuperValue.String.new({
          identifier: value.identifier,
          value: value.value,
        });
      }
      case "Bytes": {
        return duperFfi.DuperValue.Bytes.new({
          identifier: value.identifier,
          value: value.value.buffer as ArrayBuffer,
        });
      }
      case "Temporal": {
        return duperFfi.DuperValue.Temporal.new({
          identifier: value.identifier,
          value: value.value,
        });
      }
      case "Integer": {
        return duperFfi.DuperValue.Integer.new({
          identifier: value.identifier,
          value: value.value,
        });
      }
      case "Float": {
        return duperFfi.DuperValue.Float.new({
          identifier: value.identifier,
          value: value.value,
        });
      }
      case "Boolean": {
        return duperFfi.DuperValue.Boolean.new({
          identifier: value.identifier,
          value: value.value,
        });
      }
      case "Null": {
        return duperFfi.DuperValue.Null.new({
          identifier: value.identifier,
        });
      }
      default:
        let _: never = value;
        throw new Error(`Unknown Duper value ${value}`);
    }
  } else if (value === null || value === undefined) {
    return duperFfi.DuperValue.Null.new({ identifier: undefined });
  } else if (typeof value === "boolean") {
    return duperFfi.DuperValue.Boolean.new({ identifier: undefined, value });
  } else if (typeof value === "bigint") {
    return duperFfi.DuperValue.Integer.new({
      identifier: undefined,
      value: BigInt(value),
    });
  } else if (typeof value === "number") {
    return duperFfi.DuperValue.Float.new({
      identifier: undefined,
      value,
    });
  } else if (value instanceof Uint8Array) {
    return duperFfi.DuperValue.Bytes.new({
      identifier: undefined,
      value: value.buffer as ArrayBuffer,
    });
  } else if (typeof value === "string") {
    return duperFfi.DuperValue.String.new({
      identifier: undefined,
      value,
    });
  } else if (value instanceof Date) {
    throw new Error(
      `Invalid Date value; convert it into a Temporal value first`
    );
  } else if (Array.isArray(value)) {
    return duperFfi.DuperValue.Array.new({
      identifier: undefined,
      value: value.map((val) => toFfi(val)),
    });
  } else if (
    !convertingToJSON &&
    "toJSON" in value &&
    typeof value.toJSON === "function"
  ) {
    return toFfi(value.toJSON(), true);
  } else if (typeof value !== "function" && typeof value !== "symbol") {
    const map = new Map();
    for (const [key, val] of Object.entries(value)) {
      map.set(key, toFfi(val as any));
    }
    return duperFfi.DuperValue.Object.new({
      identifier: undefined,
      value: map,
    });
  }
  throw new Error(`Unknown value ${value}`);
}

function fromFfi(value: duperFfi.DuperValue): DuperValue {
  switch (value.tag) {
    case duperFfi.DuperValue_Tags.Object: {
      const obj: Record<string, DuperValue> = {};
      for (const [key, val] of value.inner.value.entries()) {
        obj[key] = fromFfi(val);
      }
      return DuperValue.Object(obj, value.inner.identifier);
    }
    case duperFfi.DuperValue_Tags.Array: {
      return DuperValue.Array(
        value.inner.value.map((val) => fromFfi(val)),
        value.inner.identifier
      );
    }
    case duperFfi.DuperValue_Tags.Tuple: {
      return DuperValue.Tuple(
        value.inner.value.map((val) => fromFfi(val)),
        value.inner.identifier
      );
    }
    case duperFfi.DuperValue_Tags.String: {
      return DuperValue.String(value.inner.value, value.inner.identifier);
    }
    case duperFfi.DuperValue_Tags.Bytes: {
      return DuperValue.Bytes(
        new Uint8Array(value.inner.value),
        value.inner.identifier
      );
    }
    case duperFfi.DuperValue_Tags.Temporal: {
      return DuperValue.Temporal(value.inner.value, value.inner.identifier);
    }
    case duperFfi.DuperValue_Tags.Integer: {
      return DuperValue.Integer(value.inner.value, value.inner.identifier);
    }
    case duperFfi.DuperValue_Tags.Float: {
      return DuperValue.Float(value.inner.value, value.inner.identifier);
    }
    case duperFfi.DuperValue_Tags.Boolean: {
      return DuperValue.Boolean(value.inner.value, value.inner.identifier);
    }
    case duperFfi.DuperValue_Tags.Null: {
      return DuperValue.Null(null, value.inner.identifier);
    }
    default:
      let _: never = value;
      throw new Error(`Unknown Duper value ${value}`);
  }
}

/**
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
      indent?: undefined;
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
    options && {
      indent:
        typeof options.indent === "number"
          ? " ".repeat(options.indent)
          : options.indent,
      stripIdentifiers: options.stripIdentifiers ?? false,
      minify: options.minify ?? false,
    }
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
