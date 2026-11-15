// DEL-08-01 REQ-022: the report source sequence's own rules and, for the round-two F2 repair,
// its activation across workspace lifetimes. The real sequence and coordinator run against a
// modeled transport that applies the native rules `rendered_report_routes.rs` implements and
// tests itself (observation; conditional activation compared on the expected incarnation, a
// mismatch changing nothing; the highest number of the current incarnation; another
// incarnation refused). Each modeled call can be held, processed without its reply ever being
// delivered, or rejected before or after its native effect. No launched application or native
// process is involved. All data are invented.
import { StrictMode, createElement, type ReactNode } from "react";
import { act, renderHook } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import {
  ReportSourceCoordinator,
  ReportSourceSequence,
  ReportSourceUnavailableError,
  useReportSourceSequence,
  useSequencedState,
  type ReportSourceMark,
  type ReportSourceTransport
} from "./reportSourceSequence";

type Kind = "observe" | "activate" | "advance";
type Held = { kind: Kind; argument: unknown; processed: boolean; process(): void; deliver(): void; reject(error: unknown): void };

/** The native rules, with controllable delivery. */
function nativeModel() {
  const native = { incarnation: 0, registered: 0 };
  const calls: string[] = [];
  const held: Held[] = [];
  const holding = new Set<Kind>();
  function op<T>(kind: Kind, argument: unknown, effect: () => T): Promise<T> {
    calls.push(`${kind} ${JSON.stringify(argument)}`);
    if (!holding.has(kind)) return Promise.resolve().then(effect);
    return new Promise<T>((resolve, reject) => {
      let outcome: { ok: true; value: T } | { ok: false; error: unknown } | null = null;
      const entry: Held = {
        kind,
        argument,
        processed: false,
        process() {
          if (entry.processed) return;
          entry.processed = true;
          try { outcome = { ok: true, value: effect() }; } catch (error) { outcome = { ok: false, error }; }
        },
        deliver() {
          entry.process();
          if (outcome!.ok) resolve(outcome!.value); else reject(outcome!.error);
        },
        reject
      };
      held.push(entry);
    });
  }
  const transport: ReportSourceTransport = {
    observe: () => op("observe", null, () => ({ incarnation: native.incarnation, sequence: native.registered })),
    activate: (expected) => op("activate", expected, () => {
      if (expected !== native.incarnation) throw { code: "REPORT-SOURCE-ACTIVATION-CONFLICT" };
      native.incarnation += 1;
      native.registered = 0;
      return { incarnation: native.incarnation, sequence: 0 };
    }),
    advance: (mark) => op("advance", mark, () => {
      if (mark.incarnation !== native.incarnation) throw { code: "REPORT-SOURCE-INCARNATION-RETIRED" };
      native.registered = Math.max(native.registered, mark.sequence);
      return { incarnation: native.incarnation, sequence: native.registered };
    })
  };
  const take = (kind: Kind) => {
    const at = held.findIndex((entry) => entry.kind === kind);
    expect(at, `a held ${kind}`).toBeGreaterThanOrEqual(0);
    return held.splice(at, 1)[0];
  };
  const activations = () => calls.filter((call) => call.startsWith("activate")).length;
  return { native, calls, held, holding, transport, take, activations };
}

const settle = async () => {
  for (let i = 0; i < 5; i += 1) await new Promise((resolve) => setTimeout(resolve, 0));
};

