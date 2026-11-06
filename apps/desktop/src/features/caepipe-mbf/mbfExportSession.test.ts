import { describe, expect, it } from "vitest";
import type { MbfCancelReceipt, MbfStarted } from "./mbfExportCancel";
import { mbfExportRetirementOwner, type ClosedMbfSaveReport } from "./mbfExportSession";
import type { MbfExportSaveOutcome } from "./mbfExport";

// DEL-17-04 slice 4b.2b: the workspace session's ownership of the export's retirements (one logical retirement per
// close, App unmount or session replacement) and of a closed dialog's save (reported by its own outcome).

const receiptOf = (tooLate: number): MbfCancelReceipt => ({ cancelled: tooLate ? 0 : 1, too_late: tooLate, epoch: 1, retirement_id: { page: "0".repeat(32), sequence: 1 } });

function host(tooLate = 0, answer: "ack" | "unacknowledged" = "ack") {
  let count = 0;
  const retireSession = () => {
    count += 1;
    return Promise.resolve(answer === "ack" ? receiptOf(tooLate) : null);
  };
  return { owner: mbfExportRetirementOwner(retireSession), count: () => count };
}

const flush = () => new Promise((resolve) => setTimeout(resolve, 0));

function deferred<T>() {
  let resolve!: (value: T) => void;
  let reject!: (error: unknown) => void;
  const promise = new Promise<T>((res, rej) => { resolve = res; reject = rej; });
  return { promise, resolve, reject };
}

const started = (outcome: MbfExportSaveOutcome): MbfStarted<MbfExportSaveOutcome> => ({ outcome, generation: 1, current: () => false });
const SAVED: Extract<MbfExportSaveOutcome, { outcome: "saved" }> = {
  outcome: "saved",
  mbf: { file_name: "swbpipe-model.mbf", byte_count: 900, sha256: "a".repeat(64), replaced_existing: false, durability: "durable" },
  identity_file: { file_name: "swbpipe-model.swbpipe-ids.json", byte_count: 2000, sha256: "b".repeat(64), replaced_existing: false, durability: "durable" },
  durability: "durable",
  path_containment: "best_effort_non_adversarial",
  limitation: "invented limitation",
  warnings: []
};

describe("the session's retirement ownership", () => {
  it("retires once when the user closes the dialog, and once more only when the session later ends", () => {
    const { owner, count } = host();
    const request = owner.openRequest(() => {});
    request.release();
    request.retire();
    request.retire();
    expect(count()).toBe(1);
    owner.sessionEnded();
    expect(count()).toBe(2);
  });

  it("retires once when the App unmounts with the dialog open: the dialog's cleanup, then the session's", () => {
    const { owner, count } = host();
    const request = owner.openRequest(() => {});
    request.retire();
    owner.sessionEnded();
    expect(count()).toBe(1);
  });

  it("retires once when the session is replaced with the dialog open: the session's end, then the dialog's cleanup", () => {
    const { owner, count } = host();
    const request = owner.openRequest(() => {});
    owner.sessionEnded();
    request.retire();
    expect(count()).toBe(1);
  });

  it("retires once when the session ends with no dialog, or with a request that never mounted", () => {
    const none = host();
    none.owner.sessionEnded();
    expect(none.count()).toBe(1);
    const unmounted = host();
    unmounted.owner.openRequest(() => {});
    unmounted.owner.sessionEnded();
    expect(unmounted.count()).toBe(1);
  });
});

describe("a closed dialog's save", () => {
  async function closeDuring(tooLate: number, settle: (save: ReturnType<typeof deferred<MbfStarted<MbfExportSaveOutcome>>>) => void) {
    const { owner } = host(tooLate);
    const reports: ClosedMbfSaveReport[] = [];
    const request = owner.openRequest((report) => reports.push(report));
    const save = deferred<MbfStarted<MbfExportSaveOutcome>>();
    request.trackSave(save.promise);
    request.release();
    request.retire();
    await flush();
    const beforeSettle = [...reports];
    settle(save);
    await flush();
    return { beforeSettle, reports };
  }

  it("past its commit: says only that it is finishing, then reports both files once it has saved", async () => {
    const { beforeSettle, reports } = await closeDuring(1, (save) => save.resolve(started(SAVED)));
    expect(beforeSettle).toEqual([{ kind: "finishing" }]);
    expect(reports).toEqual([{ kind: "finishing" }, { kind: "saved", outcome: SAVED }]);
  });

  it("past its commit: reports a write failure, or an undone save, by its own diagnostics", async () => {
    const writeFailed = [{ code: "MBF-EXPORT-SAVE-RENAME-FAILED", path: "", message: "invented write failure" }];
    const failed = await closeDuring(1, (save) => save.resolve(started({ outcome: "refused", diagnostics: writeFailed })));
    expect(failed.reports).toEqual([{ kind: "finishing" }, { kind: "failed", diagnostics: writeFailed }]);
    const thrown = await closeDuring(1, (save) => save.reject(new Error("invented native failure")));
    expect(thrown.reports[1]).toEqual({ kind: "failed", diagnostics: [{ code: "MBF-EXPORT-FAILED", path: "", message: "invented native failure" }] });
  });

  it("cancelled before its commit: reports nothing, as the close asked for it", async () => {
    const cancelled = [{ code: "MBF-EXPORT-CANCELLED", path: "", message: "cancelled" }];
    const { reports } = await closeDuring(0, (save) => save.resolve(started({ outcome: "refused", diagnostics: cancelled })));
    expect(reports).toEqual([]);
  });

  it("a save that settled before the close is the dialog's own: the session reports nothing", async () => {
    const { owner } = host(1);
    const reports: ClosedMbfSaveReport[] = [];
    const request = owner.openRequest((report) => reports.push(report));
    request.trackSave(Promise.resolve(started(SAVED)));
    await flush();
    request.release();
    request.retire();
    await flush();
    expect(reports).toEqual([]);
  });

  it("an unacknowledged retirement says nothing is finishing, but a save that wrote both files is still reported", async () => {
    const { owner } = host(0, "unacknowledged");
    const reports: ClosedMbfSaveReport[] = [];
    const request = owner.openRequest((report) => reports.push(report));
    const save = deferred<MbfStarted<MbfExportSaveOutcome>>();
    request.trackSave(save.promise);
    request.retire();
    await flush();
    expect(reports).toEqual([]);
    save.resolve(started(SAVED));
    await flush();
    expect(reports).toEqual([{ kind: "saved", outcome: SAVED }]);
  });
});
