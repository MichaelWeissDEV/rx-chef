# Rust library

The `rxchef` crate exposes operation discovery, direct execution, typed values, pipelines, Magic, and scanning. Operations are stateless boxed trait objects and can be looked up by canonical name.

## Integration API

Editor and plugin authors can use the same API that backs `operations`, `operation describe`, `bake`, and `serve --stdio`:

```rust
use rxchef::integration::{self, RecipeStep};

let catalog = integration::operations()?;
let xor = integration::describe("xor")?;
let result = integration::bake(
    b"Hello".to_vec(),
    &[RecipeStep { op: "to_base64".into(), args: vec![] }],
)?;

assert!(!catalog.is_empty());
assert_eq!(xor.name, "XOR");
assert_eq!(result.output, "SGVsbG8=");
# Ok::<(), String>(())
```

`ExecutionResult` contains a lossy display string plus exact Base64 bytes. `serve_jsonl(reader, writer)` exposes the protocol without depending on the CLI crate, so Rust hosts can reuse it over any buffered I/O transport.

`integration::bake` is the high-level recipe engine. In addition to arbitrary
left-to-right operation chains, it interprets nested `Fork`/`Merge`,
`Subsection`, registers (`$R0`, `$R1`, ...), labels, and bounded jumps. Use this
API when library callers need the same control-flow behavior as CLI and plugin
clients. The lower-level typed `Pipeline` below is intentionally a straight,
in-process operation chain.

## Direct operation use

```rust
use rxchef::operations::get_operation;

let operation = get_operation("To Upper Case").expect("registered operation");
let output = operation.run(b"Hello".to_vec(), &[])?;
assert_eq!(output, b"HELLO");
# Ok::<(), rxchef::OperationError>(())
```

Use `rxchef::runtime::resolve_operation_name` when accepting CLI-style aliases such as `to_upper_case`; `operations::get_operation` expects the canonical name, case-insensitively.

## Typed pipeline

```rust
use rxchef::{ArgValue, Pipeline};
use rxchef::operations::get_operation;

let result = Pipeline::new()
    .then(
        get_operation("To Hex").unwrap(),
        vec![ArgValue::Str("None".into()), ArgValue::Num(0.0)],
    )
    .then(
        get_operation("From Hex").unwrap(),
        vec![ArgValue::Str("Auto".into())],
    )
    .run_text("Hello")?;

assert_eq!(result, "Hello");
# Ok::<(), rxchef::PipelineError>(())
```

`Pipeline::run` accepts `OperationData`; convenience methods `run_text` and `run_bytes` preserve the expected output representation. Each boundary consults `Operation::input_type` and coerces compatible values among bytes, UTF-8 text, numbers, and JSON. The error includes the zero-based failing step index, operation name, and source `OperationError`.

An empty pipeline is a pass-through. Any number of operations can be appended with `then`; execution is deterministic and left-to-right.

## Runtime discovery and argument parsing

```rust
use rxchef::runtime;

for name in runtime::operation_names(Some("base64")) {
    let info = runtime::operation_info(&name)?;
    println!("{}: {}", info.name, info.description);
}

let output = runtime::run_operation(
    "to_base64",
    b"Hello".to_vec(),
    &[],
)?;
assert_eq!(output, b"SGVsbG8=");
# Ok::<(), String>(())
```

`OperationInfo` exposes the same name, module, description, types, broken flag, and argument schema used to generate the [operation reference](reference/operations.md). `runtime::run_operation` also understands CLI typed prefixes.

## Implementing an operation

Implement the `Operation` trait on a public unit struct in `src/operations/<module>.rs`. The build script discovers unit structs, regenerates the registry, and makes the operation available through `operation_names` and `get_operation`. Provide non-empty metadata and a test file with the same module filename under `tests/tests/operations/`.

After changes:

```console
cargo fmt --all -- --check
cargo test --workspace
cargo run --example generate_operation_docs
cargo run --example generate_operation_docs -- --check
```
