# duperq

`duperq` is a fast filter and processor of Duper files and logs, which also works with JSON.

## Installation

`duperq` is currently only available as a Crates.IO binary. To compile from source:

```bash
cargo install --locked duperq
```

## Basic usage

As an example, we'll assume the following data from a log format:

```duper
{
  traceId: UUID("a2ce2f29-84cf-47c9-a877-381855d59e77"),
  spanId: UUID("78ee3c78-c090-43f2-8568-f4542dc10ea5"),
  timestamp: "2025-11-29T22:21:45.133Z",
  level: "INFO",
  service: "store-webapp",
  development: false,
  http: {
    method: "GET",
    url: "/shopping-cart",
    statusCode: 200,
    address: ("192.168.1.100", 14567),
    userAgent: "Mozilla/5.0",
    duration: Duration('PT0.14567S'),
    history: ["/", "/search?q=headphones+", "/products/42"],
  },
}
```

You can read files by passing them after the `duperq` filter:

```bash
duperq "filter ." path/to/**/*.duper
```

You can also read lines of Duper values from stdin.

```bash
tail -f path/to/app.log | duperq "filter ."
```

### Filtering

To filter results, use the `filter` param in the query. You can bypass filtering by passing an empty query to `duperq`.

```bash
duperq "" log.duper
```

To access fields in objects or arrays of objects, use `.fieldName`. For complex keys, you can use quotes and Duper escaping, i.e. `."special key"`.

```bash
duperq "filter .level == \"INFO\"" log.duper
# ... equivalent to ...
duperq "filter .\"level\" == \"INFO\"" log.duper
```

You can concatenate fields to access nested objects:

```bash
duperq "filter .http.method == \"GET\"" log.duper
```

You can also combine filters with `and`/`&&` or `or`/`||`. To check if a field simply exists, use `exists(...)`.

```bash
duperq "filter (.http.url = \"/admin\" || .level = \"INFO\") && exists(.spanId)" log.duper
```

You can use comparison operators (`==` or `=` for equality; `!=` or `<>` for inequality; `<`, `<=`, `>`, `>=`) as you'd expect. For sized values (objects, arrays, tuples, strings and bytes), you can use the `len(...)` function.

```bash
duperq "filter .http.statusCode >= 400" log.duper
duperq "filter len(.http.history) > 2" log.duper
duperq "filter .http.duration < Duration('PT1S')" log.duper
```

You can also match strings/bytes with [Rust regexes](https://docs.rs/regex/) via `=~ "regex"`, or access the current element with a sole `.`. To filter elements in an array, add a `[selector operator value]` to the fields. For example, we can filter history values that contain "headphones" by putting all of these together:

```bash
duperq "filter .http.history[. =~ \"headphones\"]" log.duper
```

To check if a value is truthy, simply use the selector without an operator. You can also negate the result of a filter with `!`:

```bash
duperq "filter !.development" log.duper
```

To validate that a value is of a certain type, use the `is` operator. The valid right-handside operands are:

- `Object`
- `Array`
- `Tuple`
- `String`
- `Bytes`
- `Instant`
- `ZonedDateTime`
- `PlainDate`
- `PlainTime`
- `PlainDateTime`
- `PlainYearMonth`
- `PlainMonthDay`
- `Duration`
- `Temporal`
- `Integer`
- `Float`
- `Number`
- `Boolean`
- `Null`

```bash
duperq "filter .http.address is Tuple" log.duper
```

You can index into an array/tuple with `[index]`. Negative indexes also work, but they do not wrap around. To filter over an identifier, use `identifier(...)`.

```bash
duperq "filter identifier(.http.address[0]) == \"IPv4Address\"" log.duper
# We can use a regex instead
duperq "filter identifier(.http.address[0]) =~ \"(?i)^ipv\\\\daddress\$\"" log.duper
# To check if there is NO identifier
duperq "filter identifier(.http.userAgent) == null" log.duper
# To check if there is ANY identifier
duperq "filter identifier(.traceId) <> null" log.duper
```

You can use [ranges](https://doc.rust-lang.org/std/ops/struct.Range.html) over array values.

```bash
duperq "filter .http.history[..2] == \"/\"" log.duper
```

Last but not least, you can cast values into different types with `cast(..., type)`, with the same possible types from the `is` operator. This can be useful when dealing with JSON data, where there are no tuples or Temporal values, or to treat a tuple as an array. In our example, we can transform the string-only timestamp into a filterable value:

```bash
duperq "filter cast(.timestamp, Instant) > Instant('2025-11-01T00:00:00-03:00')" log.duper
```

### Manipulation

Other than filtering data, you may also skip the first values with `skip X`, or limit the number of filtered values you take with `take X`. These operations can be combined with pipes `|`:

```bash
duperq "filter .development | skip 3 | filter .level = \"ERROR\" | take 10" path/to/**/*.duper
```

### Output

By default, `duperq` serializes output data into a single-line format. You can change this by piping the output of your query to:

- `| ansi`: Prints with ANSI colors.
- `| pretty-print`: Pretty-prints values over multiple lines with indentation.
- `| format`: Allows you to print arbitrary strings, replacing `${...}` blocks with the selector inside. String values will have their quotes stripped. Missing values will be printed as `<MISSING>`.

```bash
duperq "filter . | format \"[\${.level}] \${.http.statusCode} - \${.http.method} \${.http.url}\"" log.duper
```

Formats must always be the last block in your query workflow.
