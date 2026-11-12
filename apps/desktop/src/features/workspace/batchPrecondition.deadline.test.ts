/** The precondition's deadline (DEL-07-11, the queue precondition): "A precondition that fails, or does not answer
 * within the session's stated finite bound, refuses, and an answer that comes after the bound is not accepted." The
 * bound is measured on a monotonic clock from when the precondition is asked; an answer is judged by the time it
 * arrives, not by whether the bound's timer callback has run yet. */
import { describe, expect, it } from "vitest";
import { checkBatchPrecondition, PRECONDITION_UNCHECKED_REASON } from "./workspaceSession";

/** Holds the thread for `ms` of real time without yielding, so no timer callback can run meanwhile. */
function busy(ms: number) {
  const until = performance.now() + ms;
  while (performance.now() < until) { /* hold the thread */ }
}

/** A clock the test moves by hand. */
function handClock(start = 1000) {
  const clock = { t: start, now: () => clock.t };
  return clock;
}

describe("the precondition's deadline", () => {
  it("refuses a success that arrives after the bound, though the bound's timer callback has not run yet (round 2's regression)", async () => {
    // A 10 ms bound and 30 ms of synchronous work inside the precondition: its null answer arrives 30 ms after it was
    // asked, and is delivered by microtasks before the overdue timer task can run. Before the repair, the helper took
    // whichever callback ran first and returned null here.
    const answer = await checkBatchPrecondition(async () => { busy(30); return null; }, 10);
    expect(answer).toBe(PRECONDITION_UNCHECKED_REASON);
  });

  it("refuses a success that arrives after the bound on the injected clock, with no time spent", async () => {
    const clock = handClock();
    const answer = await checkBatchPrecondition(async () => { clock.t += 10_000; return null; }, 10_000, clock.now);
    expect(answer).toBe(PRECONDITION_UNCHECKED_REASON);
  });

  it("accepts a success that arrives just inside the bound", async () => {
    const clock = handClock();
    const answer = await checkBatchPrecondition(async () => { clock.t += 9_999.999; return null; }, 10_000, clock.now);
    expect(answer).toBeNull();
  });

  it("refuses with the unchecked reason a refusal text or a throw that arrives after the bound", async () => {
    const late = handClock();
    expect(await checkBatchPrecondition(async () => { late.t += 10_001; return "The library has changed."; }, 10_000, late.now)).toBe(PRECONDITION_UNCHECKED_REASON);
    const thrown = handClock();
    expect(await checkBatchPrecondition(async () => { thrown.t += 10_001; throw new Error("read failed"); }, 10_000, thrown.now)).toBe(PRECONDITION_UNCHECKED_REASON);
    // Within the bound, a refusal text is the refusal.
    const inTime = handClock();
    expect(await checkBatchPrecondition(async () => { inTime.t += 1; return "The library has changed."; }, 10_000, inTime.now)).toBe("The library has changed.");
  });
});
