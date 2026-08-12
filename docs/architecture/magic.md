# Magic engine

The magic engine is a recursive decode workflow designed to identify and unwrap layered encodings automatically.

Instead of requiring the user to know exactly how a payload was encoded, `magic` tries a set of plausible decoding strategies, evaluates the result, and continues until it finds a likely plaintext candidate.

## Core idea

The engine operates like a best-effort decode search:

1. inspect input for likely encodings,
2. apply candidate decoders,
3. score the resulting output,
4. recurse into promising results,
5. rank the final candidates by plausibility.

## Why it matters

Real-world payloads often contain nested encodings such as:

- hex inside base64,
- URL encoding inside compressed data,
- or JSON payloads embedded in text wrappers.

Manual unpacking is tedious and error-prone. Magic reduces this burden by exploring likely chains and ranking the outputs.

## Scoring

The engine typically ranks results based on characteristics such as:

- printability,
- entropy,
- structure,
- and whether the result matches a specified crib or pattern.

This lets the engine prefer human-readable output over noisy or partially decoded garbage.

## Related pages

- [Scan](scan.md)
- [Execution engine](execution-engine.md)
- [Architecture overview](overview.md)
