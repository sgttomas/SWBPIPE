import contract from "../../../../../fixtures/results/semantic_contract_v0_2.json";
import {resultDigest} from "../result-export/resultExportAdapter";
import {canonicalJsonString} from "../../services/hashService";
import { describe,it,expect } from 'vitest';
import fixtures from '../../../../../fixtures/results/invented/result_export_v0_2.json';
import type { MechanicsResult } from '../../types';
import { resultSemantics,semanticFamily,canonicalResultMetadata } from './resultSemantics';
describe('source-qualified current semantics',()=>{
 it('covers all60 independent signatures without guessed governing ratios',()=>{
  expect(fixtures.fixtures).toHaveLength(60);
  for(const f of fixtures.fixtures){const row=f.input_row as unknown as MechanicsResult['results'][number],s=resultSemantics(row)!;
   expect(s.source_physical_semantic_dimension).toBe(f.expected.source_physical_semantic_dimension);expect(s.derivative_target_dimension).toBe(f.expected.derivative_target_dimension);expect(s.governing_ratio_eligible).toBe(false);
   if(s.category!=='physical_quantity')expect(semanticFamily(row)).toBe('other');
   if(f.expected.canonical_metadata)expect(canonicalResultMetadata(row)).toEqual(f.expected.canonical_metadata);
  }
 });
 it('does not infer unknown force/ratio from id or arbitrary unit',()=>{expect(semanticFamily({id:'force-ratio',kind:'unknown',unit:'N',value:1,entity_ref:'node'})).toBe('other');});
});

it("matches independently authored qualified UTF8/hash vectors in existing wasm authority",async()=>{for(const v of contract.hash_vectors){expect(await canonicalJsonString(v.input)).toBe(v.expected_canonical_json);expect(await resultDigest(v.input)).toBe(v.expected_sha256);}});
