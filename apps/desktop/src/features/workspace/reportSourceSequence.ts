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
// incarnation (`activate_report_source` starts one and retires the one before). Editing
// never waits for that: an advance is sent and its failure only leaves report output
// unregistered. Output preparation (a render, an HTML save, a print) waits for the
// number it was prepared from to be registered, re-checks that it is still the latest,
// and then names it in its request; the native side refuses a request whose number is
// older (retired) or not yet registered. A change made here is ordered with native
// publication only once registered: until then an earlier request may still be admitted
// and published, and the report output status says so.
import { useEffect, useRef, useState, type Dispatch, type SetStateAction } from "react";
import { invoke } from "@tauri-apps/api/core";

export type ReportSourceMark = { incarnation: number; sequence: number };

/** The native side of the sequence; `null` outside the desktop application. */
export type ReportSourceTransport = {
  activate(): Promise<ReportSourceMark>;
  advance(mark: ReportSourceMark): Promise<ReportSourceMark>;
};

/** Output on a source the native side has not confirmed is withheld, by name. */
export class ReportSourceUnavailableError extends Error {
  constructor(readonly code: "REPORT-SOURCE-STALE-VIEW" | "REPORT-SOURCE-UNREGISTERED", message: string) {
    super(`${code}: ${message}`);
    this.name = "ReportSourceUnavailableError";
  }
}

const INCARNATION_RETIRED = "REPORT-SOURCE-INCARNATION-RETIRED";

function errorCode(error: unknown): string | null {
  return typeof error === "object" && error !== null && typeof (error as { code?: unknown }).code === "string"
    ? (error as { code: string }).code
    : null;
}

/** The workspace's report source sequence (see the module comment). */
export class ReportSourceSequence {
  private allocated = 0;
  private incarnation: number | null = null;
  private confirmed = 0;
  private activation: Promise<number> | null = null;
  private readonly listeners = new Set<(sequence: number) => void>();

  constructor(private readonly transport: ReportSourceTransport | null) {}

  /** The latest number allocated in this workspace session. */
  get sequence(): number {
    return this.allocated;
  }

  /** The highest number the native side has confirmed for the current incarnation. */
  get registered(): ReportSourceMark | null {
    return this.incarnation === null ? null : { incarnation: this.incarnation, sequence: this.confirmed };
  }

  subscribe(listener: (sequence: number) => void): () => void {
    this.listeners.add(listener);
    return () => this.listeners.delete(listener);
  }

  /** A report-bound value changed: allocate the next number now, then send it. Never
   * waits and never throws; a failed send leaves output unregistered, not editing. */
  readonly advance = (): number => {
    this.allocated += 1;
    const sequence = this.allocated;
    for (const listener of this.listeners) listener(sequence);
    void this.send(sequence).catch(() => undefined);
    return sequence;
  };

  /** Starts the incarnation (an effect sends it; it allocates nothing). */
  start(): void {
    void this.send().catch(() => undefined);
  }

  private ensureActive(): Promise<number> {
    if (!this.transport) return Promise.resolve(0);
    if (this.incarnation !== null) return Promise.resolve(this.incarnation);
    if (!this.activation) {
      const transport = this.transport;
      this.activation = transport.activate().then(
        (mark) => {
          this.activation = null;
          // A newer activation of this session supersedes this one's numbers.
          if (this.incarnation === null || mark.incarnation > this.incarnation) {
            this.incarnation = mark.incarnation;
            this.confirmed = mark.sequence;
          }
          return this.incarnation;
        },
        (error: unknown) => {
          this.activation = null;
          throw error;
        }
      );
    }
    return this.activation;
  }

  /** Sends the latest number; an acknowledgement registers only within its incarnation
   * and only upwards (a delayed, older or other-incarnation reply changes nothing). */
  private async send(sequence?: number): Promise<void> {
    if (!this.transport) return;
    const incarnation = await this.ensureActive();
    // An advance sends its own number; a resynchronization sends the latest.
    if (sequence === undefined || this.incarnation !== incarnation || sequence < this.confirmed) sequence = this.allocated;
    try {
      const acknowledged = await this.transport.advance({ incarnation, sequence });
      if (acknowledged.incarnation === this.incarnation && acknowledged.sequence > this.confirmed) this.confirmed = acknowledged.sequence;
    } catch (error) {
      if (errorCode(error) === INCARNATION_RETIRED && this.incarnation === incarnation) {
        // Another activation retired ours: the next send starts a new one.
        this.incarnation = null;
        this.confirmed = 0;
      }
      throw error;
    }
  }

  private confirms(sequence: number): boolean {
    return this.incarnation !== null && this.confirmed >= sequence;
  }

  /** Output preparation: `viewSequence` is the number the preparing view was rendered
   * with. Resolves with the mark to name in the request once the native side has
   * registered it and it is still the latest; refuses a stale view, and refuses when
   * registration cannot be confirmed (after one resynchronization). */
  async registeredFor(viewSequence: number): Promise<ReportSourceMark> {
    const stale = () => new ReportSourceUnavailableError("REPORT-SOURCE-STALE-VIEW", "the workspace changed after this report was prepared; prepare it again.");
    if (viewSequence !== this.allocated) throw stale();
    if (!this.transport) return { incarnation: 0, sequence: viewSequence };
    let lastError: unknown = null;
    for (let attempt = 0; attempt < 3 && !this.confirms(viewSequence); attempt += 1) {
      try {
        await this.send();
      } catch (error) {
        lastError = error;
      }
    }
    if (viewSequence !== this.allocated) throw stale();
    if (!this.confirms(viewSequence) || this.incarnation === null) {
      const detail = lastError instanceof Error ? lastError.message : lastError === null ? "" : String(errorCode(lastError) ?? lastError);
      throw new ReportSourceUnavailableError(
        "REPORT-SOURCE-UNREGISTERED",
        `the desktop has not confirmed the current workspace, so report output is withheld; editing is unaffected.${detail ? ` (${detail})` : ""}`
      );
    }
    return { incarnation: this.incarnation, sequence: viewSequence };
  }
}

export function nativeReportSourceTransport(): ReportSourceTransport | null {
  if (typeof window === "undefined" || !("__TAURI_INTERNALS__" in window)) return null;
  return {
    activate: () => invoke<ReportSourceMark>("activate_report_source"),
    advance: (mark) => invoke<ReportSourceMark>("advance_report_source", { incarnation: mark.incarnation, sequence: mark.sequence })
  };
}

/** The session's sequence: a stable owner and its latest number as React state. */
export function useReportSourceSequence(transport?: ReportSourceTransport | null) {
  const [owner] = useState(() => new ReportSourceSequence(transport === undefined ? nativeReportSourceTransport() : transport));
  const [sequence, setSequence] = useState(owner.sequence);
  const subscribed = useRef(false);
  if (!subscribed.current) {
    subscribed.current = true;
    owner.subscribe(setSequence);
  }
  useEffect(() => {
    owner.start();
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
