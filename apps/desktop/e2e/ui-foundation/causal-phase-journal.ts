import { writeFile } from "node:fs/promises";
import path from "node:path";

export const CAUSAL_PHASES = ["initialized", "prepared", "point-stopped", "orbit-stopped", "trace-finalized", "derived", "failed"] as const;
export type CausalPhase = typeof CAUSAL_PHASES[number];

const ordinal = new Map<CausalPhase, string>(CAUSAL_PHASES.map((phase, index) => [phase, String(index).padStart(2, "0")]));

export class CausalPhaseJournal {
  readonly #directory: string;
  readonly #runToken: string;
  readonly #written = new Set<CausalPhase>();

  constructor(directory: string, runToken: string) {
    this.#directory = directory;
    this.#runToken = runToken;
  }

  async write(phase: CausalPhase, evidence: unknown): Promise<string> {
    if (this.#written.has(phase)) throw new Error(`causal phase journal is immutable and ${phase} is already written`);
    const file = path.join(this.#directory, `${ordinal.get(phase)}-${phase}.json`);
    const record = { schema: "openpipestress.ui-foundation.causal-phase-journal/v1", runToken: this.#runToken,
      phase, writtenAt: new Date().toISOString(), evidence };
    await writeFile(file, `${JSON.stringify(record, null, 2)}\n`, { encoding: "utf8", flag: "wx" });
    this.#written.add(phase);
    return file;
  }
}
