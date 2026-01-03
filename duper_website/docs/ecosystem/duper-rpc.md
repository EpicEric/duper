# Duper RPC

Duper RPC is a protocol-agnostic [remote procedure call](https://en.wikipedia.org/wiki/Remote_procedure_call) standard. Despite not being as compact as a binary, non-self-describing format, Duper's focus on legibility and self-documentation makes it shine for scenarios where text formats are prioritized (such as REST APIs) and where observability is key.

## Specification

### Request

A request object is a Duper object with the following keys:

- `duper_rpc`: Must be equal to `"0.1"`, and indicates that this is a Duper RPC request.
- `id`: Must be an integer, a string, null, or not present at all. Represents the ID of the RPC request. If missing or null, it makes this request a notification. A notification is a request that must be handled asynchronously by the RPC server, without any response in the case of a well-formed request.
- `method`: Must be a string, indicating the RPC method to be called.
- `params`: Must be a tuple of parameters, or a non-tuple value (equivalent to a unary tuple containing said value), indicating the parameters passed to the RPC method. If missing, it should be interpreted as an empty tuple (`()`). A request may have up to 8 parameters.

An RPC may be composed of a single request object, or a Duper array of at least one request object, referred to as a batch request. Examples:

```duper
RpcRequest({
  duper_rpc: "0.1",
  id: Uuid("9920aaef-cf81-45b5-9682-63d5e2d6e0d0"),
  method: "ping",
  // No `params` is equivalent to `params: ()`
})
```

```duper
RpcRequest([
  {
    duper_rpc: "0.1",
    // No `id` is equivalent to `id: null` and makes this a notification
    method: "set_name",
    params: "Eric", // Equivalent to `params: ("Eric")`
  },
  {
    duper_rpc: "0.1",
    id: 1234,
    method: "change_file_permissions",
    params: ("foobar.txt", 0o644),
  },
])
```

Servers may process request objects within a batch in any order.

### Response

A response object is one of two different Duper objects, depending on the result of the operation:

- Upon success, servers must return a Duper object with the following keys:
  - `duper_rpc`: Must be equal to `"0.1"`, and indicates that this is a Duper RPC response.
  - `id`: Must be an integer or a string, indicating which non-notification request that this response refers to. Servers may strip identifiers from the request IDs.
  - `result`: Must be any Duper value.
- Upon failure, servers must return a Duper object with the following keys:
  - `duper_rpc`: Must be equal to `"0.1"`, and indicates that this is a Duper RPC response.
  - `id`: Must be an integer, a string, or null. An integer or string indicates which request that this error refers to; while null indicates that this error originated from a request where the ID value is unknown (eg. a request with an invalid ID, or a notification request).
  - `error`: Must be a Duper object with the following keys:
    - `type`: Must be one of the following Duper strings:
      - `"ParseError"`: The request was an invalid Duper value.
      - `"InvalidRequest"`: The request was not a well-formed Duper RPC request.
      - `"MethodNotFound"`: The method was not registered on the Duper RPC server.
      - `"InvalidParams"`: The provided parameters didn't match those of the Duper RPC method.
      - `"InternalError"`: An internal error was raised by the server while handling the RPC request.
      - `"Custom"`: A user-defined error was raised by the server while handling the RPC request.
    - `value`: Must be any Duper value **only** if the error type is `"Custom"`; otherwise, the key must not be present.

Responses must always be returned for requests containing an `id`, either as an array of response objects (if a batch request with one or more requests was invoked), or as a single response object (if a single identified request was invoked, or a batch request containing a single identified request was invoked).

If there are no identified requests, servers must return an empty response. Servers must not return an empty array.

For example, given the following request:

```duper
RpcRequest([
  {
    duper_rpc: "0.1",
    id: 1,
    method: "greet",
    params: "Sam",
  },
  {
    duper_rpc: "0.1",
    id: 2,
    method: "greet",
    params: "Miles",
  },
  {
    duper_rpc: "0.1",
    id: 3,
    method: "greet",
    params: 0xdeadbeef,
  },
  {
    duper_rpc: "0.1",
    method: "greet",
    params: "Hans",
  },
])
```

A valid response from the server may be:

```duper
RpcResponse([
  {
    duper_rpc: "0.1",
    id: 1,
    result: "Hello, Sam!",
  },
  {
    duper_rpc: "0.1",
    id: 2,
    error: {
      type: "Custom",
      value: "I don't know this person.",
    },
  },
  {
    duper_rpc: "0.1",
    id: 3,
    error: {
      type: "InvalidParams",
    },
  },
])
```

Whereas for a request where all request objects lack an ID, such as:

```duper
RpcRequest({
  duper_rpc: "0.1",
  method: "greet",
  params: "Eve",
})
```

The server would return an empty response, regardless of the result of the RPC call.

## Implementations

- Rust: [`duper_rpc`](https://crates.io/crates/duper_rpc)
