# Adding an operation

Adding a new operation should usually begin with the operation registry and metadata model, then the implementation itself, then the docs and tests.

## Typical steps

1. decide the operation’s name and category,
2. define the input/output contract,
3. register the operation in the registry,
4. implement the transformation logic,
5. add tests for the expected behavior,
6. write or update the operation docs page.

## Good practices

- keep the operation contract explicit,
- prefer small, testable logic,
- validate edge cases and malformed input,
- and ensure the output remains predictable for CLI and pipeline use.

## Related pages

- [Building](building.md)
- [Documenting an operation](documenting-an-operation.md)
- [Testing](testing.md)
