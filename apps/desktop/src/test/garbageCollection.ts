// Test support: a full garbage collection on demand, for checks that an object is no longer
// held. Node exposes `gc` only under `--expose-gc`; setting that flag at run time and reading
// `gc` from a new context gives the same full collection without changing how the test runner
// starts its workers. The collection runs after the current task ends, because a WeakRef keeps
// its target alive until the end of the job that created or last dereferenced it.
import v8 from "node:v8";
import vm from "node:vm";

let collect: (() => void) | null = null;

function fullCollection(): () => void {
  if (collect) return collect;
  const exposed = (globalThis as { gc?: () => void }).gc;
  if (typeof exposed === "function") collect = exposed;
  else {
    v8.setFlagsFromString("--expose_gc");
    collect = vm.runInNewContext("gc") as () => void;
  }
  return collect;
}

const nextTask = () => new Promise<void>((resolve) => setTimeout(resolve, 0));

/** Collects garbage, fully, over a few task turns. */
export async function collectGarbage(): Promise<void> {
  const gc = fullCollection();
  for (let turn = 0; turn < 3; turn += 1) {
    await nextTask();
    gc();
  }
  await nextTask();
}
