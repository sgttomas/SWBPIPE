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
  type Receipt = "zero" | "unacknowledged" | "too late";
  const RECEIPTS: Receipt[] = ["zero", "unacknowledged", "too late"];
  const hostFor = (receipt: Receipt) => (receipt === "too late" ? host(1) : receipt === "zero" ? host(0) : host(0, "unacknowledged"));

  async function closeDuring(receipt: Receipt | number, settle: (save: ReturnType<typeof deferred<MbfStarted<MbfExportSaveOutcome>>>) => void) {
    const { owner } = typeof receipt === "number" ? host(receipt) : hostFor(receipt);
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

  const WRITE_FAILED = [{ code: "MBF-EXPORT-SAVE-RENAME-FAILED", path: "", message: "invented write failure" }];
  const ROLLBACK = [
    { code: "MBF-EXPORT-READBACK-MISMATCH", path: "", message: "invented read-back mismatch; the save was undone" },
    { code: "MBF-EXPORT-ROLLBACK-INCOMPLETE", path: "", message: "invented: kept as .swbpipe-mbf-0.bak; rename it back by hand" }
  ];
  // Only a too-late receipt for a save still unsettled says it is finishing; the outcome follows whatever the receipt.
  const finishingFor = (receipt: Receipt): ClosedMbfSaveReport[] => (receipt === "too late" ? [{ kind: "finishing" }] : []);

  it("past its commit: says only that it is finishing, then reports both files once it has saved", async () => {
    const { beforeSettle, reports } = await closeDuring(1, (save) => save.resolve(started(SAVED)));
    expect(beforeSettle).toEqual([{ kind: "finishing" }]);
    expect(reports).toEqual([{ kind: "finishing" }, { kind: "saved", outcome: SAVED }]);
  });

  for (const receipt of RECEIPTS) {
    it(`a receipt ${receipt}: a save that wrote both files is reported`, async () => {
      const { reports } = await closeDuring(receipt, (save) => save.resolve(started(SAVED)));
      expect(reports).toEqual([...finishingFor(receipt), { kind: "saved", outcome: SAVED }]);
    });

    it(`a receipt ${receipt}: a write failure is reported by its own diagnostics (a save completed before the close reached it is not in the receipt)`, async () => {
      const { reports } = await closeDuring(receipt, (save) => save.resolve(started({ outcome: "refused", diagnostics: WRITE_FAILED })));
      expect(reports).toEqual([...finishingFor(receipt), { kind: "failed", diagnostics: WRITE_FAILED }]);
    });

    it(`a receipt ${receipt}: an undone save is reported with its rollback's codes and backups`, async () => {
      const { reports } = await closeDuring(receipt, (save) => save.resolve(started({ outcome: "refused", diagnostics: ROLLBACK })));
      expect(reports).toEqual([...finishingFor(receipt), { kind: "failed", diagnostics: ROLLBACK }]);
    });

    it(`a receipt ${receipt}: a save that failed without an outcome is reported`, async () => {
      const { reports } = await closeDuring(receipt, (save) => save.reject(new Error("invented native failure")));
      expect(reports).toEqual([...finishingFor(receipt), { kind: "failed", diagnostics: [{ code: "MBF-EXPORT-FAILED", path: "", message: "invented native failure" }] }]);
    });

    it(`a receipt ${receipt}: an outcome that itself establishes nothing was written is not reported`, async () => {
      const silent: MbfExportSaveOutcome[] = [
        { outcome: "refused", diagnostics: [{ code: "MBF-EXPORT-CANCELLED", path: "", message: "cancelled before its commit" }] },
        { outcome: "cancelled" },
        { outcome: "refused", diagnostics: [{ code: "MBF-EXPORT-REVIEW-STALE", path: "", message: "stale" }] },
        { outcome: "refused", diagnostics: [{ code: "MBF-EXPORT-RETIREMENT-UNCONFIRMED", path: "", message: "not sent" }] },
        { outcome: "refused", diagnostics: [{ code: "MBF-EXPORT-EPOCH-INVALID", path: "", message: "refused at entry" }] }
      ];
      for (const outcome of silent) {
        const { reports } = await closeDuring(receipt, (save) => save.resolve(started(outcome)));
        expect(reports).toEqual(finishingFor(receipt));
      }
    });
  }

  it("a refusal that mixes a cancellation with any other code is reported", async () => {
    const mixed = [{ code: "MBF-EXPORT-CANCELLED", path: "", message: "cancelled" }, ...WRITE_FAILED];
    const { reports } = await closeDuring("zero", (save) => save.resolve(started({ outcome: "refused", diagnostics: mixed })));
    expect(reports).toEqual([{ kind: "failed", diagnostics: mixed }]);
  });

  it("an outcome that settles before the receipt arrives is reported, and no finishing follows it", async () => {
    const receipt = deferred<MbfCancelReceipt | null>();
    const owner = mbfExportRetirementOwner(() => receipt.promise);
    const reports: ClosedMbfSaveReport[] = [];
    const request = owner.openRequest((report) => reports.push(report));
    const save = deferred<MbfStarted<MbfExportSaveOutcome>>();
    request.trackSave(save.promise);
    request.retire();
    save.resolve(started({ outcome: "refused", diagnostics: WRITE_FAILED }));
    await flush();
    expect(reports).toEqual([{ kind: "failed", diagnostics: WRITE_FAILED }]);
    receipt.resolve(receiptOf(1));
    await flush();
    expect(reports).toEqual([{ kind: "failed", diagnostics: WRITE_FAILED }]);
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
});
