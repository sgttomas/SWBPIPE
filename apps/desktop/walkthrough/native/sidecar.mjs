// An attended capture's sidecar (../README.md, "Captures"): read once, bounded, and parsed from the very bytes that
// are hashed, by the runner when the launch ends (run.mjs readCaptures) and again by the index when it is composed
// (../compose-index.mjs attendedBindings), so that what a capture is counted for is what its recorded bytes say.
import { createHash } from "node:crypto";
import * as fs from "node:fs";
import { SURFACE_KEYS } from "./render.mjs";

export const SIDECAR_LIMIT = 64 * 1024;
/**
 * An account of a surface not shown that does not say what in the system or the application kept it from view: one
 * naming the attendant's own capture tool, its setup or its permission, or one that leaves the reason unresolved.
 */
export const UNACCOUNTED = /\b(tools?|tooling|set ?up|setup|not configured|unconfigured|(capture|recording|screen[- ]?(capture|recording)|accessibility) permission|unknown|unclear|unresolved|undetermined|not sure|no idea|could not (tell|determine|say)|for some reason)\b/i;
/** DEL-07-06 AC-007's use cases by their names there: an attended capture's sidecar names one of them, or the launch's journey. */
export const AC007_USE_CASES = Object.freeze(["Blank to released report", "Timed entry", "Missing data", "Failed solve", "Change a support and compare", "Review and export results", "Every required analysis", "User rule pack", "Save, close and resume"]);
const ISO_TIME = /^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}(:\d{2}(\.\d{1,9})?)?(Z|[+-]\d{2}:\d{2})$/;

export const sha256Of = (bytes) => `sha256:${createHash("sha256").update(bytes).digest("hex")}`;

/**
 * A sidecar's optional surface claims, the attendant's: `surfaces`, the native surfaces the picture shows, and
 * `prevented`, one statement or a list of { step, surface, observed, obstacle, cause } that a surface could not be
 * shown: the step attempted, the surface, what was observed when it was attempted, whether the obstacle is the
 * "system" or the "application", and the cause, a concrete account of what in that system or application kept the
 * surface from the capture. Each surface is a key of SURFACE_KEYS; an unknown one is kept apart as unexplained. A
 * statement lacking any member, or whose account names the attendant's own tool or its setup or leaves the reason
 * unresolved (UNACCOUNTED), is kept apart as not counted, with why, and makes nothing unseen: it is missing evidence.
 * { surfaces, surfaces_unexplained, prevented, prevented_incomplete, prevented_unexplained } or { why } when a member
 * is not of that form.
 */
export function surfaceClaims(value) {
  const claims = { surfaces: [], surfaces_unexplained: [], prevented: [], prevented_incomplete: [], prevented_unexplained: [] };
  if (value.surfaces !== undefined) {
    if (!Array.isArray(value.surfaces) || value.surfaces.length > 20 || !value.surfaces.every((item) => typeof item === "string" && item.length > 0 && item.length <= 80)) return { why: "its sidecar's surfaces is not a list of surface names" };
    for (const item of value.surfaces) (Object.hasOwn(SURFACE_KEYS, item) ? claims.surfaces : claims.surfaces_unexplained).push(item);
  }
  if (value.prevented !== undefined) {
    const records = Array.isArray(value.prevented) ? value.prevented : [value.prevented];
    const valid = (record) => record && typeof record === "object" && !Array.isArray(record) && typeof record.surface === "string" && record.surface.length > 0 && record.surface.length <= 80;
    if (records.length === 0 || records.length > 20 || !records.every(valid)) return { why: "its sidecar's prevented is not { step, surface, observed, obstacle, cause } or a list of them" };
    const text = (item, limit) => (typeof item === "string" && item.trim() !== "" && item.length <= limit ? item.trim() : null);
    for (const record of records) {
      const statement = { step: text(record.step, 200), surface: record.surface, observed: text(record.observed, 1000), obstacle: ["system", "application"].includes(record.obstacle) ? record.obstacle : null, cause: text(record.cause, 1000) };
      const lacks = ["step", "observed", "obstacle", "cause"].filter((member) => statement[member] === null);
      if (!Object.hasOwn(SURFACE_KEYS, record.surface)) claims.prevented_unexplained.push(statement);
      else if (lacks.length > 0) claims.prevented_incomplete.push({ ...statement, lacks, why: `it lacks ${lacks.join(", ")}: a statement names the step attempted, the surface, what was observed, whether the obstacle is the system's or the application's, and what in it kept the surface from the capture` });
      else if (UNACCOUNTED.test(statement.cause) || UNACCOUNTED.test(statement.observed)) claims.prevented_incomplete.push({ ...statement, lacks: [], why: "its account names the attendant's own tool or its setup, or leaves the reason unresolved: that is missing evidence, not a surface the system or the application kept from view" });
      else claims.prevented.push(statement);
    }
  }
  return claims;
}

