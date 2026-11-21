// DEL-08-01 REQ-022: the report source sequence. Every change of what a report is built
// from (the model, the result, the historical and analysis runs, the input manifest, the
// project summary, the rule-check aggregate and run, the within-run comparison choice, the
// report composition and the panel's export context, and their clears) advances one
// number, synchronously, at the setter that publishes it (`useSequencedState`). The number
// is held as React state beside those values, so a component rendered from one commit sees
// a basis and its number together; an effect never allocates a number.
//
// Each number is sent to the desktop application's native process
// (`advance_report_source`), which registers the highest number of the workspace's
// incarnation. A workspace lifetime observes the native incarnation
// (`observe_report_source`) and activates its own only conditionally
// (`activate_report_source` with the incarnation it expects), under the page's one
// coordinator, which retires a replaced lifetime terminally (`ReportSourceCoordinator`). Editing
// never waits for that: an advance is sent and its failure only leaves report output
// unregistered. Output preparation (a render, an HTML save, a print) waits for the
// number it was prepared from to be registered, re-checks that it is still the latest,
// and then names it in its request; the native side refuses a request whose number is
// older (retired) or not yet registered. A change made here is ordered with native
// publication only once registered: until then an earlier request may still be admitted
// and published, and the report output status says so.
//
// A render request is identified by the source it names and an ordinal this owner allocates,
// monotonically, when its preparation begins (`nextRenderRequest`, before any await). The
// allocation is output intent only: it waits for nothing and changes no model, publisher or
// solve. The native side refuses an entry whose ordinal is not above the highest it has
// entered or seen retired, and a panel's retirement names the exact request it discards.
import { useEffect, useRef, useState, type Dispatch, type SetStateAction } from "react";
import { invoke } from "@tauri-apps/api/core";

export type ReportSourceMark = { incarnation: number; sequence: number };

/** A render request's identity: the registered source it names and its owner's ordinal. */
export type ReportRenderRequest = { source: ReportSourceMark; request: number };

/** The native side of the sequence; `null` outside the desktop application. */
export type ReportSourceTransport = {
  /** The current incarnation and number; authorises nothing. */
  observe(): Promise<ReportSourceMark>;
  /** Conditional activation: succeeds only while `expectedIncarnation` is current. */
  activate(expectedIncarnation: number): Promise<ReportSourceMark>;
  advance(mark: ReportSourceMark): Promise<ReportSourceMark>;
};

/** Output on a source the native side has not confirmed is withheld, by name. */
export class ReportSourceUnavailableError extends Error {
  constructor(readonly code: "REPORT-SOURCE-STALE-VIEW" | "REPORT-SOURCE-UNREGISTERED" | "REPORT-SOURCE-RETIRED-OWNER", message: string) {
    super(`${code}: ${message}`);
    this.name = "ReportSourceUnavailableError";
  }
}

const INCARNATION_RETIRED = "REPORT-SOURCE-INCARNATION-RETIRED";
const ACTIVATION_CONFLICT = "REPORT-SOURCE-ACTIVATION-CONFLICT";
const ACTIVATION_ATTEMPTS = 3;

function errorCode(error: unknown): string | null {
  return typeof error === "object" && error !== null && typeof (error as { code?: unknown }).code === "string"
    ? (error as { code: string }).code
    : null;
}

/** One activation lifetime of a workspace's source owner: from its effect's mount to its
 * cleanup (a StrictMode replay or a remount starts another). Terminal once retired: it sends
 * nothing more, adopts no reply and returns no mark. */
class SourceLifetime {
  retired = false;
  /** Set only from a confirmed conditional activation, never from an observation. */
  incarnation: number | null = null;
  confirmed = 0;
  activation: Promise<number> | null = null;
}

const retiredOwner = () => new ReportSourceUnavailableError("REPORT-SOURCE-RETIRED-OWNER", "this workspace's report output was replaced; nothing more is sent for it.");

/** The page's one owner of report-source activation across workspace lifetimes: at most one
 * lifetime is live. Beginning a lifetime retires the one before it terminally, before the new
 * one can prepare any activation; so every activation the retired one issued expects an
 * incarnation no higher than the native one the new lifetime then observes, and the new
 * lifetime's confirmed conditional activation fences them, whether they were processed
 * already or are still queued (`activation-fence` argument in the PR). */
export class ReportSourceCoordinator {
  private live: SourceLifetime | null = null;

  begin(): SourceLifetime {
    if (this.live) this.live.retired = true;
    this.live = new SourceLifetime();
    return this.live;
  }

  end(lifetime: SourceLifetime): void {
    lifetime.retired = true;
    if (this.live === lifetime) this.live = null;
  }

  isLive(lifetime: SourceLifetime | null): lifetime is SourceLifetime {
    return lifetime !== null && !lifetime.retired && this.live === lifetime;
  }
}

/** The page's coordinator. */
export const reportSourceCoordinator = new ReportSourceCoordinator();

