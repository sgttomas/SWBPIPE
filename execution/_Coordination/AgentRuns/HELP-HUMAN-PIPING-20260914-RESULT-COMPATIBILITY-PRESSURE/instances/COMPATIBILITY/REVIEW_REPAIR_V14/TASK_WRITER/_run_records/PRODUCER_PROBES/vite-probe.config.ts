import path from "node:path";
import { fileURLToPath } from "node:url";

import desktopConfig from "../../../../../../../../../../apps/desktop/vite.config";

const here = path.dirname(fileURLToPath(import.meta.url));

export default {
  ...desktopConfig,
  test: {
    ...(desktopConfig as any).test,
    include: [path.join(here, "producer-probes.test.ts")],
  },
};
