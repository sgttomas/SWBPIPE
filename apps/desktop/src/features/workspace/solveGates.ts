// A run token is acquired synchronously before the first await. This closes the
// native-menu double-dispatch window and also lets model/open invalidation make
// every callback from an older solve inert, even when an adapter reuses a job
// identifier.
export class SolveRunGenerationGate {
  private generation = 0;
  private active: number | null = null;
  private cancelRequested = false;

  tryStart(): number | null {
    if (this.active !== null) return null;
    this.active = ++this.generation;
    this.cancelRequested = false;
    return this.active;
  }

  current(): number | null {
    return this.active;
  }

  isCurrent(token: number): boolean {
    return this.active === token;
  }

  requestCancellation(token: number): boolean {
    if (!this.isCurrent(token)) return false;
    this.cancelRequested = true;
    return true;
  }

  isCancellationRequested(token: number): boolean {
    return this.isCurrent(token) && this.cancelRequested;
  }

  invalidate(): void {
    this.generation += 1;
    this.active = null;
    this.cancelRequested = false;
  }

  finish(token: number): boolean {
    if (!this.isCurrent(token)) return false;
    this.active = null;
    this.cancelRequested = false;
    return true;
  }
}

export class RuleRevisionGenerationGate {
  private generation = 0;

  start(): number {
    return ++this.generation;
  }

  isCurrent(token: number): boolean {
    return token === this.generation;
  }

  invalidate(): void {
    this.generation += 1;
  }
}

export function commitModelAfterSolveInvalidation(
  gate: SolveRunGenerationGate,
  revision: { current: number },
  commit: () => void
): void {
  revision.current += 1;
  gate.invalidate();
  commit();
}
