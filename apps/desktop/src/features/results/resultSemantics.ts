import contract from '../../../../../fixtures/results/semantic_contract_v0_2.json';
import precisionContract from '../../../../../fixtures/results/semantic_contract_v0_3_precision_1.json';
import { sourceContract } from './numericalResultQuality';
import type { MechanicsResult } from '../../types';
export type SourceRow = MechanicsResult['results'][number];
export type SemanticSignature = (typeof contract.rows)[number];
export { contract as resultSemanticContract };
export function semanticContractForSource(source?: MechanicsResult) {
  if (!source || sourceContract(source) === 'legacy') return contract;
  if (sourceContract(source) === 'precision') return precisionContract;
  throw new Error('SOURCE_SEMANTIC_CONTRACT_UNSUPPORTED');
}
export function resultSemantics(row: SourceRow, source?: MechanicsResult): SemanticSignature | null {
  const known = semanticContractForSource(source).rows.filter(s => s.kind === row.kind);
  if (!known.length) return null;
  const units = known.filter(s => s.unit === row.unit);
  if (!units.length) throw new Error(`SOURCE_UNIT_CONTRADICTION: ${row.kind}`);
  const observedComponent = row.metadata?.component;
  const component = typeof observedComponent === "string" && observedComponent.length > 0 ? observedComponent : undefined;
  const exact = units.find(s => s.component === null || s.component === component);
  if (exact) return exact;
  if (component) throw new Error(`SOURCE_COMPONENT_CONTRADICTION: ${row.kind}`);
  return units[0]; // incomplete disclosure only; never a guessed physical variant
}
export function semanticFamily(row: SourceRow, source?: MechanicsResult): string {
  try { const s = resultSemantics(row, source); return s?.category === 'physical_quantity' ? s.family ?? 'other' : 'other'; }
  catch { return 'other'; }
}
export function semanticDimension(row: SourceRow, source?: MechanicsResult): string | null {
  try { return resultSemantics(row, source)?.derivative_target_dimension ?? null; }
  catch { return null; }
}
export function semanticCategory(row: SourceRow, source?: MechanicsResult): string {
  try { return resultSemantics(row, source)?.category ?? 'unknown'; } catch { return 'integrity_failure'; }
}
export function completeSourceMetadata(row: SourceRow): boolean {
  return ['component','coordinate_system','location','basis','sign_convention'].every(k => {
    const v = (row.metadata as unknown as Record<string, unknown> | null)?.[k];
    return typeof v === 'string' && v.length > 0;
  });
}
export function canonicalResultMetadata(row: SourceRow, source?: MechanicsResult): Record<string,string> | null {
  if (!completeSourceMetadata(row)) return null;
  const projection: Record<string,string> = {};
  for (const [key, rule] of Object.entries(semanticContractForSource(source).canonical_metadata_vocabulary)) {
    const value = (row.metadata as unknown as Record<string,string>)[key];
    const allowed = (rule as {enum?: string[]}).enum;
    if (allowed && !allowed.includes(value)) return null;
    projection[key] = value;
  }
  return projection;
}
