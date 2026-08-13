# Jsonata Query

!!! warning "Optional backend unavailable"

    This operation is feature-gated and unavailable in the minimal documentation build. See the feature matrix for the required Cargo feature.

Query and transform JSON data using jaq. Jsonata is not natively available in Rust,
        so jaq is used as an alternative. Enable with: --features jsonata

- Input: `JSON`
- Output: `JSON`
- CLI: `rxchef run "Jsonata Query"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Query | `.` | The jaq query to run |

