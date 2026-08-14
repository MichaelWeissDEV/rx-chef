# PHP Serialize

## Overview

Performs PHP serialization on JSON data.<br><br>This function does not support <code>object</code> tags.<br><br>Since PHP doesn't distinguish dicts and arrays, this operation is not always symmetric to <code>PHP Deserialize</code>.<br><br>Example:<br><code>[5,&quot;abc&quot;,true]</code><br>becomes<br><code>a:3:{i:0;i:5;i:1;s:3:&quot;abc&quot;;i:2;b:1;}<code>

## Status

| Field | Value |
|---|---|
| Implementation | `Partial` |
| Parity | `Unknown` |
| Availability | Available |
| Features | none |
| Side effects | `[]` |
| Deterministic | true |

## Input

Declared input type: `JSON`.

## Output

Declared output type: `String`. Redirect stdout or use `--output-file` for exact binary bytes.

## Arguments

This operation has no arguments.

## How it works

Performs PHP serialization on JSON data.<br><br>This function does not support <code>object</code> tags.<br><br>Since PHP doesn't distinguish dicts and arrays, this operation is not always symmetric to <code>PHP Deserialize</code>.<br><br>Example:<br><code>[5,&quot;abc&quot;,true]</code><br>becomes<br><code>a:3:{i:0;i:5;i:1;s:3:&quot;abc&quot;;i:2;b:1;}<code>

## Implementation

The implementation is in `src/operations/php_serialize.rs` and declares `JSON` input and `String` output. Its operation module owns the conversion and error rules; every public frontend invokes it through `rxchef::execution`.

## Examples

```console
printf 'input' | rxchef run "PHP Serialize"
```

For file or binary input use `rxchef run "PHP Serialize" --input-file INPUT --output-file OUTPUT`.

## Pipeline usage

```console
printf 'input' | rxchef pipe "PHP Serialize" to_base64
```

## Error conditions

Invalid input representations, invalid argument values, unavailable feature backends, and operation-specific processing failures return an error and a non-zero CLI status. Exact limitations are listed below when known.

## CyberChef compatibility

Parity status: `Unknown`. `Unknown` means compatibility has not been independently verified and must not be read as an exact-match claim.

## Security considerations

Side effects: `[]`. Treat parser inputs as untrusted and use execution limits for large data. Sensitive arguments are redacted by metadata-aware History output.

## Testing

Correctness:
- tests/tests/operations/php_serialize.rs

Known-answer:
- none recorded

Differential:
- none recorded

Property:
- none recorded

Fuzz:
- none recorded

## Performance

Not measured. Reason: No stable representative benchmark case is defined; operation remains Partial until performance evidence is reviewed.

## Limitations

No verified limitation metadata is currently recorded; this is not a claim of perfect upstream parity.

## References

- [Operation quality matrix](../reference/operation-matrix.md)
- [CLI run documentation](../cli/run.md)
