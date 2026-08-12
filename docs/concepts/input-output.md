# Input and output

rxchef is designed around clean input/output boundaries so it works well in shell pipelines and automation.

## Input sources

Operations may receive input from:

- direct arguments,
- file paths,
- stdin,
- or the previous step in a pipeline.

## Output destinations

Most CLI operations write to stdout, while diagnostic information is kept on stderr when needed. This keeps data pipelines clean and makes shell composition natural.

## Why this design matters

It allows a command to behave like a standard Unix filter and therefore compose with tools such as `grep`, `jq`, `sed`, or other shell utilities.

## Related pages

- [Data model](data-model.md)
- [Pipelines](pipelines.md)
- [CLI overview](../cli/index.md)
