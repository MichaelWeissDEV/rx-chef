# Pipeline engine

The pipeline engine is responsible for chaining multiple operations into a single, reproducible data flow.

A pipeline is effectively a sequence of transformations. Each step consumes output from the previous step and passes it forward, allowing more complex operations to be composed from simpler building blocks.

## Pipeline flow

A typical pipeline runs as follows:

1. parse the pipeline definition,
2. resolve each operation,
3. feed input into the first step,
4. carry intermediate output to the next step,
5. propagate errors and tracing information as needed.

## Why pipelines are important

Pipelines are the core of a composable terminal workflow. They allow a user to do things like:

- base64-decode then hash,
- normalize text then apply regex,
- or transform binary into JSON and then pretty print it.

The resulting workflow is easier to share, test, and automate than ad hoc shell snippets.

## Type handling

Where possible, the engine attempts to handle differences in input/output types between steps. This reduces friction in cases where one operation produces a string, another expects text, and a third works with structured data.

## Related pages

- [Execution engine](execution-engine.md)
- [Concepts: Pipelines](../concepts/pipelines.md)
- [Architecture overview](overview.md)
