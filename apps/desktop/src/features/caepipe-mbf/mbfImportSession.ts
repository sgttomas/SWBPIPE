// A request's one retirement survives either ordering of Adopt's completion and the retiring React generation's
// passive cleanup. Only bounded ownership metadata is retained here; no review, declaration, document or DOM node.
export function mbfImportRetirementOwner(send: () => void) {
  let retiredThrough = -1;
  let current: { generation: number; retire: () => void } | null = null;

  return {
    openRequest(generation: number): { retire: () => void } {
      if (current && current.generation > generation) return { retire: () => {} };
      // Issue any older request's retirement before a newer request can send its first native start.
      current?.retire();
      let retired = false;
      const retire = () => {
        if (retired) return;
        retired = true;
        retiredThrough = Math.max(retiredThrough, generation);
        send();
      };
      current = { generation, retire };
      return { retire };
    },
    /** False when the cleanup belongs to an older generation and must not close the newer one's dialog. */
    sessionEnded(generation: number): boolean {
      if (current && current.generation > generation) {
        retiredThrough = Math.max(retiredThrough, generation);
        return false;
      }
      if (current) {
        current.retire();
        current = null;
      } else if (generation > retiredThrough) {
        send();
      }
      retiredThrough = Math.max(retiredThrough, generation);
      return true;
    }
  };
}
