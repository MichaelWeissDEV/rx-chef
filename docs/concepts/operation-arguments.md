# Operation arguments

Every operation in rxchef is described by an argument schema. This schema tells the runtime which parameters exist, what they are called, what defaults they carry, and in what order they are expected.

## Why this matters

Argument metadata is important for:

- CLI auto-help,
- validation,
- default handling,
- and consistent behavior across runtime interfaces.

## Example

A typical operation may declare:

- a string or bytes input,
- a delimiter,
- a mode flag,
- or a numeric option.

These values are then passed to the transformation logic when the operation is executed.

## Related pages

- [Data model](data-model.md)
- [Input and output](input-output.md)
- [Pipelines](pipelines.md)
