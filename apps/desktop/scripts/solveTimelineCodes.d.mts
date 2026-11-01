// Types of solveTimelineCodes.mjs, for solveTimelineCodes.test.ts.
export type SolveTimelineCodeTable = { solvePath: string[]; reader: string[]; local: string[] };
export const TABLE_FILE: string;
export const LOCAL_REASONS: string[];
export function extractSolveTimelineCodes(): SolveTimelineCodeTable;
export function renderTable(table: SolveTimelineCodeTable): string;
