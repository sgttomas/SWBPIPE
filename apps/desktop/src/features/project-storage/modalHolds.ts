// Modal ownership for the project prompts (W-12 S1; Codex r4). The replace
// prompt and the unsaved-changes prompt both hold the window: while either is
// open, menu commands and shell accelerators wait (`flag`). Each prompt takes
// a hold and releases it when it closes; the flag stays set until the last
// hold is released, so a prompt that closes first never unblocks input while
// another is still open. A release is idempotent.
import type { MutableRefObject } from "react";

export type ModalHolds = {
  /** Takes a hold; the returned function releases it (once). */
  hold: () => () => void;
  /** Holds currently taken. */
  count: () => number;
};

export function createModalHolds(flag: MutableRefObject<boolean>): ModalHolds {
  let holds = 0;
  return {
    hold() {
      holds += 1;
      flag.current = true;
      let released = false;
      return () => {
        if (released) return;
        released = true;
        holds -= 1;
        flag.current = holds > 0;
      };
    },
    count: () => holds
  };
}
