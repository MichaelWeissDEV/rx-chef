/**
 * Capture reference outputs from an upstream CyberChef checkout.
 *
 * This exists so the differential fixture holds values that were *observed*
 * from upstream rather than recalled or reconstructed. It is a developer tool,
 * not part of the rx-chef build: the release gate compares rx-chef against the
 * recorded fixture, and this script is what records it.
 *
 * Usage:
 *   CYBERCHEF_DIR=/path/to/CyberChef node tools/cyberchef-reference/capture.mjs requests.json
 *
 * Input:  JSON array of { operation, input, input_encoding, args }
 * Output: an object on stdout with a `provenance` block and a `results` array.
 *         Each result carries `reference` and `reference_encoding`, or `error`
 *         when the operation rejected the input.
 *
 * Provenance is recorded because a package version alone cannot identify what
 * was actually observed: "CyberChef 11.0.0" covers every commit between two
 * releases. The upstream git commit is what makes a recorded value
 * reproducible, so it is required unless the caller opts out explicitly.
 */

import { execFileSync } from "child_process";
import fs from "fs";
import path from "path";
import { pathToFileURL } from "url";

const CAPTURE_TOOL_VERSION = "2";

const cyberchefDir = process.env.CYBERCHEF_DIR;
if (!cyberchefDir) {
    console.error("CYBERCHEF_DIR must point at a CyberChef checkout with node_modules installed");
    process.exit(2);
}

const pkg = JSON.parse(fs.readFileSync(path.join(cyberchefDir, "package.json"), "utf8"));
const operationsDir = path.join(cyberchefDir, "src/core/operations");
const BigNumber = (await import(pathToFileURL(path.join(
    cyberchefDir,
    "node_modules/bignumber.js/dist/bignumber.mjs"
)))).default;

/**
 * Run git in the checkout.
 *
 * `trim` is opt-out because `git status --porcelain` encodes the status in the
 * first two columns: trimming the whole output eats the leading space of the
 * first line and corrupts that entry's path.
 */
function git(args, { trim = true } = {}) {
    try {
        const out = execFileSync("git", ["-C", cyberchefDir, ...args], {
            encoding: "utf8",
            stdio: ["ignore", "pipe", "ignore"],
        });
        return trim ? out.trim() : out;
    } catch {
        return null;
    }
}

/**
 * Describe exactly which upstream tree produced these values.
 *
 * A dirty checkout is not silently accepted: uncommitted changes mean the
 * recorded commit does not describe what actually ran. Modified operation
 * sources invalidate the capture outright and abort it; other local changes
 * (lockfiles, scratch files) are recorded so a reviewer can judge them.
 */
function provenance() {
    const commit = git(["rev-parse", "HEAD"]);
    if (!commit) {
        console.error(
            `${cyberchefDir} is not a git checkout; a recorded reference must name an upstream commit.\n` +
                "Clone CyberChef with git, or set RXCHEF_ALLOW_UNPINNED_REFERENCE=1 to record an unpinned capture."
        );
        if (process.env.RXCHEF_ALLOW_UNPINNED_REFERENCE !== "1") process.exit(2);
    }

    const status = git(["status", "--porcelain"], { trim: false }) ?? "";
    // Porcelain v1 lines are a two-character status field, a space, then the
    // path. Renames read "old -> new"; the new path is what matters here.
    const dirtyPaths = status
        .split("\n")
        .filter(Boolean)
        .map((line) => line.slice(2).trim().split(" -> ").pop().trim())
        .filter(Boolean);
    const dirtyOperations = dirtyPaths.filter(
        (entry) => entry.startsWith("src/core/operations") || entry.startsWith("src/core/lib")
    );
    if (dirtyOperations.length > 0) {
        console.error(
            "Refusing to capture: the CyberChef checkout has uncommitted changes to operation sources, " +
                `so the recorded commit would not describe what ran:\n  ${dirtyOperations.join("\n  ")}`
        );
        process.exit(2);
    }

    return {
        cyberchef_commit: commit,
        cyberchef_describe: git(["describe", "--tags", "--always"]),
        cyberchef_version: pkg.version,
        cyberchef_dirty: dirtyPaths.length > 0,
        // Recorded so a reviewer can see the local changes were irrelevant to
        // the operations, rather than having to take that on trust.
        cyberchef_dirty_paths: dirtyPaths,
        captured_at: new Date().toISOString(),
        capture_tool: "tools/cyberchef-reference/capture.mjs",
        capture_tool_version: CAPTURE_TOOL_VERSION,
        node_version: process.version,
    };
}