/** The workspace's report source sequence (see the module comment). */
export class ReportSourceSequence {
  private allocated = 0;
  private renderRequests = 0;
  private lifetime: SourceLifetime | null = null;
  private readonly listeners = new Set<(sequence: number) => void>();

  constructor(
    private readonly transport: ReportSourceTransport | null,
    private readonly coordinator: ReportSourceCoordinator = reportSourceCoordinator
  ) {}

  /** The latest number allocated in this workspace session. */
  get sequence(): number {
    return this.allocated;
  }

  /** The highest number the native side has confirmed for this owner's live lifetime. */
  get registered(): ReportSourceMark | null {
    const lifetime = this.lifetime;
    return this.coordinator.isLive(lifetime) && lifetime.incarnation !== null ? { incarnation: lifetime.incarnation, sequence: lifetime.confirmed } : null;
  }

  subscribe(listener: (sequence: number) => void): () => void {
    this.listeners.add(listener);
    return () => this.listeners.delete(listener);
  }

  /** A report-bound value changed: allocate the next number now, then send it for the live
   * lifetime. Never waits and never throws; a failed send leaves output unregistered, not
   * editing. Subscribers are contained: one subscriber's failure stops neither the other
   * subscribers, nor the native send, nor the write that advanced; it is reported, not
   * swallowed (the first failure of an advance, once, on the console). A view that did not
   * hear the advance still holds the older number, so output prepared from it is refused as
   * a stale view. */
  readonly advance = (): number => {
    this.allocated += 1;
    const sequence = this.allocated;
    let failure: { error: unknown } | null = null;
    for (const listener of this.listeners) {
      try {
        listener(sequence);
      } catch (error) {
        failure ??= { error };
      }
    }
    if (failure) console.error("A report-source subscriber failed; the others were still told and the advance was sent.", failure.error);
    const lifetime = this.lifetime;
    if (lifetime) void this.send(lifetime, sequence).catch(() => undefined);
    return sequence;
  };

  /** A render's preparation begins: the next render request ordinal of this owner,
   * allocated synchronously and never reused. */
  readonly nextRenderRequest = (): number => {
    this.renderRequests += 1;
    return this.renderRequests;
  };

  /** Begins this owner's lifetime (its effect's mount) and sends the latest number. It
   * allocates nothing. */
  start(): void {
    const lifetime = this.coordinator.begin();
    this.lifetime = lifetime;
    void this.send(lifetime).catch(() => undefined);
  }

  /** Ends this owner's lifetime (its effect's cleanup): terminal for every continuation of it. */
  stop(): void {
    if (this.lifetime) this.coordinator.end(this.lifetime);
    this.lifetime = null;
  }

  private assertLive(lifetime: SourceLifetime): void {
    if (!this.coordinator.isLive(lifetime)) throw retiredOwner();
  }

  /** Observation, then conditional activation from the observed incarnation, repeated only by
   * the live lifetime on a conflict. Nothing observed becomes authority: the incarnation is
   * adopted only from a confirmed activation, and only while the lifetime is live. */
  private ensureActive(lifetime: SourceLifetime): Promise<number> {
    const transport = this.transport as ReportSourceTransport;
    this.assertLive(lifetime);
    if (lifetime.incarnation !== null) return Promise.resolve(lifetime.incarnation);
    if (!lifetime.activation) {
      lifetime.activation = (async () => {
        try {
          for (let attempt = 0; attempt < ACTIVATION_ATTEMPTS; attempt += 1) {
            this.assertLive(lifetime);
            const observed = await transport.observe();
            this.assertLive(lifetime);
            let mark: ReportSourceMark;
            try {
              mark = await transport.activate(observed.incarnation);
            } catch (error) {
              if (errorCode(error) === ACTIVATION_CONFLICT) continue;
              throw error;
            }
            this.assertLive(lifetime);
            lifetime.incarnation = mark.incarnation;
            lifetime.confirmed = mark.sequence;
            return mark.incarnation;
          }
          throw new ReportSourceUnavailableError("REPORT-SOURCE-UNREGISTERED", "another workspace kept activating first; report output is withheld, editing is unaffected.");
        } finally {
          lifetime.activation = null;
        }
      })();
    }
    return lifetime.activation;
  }

  /** Sends a number for `lifetime`; an acknowledgement registers only while the lifetime is
   * live, within its incarnation and only upwards. */
  private async send(lifetime: SourceLifetime, sequence?: number): Promise<void> {
    if (!this.transport) return;
    this.assertLive(lifetime);
    const incarnation = await this.ensureActive(lifetime);
    this.assertLive(lifetime);
    // An advance sends its own number; a resynchronization sends the latest.
    if (sequence === undefined || lifetime.incarnation !== incarnation || sequence < lifetime.confirmed) sequence = this.allocated;
    try {
      const acknowledged = await this.transport.advance({ incarnation, sequence });
      this.assertLive(lifetime);
      if (acknowledged.incarnation === lifetime.incarnation && acknowledged.sequence > lifetime.confirmed) lifetime.confirmed = acknowledged.sequence;
    } catch (error) {
      if (errorCode(error) === INCARNATION_RETIRED && this.coordinator.isLive(lifetime) && lifetime.incarnation === incarnation) {
        // Another activation retired this live lifetime's incarnation: it may activate
        // again, conditionally, on its next send.
        lifetime.incarnation = null;
        lifetime.confirmed = 0;
      }
      throw error;
    }
  }