describe("report source sequence", () => {
  it("allocates synchronously, never waits on an edit, and registers acknowledgements only upwards", async () => {
    const model = nativeModel();
    const source = new ReportSourceSequence(model.transport, new ReportSourceCoordinator());
    source.start();
    await settle();
    model.holding.add("advance");
    const seen: number[] = [];
    source.subscribe((sequence) => seen.push(sequence));
    expect([source.advance(), source.advance(), source.advance()]).toEqual([1, 2, 3]);
    expect(seen).toEqual([1, 2, 3]);
    await settle();
    expect(model.held.map((entry) => entry.argument)).toEqual([1, 2, 3].map((sequence) => ({ incarnation: 1, sequence })));
    // Out of order: 3 is acknowledged first, then a delayed 2 arrives.
    model.take("advance").process();
    const second = model.take("advance");
    model.take("advance").deliver();
    await settle();
    second.deliver();
    await settle();
    expect(source.registered).toEqual({ incarnation: 1, sequence: 3 });
    await expect(source.registeredFor(3)).resolves.toEqual({ incarnation: 1, sequence: 3 });
  });

  it("refuses output prepared from a stale view, before and after waiting", async () => {
    const model = nativeModel();
    const source = new ReportSourceSequence(model.transport, new ReportSourceCoordinator());
    source.start();
    await settle();
    model.holding.add("advance");
    source.advance();
    await expect(source.registeredFor(0)).rejects.toMatchObject({ code: "REPORT-SOURCE-STALE-VIEW" });
    const waiting = source.registeredFor(1);
    source.advance(); // the workspace changes while the output waits for registration
    await settle();
    for (const entry of model.held.splice(0)) entry.deliver();
    await expect(waiting).rejects.toBeInstanceOf(ReportSourceUnavailableError);
  });

  it("withholds output after failed resynchronization while edits continue, and the live owner recovers", async () => {
    const model = nativeModel();
    const source = new ReportSourceSequence(model.transport, new ReportSourceCoordinator());
    source.start();
    await settle();
    const advance = model.transport.advance;
    let failing = true;
    model.transport.advance = (mark) => (failing ? Promise.reject(new Error("invented transport failure")) : advance(mark));
    expect(() => { source.advance(); source.advance(); }).not.toThrow();
    await expect(source.registeredFor(2)).rejects.toMatchObject({ code: "REPORT-SOURCE-UNREGISTERED" });
    failing = false;
    await expect(source.registeredFor(2)).resolves.toEqual({ incarnation: 1, sequence: 2 });
  });

  it("outside the desktop application nothing is sent and output is not held", async () => {
    const source = new ReportSourceSequence(null, new ReportSourceCoordinator());
    source.start();
    source.advance();
    await expect(source.registeredFor(1)).resolves.toEqual({ incarnation: 0, sequence: 1 });
  });
});

