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
 * Output: the same array on stdout with `reference`, `reference_encoding`,
 *         and `cyberchef_version` filled in, or `error` when the operation
 *         rejected the input.
 */

import fs from "fs";
import path from "path";

const cyberchefDir = process.env.CYBERCHEF_DIR;
if (!cyberchefDir) {
    console.error("CYBERCHEF_DIR must point at a CyberChef checkout with node_modules installed");
    process.exit(2);
}

const pkg = JSON.parse(fs.readFileSync(path.join(cyberchefDir, "package.json"), "utf8"));
const operationsDir = path.join(cyberchefDir, "src/core/operations");

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
        case "JSON":
            return JSON.parse(buffer.toString("utf8") || "null");
        default:
            return buffer.toString("utf8");
    }
}

/** Normalise whatever the operation returned into a Buffer. */
function toBuffer(result, outputType) {
    if (result instanceof ArrayBuffer) return Buffer.from(result);
    if (Array.isArray(result)) return Buffer.from(result);
    if (typeof result === "string") return Buffer.from(result, "utf8");
    if (typeof result === "number") return Buffer.from(String(result), "utf8");
    if (result && typeof result === "object") return Buffer.from(JSON.stringify(result), "utf8");
    return Buffer.from(String(result ?? ""), "utf8");
}

const operations = await buildOperationMap();
const requests = JSON.parse(fs.readFileSync(process.argv[2], "utf8"));
const results = [];

for (const request of requests) {
    const entry = { ...request, cyberchef_version: pkg.version };
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

process.stdout.write(JSON.stringify(results, null, 2) + "\n");
