#!/usr/bin/env node
import { createHash } from "node:crypto";
import { readFile, writeFile } from "node:fs/promises";
import path from "node:path";

const [oraclePath, outputPath] = process.argv.slice(2);
if (!oraclePath || !outputPath) {
  throw new Error("usage: verify-baseline-point-actionability.mjs <point-oracle.json> <output.json>");
}

const bytes = await readFile(oraclePath);
const oracle = JSON.parse(bytes.toString("utf8"));
const camera = oracle.baseline_camera;
if (!camera || !Array.isArray(oracle.probes)) throw new Error("baseline camera/probes unavailable");

const subtract = (a, b) => a.map((value, index) => value - b[index]);
const dot = (a, b) => a.reduce((sum, value, index) => sum + value * b[index], 0);
const cross = (a, b) => [
  a[1] * b[2] - a[2] * b[1],
  a[2] * b[0] - a[0] * b[2],
  a[0] * b[1] - a[1] * b[0]
];
const normalize = (value) => {
  const length = Math.sqrt(dot(value, value));
  if (!Number.isFinite(length) || length === 0) throw new Error("invalid camera basis vector");
  return value.map((component) => component / length);
};

const forward = normalize(subtract(camera.target, camera.position));
const right = normalize(cross(forward, camera.up));
const correctedUp = cross(right, forward);
const tangent = Math.tan(camera.fovDegrees * Math.PI / 360);
const results = oracle.probes.map((probe) => {
  const relative = subtract(probe.authored_anchor, camera.position);
  const axialDepth = dot(relative, forward);
  const recomputedNdc = [
    dot(relative, right) / (axialDepth * tangent * camera.aspect),
    dot(relative, correctedUp) / (axialDepth * tangent)
  ];
  const declaredNdc = probe.baseline.ndc;
  const maximumNdcError = Math.max(...recomputedNdc.map((value, index) => Math.abs(value - declaredNdc[index])));
  const insideClosedNdcSquare = recomputedNdc.every((value) => value >= -1 && value <= 1);
  const declaredOutside = probe.baseline.actionability === "NOT_ATTEMPTED_OUTSIDE_BASELINE_FRUSTUM";
  return {
    sample: probe.sample,
    probeAnchorRef: probe.probe_anchor_ref,
    authoredAnchor: probe.authored_anchor,
    declaredNdc,
    recomputedNdc,
    maximumNdcError,
    insideClosedNdcSquare,
    declaredActionability: probe.baseline.actionability,
    declaredOracleStatus: probe.baseline.oracle?.status ?? null,
    declaredOracleReason: probe.baseline.oracle?.reason ?? null,
    classification: declaredOutside && !insideClosedNdcSquare
      ? "VERIFIED_OUTSIDE_BASELINE_CLOSED_NDC_SQUARE"
      : !declaredOutside && insideClosedNdcSquare
      ? "VERIFIED_INSIDE_BASELINE_CLOSED_NDC_SQUARE"
      : "FAIL_DECLARATION_RECOMPUTATION_MISMATCH"
  };
});

const outside = results.filter((result) => result.classification === "VERIFIED_OUTSIDE_BASELINE_CLOSED_NDC_SQUARE");
const mismatch = results.filter((result) => result.classification === "FAIL_DECLARATION_RECOMPUTATION_MISMATCH");
const maxError = Math.max(...results.map((result) => result.maximumNdcError));
const output = {
  schema: "openpipestress.ui-foundation.baseline-point-actionability-verification/v1",
  status: mismatch.length === 0 && outside.length === 20 && maxError <= 1e-9
    ? "PASS_20_DECLARED_OUTSIDE_PROBES_INDEPENDENTLY_REPROJECTED"
    : "FAIL_ACTIONABILITY_VERIFICATION",
  oracle: {
    path: path.resolve(oraclePath),
    sha256: createHash("sha256").update(bytes).digest("hex")
  },
  method: {
    organization: "independent exhaustive authored-anchor reprojection; no product chunk/index reuse",
    cameraBasis: "forward=normalize(target-position); right=normalize(cross(forward,up)); correctedUp=cross(right,forward)",
    projection: "x=dot(relative,right)/(axialDepth*tan(fov/2)*aspect); y=dot(relative,correctedUp)/(axialDepth*tan(fov/2))",
    closedNdcSquare: "-1 <= x <= 1 and -1 <= y <= 1",
    ndcAgreementTolerance: 1e-9
  },
  counts: {
    probes: results.length,
    verifiedInside: results.filter((result) => result.classification === "VERIFIED_INSIDE_BASELINE_CLOSED_NDC_SQUARE").length,
    verifiedOutside: outside.length,
    mismatches: mismatch.length
  },
  maximumDeclaredVsRecomputedNdcError: maxError,
  outsideSamples: outside,
  mismatchSamples: mismatch
};
await writeFile(outputPath, `${JSON.stringify(output, null, 2)}\n`);
if (output.status.startsWith("FAIL_")) process.exitCode = 1;
