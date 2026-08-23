# What "verified" means

The operation audit (`cargo run -p xtask -- audit-operations`) reports every
registered operation as 100% "correctness verified". That is a real, checked
claim, but it is narrower than it sounds, and it is a different claim from
CyberChef parity. This page defines both precisely.

## Correctness verified is not the same as independently verified

"Correctness verified" means: the operation has executable tests, at least
one of them asserts an exact expected value (not just "did not error"), and
either a negative-input case or a recorded justification for why one does
not apply. It says the operation's behavior is pinned down by tests.

It does **not** by itself say where the expected values in those tests came
from. A known-answer test can assert an exact value that was established by
running this crate's own implementation once and hardcoding the result —
that catches regressions, but it cannot catch the implementation being wrong
in a way that is consistent with itself. Only evidence checked against
something outside this repository can do that.

## Evidence provenance

Each operation's entry in `verification/operations.json` may carry an
`evidence_provenance` array, recording where a specific evidence bucket's
expected values actually came from:

```json
"evidence_provenance": [
  {
    "target": "differential",
    "type": "cyberchef_differential",
    "source": "gchq/CyberChef",
    "commit": "b92501ee354256a127479f93d4c31a4f1d0dd657",
    "path_in_source": "src/core/vendor/gost/gostCipher.mjs",
    "notes": "wrapKeyGOST/wrapKeyCP invoked directly under Node to produce reference vectors."
  }
]
```

`target` names which evidence bucket (`correctness`, `known_answer`,
`differential`, or `property`) the record documents. The audit rejects a
record whose `target` points at a bucket that is empty for that operation —
provenance cannot be recorded for evidence that does not exist.

### Provenance types

| Type | Independent? | Meaning |
|---|:---:|---|
| `rfc` | yes | Checked against a value published in an RFC. |
| `nist` | yes | Checked against a NIST-published test vector. |
| `fips` | yes | Checked against a FIPS-published test vector. |
| `standard` | yes | Checked against another named published standard. |
| `published_test_vector` | yes | A known-answer vector published outside this project, not covered by the above. |
| `upstream_fixture` | yes | Checked against a fixture shipped by an upstream project (not CyberChef itself). |
| `cyberchef_differential` | yes | Checked directly against CyberChef's own implementation (including a vendored library CyberChef itself calls), by running it and comparing output. |
| `independent_implementation` | yes | Checked against a second, independently written implementation. |
| `mathematical_invariant` | no | Checked against a property the operation must satisfy (e.g. an algebraic identity), not an external expected value. |
| `property_test` | no | Randomized/property-based testing over the operation's own contract. |
| `internal_regression` | no | An expected value pinned by this project, with no external source. |
| `roundtrip` | no | Self-consistency only (e.g. `decode(encode(x)) == x`); never compared to an external value. |
| `self_generated_expected_value` | no | The expected value was produced by running this crate's own implementation and recording the output. |

"Independent" evidence types are the ones that can, on their own, answer "is
this correct" rather than only "did this change". The audit enforces the
type list, and requires `source` on the published-reference types and
`commit`/`version` on the types that name a specific external artifact — so
a provenance record has to say concretely what was checked against, not
just assert a category.

An operation with no `evidence_provenance` entries is not flagged as wrong;
it means its evidence has not been classified yet. That is reported
honestly as "not yet classified", never silently counted as either
independent or self-generated, and never backfilled with an invented
source. Classifying the remaining operations is ongoing, incremental work —
see the audit summary's "with typed provenance" and "independent evidence
(typed)" counts, and the [operation matrix](operation-matrix.md), for
current coverage.

## Correctness verification is not CyberChef parity

Separately from correctness, each operation carries a `parity` status:

- **Exact** — this operation's output is verified to match CyberChef's
  output for the same input, via recorded differential evidence.
- **Compatible** — behavior matches CyberChef's documented contract but
  byte-for-byte parity has not been established.
- **Documented difference** — a specific, intentional divergence from
  CyberChef is recorded (see `known_limitations` in the registry).
- **Unverified** — parity with CyberChef has not been checked either way.
- **Not applicable** — the operation has no CyberChef counterpart.

100% correctness verification does **not** mean 100% CyberChef parity. See
the [operation matrix](operation-matrix.md) for the current parity
breakdown. A claim of "N/478 operations verified" refers to correctness
verification as defined on this page; it is never shorthand for CyberChef
parity, and should not be read as one.

## Related pages

- [Operation matrix](operation-matrix.md)
- [Reference index](operations.md)
- [Adding an operation](../development/adding-an-operation.md)
