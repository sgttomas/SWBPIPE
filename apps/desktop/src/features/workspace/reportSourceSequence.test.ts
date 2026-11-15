// DEL-08-01 REQ-022: the report source sequence's own rules, against an invented transport.
// Edits allocate synchronously and never wait; acknowledgements register only upwards and
// only within the current incarnation; output preparation waits for registration, refuses a
// stale view and resynchronizes after a failure. All data are invented.
import { act, renderHook } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import { ReportSourceSequence, ReportSourceUnavailableError, useSequencedState, type ReportSourceMark } from "./reportSourceSequence";

type Pending = { mark: ReportSourceMark; resolve: (ack: ReportSourceMark) => void; reject: (error: unknown) => void };

/** A transport whose acknowledgements the test releases, in any order. */
function heldTransport() {
  let incarnation = 0;
  const pending: Pending[] = [];
  const activations: number[] = [];
  return {
    pending,
    activations,
    transport: {
      activate: async () => {
        incarnation += 1;
        activations.push(incarnation);
        return { incarnation, sequence: 0 };
      },
      advance: (mark: ReportSourceMark) => new Promise<ReportSourceMark>((resolve, reject) => pending.push({ mark, resolve, reject }))
    }
  };
}

const settle = () => new Promise((resolve) => setTimeout(resolve, 0));

describe("report source sequence", () => {
  it("allocates synchronously, never waits on an edit, and registers acknowledgements only upwards", async () => {
    const { pending, transport } = heldTransport();
    const source = new ReportSourceSequence(transport);
    const seen: number[] = [];
    source.subscribe((sequence) => seen.push(sequence));
    expect([source.advance(), source.advance(), source.advance()]).toEqual([1, 2, 3]);
    expect(seen).toEqual([1, 2, 3]);
    await settle();
    expect(pending.map((entry) => entry.mark)).toEqual([1, 2, 3].map((sequence) => ({ incarnation: 1, sequence })));
    // Out of order: 3 is acknowledged first, then a delayed 2 arrives.
    pending[2].resolve({ incarnation: 1, sequence: 3 });
    await settle();
    pending[1].resolve({ incarnation: 1, sequence: 2 });
    await settle();
    expect(source.registered).toEqual({ incarnation: 1, sequence: 3 });
    await expect(source.registeredFor(3)).resolves.toEqual({ incarnation: 1, sequence: 3 });
  });

  it("refuses output prepared from a stale view, before and after waiting", async () => {
    const { pending, transport } = heldTransport();
    const source = new ReportSourceSequence(transport);
    source.advance();
    await expect(source.registeredFor(0)).rejects.toMatchObject({ code: "REPORT-SOURCE-STALE-VIEW" });
    const waiting = source.registeredFor(1);
    await settle();
    source.advance(); // the workspace changes while the output waits for registration
    for (const entry of pending.splice(0)) entry.resolve({ incarnation: 1, sequence: entry.mark.sequence });
    await expect(waiting).rejects.toBeInstanceOf(ReportSourceUnavailableError);
  });

  it("ignores an acknowledgement from a retired incarnation and re-registers under a new one", async () => {
    const { pending, activations, transport } = heldTransport();
    const source = new ReportSourceSequence(transport);
    source.advance();
    await settle();
    // Another activation retired incarnation 1: its advance is refused by name.
    pending.shift()!.reject({ code: "REPORT-SOURCE-INCARNATION-RETIRED" });
    await settle();
    expect(source.registered).toBeNull();
    const output = source.registeredFor(1);
    await settle();
    expect(activations).toEqual([1, 2]);
    const resend = pending.shift()!;
    expect(resend.mark).toEqual({ incarnation: 2, sequence: 1 });
    // A delayed acknowledgement naming incarnation 1 changes nothing.
    resend.resolve({ incarnation: 1, sequence: 9 });
    await settle();
    expect(source.registered).toEqual({ incarnation: 2, sequence: 0 });
    const again = pending.shift()!;
    again.resolve({ incarnation: 2, sequence: 1 });
    await expect(output).resolves.toEqual({ incarnation: 2, sequence: 1 });
  });

  it("withholds output after failed resynchronization while edits continue, and resumes once registration succeeds", async () => {
    let failing = true;
    let registered = 0;
    const source = new ReportSourceSequence({
      activate: async () => ({ incarnation: 1, sequence: 0 }),
      advance: async (mark) => {
        if (failing) throw new Error("invented transport failure");
        registered = Math.max(registered, mark.sequence);
        return { incarnation: 1, sequence: registered };
      }
    });
    expect(() => { source.advance(); source.advance(); }).not.toThrow();
    await expect(source.registeredFor(2)).rejects.toMatchObject({ code: "REPORT-SOURCE-UNREGISTERED" });
    failing = false;
    await expect(source.registeredFor(2)).resolves.toEqual({ incarnation: 1, sequence: 2 });
  });

  it("outside the desktop application nothing is sent and output is not held", async () => {
    const source = new ReportSourceSequence(null);
    source.advance();
    await expect(source.registeredFor(1)).resolves.toEqual({ incarnation: 0, sequence: 1 });
  });
});

describe("sequenced state cells", () => {
  it("advance once per change of value, not for a repeated value, and resolve functional updates in order", () => {
    let advances = 0;
    const advance = () => ++advances;
    const { result } = renderHook(() => useSequencedState<string | null>(null, advance));
    act(() => result.current[1]("a"));
    act(() => result.current[1]("a"));
    expect([result.current[0], advances]).toEqual(["a", 1]);
    act(() => {
      result.current[1]((previous) => `${previous}b`);
      result.current[1]((previous) => `${previous}c`);
    });
    expect([result.current[0], advances]).toEqual(["abc", 3]);
    act(() => result.current[2]("derived"));
    expect([result.current[0], advances]).toEqual(["derived", 3]);
    act(() => result.current[1]((previous) => (previous === "derived" ? null : previous)));
    expect([result.current[0], advances]).toEqual([null, 4]);
  });
});