  private confirms(lifetime: SourceLifetime, sequence: number): boolean {
    return lifetime.incarnation !== null && lifetime.confirmed >= sequence;
  }

  /** Output preparation: `viewSequence` is the number the preparing view was rendered
   * with. Resolves with the mark to name in the request once the native side has
   * registered it for this owner's live lifetime and it is still the latest; refuses a
   * stale view, a retired owner, and a registration that cannot be confirmed (after
   * bounded resynchronization). */
  async registeredFor(viewSequence: number): Promise<ReportSourceMark> {
    const stale = () => new ReportSourceUnavailableError("REPORT-SOURCE-STALE-VIEW", "the workspace changed after this report was prepared; prepare it again.");
    if (viewSequence !== this.allocated) throw stale();
    if (!this.transport) return { incarnation: 0, sequence: viewSequence };
    const lifetime = this.lifetime;
    if (!this.coordinator.isLive(lifetime)) throw retiredOwner();
    let lastError: unknown = null;
    for (let attempt = 0; attempt < 3 && !this.confirms(lifetime, viewSequence); attempt += 1) {
      try {
        await this.send(lifetime);
      } catch (error) {
        lastError = error;
      }
      if (!this.coordinator.isLive(lifetime)) throw retiredOwner();
    }
    if (viewSequence !== this.allocated) throw stale();
    if (!this.coordinator.isLive(lifetime)) throw retiredOwner();
    if (!this.confirms(lifetime, viewSequence) || lifetime.incarnation === null) {
      const detail = lastError instanceof Error ? lastError.message : lastError === null ? "" : String(errorCode(lastError) ?? lastError);
      throw new ReportSourceUnavailableError(
        "REPORT-SOURCE-UNREGISTERED",
        `the desktop has not confirmed the current workspace, so report output is withheld; editing is unaffected.${detail ? ` (${detail})` : ""}`
      );
    }
    return { incarnation: lifetime.incarnation, sequence: viewSequence };
  }
}

export function nativeReportSourceTransport(): ReportSourceTransport | null {
  if (typeof window === "undefined" || !("__TAURI_INTERNALS__" in window)) return null;
  return {
    observe: () => invoke<ReportSourceMark>("observe_report_source"),
    activate: (expectedIncarnation) => invoke<ReportSourceMark>("activate_report_source", { expectedIncarnation }),
    advance: (mark) => invoke<ReportSourceMark>("advance_report_source", { incarnation: mark.incarnation, sequence: mark.sequence })
  };
}

/** The session's sequence: a stable owner and its latest number as React state. Each effect
 * mount begins a lifetime and its cleanup ends it (so a StrictMode replay or a remount
 * activates afresh and the replaced lifetime is retired); the number's listener is released
 * on cleanup and restored on the next mount. */
export function useReportSourceSequence(transport?: ReportSourceTransport | null) {
  const [owner] = useState(() => new ReportSourceSequence(transport === undefined ? nativeReportSourceTransport() : transport));
  const [sequence, setSequence] = useState(owner.sequence);
  const unsubscribe = useRef<(() => void) | null>(null);
  if (unsubscribe.current === null) unsubscribe.current = owner.subscribe(setSequence);
  useEffect(() => {
    if (unsubscribe.current === null) unsubscribe.current = owner.subscribe(setSequence);
    setSequence(owner.sequence);
    owner.start();
    return () => {
      owner.stop();
      unsubscribe.current?.();
      unsubscribe.current = null;
    };
  }, [owner]);
  return { owner, sequence };
}

const NO_ADVANCE = () => 0;

/** A report-bound state cell: each change of value (by identity) advances the sequence in
 * the same synchronous step as the write, so the new value and its number are committed
 * together. Writing the same value again advances nothing. The third element writes a
 * value derived from an already-sequenced change (React's render-phase adjustment) without
 * advancing; nothing else may bypass the sequencing. */
export function useSequencedState<T>(
  initial: T | (() => T),
  advance: () => number = NO_ADVANCE
): [T, Dispatch<SetStateAction<T>>, (derived: T) => void] {
  const [value, setRaw] = useState(initial);
  const latest = useRef(value);
  const advanceRef = useRef(advance);
  advanceRef.current = advance;
  const [setters] = useState(() => ({
    sequenced: (action: SetStateAction<T>) => {
      const next = typeof action === "function" ? (action as (previous: T) => T)(latest.current) : action;
      if (Object.is(next, latest.current)) return;
      latest.current = next;
      advanceRef.current();
      setRaw(next);
    },
    derived: (next: T) => {
      latest.current = next;
      setRaw(next);
    }
  }));
  return [value, setters.sequenced, setters.derived];
}
