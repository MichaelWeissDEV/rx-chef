# Rust library

The `rxchef` crate exposes operation discovery, direct execution, typed values, pipelines, Magic, and scanning. Operations are stateless boxed trait objects and can be looked up by canonical name.

## Choose the right API layer

| Need | API |
|---|---|
| Stable operation descriptors | `rxchef::catalog` |
| One operation with structured execution errors | `rxchef::execute::run` |
| Complete recipe, variables, tracing, and limits | `rxchef::execute::bake` |
| Serializable UI/server result envelope | `rxchef::integration` |
| Typed in-process linear pipeline | `rxchef::Pipeline` |
| Low-level metadata and argument parsing | `rxchef::runtime` |
| Recursive decoder search | `rxchef::magic` |
| Token/entropy scanning | `rxchef::scan` |
| Direct operation trait objects | `rxchef::operations` |

Applications should begin with `catalog` and `execute`. The lower layers are
public for hosts that need typed composition or specialized policies, but they
expose more internal concepts and require more decisions from the caller.

The Store is a separate crate. Depending on `rxchef` does not create user files,
discover `.rxchef`, or import CLI state.

## Add the dependency

From a checkout or workspace:

```toml
[dependencies]
rxchef = { path = "../rxchef" }
```

Enable optional operation backends explicitly:

```toml
[dependencies]
rxchef = { path = "../rxchef", features = ["pgp", "jsonata"] }
```

`full` enables all optional backends and may require native OCR libraries. A
minimal embedder should select only the capabilities it exposes.

## Stable catalog API

List descriptors in stable name order and resolve normalized names:

```rust
use rxchef::catalog;

let operations = catalog::operations()?;
let base64 = catalog::describe("from-base64")?;

assert_eq!(base64.name, "From Base64");
assert!(operations.iter().any(|item| item.id == base64.id));
# Ok::<(), rxchef::catalog::CatalogError>(())
```

An `OperationDescriptor` contains the complete argument schema plus input/output
types, input requirement, implementation status, availability, features,
platform targets, side effects, determinism, parity, limitations, and the
documentation slug. Render forms directly from descriptors instead of copying
argument lists into application code.

Catalog errors distinguish “not found” from invalid registry metadata. Feature-
disabled operations remain describable; check `availability` before presenting
an execution action.

## High-level single-operation execution

```rust
use rxchef::execute;

let outcome = execute::run(
    "XOR",
    vec![0x00, 0xff, 0x41],
    vec![
        "hex:2a".into(),
        "Standard".into(),
        "false".into(),
    ],
)?;

assert_eq!(outcome.output.len(), 3);
# Ok::<(), execute::ExecutionError>(())
```

`execute::run` treats the supplied vector as present input even when it is empty.
This is usually the desired Rust API behavior. Hosts that model “no input
source” must construct an `ExecutionRequest` and set `input_supplied` explicitly.

Arguments use the same string parser as CLI recipes. This makes configuration
portable, while the lower typed `Pipeline` API avoids string parsing when the
caller already owns typed values.

## Complete execution request

```rust
use rxchef::execute::{
    bake, ExecutionOptions, ExecutionRequest, RecipeStep, VariableContext,
};

let variables = VariableContext::new([
    ("KEY".to_string(), "hex:2a".to_string()),
]);

let request = ExecutionRequest {
    input: b"hello".to_vec(),
    input_supplied: true,
    recipe: vec![
        RecipeStep {
            op: "XOR".into(),
            args: vec!["$KEY".into(), "Standard".into(), "false".into()],
        },
        RecipeStep {
            op: "To Base64".into(),
            args: vec![],
        },
    ]
    .into(),
    variables,
    options: ExecutionOptions {
        trace: true,
        max_steps: 1_000,
        max_output_bytes: Some(16 * 1024 * 1024),
    },
};

let outcome = bake(request)?;
assert_eq!(outcome.trace.len(), 2);
assert!(!outcome.output.is_empty());
# Ok::<(), rxchef::execute::ExecutionError>(())
```

The recipe is validated before execution. `max_steps` counts control-flow and
repeated branch/jump work, not only the source array length. `max_output_bytes`
applies to intermediate as well as final results.

Trace entries contain operation identity, byte counts, duration, and status.
They deliberately do not retain payloads or arguments, which keeps secrets and
large intermediate data out of diagnostics.

## Missing versus empty input

The two states are modeled explicitly:

```rust
use rxchef::execute::{
    bake, ExecutionOptions, ExecutionRequest, RecipeStep, VariableContext,
};

let request = ExecutionRequest {
    input: Vec::new(),
    input_supplied: false,
    recipe: vec![RecipeStep {
        op: "From Base64".into(),
        args: vec![],
    }]
    .into(),
    variables: VariableContext::default(),
    options: ExecutionOptions::default(),
};

assert!(bake(request).is_err());
```

Set `input_supplied: true` with the same empty vector to represent an explicitly
empty file or body. Required-input operations accept the latter and reject the
former.

## Structured execution errors

Match variants rather than parsing `Display` text:

```rust
use rxchef::execute::ExecutionError;

fn classify(error: &ExecutionError) -> &'static str {
    match error {
        ExecutionError::InvalidRecipe(_) => "recipe",
        ExecutionError::Step { .. } => "step",
        ExecutionError::RuntimeStep { .. } => "runtime",
        ExecutionError::StepLimitExceeded { .. } => "step-limit",
        ExecutionError::OutputLimitExceeded { .. } => "output-limit",
    }
}
```

`RuntimeStep` retains a `runtime::RuntimeError` source with variants for unknown
operations, feature/platform availability, invalid arguments, operation domain
errors, and output validation. The one-based step index and operation name are
available at the recipe error layer.

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
# Ok::<(), rxchef::runtime::RuntimeError>(())
```

`OperationInfo` exposes implementation status and availability separately, plus
the same name, module, description, types, input requirement, side effects,
feature requirements, and explicit argument schema used by every frontend.
`runtime::run_operation` returns the structured `RuntimeError` variants
`UnknownOperation`, `Unavailable`, `InvalidArgument`, `Operation`, and
`OutputValidation`; it also understands CLI typed prefixes.

## Implementing an operation

Implement the `Operation` trait on a public unit struct in
`src/operations/<module>.rs`. Registry generation is an explicit developer
action and never rewrites the source tree from `build.rs`. Provide complete
metadata, explicit verification evidence, documentation, and a test module.

After changes:

```console
cargo fmt --all -- --check
cargo test --workspace
cargo xtask generate-registry
cargo xtask check-registry
cargo xtask audit-operations
cargo xtask docs
cargo xtask docs --check
```
