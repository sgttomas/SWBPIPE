import { describe, expect, it, vi } from "vitest";
import { mbfImportRetirementOwner } from "./mbfImportSession";

describe("an import request's generation-owned retirement", () => {
  it.each(["completion-first", "cleanup-first"])("retires exactly once with %s", (order) => {
    const send = vi.fn();
    const owner = mbfImportRetirementOwner(send);
    const request = owner.openRequest(4);
    if (order === "completion-first") { request.retire(); owner.sessionEnded(4); }
    else { owner.sessionEnded(4); request.retire(); }
    expect(send).toHaveBeenCalledTimes(1);
    request.retire(); owner.sessionEnded(4);
    expect(send).toHaveBeenCalledTimes(1);
  });

  it("an older cleanup neither retires nor closes a newer request", () => {
    const send = vi.fn();
    const owner = mbfImportRetirementOwner(send);
    const older = owner.openRequest(4);
    const newer = owner.openRequest(5); // Retires 4 before 5's first native start.
    expect(send).toHaveBeenCalledTimes(1);
    older.retire();
    expect(owner.sessionEnded(4)).toBe(false);
    expect(send).toHaveBeenCalledTimes(1);
    expect(owner.sessionEnded(5)).toBe(true);
    newer.retire();
    expect(send).toHaveBeenCalledTimes(2);
  });

  it("closing then reopening in one generation gives each request its own retirement", () => {
    const send = vi.fn();
    const owner = mbfImportRetirementOwner(send);
    owner.openRequest(4).retire();
    const reopened = owner.openRequest(4);
    expect(send).toHaveBeenCalledTimes(1);
    owner.sessionEnded(4); reopened.retire();
    expect(send).toHaveBeenCalledTimes(2);
  });

  it("retires an unowned session once, including duplicate cleanup", () => {
    const send = vi.fn();
    const owner = mbfImportRetirementOwner(send);
    owner.sessionEnded(0); owner.sessionEnded(0);
    expect(send).toHaveBeenCalledTimes(1);
  });
});
