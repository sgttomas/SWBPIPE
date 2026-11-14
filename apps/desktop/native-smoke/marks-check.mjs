// The three marks of an agent operating the App on the host's screen (Agent 0 and the Inter-Project): the
// supplier's window service under launchd or as a process, the Codex app-server helper on stdio, a running App v4
// candidate bundle. `run.mjs` and `mbf-journey.mjs` check them directly before each app start and refuse the launch
// when any is present or a check cannot be made. `report-standing-journey.mjs` carries the same check inline.
// macOS only, like the launches it guards.
import { spawnSync } from "node:child_process";

const runCommand = (command, commandArgs) => spawnSync(command, commandArgs, { encoding: "utf8", stdio: ["ignore", "pipe", "pipe"], timeout: 15000 });

/** The marks as found now, with any check that could not be made. `run` answers each command as `spawnSync` does;
 * it is an internal seam for `judged-launch.test.mjs`, which gives it controlled responses. */
export function marksCheck(run = runCommand) {
  const launchd = run("/bin/launchctl", ["list"]);
  const sky = run("/usr/bin/pgrep", ["-f", "SkyComputerUseService"]);
  const codex = run("/usr/bin/pgrep", ["-f", "codex app-server --listen stdio://"]);
  const apps = run("/usr/bin/lsappinfo", ["list"]);
  const failed = [["launchctl list", launchd, [0]], ["pgrep SkyComputerUseService", sky, [0, 1]], ["pgrep codex app-server", codex, [0, 1]], ["lsappinfo list", apps, [0]]]
    .filter(([, result, ok]) => result.error || !ok.includes(result.status)).map(([name, result]) => `${name}: ${result.error?.message ?? `exit ${result.status}`}`);
  return {
    checked_at: new Date().toISOString(),
    sky_computer_use_service_launchd: /SkyComputerUseService/i.test(launchd.stdout ?? ""),
    sky_computer_use_service_process: sky.status === 0,
    codex_app_server_stdio: codex.status === 0,
    app_v4_candidate_bundle: /dev\.chirality\.app-v4\.candidate/i.test(apps.stdout ?? ""),
    check_failures: failed
  };
}

/** True when the launch must be refused: a mark is present or a check could not be made. */
export function marksRefuse(marks) {
  return marks.check_failures.length > 0 || marks.sky_computer_use_service_launchd || marks.sky_computer_use_service_process || marks.codex_app_server_stdio || marks.app_v4_candidate_bundle;
}
