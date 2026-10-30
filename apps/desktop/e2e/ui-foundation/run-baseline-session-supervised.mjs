#!/usr/bin/env node
import { spawn, execFile } from "node:child_process";
import { access, mkdir, readFile, writeFile } from "node:fs/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { promisify } from "node:util";

const execFileAsync = promisify(execFile);
const evidenceRoot = process.env.UI_FOUNDATION_EVIDENCE_DIR;
const runText = process.env.UI_FOUNDATION_RUNS;
if (!evidenceRoot) throw new Error("UI_FOUNDATION_EVIDENCE_DIR is required");
if (!runText || !/^\d+$/.test(runText)) throw new Error("UI_FOUNDATION_RUNS must name exactly one numeric run");
const run = Number(runText);
const desktopDir = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "../..");
const runner = path.resolve(desktopDir, "../../node_modules/.bin/playwright");
const recordsDir = path.join(evidenceRoot, "_run_records");
const summaryPath = path.join(evidenceRoot, "raw", `run-${String(run).padStart(2, "0")}`, "1000", "run-summary.json");
const logPath = path.join(recordsDir, `run-${String(run).padStart(2, "0")}.log`);
const supervisorPath = path.join(recordsDir, `run-${String(run).padStart(2, "0")}-supervisor.json`);
const graceMs = Number(process.env.UI_FOUNDATION_TEARDOWN_GRACE_MS ?? 10_000);
if (!Number.isFinite(graceMs) || graceMs < 1_000 || graceMs > 60_000) throw new Error("invalid teardown grace");
await mkdir(recordsDir, { recursive: true });
try {
  await access(summaryPath);
  throw new Error(`refusing to reuse completed run summary: ${summaryPath}`);
} catch (error) {
  if (error?.code !== "ENOENT") throw error;
}

const readProcessTable = async () => {
  const stdout = (await execFileAsync("/bin/ps", ["-axo", "pid=,ppid=,pgid=,lstart=,command="])).stdout;
  return stdout.split("\n").flatMap((line) => {
    const match = line.match(/^\s*(\d+)\s+(\d+)\s+(\d+)\s+(\w{3}\s+\w{3}\s+\d+\s+\d+:\d+:\d+\s+\d{4})\s+(.*)$/);
    return match ? [{
      pid: Number(match[1]),
      ppid: Number(match[2]),
      pgid: Number(match[3]),
      started: match[4],
      command: match[5]
    }] : [];
  });
};

const collectDescendants = (rows, rootPid) => {
  const selected = new Map();
  const root = rows.find((row) => row.pid === rootPid);
  if (!root) return [];
  selected.set(root.pid, { ...root, depth: 0 });
  let frontier = [rootPid];
  let depth = 1;
  while (frontier.length) {
    const parentIds = new Set(frontier);
    const next = [];
    for (const row of rows) {
      if (parentIds.has(row.ppid) && !selected.has(row.pid)) {
        selected.set(row.pid, { ...row, depth });
        next.push(row.pid);
      }
    }
    frontier = next;
    depth += 1;
  }
  return [...selected.values()].sort((a, b) => a.depth - b.depth || a.pid - b.pid);
};

const exactOwnedMatches = async (owned) => {
  const current = new Map((await readProcessTable()).map((row) => [row.pid, row]));
  return owned.filter((record) => {
    const row = current.get(record.pid);
    return row?.started === record.started && row.command === record.command;
  });
};

const signalExactOwned = async (owned, signal) => {
  const matched = await exactOwnedMatches(owned);
  const signalled = [];
  for (const record of [...matched].sort((a, b) => b.depth - a.depth || b.pid - a.pid)) {
    try {
      process.kill(record.pid, signal);
      signalled.push(record.pid);
    } catch (error) {
      if (error?.code !== "ESRCH") throw error;
    }
  }
  return signalled;
};

const chunks = [];
const child = spawn(runner, ["test", "--config", "e2e/ui-foundation/playwright.baseline-points.config.ts"], {
  cwd: desktopDir,
  env: process.env,
  detached: true,
  stdio: ["ignore", "pipe", "pipe"]
});
const signalRunnerProcessGroup = (signal) => {
  try {
    process.kill(-child.pid, signal);
    return true;
  } catch (error) {
    if (error?.code === "ESRCH") return false;
    throw error;
  }
};
const childExit = new Promise((resolve) => child.once("close", (code, signal) => resolve({ code, signal, at: new Date().toISOString() })));
for (const stream of [child.stdout, child.stderr]) {
  stream.on("data", (chunk) => {
    chunks.push(Buffer.from(chunk));
    (stream === child.stdout ? process.stdout : process.stderr).write(chunk);
  });
}

