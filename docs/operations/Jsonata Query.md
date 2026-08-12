# Jsonata Query

!!! warning "Experimental / known broken"

    This operation is marked as broken in the runtime registry.

Query and transform JSON data using jaq. Jsonata is not natively available in Rust,
        so jaq is used as an alternative. Enable with: --features jsonata

- Input: `JSON`
- Output: `JSON`
- CLI: `rxchef run "Jsonata Query"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Query | `.` | The jaq query to run |