describe("activation across workspace lifetimes (F2)", () => {
  it("a retired owner's delayed retirement reply neither reactivates nor disturbs its replacement (the review's witness)", async () => {
    const model = nativeModel();
    const coordinator = new ReportSourceCoordinator();
    const a = new ReportSourceSequence(model.transport, coordinator);
    a.start();
    await settle();
    model.holding.add("advance");
    a.advance();
    const aOutput = a.registeredFor(1).catch((error: unknown) => error);
    await settle();
    const aAdvances = model.held.splice(0); // A's advances, held at incarnation 1
    expect(aAdvances.map((entry) => entry.kind)).toEqual(aAdvances.map(() => "advance"));
    a.stop();
    model.holding.delete("advance");
    const b = new ReportSourceSequence(model.transport, coordinator);
    b.start();
    await settle();
    expect(b.registered).toEqual({ incarnation: 2, sequence: 0 });
    for (const entry of aAdvances) entry.deliver(); // REPORT-SOURCE-INCARNATION-RETIRED, to a retired owner
    await settle();
    expect(await aOutput).toMatchObject({ code: "REPORT-SOURCE-RETIRED-OWNER" });
    expect({ native: model.native.incarnation, activations: model.activations() }).toEqual({ native: 2, activations: 2 });
    await expect(b.registeredFor(0)).resolves.toEqual({ incarnation: 2, sequence: 0 });
  });

  it("the replacement wins: the old activation, processed late with its reply never delivered, changes nothing", async () => {
    const model = nativeModel();
    const coordinator = new ReportSourceCoordinator();
    const a = new ReportSourceSequence(model.transport, coordinator);
    model.holding.add("activate");
    a.start();
    await settle();
    const aActivation = model.take("activate"); // expecting 0, still queued
    a.stop();
    model.holding.delete("activate");
    const b = new ReportSourceSequence(model.transport, coordinator);
    b.start();
    await settle();
    expect(b.registered).toEqual({ incarnation: 1, sequence: 0 });
    aActivation.process(); // processed after the replacement: conflict, no change; reply never delivered
    expect(model.native).toEqual({ incarnation: 1, registered: 0 });
    b.advance();
    await expect(b.registeredFor(1)).resolves.toEqual({ incarnation: 1, sequence: 1 });
  });

  it("the old activation wins first with its reply lost: the replacement conflicts, observes again and fences it", async () => {
    const model = nativeModel();
    const coordinator = new ReportSourceCoordinator();
    const a = new ReportSourceSequence(model.transport, coordinator);
    model.holding.add("activate");
    a.start();
    await settle();
    const aActivation = model.take("activate");
    a.stop();
    const b = new ReportSourceSequence(model.transport, coordinator);
    b.start();
    await settle();
    const bFirst = model.take("activate"); // B observed 0 too
    aActivation.process(); // A's activation reaches the native side first: incarnation 1; its reply is lost
    model.holding.delete("activate");
    bFirst.deliver(); // conflict: B observes 1 and activates from it
    await settle();
    expect(b.registered).toEqual({ incarnation: 2, sequence: 0 });
    expect(model.native.incarnation).toBe(2);
    await expect(b.registeredFor(0)).resolves.toEqual({ incarnation: 2, sequence: 0 });
  });

  it("a rejected activation is not evidence of native completion: before and after processing, output waits for a confirmed activation", async () => {
    for (const processedFirst of [false, true]) {
      const model = nativeModel();
      const source = new ReportSourceSequence(model.transport, new ReportSourceCoordinator());
      model.holding.add("activate");
      source.start();
      await settle();
      const activation = model.take("activate");
      if (processedFirst) activation.process();
      activation.reject(new Error("invented lost reply"));
      await settle();
      expect(source.registered).toBeNull(); // nothing observed was promoted to authority
      source.advance(); // editing continues
      const output = source.registeredFor(1);
      await settle();
      while (model.held.length > 0) model.held.shift()!.deliver();
      await expect(output).resolves.toEqual({ incarnation: processedFirst ? 2 : 1, sequence: 1 });
    }
  });

  it("an owner retired during its observation, its activation or its registration sends nothing more and adopts nothing", async () => {
    for (const phase of ["observe", "activate", "advance"] as const) {
      const model = nativeModel();
      const coordinator = new ReportSourceCoordinator();
      const a = new ReportSourceSequence(model.transport, coordinator);
      if (phase === "advance") {
        a.start();
        await settle();
      }
      model.holding.add(phase);
      if (phase !== "advance") a.start();
      if (phase === "advance") a.advance();
      await settle();
      const pending = model.take(phase);
      const callsBefore = model.calls.length;
      a.stop();
      pending.deliver();
      await settle();
      expect(model.calls.length, `${phase}: no further call`).toBe(callsBefore);
      expect(a.registered).toBeNull();
      await expect(a.registeredFor(a.sequence)).rejects.toMatchObject({ code: "REPORT-SOURCE-RETIRED-OWNER" });
    }
  });

  it("a replacement that is itself retired is fenced the same way; the live owner holds authority", async () => {
    const model = nativeModel();
    const coordinator = new ReportSourceCoordinator();
    const a = new ReportSourceSequence(model.transport, coordinator);
    a.start();
    await settle();
    model.holding.add("activate");
    const b = new ReportSourceSequence(model.transport, coordinator);
    b.start(); // retires A
    await settle();
    const bActivation = model.take("activate");
    model.holding.delete("activate");
    const c = new ReportSourceSequence(model.transport, coordinator);
    c.start(); // retires B
    await settle();
    bActivation.deliver(); // B expected 1, C already moved it to 2: conflict, and B is retired anyway
    await settle();
    expect([a.registered, b.registered, c.registered]).toEqual([null, null, { incarnation: 2, sequence: 0 }]);
    expect(model.native.incarnation).toBe(2);
    await expect(c.registeredFor(0)).resolves.toEqual({ incarnation: 2, sequence: 0 });
  });

  it("an actual hook unmount and remount, and a StrictMode effect replay, leave one live activation", async () => {
    const model = nativeModel();
    const first = renderHook(() => useReportSourceSequence(model.transport));
    await act(settle);
    expect(first.result.current.owner.registered).toEqual({ incarnation: 1, sequence: 0 });
    first.unmount();
    expect(first.result.current.owner.registered).toBeNull();
    const second = renderHook(() => useReportSourceSequence(model.transport));
    await act(settle);
    expect(second.result.current.owner.registered).toEqual({ incarnation: 2, sequence: 0 });
    second.unmount();
    const strict = renderHook(() => useReportSourceSequence(model.transport), { wrapper: ({ children }: { children: ReactNode }) => createElement(StrictMode, null, children) });
    await act(settle);
    const owner = strict.result.current.owner;
    // The replayed effect retired its first lifetime; the replay's lifetime is live and usable.
    expect(owner.registered?.incarnation).toBe(model.native.incarnation);
    act(() => { owner.advance(); });
    await act(async () => { await expect(owner.registeredFor(owner.sequence)).resolves.toEqual({ incarnation: model.native.incarnation, sequence: 1 }); });
    expect(strict.result.current.sequence).toBe(1);
    strict.unmount();
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

// Unused import guard for the mark type (kept for readers of the model above).
export type _Mark = ReportSourceMark;