let summary = null;
let earlyExit = null;
while (!summary && !earlyExit) {
  try {
    summary = JSON.parse(await readFile(summaryPath, "utf8"));
  } catch (error) {
    if (error?.code !== "ENOENT" && !(error instanceof SyntaxError)) throw error;
  }
  if (!summary) earlyExit = await Promise.race([childExit, new Promise((resolve) => setTimeout(() => resolve(null), 250))]);
}
if (earlyExit) {
  await writeFile(logPath, Buffer.concat(chunks));
  await writeFile(supervisorPath, `${JSON.stringify({
    schema: "openpipestress.ui-foundation.runner-supervisor/v1",
    status: "FAIL_RUNNER_EXITED_BEFORE_COMPLETE_RUN_SUMMARY",
    run,
    summaryPath,
    runner: { pid: child.pid, ...earlyExit }
  }, null, 2)}\n`);
  process.exitCode = earlyExit.code || 1;
} else {
  const expectedFilters = process.env.UI_FOUNDATION_FEASIBILITY_SAMPLE_COUNT ? 0 : 20;
  const bodyComplete = summary.recordedCount === 200 && summary.treeFilter?.recordedCount === expectedFilters && Boolean(summary.finishedAt);
  const ownedBeforeTeardown = collectDescendants(await readProcessTable(), child.pid);
  const ownershipEvidenceComplete = ownedBeforeTeardown.some((record) => record.pid === child.pid);
  const naturalExit = await Promise.race([childExit, new Promise((resolve) => setTimeout(() => resolve(null), graceMs))]);
  let teardown = naturalExit ? { status: "RUNNER_EXITED_WITHIN_GRACE", runnerExit: naturalExit } : null;
  if (!naturalExit) {
    signalRunnerProcessGroup("SIGINT");
    const afterSigint = await Promise.race([childExit, new Promise((resolve) => setTimeout(() => resolve(null), 5_000))]);
    if (!afterSigint) signalRunnerProcessGroup("SIGTERM");
    const afterSigterm = afterSigint ?? await Promise.race([childExit, new Promise((resolve) => setTimeout(() => resolve(null), 2_000))]);
    if (!afterSigterm) signalRunnerProcessGroup("SIGKILL");
    const finalExit = afterSigterm ?? await childExit;
    teardown = {
      status: "RUNNER_PROCESS_GROUP_STOPPED_AFTER_BODY_AND_GRACE",
      graceMs,
      firstSignal: "SIGINT",
      escalation: afterSigint ? null : afterSigterm ? "SIGTERM" : "SIGTERM_THEN_SIGKILL",
      runnerExit: finalExit
    };
  }
  const ownedRemainingAfterRunner = await exactOwnedMatches(ownedBeforeTeardown);
  const descendantCleanupSignals = [];
  let remainingOwned = ownedRemainingAfterRunner;
  for (const [signal, waitMs] of [["SIGINT", 2_000], ["SIGTERM", 2_000], ["SIGKILL", 1_000]]) {
    if (!remainingOwned.length) break;
    descendantCleanupSignals.push({ signal, pids: await signalExactOwned(remainingOwned, signal) });
    await new Promise((resolve) => setTimeout(resolve, waitMs));
    remainingOwned = await exactOwnedMatches(ownedBeforeTeardown);
  }
  const currentRows = await readProcessTable();
  const remainingProcessGroup = currentRows.filter((row) => row.pgid === child.pid);
  let port5176Listener = "";
  try {
    port5176Listener = (await execFileAsync("/usr/sbin/lsof", ["-nP", "-iTCP:5176"])).stdout.trim();
  } catch (error) {
    if (error?.code !== 1) port5176Listener = `port inventory failed: ${String(error)}`;
  }
  const cleanupComplete = remainingOwned.length === 0 && remainingProcessGroup.length === 0 && port5176Listener === "";
  const naturalRunnerPass = naturalExit?.code === 0 && naturalExit.signal === null;
  const naturalLifecyclePass = naturalRunnerPass && ownershipEvidenceComplete && ownedRemainingAfterRunner.length === 0;
  await writeFile(logPath, Buffer.concat(chunks));
  const status = bodyComplete && !ownershipEvidenceComplete
    ? "FAIL_BODY_COMPLETE_WITHOUT_PRE_TEARDOWN_OWNERSHIP_INVENTORY"
    : bodyComplete && cleanupComplete && naturalLifecyclePass
    ? "PASS_BODY_COMPLETE_NATURAL_RUNNER_EXIT"
    : bodyComplete && cleanupComplete && naturalExit && !naturalRunnerPass
    ? "FAIL_BODY_COMPLETE_NATURAL_RUNNER_NONZERO_OR_SIGNAL"
    : bodyComplete && cleanupComplete && naturalExit
    ? "FAIL_BODY_COMPLETE_NATURAL_RUNNER_LEFT_OWNED_DESCENDANTS_SUPERVISOR_CLEANED"
    : bodyComplete && cleanupComplete
    ? "BODY_COMPLETE_WITH_BOUNDED_FORCED_RUNNER_TEARDOWN"
    : bodyComplete
    ? "FAIL_BODY_COMPLETE_WITH_REMAINING_OWNED_PROCESS_OR_PORT"
    : "FAIL_INCOMPLETE_BODY_SUMMARY";
  await writeFile(supervisorPath, `${JSON.stringify({
    schema: "openpipestress.ui-foundation.runner-supervisor/v1",
    status,
    run,
    summaryPath,
    observedSummary: {
      recordedCount: summary.recordedCount,
      filterRecordedCount: summary.treeFilter?.recordedCount ?? null,
      finishedAt: summary.finishedAt ?? null
    },
    runner: { pid: child.pid },
    ownedDescendantInventoryBeforeTeardown: ownedBeforeTeardown,
    ownershipEvidenceComplete,
    teardown,
    cleanupPostcondition: {
      processGroupId: child.pid,
      ownedRemainingAfterRunner,
      descendantCleanupSignals,
      remainingOwned,
      remainingProcessGroup,
      port5176Listener,
      cleanupComplete
    },
    interpretation: "Measurement rows and endpoints are produced by the hash-bound Playwright test. This supervisor changes only post-body process-group ownership and records forced teardown distinctly."
  }, null, 2)}\n`);
  if (!bodyComplete || !ownershipEvidenceComplete || !cleanupComplete || (naturalExit && !naturalLifecyclePass)) {
    process.exitCode = naturalExit?.code && naturalExit.code !== 0 ? naturalExit.code : 1;
  }
}