/**
 * A sidecar's claims from its bytes, every one the attendant's statement: { use_case, step, acted, note, taken_at,
 * taken_at_ms, ...surfaceClaims, raw } or { why } when it is not one. use_case (one of AC007_USE_CASES by its name, or
 * the launch's journey: checked by readCaptures), step and acted (what was done since the previous capture; "observed
 * only" when nothing was) are required and not empty; note is a string; taken_at is an ISO 8601 time with its zone.
 */
export function parseSidecar(bytes) {
  let value;
  try { value = JSON.parse(Buffer.from(bytes).toString("utf8")); } catch { return { why: "its sidecar is not JSON" }; }
  if (!value || typeof value !== "object" || Array.isArray(value)) return { why: "its sidecar is not a JSON object" };
  if (typeof value.use_case !== "string" || value.use_case.trim() === "" || value.use_case.length > 200) return { why: "its sidecar names no use case (DEL-07-06 AC-007's name for it, or the launch's journey)" };
  if (typeof value.step !== "string" || value.step.trim() === "" || value.step.length > 200) return { why: "its sidecar names no step (a string of 1 to 200 characters)" };
  if (!Array.isArray(value.acted) || value.acted.length === 0 || value.acted.length > 200 || !value.acted.every((item) => typeof item === "string" && item.trim() !== "" && item.length <= 1000)) return { why: "its sidecar's acted does not say what was done since the previous capture (a list of at least one non-empty string; \"observed only\" when nothing was done)" };
  if (typeof value.note !== "string" || value.note.length > 4000) return { why: "its sidecar has no note (a string of at most 4000 characters)" };
  const takenAt = typeof value.taken_at === "string" && ISO_TIME.test(value.taken_at) ? Date.parse(value.taken_at) : Number.NaN;
  if (!Number.isFinite(takenAt)) return { why: "its sidecar does not say when the picture was taken (taken_at, an ISO 8601 time with its zone)" };
  const claims = surfaceClaims(value);
  if (claims.why) return claims;
  return { use_case: value.use_case.trim(), step: value.step.trim(), acted: value.acted.map((item) => item.trim()), note: value.note, taken_at: value.taken_at, taken_at_ms: takenAt, ...claims, raw: value };
}

/**
 * A regular file's bytes, read once through one descriptor opened without following a link, at most `limit` of them:
 * { bytes } or { why }.
 */
export function readOnce(file, limit, what = "it") {
  let fd;
  try {
    fd = fs.openSync(file, fs.constants.O_RDONLY | (fs.constants.O_NOFOLLOW ?? 0));
  } catch (error) {
    return { why: error.code === "ELOOP" ? `${what} is a link, not followed` : `${what} could not be read (${error.code ?? error.message})` };
  }
  try {
    const stat = fs.fstatSync(fd);
    if (!stat.isFile()) return { why: `${what} is not a regular file` };
    if (stat.size > limit) return { why: `${what} is larger than ${limit} bytes` };
    const buffer = Buffer.alloc(limit + 1);
    let length = 0;
    for (;;) {
      const read = fs.readSync(fd, buffer, length, buffer.length - length, null);
      if (read === 0) break;
      length += read;
      if (length > limit) return { why: `${what} is larger than ${limit} bytes` };
    }
    return { bytes: buffer.subarray(0, length) };
  } catch (error) {
    return { why: `${what} could not be read (${error.code ?? error.message})` };
  } finally {
    fs.closeSync(fd);
  }
}

/**
 * A sidecar read once: its claims (parseSidecar) and the SHA-256 of the very bytes they were parsed from
 * (`sidecar_sha256`, also given with a `why` when the bytes were read but say nothing usable), or { why }.
 */
export function readSidecar(file) {
  const read = readOnce(file, SIDECAR_LIMIT, "its sidecar");
  if (read.why) return { why: read.why };
  return { ...parseSidecar(read.bytes), sidecar_sha256: sha256Of(read.bytes) };
}
