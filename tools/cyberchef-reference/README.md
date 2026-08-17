# CyberChef reference capture

`capture.mjs` runs operations in an upstream CyberChef checkout and records
their output, so that the differential fixture in
`tests/fixtures/differential/cases.json` holds values that were **observed**
from upstream rather than recalled or reconstructed.

This is a developer tool. It is not part of the build and not part of the
release gate: the gate compares rx-chef against the recorded fixture, and this
script is what records the fixture in the first place. Nothing in CI needs Node
or a CyberChef checkout.

## Requirements

A CyberChef checkout with dependencies installed:

```bash
git clone https://github.com/gchq/CyberChef && cd CyberChef && npm install
```

## Usage

Write a request file — a JSON array of cases:

```json
[
  { "operation": "To Base64", "input": "foobar", "input_encoding": "text", "args": ["A-Za-z0-9+/="] },
  { "operation": "From Hex",  "input": "666f6f", "input_encoding": "text", "args": ["Auto"] }
]
```

`input_encoding` is `text`, `hex`, or `empty`. `args` are positional and must
match the upstream operation's argument order.

Then capture:

```bash
CYBERCHEF_DIR=/path/to/CyberChef node tools/cyberchef-reference/capture.mjs requests.json
```

Each entry comes back with `reference`, `reference_encoding` (`text` when the
output round-trips through UTF-8, otherwise `hex`), and `cyberchef_version`, or
with `error` when the operation rejected the input.

## Turning results into fixture cases

Copy the observed value into a fixture case and cite where it came from:

```json
{
  "operation": "To Base64",
  "input": "foobar",
  "input_encoding": "text",
  "args": ["A-Za-z0-9+/="],
  "reference": "Zm9vYmFy",
  "reference_encoding": "text",
  "expect": "exact",
  "reference_source": "observed: CyberChef 11.0.0, operation defaults"
}
```

Every case must carry a `reference_source` a reviewer can check: either an
observation like the above, or a published specification both implementations
follow. **Never** fill `reference` with rx-chef's own output — that turns the
differential test into a change detector and proves nothing about parity.

## When a comparison disagrees

A mismatch is a question, not a verdict. Before touching either side, work out
which of these it is:

1. **rx-chef is wrong** — fix rx-chef, add a regression test naming the defect.
2. **The operations were given different arguments** — most often the two
   projects disagree about a *default*. Check the schemas on both sides before
   concluding anything about the algorithm.
3. **CyberChef relies on browser behaviour** this harness cannot reproduce
   (canvas, `File`, workers). Mark the case `not_comparable`.
4. **The difference is deliberate** — record it as `documented_difference` and
   state the reason in `known_limitations()` on the operation.

## Limitations

- Operations needing browser-only globals cannot be loaded; a few also abort the
  Node process outright via emscripten modules, so bulk runs should isolate
  operations in child processes.
- Non-deterministic operations (time, randomness, network, filesystem) are
  excluded from comparison by the harness and must stay `not_comparable`.
- Argument defaults are read from the operation's own schema. Nested option
  groups do not always reduce to a single scalar, so a captured value is only
  trustworthy when the arguments were passed explicitly or both sides agree.
