import type { ComponentBodyRecord, MechanicsResult } from "../../types";

/** W-15 slice d (DEL-03-05 Design `rigid-component-stiffness-scaling.md` §5): the
 * component bodies of a result. A member solved with a rigid-family component's DEC-045
 * stiffness scale carries `component_body` on each of its stress rows; those rows are
 * stresses of the mapped pipe section as a reference section, not of the body. The
 * producer's headline never names a body; a view that includes a body row or member
 * for another reason shows this identification. */
export function componentBodies(result: MechanicsResult | null | undefined): Map<string, ComponentBodyRecord> {
  const bodies = new Map<string, ComponentBodyRecord>();
  for (const row of result?.results ?? []) {
    if (row.component_body && !bodies.has(row.entity_ref)) bodies.set(row.entity_ref, row.component_body);
  }
  return bodies;
}

/** The identification text of one body member. */
export function componentBodyNotice(pipeId: string, record: ComponentBodyRecord): string {
  return `${pipeId} is the body of component ${record.component_ref} (stiffness scale ${record.stiffness_scale}); its stress rows are those of the mapped pipe section as a reference section, not a stress of the component body`;
}

/** The identification of each listed result row or member that is a component body, in order. */
export function componentBodyNotices(result: MechanicsResult | null | undefined, ids: { resultIds?: string[]; pipeIds?: string[] }): string[] {
  const bodies = componentBodies(result);
  const pipes = new Set<string>(ids.pipeIds ?? []);
  for (const id of ids.resultIds ?? []) {
    const row = result?.results.find((candidate) => candidate.id === id);
    if (row?.component_body) pipes.add(row.entity_ref);
  }
  return Array.from(pipes).filter((pipe) => bodies.has(pipe)).sort().map((pipe) => componentBodyNotice(pipe, bodies.get(pipe)!));
}
