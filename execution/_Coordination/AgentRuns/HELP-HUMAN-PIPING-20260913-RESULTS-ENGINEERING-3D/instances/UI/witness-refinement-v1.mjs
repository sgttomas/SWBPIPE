/* Bounded manager label/layout self-check; not independent acceptance. */
import fs from 'node:fs';
import path from 'node:path';
import {createHash} from 'node:crypto';
import {fileURLToPath,pathToFileURL} from 'node:url';
const root=path.dirname(fileURLToPath(import.meta.url));
const output=path.join(root,'_run_records/refinement-witness-v1');
fs.mkdirSync(output,{recursive:true});
const hash=p=>createHash('sha256').update(fs.readFileSync(p)).digest('hex');
const assets=()=>Object.fromEntries(['index.html','workspace.js','workspace.css','capabilities.js'].map(n=>[n,hash(path.join(root,'prototype-v2',n))]));
const modulePath=process.env.PROTOTYPE_PLAYWRIGHT, executablePath=process.env.PROTOTYPE_CHROMIUM;
if(!modulePath||!executablePath)throw Error('Exact approved module and browser paths required');
const pre=assets(),{chromium}=await import(modulePath);
const browser=await chromium.launch({headless:true,executablePath});
const identity={node:{path:process.execPath,version:process.version,sha256:hash(process.execPath)},playwright:{path:modulePath,version:JSON.parse(fs.readFileSync(path.join(path.dirname(modulePath),'package.json'))).version,sha256:hash(modulePath)},browser:{path:executablePath,version:browser.version(),sha256:hash(executablePath)},qualification:'Package expects headless-shell1234; approved installed shell1223 is used. Outcomes qualify only this bounded simulation self-check.'};
fs.writeFileSync(path.join(output,'IDENTITY.json'),JSON.stringify(identity,null,2));
const reports=[],assert=(ok,msg)=>{if(!ok)throw Error(msg);};
try {
matrix:for(const [width,height] of [[1024,768],[1280,800],[1440,920]])for(const density of ['comfortable','compact']){
 const context=await browser.newContext({viewport:{width,height}}),page=await context.newPage(),errors=[],network=[];
 page.on('pageerror',e=>errors.push(e.message));page.on('console',m=>{if(m.type()==='error')errors.push(m.text());});page.on('request',r=>{if(!r.url().startsWith('file:'))network.push(r.url());});
 let failure=null;
 try{
  await page.goto(pathToFileURL(path.join(root,'prototype-v2/index.html')).href);
  await page.locator('#density').selectOption(density);
  assert(await page.locator('.simulation').isVisible(),'Simulation label retained');
  const capabilities=await page.evaluate(()=>CAPABILITIES.map(c=>({row:c.row,label:c.label})));
  for(const {row,label} of capabilities){
   await page.locator('#tool-search').fill(label);const command=page.locator(`[data-capability="${row}"]`);await command.click();
   assert((await command.textContent()).trim()===(typeof row==='number'?label:label+'ROADMAP'),'Plain label with ROADMAP classification only: '+row);
  }
  await page.locator('#tool-search').fill('');await page.locator('[data-group="Build"]').click();await page.locator('[data-capability="1"]').click();
  await page.locator('#add-draft').click();await page.locator('#validate-simulation').click();
  assert((await page.evaluate(()=>prototypeInspect())).frozen.submission.operations.length===2,'Route still freezes atomic two-member payload');
  await page.screenshot({path:path.join(output,`${width}x${height}-${density}-route.png`)});
  await page.locator('#cancel-draft').click();await page.locator('[data-group="Properties"]').click();await page.locator('[data-property]').click();
  const rail=await page.locator('#editor').innerText();
  assert(rail.includes('Pipe properties')&&rail.includes('Wall thickness')&&!rail.includes('set_field')&&!rail.includes('wall_thickness')&&!rail.includes('Typed pipe target'),'Plain task property labels');
  await page.locator('#pipe').selectOption('P5');await page.locator('#value').fill('7.1');await page.locator('#provenance').fill('Invented refinement witness property correction');
  await page.locator('#add-draft').click();const frozen=await page.evaluate(()=>prototypeInspect());
  assert(frozen.frozen.submission.intent.change.change_kind==='set_field'&&frozen.frozen.submission.intent.change.before==='6.02'&&frozen.frozen.submission.intent.change.after==='7.1','Underlying exact property intent retained');
  await page.locator('#tab-state').click();assert((await page.locator('#full-intent-json').innerText()).includes('set_field'),'Internal mapping inspectable in State / intent');
  await page.locator('#tab-review').click();await page.locator('#validate-simulation').click();await page.locator('#apply').click();
  await page.locator('#tab-results').click();await page.locator('#solve').click();assert((await page.evaluate(()=>prototypeInspect())).result.status==='Current','Simulated Current still reachable');
  await page.screenshot({path:path.join(output,`${width}x${height}-${density}-current.png`)});
  const geometry=await page.evaluate(()=>{const b=id=>{const e=document.getElementById(id),r=e.getBoundingClientRect();return{x:r.x,y:r.y,width:r.width,height:r.height,scrollHeight:e.scrollHeight,clientHeight:e.clientHeight};};return{body:{width:document.body.scrollWidth,height:document.body.scrollHeight},canvas:b('canvas'),editor:b('editor'),apply:b('apply'),cancel:b('cancel-draft')};});
  assert(geometry.body.width<=width&&geometry.body.height<=height,'No body overflow');
  for(const key of ['apply','cancel']){const r=geometry[key];assert(r.x>=0&&r.y>=0&&r.x+r.width<=width&&r.y+r.height<=height,'Action remains unclipped: '+key);}
  assert(errors.length===0&&network.length===0,'No console/page errors or unexpected network');
  reports.push({width,height,density,pass:true,geometry,errors,network});
 }catch(e){failure=String(e.stack||e);reports.push({width,height,density,pass:false,failure,errors,network});await page.screenshot({path:path.join(output,`${width}x${height}-${density}-failure.png`)});}
 finally{await context.close();}
 fs.writeFileSync(path.join(output,'REPORT.json'),JSON.stringify(reports,null,2));console.log(JSON.stringify(reports.at(-1)));if(failure)break matrix;
}
}finally{await browser.close();fs.writeFileSync(path.join(output,'ASSET_BINDINGS.json'),JSON.stringify({pre,post:assets(),unchanged:JSON.stringify(pre)===JSON.stringify(assets())},null,2));}
if(reports.length!==6||reports.some(r=>!r.pass))process.exitCode=1;
console.log('Manager refinement self-check only; all engineering outcomes simulated.');