/** Map every loadable operation's display name to its module file. */
async function buildOperationMap() {
    const map = new Map();
    for (const file of fs.readdirSync(operationsDir).filter((f) => f.endsWith(".mjs"))) {
        try {
            const mod = await import(path.join(operationsDir, file));
            const op = new mod.default();
            map.set(op.name, { file, ctor: mod.default });
        } catch {
            // Operations needing browser-only globals cannot be compared here.
        }
    }
    return map;
}

function decodeInput(value, encoding) {
    switch (encoding) {
        case "empty":
            return Buffer.alloc(0);
        case "hex":
            return Buffer.from(value, "hex");
        case "text":
        default:
            return Buffer.from(value, "utf8");
    }
}

/** Present the input in whatever shape the operation declares it wants. */
function coerceForOperation(buffer, inputType) {
    switch (inputType) {
        case "string":
        case "html":
            return buffer.toString("utf8");
        case "byteArray":
            return Array.from(buffer);
        case "ArrayBuffer":
            return buffer.buffer.slice(buffer.byteOffset, buffer.byteOffset + buffer.byteLength);
        case "number":
            return Number(buffer.toString("utf8"));
        case "BigNumber":
            return new BigNumber(buffer.toString("utf8"));
        case "JSON":
            return JSON.parse(buffer.toString("utf8") || "null");
        default:
            return buffer.toString("utf8");
    }
}

/** Normalise whatever the operation returned into a Buffer. */
function toBuffer(result, outputType) {
    if (outputType === "BigNumber") return Buffer.from(result.toString(10), "utf8");
    if (result instanceof ArrayBuffer) return Buffer.from(result);
    if (Array.isArray(result)) return Buffer.from(result);
    if (typeof result === "string") return Buffer.from(result, "utf8");
    if (typeof result === "number") return Buffer.from(String(result), "utf8");
    if (result && typeof result === "object") return Buffer.from(JSON.stringify(result), "utf8");
    return Buffer.from(String(result ?? ""), "utf8");
}

const capture = provenance();
const operations = await buildOperationMap();
const requests = JSON.parse(fs.readFileSync(process.argv[2], "utf8"));
const results = [];

for (const request of requests) {
    const entry = { ...request };
    const found = operations.get(request.operation);
    if (!found) {
        entry.error = `operation not loadable in this CyberChef checkout: ${request.operation}`;
        results.push(entry);
        continue;
    }
    try {
        const op = new found.ctor();
        const buffer = decodeInput(request.input ?? "", request.input_encoding ?? "text");
        const value = coerceForOperation(buffer, op.inputType);
        const output = toBuffer(await op.run(value, request.args ?? []), op.outputType);
        // Prefer readable text when the result round-trips through UTF-8.
        const asText = output.toString("utf8");
        if (Buffer.from(asText, "utf8").equals(output)) {
            entry.reference = asText;
            entry.reference_encoding = "text";
        } else {
            entry.reference = output.toString("hex");
            entry.reference_encoding = "hex";
        }
    } catch (error) {
        entry.error = String(error && error.message ? error.message : error);
    }
    results.push(entry);
}

// A short, human-readable citation for `reference_source` in fixture cases.
const shortCommit = capture.cyberchef_commit ? capture.cyberchef_commit.slice(0, 12) : "unpinned";
capture.citation =
    `observed: CyberChef ${capture.cyberchef_version} @ ${shortCommit}` +
    (capture.cyberchef_dirty ? " (checkout dirty; operation sources clean)" : "");

process.stdout.write(JSON.stringify({ provenance: capture, results }, null, 2) + "\n");
