from pathlib import Path
out=Path(__file__).resolve().parent
p=out/'witness.mjs';s=p.read_text().replace("const {chromium}=await import('/Users/ryan/.cache/codex-runtimes/codex-primary-runtime/dependencies/node/node_modules/playwright/index.mjs');", "const modulePath=process.env.PROTOTYPE_PLAYWRIGHT;\nif(!modulePath)throw new Error('PROTOTYPE_PLAYWRIGHT exact module path is required and must be recorded in invocation evidence');\nconst {chromium}=await import(modulePath);")
s=s.replace("for (const [width,height]", "matrix: for (const [width,height]")
s=s.replace("page.locator('.warning').isVisible()", "page.locator('.task-rail .warning').isVisible()")
s=s.replace("if(failure)break;", "if(failure)break matrix;")
s=s.replace("for(const row of ['R1','R2','R3'])", "const normative=await page.evaluate(()=>CAPABILITIES.filter(c=>typeof c.row==='number').map(c=>({row:c.row,label:c.label})));\n  for(const {row,label} of normative){await page.locator('#tool-search').fill(label);assert(await page.locator(`[data-capability=\"${row}\"]`).isVisible(),'normative row '+row+' discoverable');await click(`[data-capability=\"${row}\"]`);}\n  for(const row of ['R1','R2','R3'])")
p.write_text(s)
p=out/'freeze_candidate.py';s=p.read_text().replace('import json,hashlib,datetime','import json,hashlib,datetime,sys').replace("(out/'_run_records/CANDIDATE_FREEZE_V1.json').write_text", "version=sys.argv[1] if len(sys.argv)>1 else 'V2'\nfreeze=out/f'_run_records/CANDIDATE_FREEZE_{version}.json'\nif freeze.exists():raise RuntimeError('Freeze already exists; use a new immutable version')\nfreeze.write_text")
s=s.replace("f.write_text(json.dumps(m,indent=2)+'\\n')", "m['context']=list({r['origin']:r for r in m['context']}.values())\nf.write_text(json.dumps(m,indent=2)+'\\n')")
p.write_text(s)
