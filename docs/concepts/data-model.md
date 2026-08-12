# Data model

rxchef works with heterogeneous input data and normalizes it into forms that operations can consume predictably.

## Core concept

The project is built around a few common data shapes:

- text,
- binary data,
- structured content such as JSON or YAML,
- and metadata-rich values such as file information or reports.

## Why a shared model matters

Operations are not all the same. Some accept strings, some accept bytes, and some consume structured input. The data model provides a common vocabulary so the runtime can adapt inputs without requiring every operation to reimplement conversion logic.

## Operational view

The runtime tries to:

- preserve raw bytes when needed,
- decode textual input when appropriate,
- and keep metadata visible for debugging and trace output.

This is particularly important in pipelines, where the output of one operation becomes the input to the next.

## Related pages

- [Input and output](input-output.md)
- [Operation arguments](operation-arguments.md)
- [Pipelines](pipelines.md)
