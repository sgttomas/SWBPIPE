/** Observes an element's size and calls back when it changes; returns the function that stops it. */
export type ObserveSize = (element: Element, callback: () => void) => () => void;

type Watch = { element: Element; callback: () => void; stop: (() => void) | null };

/**
 * The size watches of the view's furniture (the bar and its groups, the canvas's controls, legend
 * and status line), all served by the view resource's one ResizeObserver, so the view creates no
 * other observer and its resource ledger counts every observer it has. A watch made before the
 * resource exists (the bar mounts before the resource's effect runs) is attached when it does; the
 * watches are detached before the resource is disposed.
 */
export class ViewportSizeWatch {
  private readonly watches = new Set<Watch>();
  private observe: ObserveSize | null = null;

  /** Serves every watch, present and future, through `observe`. */
  attach(observe: ObserveSize): void {
    this.detach();
    this.observe = observe;
    for (const watch of this.watches) watch.stop = observe(watch.element, watch.callback);
  }

  /** Stops serving the watches; they are kept for the next attach. */
  detach(): void {
    for (const watch of this.watches) {
      watch.stop?.();
      watch.stop = null;
    }
    this.observe = null;
  }

  readonly watch: ObserveSize = (element, callback) => {
    const watch: Watch = { element, callback, stop: this.observe?.(element, callback) ?? null };
    this.watches.add(watch);
    return () => {
      if (!this.watches.delete(watch)) return;
      watch.stop?.();
      watch.stop = null;
    };
  };
}
